use rocket::serde::Serialize;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;
use sqlx::Acquire;

use crate::persist::{self, Db};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Serialize)]
pub enum StatsResponse {
    Data {
        total: i64,
        rising: i64,
        falling: i64,
        stale: i64,
    },
    Error(&'static str),
}

#[get("/")]
pub async fn get_stats(mut db: Connection<Db>) -> Result<(Status, Json<StatsResponse>)> {
    let total = persist::stats::get_total_trackers(db.acquire().await?).await;
    let rising = persist::stats::get_rising_trackers(db.acquire().await?).await;
    let falling = persist::stats::get_falling_trackers(db.acquire().await?).await;
    let stale = persist::stats::get_stale_trackers(db.acquire().await?).await;

    match (total, rising, falling, stale) {
        (Ok(total), Ok(rising), Ok(falling), Ok(stale)) => Ok((
            Status::Ok,
            Json(StatsResponse::Data {
                total,
                rising,
                falling,
                stale,
            }),
        )),
        _ => Ok((
            Status::InternalServerError,
            Json(StatsResponse::Error("Failed to get stats")),
        )),
    }
}

#[derive(Serialize)]
pub enum NotificationsResponse {
    Data(Vec<String>),
    Error(&'static str),
}

#[get("/notifications")]
pub async fn get_notifications(db: Connection<Db>) -> (Status, Json<NotificationsResponse>) {
    todo!("Implement get_notifications")
}

#[derive(Serialize)]
pub enum FavoriteResponse {
    Data(Vec<String>),
    Error(&'static str),
}

#[get("/favorite")]
pub async fn get_favorite(db: Connection<Db>) -> (Status, Json<FavoriteResponse>) {
    todo!("Implement get_favorite")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_stats, get_notifications, get_favorite]
}
