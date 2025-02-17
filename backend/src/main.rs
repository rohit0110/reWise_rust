#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use rocket::State;
use rocket::serde::json::Json;
use crate::db::establish_connection;
use crate::models::User;
use crate::schema::users::dsl::*;

mod db;
mod models;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(establish_connection())
        .mount("/", routes![get_users])
}

#[get("/users")]
async fn get_users(conn: &State<db::Pool>) -> Json<Vec<User>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading users
    let results = users
        .load::<User>(&conn) 
        .expect("Error loading users");

    Json(results)
}
