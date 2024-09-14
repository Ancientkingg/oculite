use crate::persist::{self, category::Category, Db};
use rocket::{http::Status, serde::{Serialize, json::Json}};
use rocket_db_pools::Connection;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum CategoryResponse {
    Data(Vec<Category>),
    Error(&'static str),
}

#[get("/")]
pub async fn all(db: Connection<Db>) -> (Status, Json<CategoryResponse>) {
    match persist::category::all(db).await {
        Ok(categories) => {
            info!("Categories: {:?}", categories);
            (Status::Ok, Json(CategoryResponse::Data(categories)))
        }
        Err(x) => {
            error!("Failed to get categories: {}", x);
            (Status::InternalServerError, Json(CategoryResponse::Error("Failed to get categories")))
        }
    }
}

#[post("/", data = "<category>")]
pub async fn add(db: Connection<Db>, category: Json<Category>) -> (Status, &'static str) {
    match persist::category::add(db, category.clone().into_inner()).await {
        Ok(_) => {
            info!("Category '{}' added @ {}", category.name, category.url);
            (Status::Created, "Category added")
        }
        Err(x) => {
            match &x {
                sqlx::Error::Database(db_err) => {
                    if db_err.is_unique_violation() {
                        error!("Category '{}' already exists: {}", category.name, x);
                        return (Status::Conflict, "Category already exists");
                    }
                }
                _ => {}
            }
            error!("Failed to add category '{}': {}", category.name, x);
            (Status::InternalServerError, "Failed to add category")
        }
    }
}
