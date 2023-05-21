use rocket::serde::{json::Json};
use crate::models::user::User;

#[get("/user")]
pub fn index() -> Json<User> {
    let user = User {
        name: "Jon Snow".to_string(),
        age: 21,
        alive: true,
    };
    return Json(user)
}