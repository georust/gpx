//! Handles parsing GPX format.

// Just a shared macro for testing 'consume'.
#[macro_export]
macro_rules! consume {
    ( $xml:expr ) => {{
        let reader = BufReader::new($xml.as_bytes());
        consume(&mut EventReader::new(reader).into_iter().peekable())
    }};
}

pub mod email;
pub mod gpx;
pub mod link;
pub mod metadata;
pub mod person;
pub mod string;
pub mod time;
pub mod track;
pub mod tracksegment;
pub mod waypoint;
