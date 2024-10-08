use chrono::Utc;
use sqlx::PgPool;

use crate::persist;

pub async fn insert_login(db: &PgPool, ip: &str) -> Result<(), sqlx::Error> {
    let message = std::format!(
        "A login has been detected on [{}] from {}",
        Utc::now()
            .with_timezone(&chrono_tz::Europe::Amsterdam)
            .format("%Y-%m-%d %H:%M:%S"),
        ip
    );
    persist::notification::insert(db, &message, "pi-sign-in", "yellow", Utc::now()).await?;

    Ok(())
}

pub async fn insert_price_change(db: &PgPool, it_name: &str, price_change_percent: f64, new_price: f64, currency: &str) -> Result<(), sqlx::Error> {
    let message = std::format!(
        "{} has significantly changed price ({:.0}%). Now {} {:.2} [{}]",
        it_name,
        price_change_percent,
        currency,
        new_price,
        Utc::now()
            .with_timezone(&chrono_tz::Europe::Amsterdam)
            .format("%Y-%m-%d %H:%M:%S"),
    );
    persist::notification::insert(db, &message, "pi-dollar", "green", Utc::now()).await?;

    Ok(())
}

pub async fn insert_category_not_responding(db: &PgPool, category_name: &str) -> Result<(), sqlx::Error> {
    let message = std::format!(
        "Category {} is not responding [{}]",
        category_name,
        Utc::now()
            .with_timezone(&chrono_tz::Europe::Amsterdam)
            .format("%Y-%m-%d %H:%M:%S"),
    );
    persist::notification::insert(db, &message, "pi-exclamation-triangle", "red", Utc::now()).await?;

    Ok(())
}

pub async fn insert_category_added(db: &PgPool, category_name: &str) -> Result<(), sqlx::Error> {
    let message = std::format!(
        "A new category has been added on [{}]: {}",
        Utc::now()
            .with_timezone(&chrono_tz::Europe::Amsterdam)
            .format("%Y-%m-%d %H:%M:%S"),
        category_name
    );
    persist::notification::insert(db, &message, "pi-plus", "blue", Utc::now()).await?;

    Ok(())
}