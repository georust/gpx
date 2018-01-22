//! Reads an activity from GPX format.

use std::io::Read;

use errors::*;
use xml::reader::EventReader;

use parser::{gpx, Context};
use Gpx;
use GpxVersion;

/// Reads an activity in GPX format.
///
/// Takes any `std::io::Read` as its reader, and returns a
/// `Result<Gpx>`.
///
/// ```
/// use std::io::BufReader;
/// use gpx::read;
/// use gpx::Gpx;
/// use gpx::errors::*;
///
/// // You can give it anything that implements `std::io::Read`.
/// let data = BufReader::new("<gpx></gpx>".as_bytes());
///
/// let res: Result<Gpx> = read(data);
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
    let events = parser.into_iter().peekable();
    let mut context = Context::new(events, GpxVersion::Unknown);

    return gpx::consume(&mut context);
}
