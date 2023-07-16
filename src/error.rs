use anyhow::{anyhow, Error as AnyhowError};

use std::io;

#[derive(Debug)]
pub struct WError(pub AnyhowError);

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
