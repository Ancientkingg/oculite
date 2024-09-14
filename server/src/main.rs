use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod api;
mod persist;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(persist::Db::init())
        .attach(AdHoc::try_on_ignite(
            "DB Migrations",
            persist::run_migrations,
        ))
        .mount("/", routes![api::index])
        .mount(
            "/category",
            routes![api::category::all, api::category::add],
        )
}
