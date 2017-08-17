//! waypoint handles parsing of GPX-spec waypoints.

extern crate xml;
extern crate geo;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;
use chrono::DateTime;
use chrono::prelude::Utc;

use geo::Point;
use geo::{ToGeo, Geometry};

use parser::string;
use parser::link;
use parser::time;

/// Waypoint represents a waypoint, point of interest, or named feature on a
/// map.
#[derive(Default, Debug)]
pub struct Waypoint {
    point: Option<Point<f64>>,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Creation/modification timestamp for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time representation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<DateTime<Utc>>,

    /// The GPS name of the waypoint. This field will be transferred to and
    /// from the GPS. GPX does not place restrictions on the length of this
    /// field or the characters contained in it. It is up to the receiving
    /// application to validate the field before sending it to the GPS.
    pub name: Option<String>,

    /// GPS waypoint comment. Sent to GPS as comment.
    pub cmt: Option<String>,

    /// A text description of the element. Holds additional information about
    /// the element intended for the user, not the GPS.
    pub desc: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data. "Garmin eTrex", "USGS quad Boston North", e.g.
    pub src: Option<String>,

    /// Links to additional information about the waypoint.
    pub links: Vec<link::Link>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS. If the GPS
    /// abbreviates words, spell them out.
    pub sym: Option<String>,

    /// Type (classification) of the waypoint.
    pub _type: Option<String>,

    // <magvar> degreesType </magvar> [0..1] ?
    // <geoidheight> xsd:decimal </geoidheight> [0..1] ?
    // <fix> fixType </fix> [0..1] ?
    // <sat> xsd:nonNegativeInteger </sat> [0..1] ?
    // <hdop> xsd:decimal </hdop> [0..1] ?
    // <vdop> xsd:decimal </vdop> [0..1] ?
    // <pdop> xsd:decimal </pdop> [0..1] ?
    // <ageofdgpsdata> xsd:decimal </ageofdgpsdata> [0..1] ?
    // <dgpsid> dgpsStationType </dgpsid> [0..1] ?
    // <extensions> extensionsType </extensions> [0..1] ?
}

impl Waypoint {
    /// Gives the geographical point of the waypoint.
    pub fn point(&self) -> Point<f64> {
        self.point.unwrap()
    }
}

impl ToGeo<f64> for Waypoint {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::Point(self.point())
    }
}


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
                    "cmt" => waypoint.cmt = Some(string::consume(reader)?),
                    "desc" => waypoint.desc = Some(string::consume(reader)?),
                    "src" => waypoint.src = Some(string::consume(reader)?),
                    "link" => waypoint.links.push(link::consume(reader)?),
                    "sym" => waypoint.sym = Some(string::consume(reader)?),
                    "type" => waypoint._type = Some(string::consume(reader)?),
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
            waypoint.cmt.unwrap(),
            "This is a comment about the white house."
        );
        assert_eq!(waypoint.desc.unwrap(), "The white house is very nice!");
        assert_eq!(waypoint.src.unwrap(), "Garmin eTrex");
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
