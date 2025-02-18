use rocket::serde::json::Json;
use rocket::State;
use crate::db::connection::Pool;
use crate::models::topic::{Topic,NewTopic};
use crate::schema::topics::dsl::*;
use crate::schema::{topics, notes, tags, topic_tags};
use crate::models::topic_details::{TopicWithDetails,NoteDetails,TagDetails};
use std::collections::HashMap;
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

// #[get("/topics_details/<requested_user_id>")]
// pub async fn get_topics_details(conn: &State<Pool>, requested_user_id: i32) -> Json<Vec<TopicWithDetails>> {
//     let conn = conn.get().expect("Couldn't get db connection from pool");
    
//     // Assuming raw_data is fetched using a JOIN query
//     let raw_data = topics::table
//     .left_join(notes::table.on(notes::topic_id.eq(topics::id)))
//     .left_join(topic_tags::table.on(topic_tags::topic_id.eq(topics::id)))
//     .left_join(tags::table.on(tags::id.eq(topic_tags::tag_id)))
//     .filter(topics::user_id.eq(requested_user_id))
//     .select((
//         topics::id,
//         topics::name,
//         notes::id.nullable(),
//         notes::content.nullable(),
//         tags::id.nullable(),
//         tags::name.nullable(),
//     ))
//     .load::<(i32, String, Option<i32>, Option<String>, Option<i32>, Option<String>)>(&conn)
//     .expect("Error loading topics with details");

//     // Group topics by ID (no duplicates at the topic level)
//     let mut topic_map: HashMap<i32, TopicWithDetails> = HashMap::new();

//     for (topic_id, topic_name, note_id, note_content, tag_id, tag_name) in raw_data {
//     // If topic isn't in the map yet, initialize it
//     let topic_entry = topic_map.entry(topic_id).or_insert(TopicWithDetails {
//         id: topic_id,
//         name: topic_name,
//         notes: Vec::new(),
//         tags: Vec::new(),
//     });

//     // Add note if it exists (nested loop logic to avoid duplication in notes)
//     if let (Some(note_id), Some(content)) = (note_id, note_content) {
//         if !topic_entry.notes.iter().any(|n| n.id == note_id) {
//             topic_entry.notes.push(NoteDetails { id: note_id, content });
//         }
//     }

//     // Add tag if it exists (nested loop logic to avoid duplication in tags)
//     if let (Some(tag_id), Some(tag_name)) = (tag_id, tag_name) {
//         if !topic_entry.tags.iter().any(|t| t.id == tag_id) {
//             topic_entry.tags.push(TagDetails { id: tag_id, name: tag_name });
//         }
//     }
//     }

//     // Return the topics with their associated notes and tags
//     Json(topic_map.into_values().collect::<Vec<_>>())

// }


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

