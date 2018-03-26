//! fix handles parsing of xsd:simpleType "fixType".

use errors::*;
use std::io::Read;

use parser::string;
use parser::Context;

use types::Fix;

/// consume consumes an element as a fix.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Fix> {
    let fix_string = string::consume(context, "fix")?;

    let fix = match fix_string.as_ref() {
        "none" => Fix::None,
        "2d" => Fix::TwoDimensional,
        "3d" => Fix::ThreeDimensional,
        "dgps" => Fix::DGPS,
        "pps" => Fix::PPS,
        _ => Fix::Other(fix_string),
    };

    return Ok(fix);
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    use GpxVersion;
    use Fix;
    use parser::Context;

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
