use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::{
    category::{self, Category},
    Db,
};

pub type ItemTrackerId = i32;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct ItemTracker {
    pub category: Category,

    pub id: ItemTrackerId,
    pub name: String,
    pub currency: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
    pub favorite: Option<bool>,
}

pub async fn all_ids(
    mut db: Connection<Db>,
) -> Result<Vec<(ItemTrackerId, Category)>, sqlx::Error> {
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
    .fetch_all(&mut **db)
    .await;

    return item_tracker_ids;
}

#[allow(dead_code)]
pub async fn get_by_id(
    mut db: Connection<Db>,
    id: ItemTrackerId,
) -> Result<ItemTracker, sqlx::Error> {
    let item_tracker = sqlx::query!(
        "SELECT * FROM item_trackers JOIN categories USING (category_id) WHERE id = $1",
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
            category,
            id: row.id,
            name: row.name,
            currency: row.currency,
            icon: row.icon,
            link: row.link,
            favorite: row.favorite,
        }
    })
    .fetch_one(&mut **db)
    .await;

    return item_tracker;
}

pub async fn add(mut db: Connection<Db>, category: Category) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "INSERT INTO categories (category_id, category_name, config, url) VALUES ($1, $2, $3, $4) RETURNING *",
        category.category_id,
        category.category_name,
        category.config,
        category.url
    )
    .fetch_one(&mut **db)
    .await
}

pub async fn get_ids_by_category(
    mut db: Connection<Db>,
    category_id: i32,
) -> Result<Vec<ItemTrackerId>, sqlx::Error> {
    let item_tracker_ids = sqlx::query!(
        "SELECT id FROM item_trackers JOIN categories USING (category_id) WHERE category_id = $1",
        category_id
    )
    .map(|row| row.id)
    .fetch_all(&mut **db)
    .await;

    return item_tracker_ids;
}

pub async fn set_favorite(
    mut db: Connection<Db>,
    id: ItemTrackerId,
    favorite: bool,
) -> Result<ItemTrackerId, sqlx::Error> {
    let item_tracker = sqlx::query!(
        "UPDATE item_trackers SET favorite = $2 WHERE id = $1 RETURNING id",
        id,
        favorite
    )
    .map(|row| row.id)
    .fetch_one(&mut **db)
    .await;

    return item_tracker;
}
