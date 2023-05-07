#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

pub extern crate lazy_static;

mod cors;
use crate::error::Error;
use mongodb::bson::{self, doc};

use rocket::{delete, get, post, put, serde::json::Json, Request, figment::{Figment, providers::{Serialized, Toml, Format}}};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
mod error;
mod model;
use model::User;
use rocket::config::{Config};
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





#[get("/api/users")]
async fn get_all_users(db: Connection<DB>) -> Result<Json<Vec<User>>> {
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

#[post("/api/users", data = "<user>")]
async fn new_user(user: Json<User>, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection("users");
    users_collection.insert_one(user.into_inner(), None).await?;
    Ok(())
}

#[get("/api/users/<id>")]
async fn get_user(id: String, db: Connection<DB>) -> Result<Json<User>> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");

    let filter = doc! { "id": id };
    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Error::NoUserFound),
        Err(error) => Err(Error::DB(error)),
    }
}

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(id: String, user: Json<User>, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");
    let filter = doc! { "id": id };
    let update_doc = match bson::to_document(&user.into_inner()) {
        Ok(doc) => doc,
        Err(_) => return Err(Error::UpdateError),
    };

    users_collection
        .update_one(filter, update_doc, None)
        .await
        .map_err(|_| Error::UpdateError)?;

    Ok(())
}


#[delete("/api/users/<id>")]
async fn delete_user(id: String, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");
    let filter = doc! { "id": id };

    users_collection
        .delete_one(filter, None)
        .await
        .map_err(|error| Error::DB(error.into()))?;

    Ok(())
}


#[launch]
pub fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv().ok();


    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("Rocket.toml").nested());


    rocket::custom(figment)
        .register("/", catchers![catch_malformed_request])
        .attach(DB::init())
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
                get_all_users
            ],
        )
}
