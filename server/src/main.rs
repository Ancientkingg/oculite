use rocket::{fairing::AdHoc, fs::FileServer, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod api;
mod persist;
mod services;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] conn_str: String) -> shuttle_rocket::ShuttleRocket {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let db_figment = rocket::Config::figment().merge(("databases.db.url", conn_str));

    let rocket = rocket::custom(db_figment)
        .attach(persist::Db::init())
        .attach(services::Logins::default())
        .attach(AdHoc::try_on_ignite(
            "DB Migrations",
            persist::run_migrations,
        ))
        .attach(cors.to_cors().expect("CORS failed to initialize"))
        .mount("/", FileServer::from("public"))
        .mount("/api", routes![api::index])
        .mount("/api/category", api::category::routes())
        .mount("/api/it", api::itemtracker::routes())
        .mount("/api/stats", api::stats::routes());

    services::register_monitor(&rocket).await;

    Ok(rocket.into())
}
