use rocket_db_pools::{Connection};
use crate::DB;
use crate::error::Error;
use crate::model::Community;
use mongodb::{bson::{doc, self}, Collection};

use rocket::{serde::json::Json, futures::TryStreamExt};
use crate::{ACTIVE_DB};

type Result<T> = std::result::Result<T, Error>;


#[delete("/api/communities/<id>")]
pub async fn delete_community(id: String, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let communities_collection = database.collection::<Community>("communities");
    let filter = doc! { "id": id };

    communities_collection
        .delete_one(filter, None)
        .await
        .map_err(|error| Error::DB(error.into()))?;

    Ok(())
}

#[put("/api/communities/<id>", data = "<community>")]
pub async fn update_community(id: String, community: Json<Community>, db: Connection<DB>) -> Result<()> {
    let database = db.database(ACTIVE_DB);
    let communities_collection = database.collection::<Community>("communities");
    let filter = doc! { "id": id };
    let update_doc = match bson::to_document(&community.into_inner()) {
        Ok(doc) => doc,
        Err(_) => return Err(Error::UpdateError),
    };

    communities_collection
        .update_one(filter, update_doc, None)
        .await
        .map_err(|_| Error::UpdateError)?;

    Ok(())
}




#[get("/api/community/<id>")]
pub async fn get_community(id: String, db: Connection<DB>) -> Result<Json<Community>> {
    let database = db.database(ACTIVE_DB);
    let communities_collection = database.collection::<Community>("communities");

    let filter = doc! { "id": id };
    match communities_collection.find_one(filter, None).await {
        Ok(Some(community)) => Ok(Json(community)),
        Ok(None) => Err(Error::NoCommunityFound),
        Err(error) => Err(Error::DB(error)),
    }
}


#[post("/api/new/community", data = "<community>")]
pub async fn new_community(community: Json<Community>, db: Connection<DB>) -> Result<Json<Community>> {
    let database = db.database(ACTIVE_DB);
    let communities_collection = database.collection("communities");
    let inserted_community = communities_collection.insert_one(community.into_inner(), None).await?;
    let inserted_community = inserted_community
        .inserted_id
        .as_object_id()
        .expect("Failed to get inserted community ID");
    let inserted_community = communities_collection
        .find_one(
            doc! {
                "_id": inserted_community.clone(),
            },
            None,
        )
        .await?
        .expect("Failed to find inserted community");
    Ok(Json(inserted_community))
}

#[get("/api/communities")]
pub async fn get_all_communities(db: Connection<DB>) -> Result<Json<Vec<Community>>> {
    let mut cursor = db
        .database(ACTIVE_DB)
        .collection::<Community>("communities")
        .find(doc! {}, None)
        .await?;
    let mut communities = Vec::new();
    while cursor.advance().await? {
        if let Ok(study) = cursor.deserialize_current() {
            communities.push(study);
        }
    }
    Ok(Json(communities))
}


#[get("/api/communities?<query>")]
pub async fn search_communities(query: Option<String>, db: Connection<DB>) -> Result<Json<Vec<Community>>> {
    let communities_collection: Collection<Community> = db
        .database(ACTIVE_DB)
        .collection::<Community>("communities");

    // Create the search filter based on the query parameter
    let filter = match query {
        Some(q) => doc! { "name": { "$regex": q, "$options": "i" } },
        None => doc! {},
    };

    // Execute the search query
    let cursor_result = communities_collection.find(filter, None).await;
    // Unwrap the cursor from the result
    let mut cursor = match cursor_result {
        Ok(cursor) => cursor,
        Err(_) => return Err(Error::NoCommunityFound),
    };
    // Process the search results
    let mut communities = Vec::new();
    while let Ok(Some(user)) = cursor.try_next().await {
        communities.push(user);
    }

    Ok(Json(communities))
}
