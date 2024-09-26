use crate::{
    persist::{self, category::Category, itemtracker::ItemTrackerId, Db},
    services::notification,
};
use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;

#[derive(Serialize)]
pub struct CategoryOpaque {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub config: Option<String>,
}

impl From<CategoryRequest> for Category {
    fn from(req: CategoryRequest) -> Self {
        Category {
            category_id: rand::random::<u16>() as i32,
            category_name: req.name.unwrap_or("".to_string()),
            config: req.config,
            url: req.url.unwrap_or("".to_string()),
        }
    }
}

#[derive(Serialize)]
pub enum CategoryResponse {
    Data(Vec<CategoryOpaque>),
    Error(&'static str),
}

#[derive(Serialize)]
pub enum SingleCategoryResponse {
    Data(CategoryRequest),
    Error(&'static str),
}

#[get("/")]
pub async fn all(db: &Db) -> (Status, Json<CategoryResponse>) {
    match persist::category::all(db).await {
        Ok(categories) => {
            info!("Categories: {:?}", categories);
            (
                Status::Ok,
                Json(CategoryResponse::Data(
                    categories
                        .into_iter()
                        .map(|c| CategoryOpaque {
                            id: c.category_id,
                            name: c.category_name,
                        })
                        .collect(),
                )),
            )
        }
        Err(x) => {
            error!("Failed to get categories: {}", x);
            (
                Status::InternalServerError,
                Json(CategoryResponse::Error("Failed to get categories")),
            )
        }
    }
}

#[post("/", data = "<category>")]
pub async fn add(db: &Db, category: Json<CategoryRequest>) -> (Status, &'static str) {
    if category.0.name.is_none() || category.0.url.is_none() {
        return (Status::BadRequest, "Category name and url are required");
    }

    match persist::category::insert(db, category.clone().into_inner().into()).await {
        Ok(category) => {
            info!("Category {} added @ {}", category, category.url);
            let _ = notification::insert_category_added(db, &category.category_name).await;
            (Status::Created, "Category added")
        }
        Err(x) => {
            match &x {
                sqlx::Error::Database(db_err) => {
                    if db_err.is_unique_violation() {
                        error!(
                            "Category {} already exists: {}",
                            Into::<Category>::into(category.0),
                            x
                        );
                        return (Status::Conflict, "Category already exists");
                    }
                }
                _ => {}
            }
            error!(
                "Failed to add category {}: {}",
                Into::<Category>::into(category.0),
                x
            );
            (Status::InternalServerError, "Failed to add category")
        }
    }
}

#[get("/<category_id>")]
pub async fn get(db: Connection<Db>, category_id: i32) -> (Status, Json<SingleCategoryResponse>) {
    match persist::category::get_by_id(db, category_id).await {
        Ok(category) => {
            info!("Category {} found @ {}", category, category.url);
            (
                Status::Ok,
                Json(SingleCategoryResponse::Data(CategoryRequest {
                    name: Some(category.category_name),
                    config: category.config,
                    url: Some(category.url),
                })),
            )
        }
        Err(x) => {
            error!("Failed to get category: {}", x);
            (
                Status::NotFound,
                Json(SingleCategoryResponse::Error("Failed to get category")),
            )
        }
    }
}

#[put("/<category_id>", data = "<category>")]
pub async fn update(
    db: Connection<Db>,
    category_id: i32,
    category: Json<CategoryRequest>,
) -> (Status, &'static str) {
    let c = category.clone().into_inner();
    match persist::category::update(db, category_id, c.name, c.config, c.url).await {
        Ok(category) => {
            info!("Category {} updated @ {}", category, category.url);
            (Status::Ok, "Category updated")
        }
        Err(x) => match x {
            sqlx::error::Error::RowNotFound => {
                error!("Category {} not found: {}", category_id, x);
                (Status::NotFound, "Category not found")
            }
            _ => {
                error!(
                    "Failed to update category {}: {}",
                    Into::<Category>::into(category.0),
                    x
                );
                (Status::InternalServerError, "Failed to update category")
            }
        },
    }
}

#[derive(Serialize)]
pub enum ItemTrackerPerCategoryResponse {
    Data(Vec<ItemTrackerId>),
    Error(&'static str),
}

#[get("/<category_id>/it")]
pub async fn get_itemtrackers(
    db: &Db,
    category_id: i32,
) -> (Status, Json<ItemTrackerPerCategoryResponse>) {
    match persist::itemtracker::get_ids_by_category(db, category_id).await {
        Ok(it) => {
            info!("Item trackers found for category {}", category_id);
            (Status::Ok, Json(ItemTrackerPerCategoryResponse::Data(it)))
        }
        Err(x) => {
            error!("Failed to get item trackers: {}", x);
            (
                Status::NotFound,
                Json(ItemTrackerPerCategoryResponse::Error(
                    "Failed to get category",
                )),
            )
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all, add, get, update, get_itemtrackers,]
}
