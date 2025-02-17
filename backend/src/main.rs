#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use crate::db::db::establish_connection;
use crate::routes::users_routes::get_users;

mod db;
mod models;
mod schema;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(establish_connection())
        .mount("/", routes![get_users])
}

