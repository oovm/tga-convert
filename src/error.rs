use glob::{GlobError, PatternError};
use image::ImageError;
use std::{io::ErrorKind, option::NoneError};
use toml;

#[derive(Debug)]
pub enum Error {
    NullException,
    FileNotFound,
    PermissionDenied,
    UnknownIOError,
    ParseFailed,
    InvalidGlob,
    ConfigError,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => Error::FileNotFound,
            ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::UnknownIOError,
        }
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NullException
    }
}

impl From<ImageError> for Error {
    fn from(_: ImageError) -> Self {
        Error::ParseFailed
    }
}

impl From<GlobError> for Error {
    fn from(_: GlobError) -> Self {
        Error::InvalidGlob
    }
}

impl From<PatternError> for Error {
    fn from(_: PatternError) -> Self {
        Error::InvalidGlob
    }
}

impl From<toml::de::Error> for Error {
    fn from(_: toml::de::Error) -> Self {
        Error::ConfigError
    }
}
