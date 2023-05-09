
pub mod authentication{
#[post("/api/auth/signup", data = "<user>")]
async fn new_user(user: Json<User>, db: Connection<DB>) -> Result<Json<User>> {
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

#[post("/api/auth/signin", data = "<auth>")]
async fn signin(auth: Json<Auth>, db: Connection<DB>) -> Result<Json<User>> {
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
}