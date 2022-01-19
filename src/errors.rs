//! errors provides error generics for the gpx parser.

use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

pub(crate) type GpxResult<T> = Result<T, GpxError>;

#[derive(Error, Debug)]
#[non_exhaustive]
/// Errors that can occur when reading or writing GPX files
pub enum GpxError {
    #[error("error while casting to f64")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Could not convert value to u32")]
    ParseIntegerError(#[from] ParseIntError),
    #[error("invalid child element `{0}` in `{1}`")]
    InvalidChildElement(String, &'static str),
    #[error("invalid closing tag `{0}` in `{1}`")]
    InvalidClosingTag(String, &'static str),
    #[error("missing closing tag in `{0}`")]
    MissingClosingTag(&'static str),
    #[error("missing opening tag in `{0}`")]
    MissingOpeningTag(&'static str),
    #[error("invalid element, `{1}` lacks required attribute `{0}`")]
    InvalidElementLacksAttribute(&'static str, &'static str),
    #[error("minimum `{0}` larger than maximum `{0}`")]
    OutOfBounds(&'static str),
    #[error("error while parsing XML")]
    XmlParseError(#[from] xml::reader::Error),
    #[error("unknown GPX version: `{0}`")]
    UnknownVersionError(crate::types::GpxVersion),
    #[error("tag opened twice: `{0}`")]
    TagOpenedTwice(&'static str),
    #[error("error while parsing 'track' segment")]
    TrackSegmentError(),
    #[error("no string content")]
    NoStringContent,
    #[error("error while writing XML")]
    XmlWriteError(#[from] xml::writer::Error),
    #[error("missing `{0}` part in email")]
    MissingEmailPartError(&'static str),
    #[error("email contains multiple `@` symbols")]
    TooManyAtsError,
    #[error("error while parsing `{0}`")]
    EventParsingError(&'static str),
    #[error("error while parsing metadata")]
    MetadataParsingError(),
    #[error("invalid `{0}`: must be between `{1}`. Actual value: `{2}`")]
    LonLatOutOfBoundsError(&'static str, &'static str, f64),
    #[error("error trying to parse RFC3339 formatted date")]
    Rfc3339Error(#[from] time::error::Parse),
    #[error("error trying to write RFC3339 formatted date")]
    Rfc3339ErrorWriting(#[from] time::error::Format),
}
