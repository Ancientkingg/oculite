use std::collections::{HashMap, HashSet};

use chrono::Utc;
use rocket::futures::future::join_all;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    persist::{
        self,
        category::Category,
        itemtracker::{ItemTracker, PriceData},
    },
    services::notification,
};

pub async fn filter_inactive_categories(it: Vec<(i32, Category)>) -> Vec<i32> {
    let mut cached_categories = HashMap::<Category, bool>::new();

    let mut active_it = Vec::with_capacity(it.len());

    for (id, category) in it {
        if let Some(category_status) = cached_categories.get(&category) {
            if category_status.to_owned() {
                active_it.push(id);
            }
        } else {
            let resp = reqwest::get(&category.url).await;

            if let Ok(resp) = resp {
                let status = resp.status();

                if status.is_success() {
                    cached_categories.insert(category.clone(), true);
                    active_it.push(id);
                } else {
                    error!("Failed to get category: {}", category);
                    cached_categories.insert(category.clone(), false);
                }
            } else {
                error!("Failed to get category: {}", category);
            }
        }
    }

    active_it
}

#[derive(thiserror::Error, Debug)]
pub enum RefreshError {
    #[error("SQL error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub async fn refresh_all(db: &PgPool) -> Result<(), RefreshError> {
    // get all categories first
    let categories = persist::category::all(db).await?;

    let mut handles = Vec::with_capacity(categories.len());

    for category in categories.clone() {
        let it = fetch_category(category);
        handles.push(it);
    }

    let results = join_all(handles).await;

    let mut current_ids = HashSet::<i32>::new();
    let mut active_ids = HashSet::<i32>::new();

    // Go through the results
    for (idx, result) in results.into_iter().enumerate() {
        let it = match result {
            Ok(it) => it,
            Err(e) => {
                let category = categories.get(idx).expect("Unable to get corresponding category to item tracker result. Categories don't match!");
                notification::insert_category_not_responding(db, &category.category_name).await?;
                error!(
                    "SKIPPING CATEGORY | Failed to fetch category {}: {}",
                    category, e
                );
                continue;
            }
        };

        let category_id = categories.get(idx).expect("Unable to get corresponding category to item tracker result. Categories don't match!").category_id;
        let current_category_it_ids =
            persist::itemtracker::get_ids_by_category(db, category_id).await?;

        current_ids.extend(current_category_it_ids.clone());

        for item in it {
            let id = item.get_id();
            active_ids.insert(id);

            let price_data = item.price_data.clone();

            if current_category_it_ids.contains(&id) {
                // Update the item tracker
                persist::itemtracker::update(db, &item).await?;
            } else {
                // Insert the item tracker
                persist::itemtracker::insert(db, item).await?;
            }

            // Update price data
            match price_data {
                Some(price_data) => {
                    for price in price_data {
                        persist::itemtracker::add_price_data(db, id, price.price, price.date)
                            .await?;
                    }
                }
                None => {
                    error!("No price data for item tracker: {}", id);
                }
            }
        }
    }

    // Delete inactive item trackers
    for id in current_ids.difference(&active_ids) {
        info!("Deleting inactive item tracker: {}", id);
        persist::itemtracker::delete(db, *id).await?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct ItemTrackerResponse {
    data: Vec<ItemTrackerRaw>,
}

#[derive(serde::Deserialize)]
struct ItemTrackerRaw {
    id: i32,
    name: String,
    currency: String,
    icon: String,
    link: String,
    price_data: f64,
}

pub async fn fetch_category(category: Category) -> Result<Vec<ItemTracker>, reqwest::Error> {
    let client = reqwest::Client::new();

    info!("Fetching category: {:?}", category);

    let resp = match &category.config {
        Some(config) => {
            client
                .post(&category.url)
                .json(&json!({"config": config}))
                .send()
                .await?
        }
        None => {
            client
                .post(&category.url)
                .json(&json!({"config": null}))
                .send()
                .await?
        }
    };

    let it_resp = resp.json::<ItemTrackerResponse>().await?;

    Ok(it_resp
        .data
        .into_iter()
        .map(|x| ItemTracker {
            category: Some(category.clone()),
            id: x.id ^ category.category_id,
            name: x.name,
            currency: Some(x.currency),
            icon: Some(x.icon),
            link: Some(x.link),
            favorite: Some(false),
            price_data: Some(vec![PriceData {
                price: x.price_data,
                date: Utc::now(),
            }]),
        })
        .collect())
}

pub async fn analyze_all(db: &PgPool, significance_delta: f64) -> Result<(), sqlx::Error> {
    let its = persist::itemtracker::all(db).await?;

    for it in its {
        let price_data = it.price_data.expect("No price data for item tracker");

        let default_price = PriceData {
            price: 0.0,
            date: Utc::now(),
        };
        let latest_price = price_data.get(0).unwrap_or(&default_price);
        let previous_price = price_data.get(1).unwrap_or(&default_price);

        let price_change_percent =
            ((latest_price.price - previous_price.price) / previous_price.price) * 100.0;

        if price_change_percent.abs() >= significance_delta {
            let currency = it.currency.as_ref().expect("No currency for item tracker");
            let it_name = &it.name;

            notification::insert_price_change(
                db,
                it_name,
                price_change_percent,
                latest_price.price,
                currency,
            )
            .await?;
        }
    }

    Ok(())
}
