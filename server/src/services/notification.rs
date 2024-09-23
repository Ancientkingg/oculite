// Login notification
// Price change notification
// Category not responding notification
// Category added notification
// Inactive/old trackers deletion notification

use chrono::Utc;
use sqlx::PgPool;

use crate::persist;

pub async fn insert_login(db: &PgPool, ip: &str) -> Result<(), sqlx::Error> {
    let message = std::format!(
        "A login has been detected on {} from {}",
        Utc::now()
            .with_timezone(&chrono_tz::Europe::Amsterdam)
            .format("%Y-%m-%d %H:%M:%S"),
        ip
    );
    persist::notification::insert(db, &message, "pi-sign-in", "green", Utc::now()).await?;

    Ok(())
}

pub async fn insert_price_change(db: &PgPool) -> Result<(), sqlx::Error> {
    todo!("Insert price change notification")
}

pub async fn insert_category_not_responding(db: &PgPool) -> Result<(), sqlx::Error> {
    todo!("Insert category not responding notification")
}

pub async fn insert_category_added(db: &PgPool) -> Result<(), sqlx::Error> {
    todo!("Insert category added notification")
}

pub async fn insert_inactive_trackers_deletion(db: &PgPool) -> Result<(), sqlx::Error> {
    todo!("Insert inactive/old trackers deletion notification")
}
