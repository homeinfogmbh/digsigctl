use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Errors {
    errors: Vec<Error>,
    status_code: Option<u16>,
}

impl Errors {
    pub fn new(errors: Vec<Error>, status_code: Option<u16>) -> Self {
        Self {
            errors,
            status_code,
        }
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Self::new(vec![error], None)
    }
}

impl From<(Error, u16)> for Errors {
    fn from((error, status_code): (Error, u16)) -> Self {
        Self::new(vec![error], Some(status_code))
    }
}

impl From<&[Error]> for Errors {
    fn from(errors: &[Error]) -> Self {
        Self::new(Vec::from(errors), None)
    }
}

impl From<(&[Error], u16)> for Errors {
    fn from((errors, status_code): (&[Error], u16)) -> Self {
        Self::new(Vec::from(errors), Some(status_code))
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
