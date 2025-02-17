// src/routes/user_routes.rs
use rocket::serde::json::Json;
use rocket::State;
use crate::db::db::Pool;
use crate::models::note::Note;
use crate::schema::notes::dsl::*;
use diesel::prelude::*;

#[get("/notes")]
pub async fn get_notes(conn: &State<Pool>) -> Json<Vec<Note>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading users
    let results = notes
        .load::<Note>(&conn)
        .expect("Error loading users");

    Json(results)
}


