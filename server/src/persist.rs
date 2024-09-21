use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::PgPool);

pub mod category;
pub mod itemtracker;
pub mod stats;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("persist/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
