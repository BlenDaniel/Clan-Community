#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

pub extern crate lazy_static;

mod cors;
use mongodb::bson::doc;
use rocket::Request;

use rocket::serde::json::Json;
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};

use crate::error::Error;

mod community;
mod error;
mod model;
mod posts;
mod users;

use crate::users::{
    delete_user, get_all_users, get_user, new_user, options_log_in, options_new_user, search_users,
    signin, update_user,
};

use crate::community::{
    delete_community, get_all_communities, get_community, new_community, search_communities,
    update_community,
};
use crate::posts::{delete_post, get_all_posts, get_post, new_post, search_posts, update_post};

type Result<T> = std::result::Result<T, Error>;

/// Automatic database connection using the connection string in Rocket.toml
///
/// https://rocket.rs/v0.5-rc/guide/state/#databases
#[derive(Database)]
#[database("mongodb")]
pub struct DB(mongodb::Client);

#[cfg(debug_assertions)]
pub const ACTIVE_DB: &str = "clan_community";

#[cfg(not(debug_assertions))]
pub const ACTIVE_DB: &str = "clan-community";

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub study_id: String,
    pub api_key: String,
}

#[catch(422)]
fn catch_malformed_request(req: &Request) -> String {
    format!("{req}")
}

#[get("/api/status")]
fn status() -> &'static str {
    "The V1 API is live!"
}

#[get("/")]
fn ping() -> Result<Json<String>> {
    Ok(Json(String::from("I ping you back ... ")))
}

#[launch]
pub fn rocket() -> _ {
    let database = DB::init();

    rocket::build()
        .register("/", catchers![catch_malformed_request])
        .attach(database)
        .attach(cors::CORS)
        .mount(
            "/",
            routes![
                status,
                ping,
                delete_user,
                new_user,
                update_user,
                get_user,
                get_all_users,
                signin,
                options_new_user,
                options_log_in,
                delete_post,
                update_post,
                get_post,
                new_post,
                get_all_posts,
                delete_community,
                get_all_communities,
                get_community,
                new_community,
                update_community,
                search_users,
                search_communities,
                search_posts,
            ],
        )
}
