//! Reads an activity from GPX format.

extern crate xml;

use std::io::Read;

use errors::*;
use xml::reader::EventReader;

use parser::gpx;
use Gpx;

/// Reads an activity in GPX format.
///
/// Takes any `std::io::Read` as its reader, and returns a
/// `Result<Gpx, Error>`.
pub fn read<R: Read>(reader: R) -> Result<Gpx> {
    let parser = EventReader::new(reader);
    let mut events = parser.into_iter().peekable();

    return gpx::consume(&mut events);
}
