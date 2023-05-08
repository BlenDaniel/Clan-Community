use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    // Add other fields as needed
}

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct Auth {
    #[serde(rename = "email")]
    pub(crate) email_private: String,
    #[serde(rename = "password")]
    pub(crate) password_private: String,
    // Add other fields as needed
}