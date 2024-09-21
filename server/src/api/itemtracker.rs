use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use rocket_db_pools::Connection;

use crate::{
    persist::{self, itemtracker::ItemTrackerId, Db},
    services::category::filter_inactive_categories,
};

#[derive(Serialize)]
pub enum ItemTrackerResponse {
    Data(Vec<ItemTrackerId>),
    Error(&'static str),
}

#[derive(Serialize)]
pub enum SingleItemTrackerResponse {
    Data(ItemTrackerId),
    Error(&'static str),
}

#[get("/")]
pub async fn all(db: Connection<Db>) -> (Status, Json<ItemTrackerResponse>) {
    match persist::itemtracker::all_ids(db).await {
        Ok(it_ids) => {
            info!("All item trackers: {:?}", it_ids);

            let active_it_ids: Vec<ItemTrackerId> = filter_inactive_categories(it_ids).await;

            (Status::Ok, Json(ItemTrackerResponse::Data(active_it_ids)))
        }
        Err(x) => {
            error!("Failed to get item tracker ids: {}", x);
            (
                Status::InternalServerError,
                Json(ItemTrackerResponse::Error("Failed to get categories")),
            )
        }
    }
}

#[get("/<id>")]
pub async fn get(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {}

#[put("/<id>/favorite")]
pub async fn favorite(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {}

#[put("/<id>/unfavorite")]
pub async fn unfavorite(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {}
