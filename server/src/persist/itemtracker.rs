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

    pub price_data: Option<Vec<f64>>,
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
        "SELECT category_id, category_name, config, url, id, name, currency, icon, link, favorite, json_agg(pd.*) as price_data FROM item_trackers it JOIN categories c USING (category_id) LEFT JOIN price_data pd ON it.id=pd.item_tracker WHERE id = $1 GROUP BY it.id, c.category_id, c.url, c.category_name, c.config",
        id
    )
    .map(|row| {
        let category = Category {
            category_id: row.category_id,
            category_name: row.category_name,
            config: row.config,
            url: row.url,
        };

        ItemTracker {
            category: Some(category),
            id: row.id,
            name: row.name,
            currency: row.currency,
            icon: row.icon,
            link: row.link,
            favorite: row.favorite,
            price_data: Some(row.price_data.unwrap().as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect()),
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
    let item_tracker_id = sqlx::query_scalar!(
        "DELETE FROM item_trackers WHERE id = $1 RETURNING id",
        id
    )
    .fetch_one(db)
    .await;

    return item_tracker_id;
}