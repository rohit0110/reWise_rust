// src/routes/user_routes.rs
use rocket::serde::json::Json;
use rocket::State;
use crate::db::db::Pool;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

#[get("/users")]
pub async fn get_users(conn: &State<Pool>) -> Json<Vec<User>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading users
    let results = users
        .load::<User>(&conn)
        .expect("Error loading users");

    Json(results)
}
