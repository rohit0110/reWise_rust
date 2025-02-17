#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use crate::db::db::establish_connection;
use crate::routes::users_routes::{get_users,add_user};
use crate::routes::notes_routes::get_notes;

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
            get_notes,
            add_user,
            ])
}

