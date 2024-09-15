pub mod category;
pub mod itemtracker;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
