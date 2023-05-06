#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

pub extern crate lazy_static;

mod cors;
use mongodb::bson::doc;
use rocket::Request;
use rocket::{serde::json::Json};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};

use crate::error::Error;
mod error;
mod model;

use model::User;

type Result<T> = std::result::Result<T, Error>;

/// Automatic database connection using the connection string in Rocket.toml
///
/// https://rocket.rs/v0.5-rc/guide/state/#databases
#[derive(Database)]
#[database("mongodb")]
pub struct DB(mongodb::Client);

#[cfg(debug_assertions)]
pub const ACTIVE_DB: &str = "clan-community-dev";

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

#[get("/api/v1/status")]
fn status() -> &'static str {
    "The V1 API is live!"
}

#[get("/")]
fn ping() -> Result<Json<String>> {
    Ok(Json(String::from("I ping you back ... ")))
}

#[get("/api/v1/users")]
async fn all_users(db: Connection<DB>) -> Result<Json<Vec<User>>> {
    let mut cursor = db
        .database(ACTIVE_DB)
        .collection::<User>("users")
        .find(doc! {}, None)
        .await?;
    let mut users = Vec::new();
    while cursor.advance().await? {
        if let Ok(study) = cursor.deserialize_current() {
            users.push(study);
        }
    }
    Ok(Json(users))
}

#[post("/api/v1/new/user", data = "<data>")]
async fn new_user(data: Json<User>, db: Connection<DB>) -> Result<()> {
    db.database(ACTIVE_DB)
        .collection("users")
        .insert_one(data.into_inner(), None)
        .await?;

    Ok(())
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catch_malformed_request])
        .attach(DB::init())
        .attach(cors::CORS)
        .mount("/", routes![status, ping, all_users, new_user,])
}
