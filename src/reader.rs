//! Reads an activity from GPX format.

use std::io::Read;

use crate::errors::GpxResult;
use crate::parser::{create_context, gpx};
use crate::{Gpx, GpxVersion};

/// Reads an activity in GPX format.
///
/// Takes any `std::io::Read` as its reader, and returns a
/// `Result<Gpx>`.
///
/// ```
/// use std::io::BufReader;
/// use gpx::read;
/// use gpx::Gpx;
/// use gpx::errors::GpxError;
///
/// // You can give it anything that implements `std::io::Read`.
/// let data = BufReader::new("<gpx></gpx>".as_bytes());
///
/// let res: Result<Gpx, GpxError> = read(data);
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
pub fn read<R: Read>(reader: R) -> GpxResult<Gpx> {
    gpx::consume(&mut create_context(reader, GpxVersion::Unknown))
}
