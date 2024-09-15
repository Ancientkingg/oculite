use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use std::hash::{Hash, Hasher};

use super::Db;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub category: String,
    pub url: String,
}

impl Hash for Category {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category.hash(state);
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category
    }
}

impl Eq for Category {}

pub async fn all(mut db: Connection<Db>) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&mut **db)
        .await
}

pub async fn get_by_name(mut db: Connection<Db>, name: String) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE category = $1", name)
        .fetch_one(&mut **db)
        .await
}

pub async fn add(mut db: Connection<Db>, category: Category) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "INSERT INTO categories (category, url) VALUES ($1, $2) RETURNING *",
        category.category,
        category.url
    )
    .fetch_one(&mut **db)
    .await
}
