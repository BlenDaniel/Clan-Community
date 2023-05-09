use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::serde_json,
    Request,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Authorization header is malformed. Please provide it in the following form: 
    `email:password` base64 encoded. \n Error: {0} "
    )]
    AuthMalformed(String),

    #[error("No email provided")]
    AuthNoEmail,

    #[error("No password provided")]
    AuthNoPassword,

    #[error("Email or password incorrect")]
    AuthIncorrect,

    #[error("You are not an admin user")]
    NotAdmin,

    #[error("User not found")]
    NoUserFound,

    #[error("Post not found")]
    NoPostFound,

    #[error("Community not found")]
    NoCommunityFound,

    #[error("No corresponding API key found")]
    NoCorrespondingAPIKey,

    #[error("Update Error")]
    UpdateError,

    #[error("Database Error")]
    DB(#[from] mongodb::error::Error),

    #[error("Mongo Database Error")]
    MongoDB(#[from] mongodb::bson::oid::Error),

    #[error("Request error")]
    Request(#[from] reqwest::Error),

    #[error("Response deserialization error")]
    ResponseDeserialization(#[from] serde_json::Error),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        println!("Returning error: {self}");
        match self {
            Error::AuthMalformed(_) => {
                response::status::BadRequest(Some(self.to_string())).respond_to(req)
            }
            Error::AuthNoEmail => {
                response::status::BadRequest(Some(self.to_string())).respond_to(req)
            }
            Error::AuthNoPassword => {
                response::status::BadRequest(Some(self.to_string())).respond_to(req)
            }
            Error::AuthIncorrect => {
                response::status::Unauthorized(Some(self.to_string())).respond_to(req)
            }
            Error::NotAdmin => {
                response::status::Unauthorized(Some(self.to_string())).respond_to(req)
            }
            Error::UpdateError => {
                response::status::Custom(Status::NotFound, "User not found".to_string())
                    .respond_to(req)
            }

            Error::NoCorrespondingAPIKey => {
                response::status::Unauthorized(Some(self.to_string())).respond_to(req)
            }

            Error::NoUserFound => {
                response::status::Custom(Status::NotFound, "User not found".to_string())
                    .respond_to(req)
            }

            Error::NoPostFound => {
                response::status::Custom(Status::NotFound, "Post not found".to_string())
                    .respond_to(req)
            }

            Error::NoCommunityFound => {
                response::status::Custom(Status::NotFound, "Community not found".to_string())
                    .respond_to(req)
            }

            Error::ResponseDeserialization(err) => {
                response::status::Custom(Status::BadRequest, err.to_string()).respond_to(req)
            }
            Error::DB(err) => {
                response::status::Custom(Status::InternalServerError, err.to_string())
                    .respond_to(req)
            }
            Error::MongoDB(err) => {
                response::status::Custom(Status::InternalServerError, err.to_string())
                    .respond_to(req)
            }
            Error::Request(err) => {
                response::status::Custom(Status::InternalServerError, err.to_string())
                    .respond_to(req)
            }
        }
    }
}
