// src/routes/user_routes.rs
use rocket::serde::json::Json;
use rocket::State;
use crate::db::connection::Pool;
use crate::models::note::{Note, NewNote};
use crate::schema::notes::dsl::*;
use crate::schema::notes;
use diesel::prelude::*;

#[get("/notes")]
pub async fn get_notes(conn: &State<Pool>) -> Json<Vec<Note>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading notes
    let results = notes
        .load::<Note>(&conn)
        .expect("Error loading notes");

    Json(results)
}

#[post("/notes", format="json", data="<new_note>")]
pub async fn add_note(conn: &State<Pool>, new_note: Json<NewNote>) -> Json<Note> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Insert the new Note into the database
    let inserted_note: Note = diesel::insert_into(notes::table)
        .values((
            notes::content.eq(new_note.content.clone()),
            notes::topic_id.eq(new_note.topic_id),
        ))
        .get_result(&conn)
        .expect("Error inserting new Note");

    // Return the newly inserted Note as a JSON response
    Json(inserted_note)
}


