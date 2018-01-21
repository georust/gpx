//! fix handles parsing of xsd:simpleType "fixType".

use errors::*;
use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;

use parser::string;
use types::Fix;

/// consume consumes an element as a fix.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Fix> {
    let fix_string = string::consume(reader)?;

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
    use Fix;

    #[test]
    fn consume_fix() {
        let result = consume!("<fix>dgps</fix>");
        assert!(result.is_ok());

        let result = consume!("<fix>none</fix>");
        assert_eq!(result.unwrap(), Fix::None);

        let result = consume!("<fix>2d</fix>");
        assert_eq!(result.unwrap(), Fix::TwoDimensional);

        let result = consume!("<fix>3d</fix>");
        assert_eq!(result.unwrap(), Fix::ThreeDimensional);

        let result = consume!("<fix>dgps</fix>");
        assert_eq!(result.unwrap(), Fix::DGPS);

        let result = consume!("<fix>pps</fix>");
        assert_eq!(result.unwrap(), Fix::PPS);

        // Not in the specification
        let result = consume!("<fix>KF_4SV_OR_MORE</fix>");
        assert_eq!(result.unwrap(), Fix::Other("KF_4SV_OR_MORE".to_owned()));
    }
}
