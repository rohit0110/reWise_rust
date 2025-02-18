#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use crate::db::connection::establish_connection;
use crate::routes::users_routes::{get_users,add_user};
use crate::routes::notes_routes::{get_notes, add_note};
use crate::routes::topics_routes::{get_topics,add_topic,get_topics_details};
use crate::routes::tag_routes::{get_tags,add_tag};

mod db;
mod models;
mod schema;
mod routes;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(establish_connection())
        .mount("/", routes![
            get_users,
            add_user,

            get_notes,
            add_note,

            get_topics,
            get_topics_details,
            add_topic,

            get_tags,
            add_tag
            ])
}

