use rocket::{Build, Rocket};
use sqlx::PgPool;

use crate::services;

pub mod category;

pub async fn register_monitor(rocket: &Rocket<Build>) {
    let url = rocket
        .figment()
        .extract_inner::<String>("databases.db.url")
        .expect("Database URL not found");
    let db = PgPool::connect(&url).await.expect("Database not attached");

    rocket::tokio::spawn(async move {
        let mut interval =
            rocket::tokio::time::interval(rocket::tokio::time::Duration::from_secs(10));
        loop {
            info!("Refreshing categories");
            services::category::refresh_all(&db)
                .await
                .expect("Failed to refresh categories");
            interval.tick().await;
        }
    });
}