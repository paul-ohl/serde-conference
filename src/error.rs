use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::ExpectedArrayEnd => formatter.write_str("expected end of array"),
            Error::ExpectedArrayComma => formatter.write_str("expected comma"),
            Error::ExpectedArray => formatter.write_str("expected array"),
            Error::ExpectedBoolean => formatter.write_str("expected boolean"),
            Error::ExpectedInteger => formatter.write_str("expected integer"),
            Error::ExpectedMap => formatter.write_str("expected map"),
            Error::ExpectedMapColon => formatter.write_str("expected colon"),
            Error::ExpectedMapComma => formatter.write_str("expected comma"),
            Error::ExpectedMapEnd => formatter.write_str("expected end of map"),
            Error::ExpectedNull => formatter.write_str("expected null"),
            Error::ExpectedString => formatter.write_str("expected string"),
            Error::ExpectedEnum => formatter.write_str("expected enum"),
            Error::Syntax => formatter.write_str("syntax error"),
            Error::TrailingCharacters => formatter.write_str("trailing characters"),
        }
    }
}

impl std::error::Error for Error {}
