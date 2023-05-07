use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    // Add other fields as needed
}