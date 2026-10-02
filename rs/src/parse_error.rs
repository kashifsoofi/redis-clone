use std::{
    fmt::{self},
    string::FromUtf8Error,
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    FromUtf8,
    OutOfBounds(usize),
    UnsupportedType(u8),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            Self::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            Self::UnsupportedType(t) => write!(f, "Unsupported type {t}"),
        }
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
