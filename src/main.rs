mod routes;
mod models;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {    
    rocket::build()
        .mount("/api", routes![
            routes::user::index,
            routes::health::index,
            routes::auth::signIn,
        ])
}
