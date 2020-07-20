use derive_more::*;

use std::error::Error;

/// The ASS error type, use the `.kind` value to check the cause of the error
#[derive(Debug, Display)]
#[display(fmt = "{}", kind)]
pub struct AssError {
    pub kind: AssErrorKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

/// Describes what kind of error we're dealing with
#[derive(Debug, Display, Eq, PartialEq)]
pub enum AssErrorKind {
    #[display(fmt = "Invalid Account file: {}", file)]
    InvalidAccountFile { err: String, file: String },
    #[display(fmt = "Error accessing file ({}): {}", file, err)]
    InvalidFileName { err: String, file: String },
    #[display(fmt = "Url does not match the given account: {}", .0)]
    UrlDoesNotMatchAccount(String),
    #[display(fmt = "Invalid url")]
    InvalidUrl,
    #[display(fmt = "Reqwest Error")]
    ReqwestError,
    #[display(fmt = "IO Error")]
    IOError,
    #[display(fmt = "Json Error")]
    JsonError,
}

impl AssError {
    /// Creates an error indicating that a url does not match the client's base url
    pub fn url_does_not_match_account(url: String) -> Self {
        AssError {
            kind: AssErrorKind::UrlDoesNotMatchAccount(url),
            source: None,
        }
    }

    /// Creates an error indicating that we could not find a given file
    pub fn invalid_file_name(err: String, file: String) -> Self {
        AssError {
            kind: AssErrorKind::InvalidFileName { err, file },
            source: None,
        }
    }

    /// Creates an error indicating that account file was invalid
    pub fn invalid_account_file(err: String, file: String) -> Self {
        AssError {
            kind: AssErrorKind::InvalidAccountFile { err, file },
            source: None,
        }
    }
}

impl Error for AssError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|boxed| boxed.as_ref() as &(dyn Error + 'static))
    }
}

impl From<AssErrorKind> for AssError {
    fn from(kind: AssErrorKind) -> AssError {
        AssError { kind, source: None }
    }
}

impl From<reqwest::Error> for AssError {
    fn from(err: reqwest::Error) -> AssError {
        AssError {
            kind: AssErrorKind::ReqwestError,
            source: Some(Box::new(err)),
        }
    }
}

impl From<url::ParseError> for AssError {
    fn from(err: url::ParseError) -> AssError {
        AssError {
            kind: AssErrorKind::InvalidUrl,
            source: Some(Box::new(err)),
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for AssError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> AssError {
        AssError {
            kind: AssErrorKind::ReqwestError,
            source: Some(Box::new(err)),
        }
    }
}

impl From<reqwest::header::InvalidHeaderName> for AssError {
    fn from(err: reqwest::header::InvalidHeaderName) -> AssError {
        AssError {
            kind: AssErrorKind::ReqwestError,
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::io::Error> for AssError {
    fn from(err: std::io::Error) -> AssError {
        AssError {
            kind: AssErrorKind::IOError,
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::error::Error> for AssError {
    fn from(err: serde_json::error::Error) -> AssError {
        AssError {
            kind: AssErrorKind::JsonError,
            source: Some(Box::new(err)),
        }
    }
}
