use anyhow::{anyhow, Error as AnyhowError};

// use image::ImageError;

// use serde_json::json;
use std::io;

#[derive(Debug)]
pub struct WError(pub AnyhowError);

// pub type ApiResult = Result<HttpResponse, WError>;

impl std::fmt::Display for WError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AnyhowError> for WError {
    fn from(error: AnyhowError) -> Self {
        Self(error)
    }
}

impl From<io::Error> for WError {
    fn from(error: io::Error) -> Self {
        Self(anyhow!("{}", error))
    }
}

type ImapCodec = imap_codec::codec::DecodeError;

impl From<ImapCodec> for WError {
    fn from(err: ImapCodec) -> Self {
        Self(anyhow!("{:?}", err))
    }
}

// impl<T> From<actix_web::error::InternalError<T>> for WError
// where
//     T: std::fmt::Debug + std::fmt::Display,
// {
//     fn from(error: actix_web::error::InternalError<T>) -> Self {
//         Self(anyhow!("{}", error))
//     }
// }

// impl From<actix_web::error::Error> for WError {
//     fn from(error: actix_web::error::Error) -> Self {
//         Self(anyhow!("{}", error))
//     }
// }

// impl ResponseError for WError {
//     fn error_response(&self) -> HttpResponse {
//         let status_code = if self.0.is::<std::io::Error>() {
//             actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
//         } else if self.0.is::<serde_json::Error>() {
//             actix_web::http::StatusCode::BAD_REQUEST
//         } else {
//             actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
//         };

//         let error_json = json!({ "error": self.to_string() });

//         HttpResponse::build(status_code).json(error_json)
//     }
// }

// pub(crate) fn to_str_err(_: ToStrError) -> error::Error {
//     error::ErrorBadRequest("Invalid UTF-8 in header value")
// }

// pub(crate) fn img_error(e: ImageError) -> error::Error {
//     debug!("e: {}", e);
//     error::ErrorBadRequest("Invalid image")
// }
