use rocket_db_pools::{Connection};
use crate::DB;
use crate::error::Error;
use crate::model::Post;
use mongodb::{bson::{doc, self}, Collection};

use rocket::{serde::json::Json, futures::TryStreamExt};
use crate::{ACTIVE_DB};

type Result<T> = std::result::Result<T, Error>;


#[delete("/api/posts/<id>")]
pub async fn delete_post(id: String, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let posts_collection = database.collection::<Post>("posts");
    let filter = doc! { "id": id };

    posts_collection
        .delete_one(filter, None)
        .await
        .map_err(|error| Error::DB(error.into()))?;

    Ok(())
}

#[put("/api/posts/<id>", data = "<post>")]
pub async fn update_post(id: String, post: Json<Post>, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let posts_collection = database.collection::<Post>("posts");
    let filter = doc! { "id": id };
    let update_doc = match bson::to_document(&post.into_inner()) {
        Ok(doc) => doc,
        Err(_) => return Err(Error::UpdateError),
    };

    posts_collection
        .update_one(filter, update_doc, None)
        .await
        .map_err(|_| Error::UpdateError)?;

    Ok(())
}




#[get("/api/post/<id>")]
pub async fn get_post(id: String, db: Connection<DB>) -> Result<Json<Post>> {
    let database = db.database(ACTIVE_DB);
    let posts_collection = database.collection::<Post>("posts");

    let filter = doc! { "id": id };
    match posts_collection.find_one(filter, None).await {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err(Error::NoPostFound),
        Err(error) => Err(Error::DB(error)),
    }
}


#[post("/api/new/post", data = "<post>")]
pub async fn new_post(post: Json<Post>, db: Connection<DB>) -> Result<Json<Post>> {
    let database = db.database(ACTIVE_DB);
    let posts_collection = database.collection("posts");
    let inserted_post = posts_collection.insert_one(post.into_inner(), None).await?;
    let inserted_post = inserted_post
        .inserted_id
        .as_object_id()
        .expect("Failed to get inserted post ID");
    let inserted_post = posts_collection
        .find_one(
            doc! {
                "_id": inserted_post.clone(),
            },
            None,
        )
        .await?
        .expect("Failed to find inserted post");
    Ok(Json(inserted_post))
}

#[get("/api/posts")]
pub async fn get_all_posts(db: Connection<DB>) -> Result<Json<Vec<Post>>> {
    let mut cursor = db
        .database(ACTIVE_DB)
        .collection::<Post>("posts")
        .find(doc! {}, None)
        .await?;
    let mut posts = Vec::new();
    while cursor.advance().await? {
        if let Ok(study) = cursor.deserialize_current() {
            posts.push(study);
        }
    }
    Ok(Json(posts))
}

#[get("/api/posts?<query>")]
pub async fn search_posts(query: Option<String>, db: Connection<DB>) -> Result<Json<Vec<Post>>> {
    let posts_collection: Collection<Post> = db
        .database(ACTIVE_DB)
        .collection::<Post>("posts");

    // Create the search filter based on the query parameter
    let filter = match query {
        Some(q) => doc! { "title": { "$regex": q, "$options": "i" } },
        None => doc! {},
    };

    // Execute the search query
    let cursor_result = posts_collection.find(filter, None).await;
    // Unwrap the cursor from the result
    let mut cursor = match cursor_result {
        Ok(cursor) => cursor,
        Err(_) => return Err(Error::NoCommunityFound),
    };
    // Process the search results
    let mut posts = Vec::new();
    while let Ok(Some(user)) = cursor.try_next().await {
        posts.push(user);
    }

    Ok(Json(posts))
}
