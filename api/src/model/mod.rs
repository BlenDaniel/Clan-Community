use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub address: String,
    pub dob: Date,
    pub phone: String,
    pub urls: Vec<String>,
    pub bio: String,
    pub image_url: String,
    pub saves: Vec<String>,
    pub likes: Vec<String>,
    pub status: String,
    pub communities: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct Auth {
    #[serde(rename = "email")]
    pub(crate) email_private: String,
    #[serde(rename = "password")]
    pub(crate) password_private: String,
    // Add other fields as needed
}

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub members: Vec<String>,
    pub owner: String,
    pub admins: Vec<String>,
    pub description: String,
    pub requests: Vec<String>,
    pub group_type: u32,
    pub location: String,
    pub date: Date,
    pub is_accepting: bool,
    pub activity: Vec<String>,
}


pub enum GroupType {
    Cause,
    Personal,
    Organization,
    Club,
}

#[derive(Clone, Serialize, Deserialize, FromForm)]
pub struct Post {
    pub id: i32,
    pub communities: Vec<String>,
    pub title: String,
    pub description: String,
    pub member: String,
    pub date: Date,
    pub seen_by: Vec<String>,
}
