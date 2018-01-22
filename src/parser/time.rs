//! time handles parsing of xsd:dateTime.

/// format: [-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm]

use errors::*;
use std::io::Read;

use chrono::DateTime;
use chrono::prelude::Utc;

use parser::string;
use parser::Context;

/// consume consumes an element as a time.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<DateTime<Utc>> {
    let time = string::consume(context)?;

    let time =
        DateTime::parse_from_rfc3339(&time).chain_err(|| "error while parsing time as RFC3339")?;

    return Ok(DateTime::from_utc(time.naive_utc(), Utc));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use GpxVersion;
    use parser::Context;
    use super::consume;

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
