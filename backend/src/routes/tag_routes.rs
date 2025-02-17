use rocket::serde::json::Json;
use rocket::State;
use crate::db::db::Pool;
use crate::models::tag::{Tag, NewTag};
use crate::schema::tags::dsl::*;
use crate::schema::{tags, topic_tags};
use diesel::prelude::*;

#[get("/tags")]
pub async fn get_tags(conn: &State<Pool>) -> Json<Vec<Tag>> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Proper query for loading tags
    let results = tags
        .load::<Tag>(&conn)
        .expect("Error loading tags");

    Json(results)
}

#[post("/tags", format = "json", data = "<new_tag>")]
pub async fn add_tag(conn: &State<Pool>, new_tag: Json<NewTag>) -> Json<Tag> {
    let conn = conn.get().expect("Couldn't get db connection from pool");

    // Insert the new Tag into the tags table
    let inserted_tag: Tag = diesel::insert_into(tags::table)
        .values((
            tags::name.eq(new_tag.name.clone()),  // Assuming the tag has a name
        ))
        .get_result(&conn)
        .expect("Error inserting new Tag");

    // Now associate the new tag with the specified topic using the topic_tags table
    diesel::insert_into(topic_tags::table)
        .values((
            topic_tags::topic_id.eq(new_tag.topic_id),
            topic_tags::tag_id.eq(inserted_tag.id),  // Associate the tag with the topic using the tag's id
        ))
        .execute(&conn)
        .expect("Error inserting into topic_tags");

    // Return the newly inserted Tag as a JSON response
    Json(inserted_tag)
}



