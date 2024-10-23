use error::Errors;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::{Request, Response};
use std::io::Cursor;
use std::ops::Add;

mod error;

/// A result of an RPC call, which can either be successful or result in an error.
pub enum Result {
    /// The RPC call was successful.
    ///
    /// The return data is wrapped as a dynamic serializable in a `Box`.
    Success(Box<dyn erased_serde::Serialize>),
    /// The RPC call failed.
    ///
    /// Details about the errors that occurred can be read from the `Errors` struct.
    Error(Errors),
}

impl Add for Result {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Error(lhs), Self::Error(rhs)) => Self::Error(lhs + rhs),
            (Self::Error(lhs), _) => Self::Error(lhs),
            (_, Self::Error(rhs)) => Self::Error(rhs),
            _ => Self::Success(Box::new(())),
        }
    }
}

impl<E> From<E> for Result
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self::Error(error.to_string().into())
    }
}

impl From<Result> for (Status, String) {
    fn from(result: Result) -> Self {
        match result {
            Result::Success(value) => {
                serde_json::to_string(value.as_ref()).map(|json| (Status::Ok, json))
            }
            Result::Error(errors) => {
                serde_json::to_string(errors.errors()).map(|json| (errors.status(), json))
            }
        }
        .unwrap_or((
            Status::InternalServerError,
            "Cannot serialize message.".to_string(),
        ))
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Result {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, json): (Status, String) = self.into();
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
