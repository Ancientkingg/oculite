use rocket::{fairing::AdHoc, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod api;
mod persist;
mod services;

#[launch]
async fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let rocket = rocket::build()
        .attach(persist::Db::init())
        .attach(AdHoc::try_on_ignite(
            "DB Migrations",
            persist::run_migrations,
        ))
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![api::index])
        .mount("/category", api::category::routes())
        .mount("/itemtracker", api::itemtracker::routes())
        .mount("/stats", api::stats::routes());

    services::register_monitor(&rocket).await;

    rocket
}
