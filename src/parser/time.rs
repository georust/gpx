//! time handles parsing of xsd:dateTime.

use std::io::Read;

/// format: [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm]
#[cfg(feature = "use-serde")]
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Iso8601, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::errors::{GpxError, GpxResult};
use crate::parser::{string, Context};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct Time(OffsetDateTime);

impl Time {
    /// Render time in ISO 8601 format
    ///
    /// # Errors
    ///
    /// Propagates errors from [`time::OffsetDateTime::format`] using the
    /// [`time::format_description::well_known::Iso8601::DEFAULT`] format.
    ///
    pub fn format(&self) -> GpxResult<String> {
        self.0.format(&Iso8601::DEFAULT).map_err(GpxError::from)
    }
}

impl From<OffsetDateTime> for Time {
    fn from(t: OffsetDateTime) -> Self {
        Time(t)
    }
}

impl From<Time> for OffsetDateTime {
    fn from(t: Time) -> Self {
        t.0
    }
}

/// consume consumes an element as a time.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Time> {
    let time_str = string::consume(context, "time", false)?;

    // Try parsing as ISO 8601 with offset
    let time = OffsetDateTime::parse(&time_str, &Iso8601::PARSING).or_else(|_| {
        // Try parsing as ISO 8601 without offset, assuming UTC
        PrimitiveDateTime::parse(&time_str, &Iso8601::PARSING).map(PrimitiveDateTime::assume_utc)
    })?;

    Ok(time.to_offset(UtcOffset::UTC).into())
}

#[cfg(test)]
mod tests {
    use crate::GpxVersion;

    use super::consume;

    #[test]
    fn consume_time() {
        let result = consume!("<time>1996-12-19T16:39:57-08:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        // The following examples are taken from the xsd:dateTime examples.
        let result = consume!("<time>2001-10-26T21:32:52</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T21:32:52+02:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T19:32:52Z</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T19:32:52+00:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T21:32:52.12679</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T21:32</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        // These are invalid, again, from xsd:dateTime examples.
        let result = consume!("<time>2001-10-26</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        let result = consume!("<time>2001-10-26T25:32:52+02:00</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        let result = consume!("<time>01-10-26T21:32</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        // TODO we currently don't allow for negative years although the standard demands it
        //  see https://www.w3.org/TR/xmlschema-2/#dateTime
        let result = consume!("<time>-2001-10-26T21:32:52</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        // https://github.com/georust/gpx/issues/77
        let result = consume!("<time>2021-10-10T09:55:20.952</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());
    }
}
