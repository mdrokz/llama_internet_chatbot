use rocket::{
    http::{ContentType, Status},
    response::{self, Debug, Responder},
    Response, Request,
};

use std::io::Cursor;

pub type Result<T, E = Debug<diesel::result::Error>> = anyhow::Result<T, E>;

pub type RtResult<T> = Result<T, RocketError>;

pub struct RocketError(anyhow::Error, i32);

impl<'r> Responder<'r, 'static> for RocketError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {

        let status = match self.1 {
            400 => Status::BadRequest,
            401 => Status::Unauthorized,
            403 => Status::Forbidden,
            404 => Status::NotFound,
            409 => Status::Conflict,
            500 => Status::InternalServerError,
            _ => Status::InternalServerError,
        };

        Response::build()
            .status(status)
            .header(ContentType::Plain)
            .sized_body(self.0.to_string().len(), Cursor::new(self.0.to_string()))
            .ok()
    }
}

pub mod chat;
pub mod conversation;
