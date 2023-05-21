#[get("/ping")]
pub fn index() -> &'static str {
    "UP"
}