use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub location: String
}