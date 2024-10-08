use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::Db;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Notification {
    pub id: i32,
    pub message: String,
    pub icon: String,
    pub color: String,
    pub date: DateTime<Utc>,
}

pub async fn get_all(db: &Db) -> Result<Vec<Notification>, sqlx::Error> {
    let rows = sqlx::query_as!(Notification, "SELECT * FROM notifications")
        .fetch_all(&db.0)
        .await?;

    Ok(rows)
}

pub async fn insert(
    db: &PgPool,
    message: &str,
    icon: &str,
    color: &str,
    date: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO notifications (id, message, icon, color, date) VALUES ($1, $2, $3, $4, $5)",
        rand::random::<u16>() as i32,
        message,
        icon,
        color,
        date
    )
    .execute(db)
    .await?;

    Ok(())
}
