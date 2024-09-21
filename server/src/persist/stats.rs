use rocket_db_pools::Connection;
use sqlx::PgConnection;

use super::Db;

pub async fn get_total_trackers(db: &mut PgConnection) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_scalar!("SELECT COUNT(*) FROM item_trackers")
        .fetch_one(db)  // Mutable dereference
        .await?;

    match row {
        Some(count) => Ok(count),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_rising_trackers(db: &mut PgConnection) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_scalar!(
        "WITH latest_prices AS (
            SELECT
                item_tracker,
                price AS latest_price,
                date AS latest_date,
                LAG(price) OVER (PARTITION BY item_tracker ORDER BY date DESC) AS second_latest_price
            FROM
                price_data
        )
        SELECT
            COUNT(item_tracker)
        FROM
            latest_prices
        WHERE
            latest_price > second_latest_price"
    )
    .fetch_one(db)
    .await?;

    match row {
        Some(count) => Ok(count),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_falling_trackers(db: &mut PgConnection) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_scalar!(
        "WITH latest_prices AS (
            SELECT
                item_tracker,
                price AS latest_price,
                date AS latest_date,
                LAG(price) OVER (PARTITION BY item_tracker ORDER BY date DESC) AS second_latest_price
            FROM
                price_data
        )
        SELECT
            COUNT(item_tracker)
        FROM
            latest_prices
        WHERE
            latest_price < second_latest_price"
    )
    .fetch_one(db)
    .await?;

    match row {
        Some(count) => Ok(count),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_stale_trackers(db: &mut PgConnection) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_scalar!(
        "WITH latest_prices AS (
            SELECT
                item_tracker,
                price AS latest_price,
                date AS latest_date,
                LAG(price) OVER (PARTITION BY item_tracker ORDER BY date DESC) AS second_latest_price
            FROM
                price_data
        )
        SELECT
            COUNT(item_tracker)
        FROM
            latest_prices
        WHERE
            ABS(latest_price - second_latest_price) <= (latest_price * 0.02);"
    )
    .fetch_one(db)
    .await?;

    match row {
        Some(count) => Ok(count),
        None => Err(sqlx::Error::RowNotFound),
    }
}