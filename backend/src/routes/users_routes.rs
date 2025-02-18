// src/routes/user_routes.rs
use rocket::serde::json::Json;
use rocket::State;
use crate::db::connection::Pool;
use crate::models::user::{User,NewUser};
use crate::schema::users::dsl::*;
use crate::schema::users;
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

#[post("/users", format = "json", data = "<new_user>")]
pub async fn add_user(conn: &State<Pool>, new_user: Json<NewUser>) -> Json<User> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Insert the new user into the database
    let inserted_user: User = diesel::insert_into(users::table)
        .values((
            users::name.eq(new_user.name.clone()),
            users::email.eq(new_user.email.clone()),
        ))
        .get_result(&conn)
        .expect("Error inserting new user");

    // Return the newly inserted user as a JSON response
    Json(inserted_user)
}
