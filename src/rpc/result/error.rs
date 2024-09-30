use rocket::http::Status;
use serde::Serialize;
use std::ops::Add;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Errors {
    errors: Vec<Error>,
    status: Status,
}

impl Errors {
    pub const fn new(errors: Vec<Error>, status: Status) -> Self {
        Self { errors, status }
    }

    pub fn errors(&self) -> &Vec<Error> {
        self.errors.as_ref()
    }

    pub const fn status(&self) -> Status {
        self.status
    }
}

impl Add for Errors {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut errors = Vec::new();
        errors.extend(self.errors);
        errors.extend(rhs.errors);
        Self::new(errors, self.status)
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Self::new(vec![error], Status::BadRequest)
    }
}

impl From<(Error, Status)> for Errors {
    fn from((error, status): (Error, Status)) -> Self {
        Self::new(vec![error], status)
    }
}

impl From<&[Error]> for Errors {
    fn from(errors: &[Error]) -> Self {
        Self::new(Vec::from(errors), Status::BadRequest)
    }
}

impl From<(&[Error], Status)> for Errors {
    fn from((errors, status): (&[Error], Status)) -> Self {
        Self::new(Vec::from(errors), status)
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
