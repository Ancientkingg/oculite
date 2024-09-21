use rocket::serde::Serialize;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;

use crate::persist::Db;

#[derive(Serialize)]
pub enum StatsResponse {
    Data { total: i32, unread: i32, read: i32 },
    Error(&'static str),
}

#[get("/")]
pub async fn get_stats(db: Connection<Db>) -> (Status, Json<StatsResponse>) {
    todo!("Implement get_stats")
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
