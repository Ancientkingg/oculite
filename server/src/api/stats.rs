use rocket::serde::Serialize;
use rocket::tokio::join;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;

use crate::persist::itemtracker::ItemTrackerId;
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
pub async fn get_stats(db: &Db) -> Result<(Status, Json<StatsResponse>)> {
    let total = persist::stats::get_total_trackers(db);
    let rising = persist::stats::get_rising_trackers(db);
    let falling = persist::stats::get_falling_trackers(db);
    let stale = persist::stats::get_stale_trackers(db);

    let (total, rising, falling, stale) = join!(total, rising, falling, stale);

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
pub async fn get_notifications(db: &Db) -> (Status, Json<NotificationsResponse>) {
    todo!("Implement get_notifications")
}

#[derive(Serialize)]
pub enum FavoriteResponse {
    Data(Vec<ItemTrackerId>),
    Error(&'static str),
}

#[get("/favorite")]
pub async fn get_favorite(db: &Db) -> (Status, Json<FavoriteResponse>) {
    match persist::stats::get_ids_of_favorite_trackers(&db).await {
        Ok(ids) => (Status::Ok, Json(FavoriteResponse::Data(ids))),
        Err(_) => (Status::InternalServerError, Json(FavoriteResponse::Error("Failed to get favorite trackers"))),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_stats, get_notifications, get_favorite]
}
