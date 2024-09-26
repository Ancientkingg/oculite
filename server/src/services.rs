use chrono::{DateTime, Utc};
use rocket::{
    fairing::{Fairing, Kind},
    tokio::sync::Mutex,
    Build, Rocket,
};
use sqlx::PgPool;
use std::{collections::HashMap, net::IpAddr, sync::Arc, time::Duration};

use crate::{persist::Db, services};

pub mod category;
pub mod notification;

const DURATION: Duration = rocket::tokio::time::Duration::from_secs(3 * 60 * 60);
const DELTA: f64 = 2.0; // in percent

pub async fn register_monitor(rocket: &Rocket<Build>) {
    let url = rocket
        .figment()
        .extract_inner::<String>("databases.db.url")
        .expect("Database URL not found");
    let db = PgPool::connect(&url).await.expect("Database not attached");

    rocket::tokio::spawn(async move {
        let mut interval = rocket::tokio::time::interval(DURATION);
        loop {
            info!("Refreshing categories");

            services::category::refresh_all(&db)
                .await
                .expect("Failed to refresh categories");

            info!("Finished refreshing categories");

            services::category::analyze_all(&db, DELTA)
                .await
                .expect("Failed to analyze item trackers");
            interval.tick().await;
        }
    });
}

#[derive(Default, Clone)]
pub struct Logins {
    logins: Arc<Mutex<HashMap<IpAddr, DateTime<Utc>>>>,
}

#[rocket::async_trait]
impl Fairing for Logins {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Login Tracker",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        let mut logins = self.logins.lock().await;

        for (ip, time) in logins.clone().into_iter() {
            if Utc::now().signed_duration_since(time) > chrono::Duration::hours(1) {
                logins.remove(&ip);
            }
        }

        let db = request.guard::<&Db>().await.unwrap();

        let ip = request.client_ip().unwrap();

        if let std::collections::hash_map::Entry::Vacant(e) = logins.entry(ip) {
            notification::insert_login(db, &ip.to_string()).await.expect("Failed to generate login notification");
            e.insert(Utc::now());
        }
    }
}
