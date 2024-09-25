pub mod category;
pub mod itemtracker;
pub mod stats;

#[get("/")]
pub fn index() -> &'static str {
    "I am here! HEALTHY"
}
