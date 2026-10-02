use std::fmt::{self};

#[derive(Debug, PartialEq)]
pub enum Resp {
    SimpleString(String),
}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SimpleString(str) => write!(f, "+{str}\r\n"),
        }
    }
}

impl Resp {
    pub fn simple_string(s: impl Into<String>) -> Self {
        Self::SimpleString(s.into())
    }
}
