use http::StatusCode;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwError;
use serde_json;
use std::result;
use tokio_tungstenite::tungstenite::Error as WSError;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Utf8(std::string::FromUtf8Error),
    Api(ApiError),
    HttpInvalidHeader(InvalidHeaderValue),
    HttpError(ReqwError),
    UrlParse(ParseError),
    Websocket(WSError),
}

impl Error {
    pub fn new(code: StatusCode, content: Option<String>) -> Self {
        Error::Api(ApiError { code, content })
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub content: Option<String>,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        return Error::Utf8(e);
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        return Error::UrlParse(e);
    }
}

impl From<WSError> for Error {
    fn from(e: WSError) -> Self {
        return Error::Websocket(e);
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        return Error::HttpInvalidHeader(e);
    }
}

impl From<ReqwError> for Error {
    fn from(e: ReqwError) -> Self {
        return Error::HttpError(e);
    }
}
