use rocket::serde::json::Json;
use rocket::State;
use crate::db::db::Pool;
use crate::models::topic::{Topic,NewTopic};
use crate::schema::topics::dsl::*;
use crate::schema::topics;
use diesel::prelude::*;

#[get("/topics")]
pub async fn get_topics(conn: &State<Pool>) -> Json<Vec<Topic>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading topics
    let results = topics
        .load::<Topic>(&conn)
        .expect("Error loading topics");

    Json(results)
}

#[post("/topics", format = "json", data = "<new_topic>")]
pub async fn add_topic(conn: &State<Pool>, new_topic: Json<NewTopic>) -> Json<Topic> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Insert the new topic into the database
    let inserted_topic: Topic = diesel::insert_into(topics::table)
        .values((
            topics::name.eq(new_topic.name.clone()),
            topics::user_id.eq(new_topic.user_id),
        ))
        .get_result(&conn)
        .expect("Error inserting new topic");

    // Return the newly inserted topic as a JSON response
    Json(inserted_topic)
}

