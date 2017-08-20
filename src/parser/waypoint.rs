//! waypoint handles parsing of GPX-spec waypoints.

extern crate xml;
extern crate geo;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;
use geo::Point;

use parser::string;
use parser::link;
use parser::time;
use parser::extensions;

use Waypoint;

/// consume consumes a GPX waypoint from the `reader` until it ends.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Waypoint> {
    let mut waypoint: Waypoint = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "wpt" | "trkpt" => {
                        // get required latitude and longitude
                        let latitude =
                            attributes
                                .iter()
                                .filter(|attr| attr.name.local_name == "lat")
                                .nth(0)
                                .ok_or("no latitude attribute on waypoint tag".to_owned())?;

                        let latitude: f64 = latitude.clone().value.parse().chain_err(
                            || "error while casting latitude to f64",
                        )?;

                        let longitude =
                            attributes
                                .iter()
                                .filter(|attr| attr.name.local_name == "lon")
                                .nth(0)
                                .ok_or("no longitude attribute on waypoint tag".to_owned())?;

                        let longitude: f64 = longitude.clone().value.parse().chain_err(
                            || "error while casting longitude to f64",
                        )?;

                        waypoint.point = Some(Point::new(longitude, latitude));
                    }
                    "ele" => {
                        // Cast the elevation to an f64, from a string.
                        waypoint.elevation = Some(string::consume(reader)?.parse().chain_err(
                            || "error while casting elevation to f64",
                        )?)
                    }
                    "time" => waypoint.time = Some(time::consume(reader)?),
                    "name" => waypoint.name = Some(string::consume(reader)?),
                    "cmt" => waypoint.comment = Some(string::consume(reader)?),
                    "desc" => waypoint.description = Some(string::consume(reader)?),
                    "src" => waypoint.source = Some(string::consume(reader)?),
                    "link" => waypoint.links.push(link::consume(reader)?),
                    "sym" => waypoint.symbol = Some(string::consume(reader)?),
                    "type" => waypoint._type = Some(string::consume(reader)?),
                    "extensions" => extensions::consume(reader)?,
                    _ => {
                        return Err("bad child element".into());
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                // TODO ensure!(waypoint.point.is_some(), "waypoint must always have point");

                return Ok(waypoint);
            }

            _ => {}
        }
    }

    return Err("no end tag for waypoint".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;
    use geo::Point;

    use super::consume;

    #[test]
    fn consume_waypoint() {
        let waypoint = consume!(
            "
            <wpt lon=\"-77.0365\" lat=\"38.8977\">
                <name>The White House</name>
                <cmt>This is a comment about the white house.</cmt>
                <desc>The white house is very nice!</desc>
                <src>Garmin eTrex</src>
                <type>waypoint classification</type>
                <ele>4608.12</ele>
            </wpt>
            "
        );

        assert!(waypoint.is_ok());

        let waypoint = waypoint.unwrap();

        assert_eq!(waypoint.point(), Point::new(-77.0365, 38.8977));
        assert_eq!(waypoint.name.unwrap(), "The White House");
        assert_eq!(
            waypoint.comment.unwrap(),
            "This is a comment about the white house."
        );
        assert_eq!(waypoint.description.unwrap(), "The white house is very nice!");
        assert_eq!(waypoint.source.unwrap(), "Garmin eTrex");
        assert_eq!(waypoint._type.unwrap(), "waypoint classification");
        assert_eq!(waypoint.elevation.unwrap(), 4608.12);
    }

    #[test]
    fn consume_empty() {
        let waypoint = consume!("<trkpt lat=\"2.345\" lon=\"1.234\"></trkpt>");

        assert!(waypoint.is_ok());
        let waypoint = waypoint.unwrap();

        assert_eq!(waypoint.point(), Point::new(1.234, 2.345));
        assert_eq!(waypoint.point().lng(), 1.234);
        assert_eq!(waypoint.point().lat(), 2.345);
    }

    #[test]
    fn consume_bad_waypoint() {
        let waypoint = consume!("<wpt lat=\"32.4\" lon=\"notanumber\"></wpt>");

        assert!(waypoint.is_err());
    }
}
