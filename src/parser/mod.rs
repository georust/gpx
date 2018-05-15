//! Handles parsing GPX format.

// Just a shared macro for testing 'consume'.
#[cfg(test)]
#[macro_export]
macro_rules! consume {
    ($xml:expr, $version:expr) => {{
        let reader = BufReader::new($xml.as_bytes());
        let events = EventReader::new(reader).into_iter().peekable();
        let mut context = Context::new(events, $version);
        consume(&mut context)
    }};
}

pub mod bounds;
pub mod email;
pub mod extensions;
pub mod fix;
pub mod gpx;
pub mod link;
pub mod metadata;
pub mod person;
pub mod string;
pub mod time;
pub mod track;
pub mod tracksegment;
pub mod waypoint;

use std::io::Read;
use std::iter::Peekable;
use types::GpxVersion;
use xml::reader::Events;

pub struct Context<R: Read> {
    reader: Peekable<Events<R>>,
    version: GpxVersion,
}

impl<R: Read> Context<R> {
    pub fn new(reader: Peekable<Events<R>>, version: GpxVersion) -> Context<R> {
        Context { reader, version }
    }

    pub fn reader(&mut self) -> &mut Peekable<Events<R>> {
        &mut self.reader
    }
}
