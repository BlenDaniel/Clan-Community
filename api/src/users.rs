use mongodb::Collection;
use rocket::futures::TryStreamExt;
use rocket_db_pools::{Connection};
use crate::{DB, ACTIVE_DB};
use crate::error::Error;
use crate::model::User;
use mongodb::bson::{doc, self};
use crate::model::Auth;
use crate::{Result};
use rocket::response::status::NoContent;
use rocket::{serde::json::Json};

type ResultDB<T> = std::result::Result<T, Error>;


#[delete("/api/users/<id>")]
pub async fn delete_user(id: String, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");
    let filter = doc! { "id": id };

    users_collection
        .delete_one(filter, None)
        .await
        .map_err(|error| Error::DB(error.into()))?;

    Ok(())
}

#[put("/api/users/<id>", data = "<user>")]
pub async fn update_user(id: String, user: Json<User>, db: Connection<DB>) -> Result<()> {
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




#[get("/api/auth/get_user/<id>")]
pub async fn get_user(id: String, db: Connection<DB>) -> Result<Json<User>> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");

    let filter = doc! { "id": id };
    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Error::NoUserFound),
        Err(error) => Err(Error::DB(error)),
    }
}


#[post("/api/auth/signin", data = "<auth>")]
pub async fn signin(auth: Json<Auth>, db: Connection<DB>) -> Result<Json<User>> {
    // Extract the email and password from the request body
    let email = &auth.email_private;
    let password = &auth.password_private;

    // Retrieve the database collection
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection::<User>("users");

    // Create the filter document
    let filter = doc! { "email": email, "password": password };

    // Find the user matching the filter
    if let Some(user) = users_collection.find_one(filter, None).await? {
        Ok(Json(user))
    } else {
        Err(Error::NoUserFound)
    }
}


#[post("/api/auth/signup", data = "<user>")]
pub async fn new_user(user: Json<User>, db: Connection<DB>) -> Result<Json<User>> {
    let database = db.database(ACTIVE_DB);
    let users_collection = database.collection("users");
    let inserted_user = users_collection.insert_one(user.into_inner(), None).await?;
    let inserted_user = inserted_user
        .inserted_id
        .as_object_id()
        .expect("Failed to get inserted user ID");
    let inserted_user = users_collection
        .find_one(
            doc! {
                "_id": inserted_user.clone(),
            },
            None,
        )
        .await?
        .expect("Failed to find inserted user");
    Ok(Json(inserted_user))
}

#[get("/api/users")]
pub async fn get_all_users(db: Connection<DB>) -> Result<Json<Vec<User>>> {
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

#[get("/api/users?<query>")]
pub async fn search_users(query: Option<String>, db: Connection<DB>) -> ResultDB<Json<Vec<User>>> {
    let users_collection: Collection<User> = db
        .database(ACTIVE_DB)
        .collection::<User>("users");

    // Create the search filter based on the query parameter
    let filter = match query {
        Some(q) => doc! { "name": { "$regex": q, "$options": "i" } },
        None => doc! {},
    };

    // Execute the search query
    let cursor_result = users_collection.find(filter, None).await;
    // Unwrap the cursor from the result
    let mut cursor = match cursor_result {
        Ok(cursor) => cursor,
        Err(_) => return Err(Error::NoCommunityFound),
    };
    // Process the search results
    let mut users = Vec::new();
    while let Ok(Some(user)) = cursor.try_next().await {
        users.push(user);
    }

    Ok(Json(users))
}

#[options("/api/auth/signin")]
pub fn options_log_in() -> NoContent {
    NoContent
}
#[options("/api/auth/signup")]
pub fn options_new_user() -> NoContent {
    NoContent
}