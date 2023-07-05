use rocket::http::Status;
use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Errors {
    errors: Vec<Error>,
    status: Status,
}

impl Errors {
    pub fn new(errors: Vec<Error>, status: Option<Status>) -> Self {
        Self {
            errors,
            status: status.unwrap_or(Status::BadRequest),
        }
    }

    pub fn errors(&self) -> &Vec<Error> {
        self.errors.as_ref()
    }

    pub const fn status(&self) -> Status {
        self.status
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Self::new(vec![error], None)
    }
}

impl From<(Error, Status)> for Errors {
    fn from((error, status): (Error, Status)) -> Self {
        Self::new(vec![error], Some(status))
    }
}

impl From<&[Error]> for Errors {
    fn from(errors: &[Error]) -> Self {
        Self::new(Vec::from(errors), None)
    }
}

impl From<(&[Error], Status)> for Errors {
    fn from((errors, status): (&[Error], Status)) -> Self {
        Self::new(Vec::from(errors), Some(status))
    }
}

impl From<&str> for Errors {
    fn from(message: &str) -> Self {
        Self::from(message.to_string())
    }
}

impl From<String> for Errors {
    fn from(message: String) -> Self {
        Self::from(Error::from(message))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Error {
    message: Option<String>,
    details: Option<String>,
    exit_code: Option<u32>,
}

impl Error {
    pub const fn new(
        message: Option<String>,
        details: Option<String>,
        exit_code: Option<u32>,
    ) -> Self {
        Self {
            message,
            details,
            exit_code,
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub const fn exit_code(&self) -> Option<u32> {
        self.exit_code
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::from(message.to_string())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::new(Some(message), None, None)
    }
}

impl From<(&str, u32)> for Error {
    fn from((message, status_code): (&str, u32)) -> Self {
        Self::new(Some(message.into()), None, Some(status_code))
    }
}

impl From<(String, u32)> for Error {
    fn from((message, status_code): (String, u32)) -> Self {
        Self::new(Some(message), None, Some(status_code))
    }
}
