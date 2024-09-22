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
pub async fn all(db: &Db) -> (Status, Json<ItemTrackerResponse>) {
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
pub async fn get(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {
    match persist::itemtracker::get_by_id(db, id).await {
        Ok(it) => {
            info!("Item tracker found: {:?}", it);
            (Status::Ok, Json(SingleItemTrackerResponse::Data(it.get_id())))
        }
        Err(x) => {
            error!("Failed to get item tracker: {}", x);
            (
                Status::NotFound,
                Json(SingleItemTrackerResponse::Error(
                    "Failed to get item tracker",
                )),
            )
        }
    }
}

#[put("/<id>/favorite")]
pub async fn favorite(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {
    match persist::itemtracker::set_favorite(db, id, true).await {
        Ok(it) => {
            info!("Item tracker favorited: {:?}", it);
            (Status::Ok, Json(SingleItemTrackerResponse::Data(it)))
        }
        Err(x) => {
            error!("Failed to favorite item tracker: {}", x);
            (
                Status::InternalServerError,
                Json(SingleItemTrackerResponse::Error(
                    "Failed to favorite item tracker",
                )),
            )
        }
    }
}

#[put("/<id>/unfavorite")]
pub async fn unfavorite(db: Connection<Db>, id: i32) -> (Status, Json<SingleItemTrackerResponse>) {
    match persist::itemtracker::set_favorite(db, id, false).await {
        Ok(it) => {
            info!("Item tracker unfavorited: {:?}", it);
            (Status::Ok, Json(SingleItemTrackerResponse::Data(it)))
        }
        Err(x) => {
            error!("Failed to unfavorite item tracker: {}", x);
            (
                Status::InternalServerError,
                Json(SingleItemTrackerResponse::Error(
                    "Failed to unfavorite item tracker",
                )),
            )
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all, get, favorite, unfavorite]
}
