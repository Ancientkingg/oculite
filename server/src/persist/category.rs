use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::{Pool, Postgres};
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use super::Db;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
    pub config: Option<String>,
    pub url: String,
}

impl Hash for Category {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.category_id.hash(state);
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.category_id == other.category_id
    }
}

impl Eq for Category {}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.category_id, self.category_name)
    }
}

pub async fn all(db: &Pool<Postgres>) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(db)
        .await
}

#[allow(dead_code)]
pub async fn get_by_name(mut db: Connection<Db>, name: String) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE category_name = $1",
        name
    )
    .fetch_one(&mut **db)
    .await
}

pub async fn get_by_id(mut db: Connection<Db>, category_id: i32) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE category_id = $1",
        category_id
    )
    .fetch_one(&mut **db)
    .await
}

pub async fn insert(mut db: Connection<Db>, category: Category) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "INSERT INTO categories (category_id, category_name, config, url) VALUES ($1, $2, $3, $4) RETURNING *",
        category.category_id,
        category.category_name,
        category.config,
        category.url,
    )
    .fetch_one(&mut **db)
    .await
}

pub async fn update(
    mut db: Connection<Db>,
    category_id: i32,
    category_name: Option<String>,
    config: Option<String>,
    url: Option<String>,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as!(
        Category,
        "UPDATE categories SET category_name = $2, config = $3, url = $4 WHERE category_id = $1 RETURNING *",
        category_id,
        category_name,
        config,
        url,
    )
    .fetch_one(&mut **db)
    .await
}
