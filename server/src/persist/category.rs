use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::Db;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub name: String,
    pub url: String,
}

pub async fn all(mut db: Connection<Db>) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&mut **db)
        .await
}

pub async fn add(mut db: Connection<Db>, category: Category) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "INSERT INTO categories (name, url) VALUES ($1, $2) RETURNING *",
        category.name,
        category.url
    )
    .fetch_one(&mut **db)
    .await
}
