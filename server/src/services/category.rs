use std::collections::{HashMap, HashSet};

use chrono::Utc;
use rocket::futures::future::join_all;
use sqlx::PgPool;

use crate::persist::{self, category::Category, itemtracker::ItemTracker};

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

    for category in categories {
        let it = fetch_category(category);
        handles.push(it);
    }

    let results = join_all(handles).await;

    // Get all existing item trackers
    let current_it = persist::itemtracker::all_ids(db).await?;
    let current_it_ids = current_it
        .into_iter()
        .map(|(id, _)| id)
        .collect::<HashSet<_>>();
    let mut active_ids = HashSet::<i32>::new();

    // Go through the results
    for result in results {
        let it = result?;

        for item in it {
            let id = item.get_id();
            active_ids.insert(id);

            let price_data = item.price_data.clone();

            if current_it_ids.contains(&id) {
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
                        let date = Utc::now();
                        persist::itemtracker::add_price_data(db, id, price, date).await?;
                    }
                }
                None => {
                    error!("No price data for item tracker: {}", id);
                }
            }
        }
    }

    // Delete inactive item trackers
    for id in current_it_ids.difference(&active_ids) {
        persist::itemtracker::delete(db, *id).await?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct ItemTrackerResponse {
    data: Vec<ItemTracker>,
}

pub async fn fetch_category(category: Category) -> Result<Vec<ItemTracker>, reqwest::Error> {
    let resp = reqwest::get(&category.url).await?;
    let it_resp = resp.json::<ItemTrackerResponse>().await?;

    Ok(it_resp
        .data
        .into_iter()
        .map(|mut x| {
            x.category = Some(category.clone());
            x.id = x.get_id() ^ category.category_id;

            x
        })
        .collect())
}
