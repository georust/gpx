//! fix handles parsing of xsd:simpleType "fixType".

use std::io::Read;

use crate::errors::*;
use crate::parser::{string, Context};
use crate::types::Fix;

/// consume consumes an element as a fix.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Fix> {
    let fix_string = string::consume(context, "fix", false)?;

    let fix = match fix_string.as_ref() {
        "none" => Fix::None,
        "2d" => Fix::TwoDimensional,
        "3d" => Fix::ThreeDimensional,
        "dgps" => Fix::DGPS,
        "pps" => Fix::PPS,
        _ => Fix::Other(fix_string),
    };

    Ok(fix)
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::{Fix, GpxVersion};

    #[test]
    fn consume_fix() {
        let result = consume!("<fix>dgps</fix>", GpxVersion::Gpx11);
        assert!(result.is_ok());

        let result = consume!("<fix>none</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::None);

        let result = consume!("<fix>2d</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::TwoDimensional);

        let result = consume!("<fix>3d</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::ThreeDimensional);

        let result = consume!("<fix>dgps</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::DGPS);

        let result = consume!("<fix>pps</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::PPS);

        // Not in the specification
        let result = consume!("<fix>KF_4SV_OR_MORE</fix>", GpxVersion::Gpx11);
        assert_eq!(result.unwrap(), Fix::Other("KF_4SV_OR_MORE".to_owned()));
    }
}
