use std::fmt::Display;

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::PgPool;

use super::{
    category::{self, Category},
    Db,
};

pub type ItemTrackerId = i32;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct ItemTracker {
    pub category: Option<Category>,

    pub id: ItemTrackerId,
    pub name: String,
    pub currency: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
    pub favorite: Option<bool>,

    pub price_data: Option<Vec<PriceData>>,
}

impl Display for ItemTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ItemTracker {{ id: {}, name: {}, category: {} }}",
            self.id,
            self.name,
            self.category.clone()
                .unwrap_or(Category {
                    category_id: -1,
                    category_name: String::from("None"),
                    url: String::from("None"),
                    config: None
                })
                .to_string()
        )
    }
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct PriceData {
    pub price: f64,
    pub date: DateTime<Utc>,
}

impl ItemTracker {
    pub fn get_id(&self) -> ItemTrackerId {
        self.id
    }
}

pub async fn all_ids(db: &PgPool) -> Result<Vec<(ItemTrackerId, Category)>, sqlx::Error> {
    let item_tracker_ids = sqlx::query!(
        "SELECT id, category_id, url, category_name, config FROM item_trackers JOIN categories USING (category_id)"
    )
    .map(|row| {
        let category = category::Category {
            category_id: row.category_id,
            url: row.url,
            category_name: row.category_name,
            config: row.config,
        };

        (row.id, category)
    })
    .fetch_all(db)
    .await;

    return item_tracker_ids;
}

#[allow(dead_code)]
pub async fn get_by_id(
    mut db: Connection<Db>,
    id: ItemTrackerId,
) -> Result<ItemTracker, sqlx::Error> {
    let item_tracker = sqlx::query!(
        "SELECT category_id, category_name, config, url, id, name, currency, icon, link, favorite, json_agg(pd.* ORDER BY pd.date DESC) as price_data FROM item_trackers it JOIN categories c USING (category_id) LEFT JOIN price_data pd ON it.id=pd.item_tracker WHERE id = $1 GROUP BY it.id, c.category_id, c.url, c.category_name, c.config",
        id
    )
    .map(|row| {
        let category = Category {
            category_id: row.category_id,
            category_name: row.category_name,
            config: row.config,
            url: row.url,
        };

        let price_data = match row.price_data {
            Some(data) => {
                let arr = data.as_array().unwrap();
                arr.into_iter().map(|x| {
                    let element = x.as_object().unwrap();
                    let price = element.get("price").unwrap().as_f64().unwrap();
                    let date = element.get("date").unwrap().as_str().unwrap();
                    PriceData {
                        price,
                        date: DateTime::parse_from_rfc3339(date).unwrap().with_timezone(&Utc),
                    }
                }).collect()
            },
            None => Vec::new()
        };

        ItemTracker {
            category: Some(category),
            id: row.id,
            name: row.name,
            currency: row.currency,
            icon: row.icon,
            link: row.link,
            favorite: row.favorite,
            price_data: Some(price_data),
        }
    })
    .fetch_one(&mut **db)
    .await;

    return item_tracker;
}

pub async fn insert(db: &PgPool, it: ItemTracker) -> Result<ItemTrackerId, sqlx::Error> {
    let category_id = it.category.clone().unwrap().category_id;
    sqlx::query_scalar!(
        "INSERT INTO item_trackers (id, name, currency, icon, link, favorite, category_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        it.get_id(),
        it.name,
        it.currency,
        it.icon,
        it.link,
        it.favorite,
        category_id
    )
    .fetch_one(db)
    .await
}

pub async fn update(db: &PgPool, it: &ItemTracker) -> Result<ItemTrackerId, sqlx::Error> {
    sqlx::query_scalar!(
        "UPDATE item_trackers SET name = $2, currency = $3, icon = $4, link = $5, favorite = $6 WHERE id = $1 RETURNING id",
        it.get_id(),
        it.name,
        it.currency,
        it.icon,
        it.link,
        it.favorite
    )
    .fetch_one(db)
    .await
}

pub async fn get_ids_by_category(
    mut db: Connection<Db>,
    category_id: i32,
) -> Result<Vec<ItemTrackerId>, sqlx::Error> {
    let item_tracker_ids = sqlx::query_scalar!(
        "SELECT id FROM item_trackers JOIN categories USING (category_id) WHERE category_id = $1",
        category_id
    )
    .fetch_all(&mut **db)
    .await;

    return item_tracker_ids;
}

pub async fn set_favorite(
    mut db: Connection<Db>,
    id: ItemTrackerId,
    favorite: bool,
) -> Result<ItemTrackerId, sqlx::Error> {
    let item_tracker_id = sqlx::query_scalar!(
        "UPDATE item_trackers SET favorite = $2 WHERE id = $1 RETURNING id",
        id,
        favorite
    )
    .fetch_one(&mut **db)
    .await;

    return item_tracker_id;
}

pub async fn add_price_data(
    db: &PgPool,
    id: ItemTrackerId,
    price_data: f64,
    date: DateTime<Utc>,
) -> Result<ItemTrackerId, sqlx::Error> {
    let item_tracker_id = sqlx::query_scalar!(
        "INSERT INTO price_data (item_tracker, price, date) SELECT it, p, d FROM (SELECT CAST($1 as integer) as it, CAST($2 as double precision) as p, CAST($3 as timestamp with time zone) as d) t WHERE EXISTS (SELECT id FROM item_trackers WHERE id = it) RETURNING item_tracker",
        id,
        price_data,
        date
    ).fetch_one(db).await;

    return item_tracker_id;
}

pub async fn delete(db: &PgPool, id: ItemTrackerId) -> Result<ItemTrackerId, sqlx::Error> {
    let item_tracker_id =
        sqlx::query_scalar!("DELETE FROM item_trackers WHERE id = $1 RETURNING id", id)
            .fetch_one(db)
            .await;

    return item_tracker_id;
}
