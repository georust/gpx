//! time handles parsing of xsd:dateTime.

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
/// format: [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm]
use std::io::Read;
#[cfg(feature = "time")]
use time::{format_description::well_known::Rfc3339, OffsetDateTime as Time, UtcOffset};

#[cfg(not(feature = "time"))]
type Time = DateTime<Utc>;

use crate::errors::GpxResult;
use crate::parser::{string, Context};

/// consume consumes an element as a time.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Time> {
    let time = string::consume(context, "time", false)?;

    #[cfg(feature = "time")]
    {
        let time = Time::parse(&time, &Rfc3339)?;
        Ok(time.to_offset(UtcOffset::UTC))
    }
    #[cfg(not(feature = "time"))]
    {
        let time = DateTime::parse_from_rfc3339(&time)?;
        Ok(DateTime::from_utc(time.naive_utc(), Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

    #[test]
    fn consume_time() {
        let result = consume!("<time>1996-12-19T16:39:57-08:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        // The following examples are taken from the xsd:dateTime examples.

        // TODO, we currently don't allow dates which don't specify timezones,
        // while the spec considers these to be "undetermined".
        // let result = consume!("<time>2001-10-26T21:32:52</time>");
        // assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T21:32:52+02:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T19:32:52Z</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<time>2001-10-26T19:32:52+00:00</time>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        // let result = consume!("<time>-2001-10-26T21:32:52</time>", GpxVersion::Gpx11);
        // assert!(result.is_ok());

        // let result = consume!("<time>2001-10-26T21:32:52.12679</time>", GpxVersion::Gpx11);
        // assert!(result.is_ok());

        // These are invalid, again, from xsd:dateTime examples.
        let result = consume!("<time>2001-10-26</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        let result = consume!("<time>2001-10-26T21:32</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        let result = consume!("<time>2001-10-26T25:32:52+02:00</time>", GpxVersion::Gpx11);
        assert!(result.is_err());

        let result = consume!("<time>01-10-26T21:32</time>", GpxVersion::Gpx11);
        assert!(result.is_err());
    }
}
