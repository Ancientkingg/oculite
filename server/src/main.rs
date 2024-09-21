use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod api;
mod persist;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(persist::Db::init())
        .attach(AdHoc::try_on_ignite(
            "DB Migrations",
            persist::run_migrations,
        ))
        .mount("/", routes![api::index])
        .mount("/category", api::category::routes())
        .mount("/itemtracker", api::itemtracker::routes())
        .mount("/stats", api::stats::routes())
}
