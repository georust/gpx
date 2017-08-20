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
/// `Result<Gpx>`.
///
/// ```
/// use std::io::BufReader;
/// use gpx::Gpx;
/// use gpx::reader;
/// use gpx::errors::*;
///
/// // You can give it anything that implements `std::io::Read`.
/// let data = BufReader::new("<gpx></gpx>".as_bytes());
///
/// let res: Result<Gpx> = reader::read(data);
///
/// match res {
///     Ok(gpx) => {
///         // ..
///     }
///
///     Err(e) => {
///         // ..
///     }
/// }
/// ```
pub fn read<R: Read>(reader: R) -> Result<Gpx> {
    let parser = EventReader::new(reader);
    let mut events = parser.into_iter().peekable();

    return gpx::consume(&mut events);
}
