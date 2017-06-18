use lettre::transport::smtp::error::Error as SmtpError;
use serde_json::error::Error as JsonError;

use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Smtp(SmtpError),
    File(IoError),
    Serde(JsonError),
}

impl From<SmtpError> for Error {
    fn from(err: SmtpError) -> Self {
        Error::Smtp(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::File(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Serde(err)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use Error::*;
        use SmtpError::*;
        match self {
            &Smtp(ref err) => {
                match err {
                    &Transient(_) => "transient SMTP error",
                    &Permanent(_) => "permanent SMTP error",
                    &ResponseParsing(_) => "error parsing a response",
                    &ChallengeParsing(_) => "error parsing a base64 string in response",
                    &Client(_) => "internal client error",
                    &Resolution => "DNS resolution error",
                    &Io(_) => "I/O error",
                }
            }
            &File(ref err) => err.description(),
            &Serde(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Smtp(ref err) => write!(f, "{:?}", err)?,
            &File(ref err) => write!(f, "{}", err)?,
            &Serde(ref err) => write!(f, "{}", err)?,
        }
    }
}