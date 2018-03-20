//! waypoint handles parsing of GPX-spec waypoints.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;
use geo::Point;
use chrono::prelude::*;

use parser::string;
use parser::link;
use parser::time;
use parser::fix;
use parser::extensions;
use parser::Context;

use Link;
use Waypoint;
use Fix;

use GpxVersion;

/// consume consumes a GPX waypoint from the `reader` until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Waypoint> {
    // Here we hold all members of a waypoint, just outside of the struct.
    let mut point: Option<Point<f64>> = None;
    let mut elevation: Option<f64> = None;
    let mut speed: Option<f64> = None; // Only in GPX 1.0
    let mut time: Option<DateTime<Utc>> = None;
    let mut wptname: Option<String> = None;
    let mut comment: Option<String> = None;
    let mut description: Option<String> = None;
    let mut source: Option<String> = None;
    let mut links: Vec<Link> = vec![];
    let mut symbol: Option<String> = None;
    let mut _type: Option<String> = None;
    let mut fix: Option<Fix> = None;
    let mut sat: Option<u64> = None; // Number of satellites used for GPX fix (nonNegativeInteger)
    let mut hdop: Option<f64> = None; // Horizontal dilution of precision (decimal)
    let mut vdop: Option<f64> = None; // Vertical dilution of precision (decimal)
    let mut pdop: Option<f64> = None; // Position dilution of precision (decimal)
    let mut age_of_gps_data: Option<f64> = None; // Seconds since last DGPS update (decimal)
    let mut dgpsid: Option<u16> = None; // Id of the DGPS station (integer 0-1023)
    let mut extension_xml: Option<String> = None; //xml-string, to be parsed by user

    while let Some(event) = context.reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                match name.local_name.as_ref() {
                    "wpt" | "trkpt" => {
                        // get required latitude and longitude
                        let latitude = attributes
                            .iter()
                            .filter(|attr| attr.name.local_name == "lat")
                            .nth(0)
                            .ok_or("no latitude attribute on waypoint tag".to_owned())?;

                        let latitude: f64 = latitude
                            .clone()
                            .value
                            .parse()
                            .chain_err(|| "error while casting latitude to f64")?;

                        let longitude = attributes
                            .iter()
                            .filter(|attr| attr.name.local_name == "lon")
                            .nth(0)
                            .ok_or("no longitude attribute on waypoint tag".to_owned())?;

                        let longitude: f64 = longitude
                            .clone()
                            .value
                            .parse()
                            .chain_err(|| "error while casting longitude to f64")?;

                        point = Some(Point::new(longitude, latitude));
                    }
                    "ele" => {
                        // Cast the elevation to an f64, from a string.
                        elevation = Some(string::consume(context)?
                            .parse()
                            .chain_err(|| "error while casting elevation to f64")?)
                    }
                    "speed" if context.version == GpxVersion::Gpx10 => {
                        // Speed is from GPX 1.0
                        speed = Some(string::consume(context)?
                            .parse()
                            .chain_err(|| "error while casting speed to f64")?);
                    }
                    "time" => time = Some(time::consume(context)?),
                    "name" => wptname = Some(string::consume(context)?),
                    "cmt" => comment = Some(string::consume(context)?),
                    "desc" => description = Some(string::consume(context)?),
                    "src" => source = Some(string::consume(context)?),
                    "link" => links.push(link::consume(context)?),
                    "sym" => symbol = Some(string::consume(context)?),
                    "type" => _type = Some(string::consume(context)?),

                    // Optional accuracy information
                    "fix" => fix = Some(fix::consume(context)?),
                    "sat" => {
                        sat = Some(string::consume(context)?
                            .parse()
                            .chain_err(|| "error while casting number of satellites (sat) to u64")?)
                    }
                    "hdop" => {
                        hdop = Some(string::consume(context)?.parse().chain_err(|| {
                            "error while casting horizontal dilution of precision (hdop) to f64"
                        })?)
                    }
                    "vdop" => {
                        vdop = Some(string::consume(context)?.parse().chain_err(|| {
                            "error while casting vertical dilution of precision (vdop) to f64"
                        })?)
                    }
                    "pdop" => {
                        pdop = Some(string::consume(context)?.parse().chain_err(|| {
                            "error while casting position dilution of precision (pdop) to f64"
                        })?)
                    }
                    "ageofgpsdata" => {
                        age_of_gps_data = Some(string::consume(context)?
                            .parse()
                            .chain_err(|| "error while casting age of GPS data to f64")?)
                    }
                    "dgpsid" => {
                        dgpsid = Some(string::consume(context)?
                            .parse()
                            .chain_err(|| "error while casting DGPS station ID to u16")?)
                    }

                    // Finally the GPX 1.1 extensions
                    "extensions" => extension_xml = Some(extensions::consume(context)?),
                    child => Err(Error::from(ErrorKind::InvalidChildElement(
                        String::from(child),
                        "waypoint",
                    )))?,
                }
            }

            XmlEvent::EndElement { .. } => {
                ensure!(point.is_some(), "waypoint must always have point");

                let mut wpt = Waypoint::new(point.unwrap());

                wpt.elevation = elevation;
                wpt.time = time;
                wpt.name = wptname;
                wpt.comment = comment;
                wpt.description = description;
                wpt.source = source;
                wpt.links = links;
                wpt.symbol = symbol;
                wpt._type = _type;
                wpt.fix = fix;
                wpt.sat = sat;
                wpt.hdop = hdop;
                wpt.vdop = vdop;
                wpt.pdop = pdop;
                wpt.speed = speed;
                wpt.age = age_of_gps_data;
                wpt.dgpsid = dgpsid;
                wpt.extension_xml = extension_xml;

                return Ok(wpt);
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

    use GpxVersion;
    use Fix;
    use parser::Context;
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
                <fix>dgps</fix>
                <sat>4</sat>
                <hdop>6.058</hdop>
                <speed>0.0000</speed>
            </wpt>
            ",
            GpxVersion::Gpx10
        );

        assert!(waypoint.is_ok());

        let waypoint = waypoint.unwrap();

        assert_eq!(waypoint.point(), Point::new(-77.0365, 38.8977));
        assert_eq!(waypoint.name.unwrap(), "The White House");
        assert_eq!(
            waypoint.comment.unwrap(),
            "This is a comment about the white house."
        );
        assert_eq!(
            waypoint.description.unwrap(),
            "The white house is very nice!"
        );
        assert_eq!(waypoint.source.unwrap(), "Garmin eTrex");
        assert_eq!(waypoint._type.unwrap(), "waypoint classification");
        assert_eq!(waypoint.elevation.unwrap(), 4608.12);
        assert_eq!(waypoint.fix.unwrap(), Fix::DGPS);
        assert_eq!(waypoint.sat.unwrap(), 4);
        assert_eq!(waypoint.hdop.unwrap(), 6.058);
    }

    #[test]
    fn consume_empty() {
        let waypoint = consume!(
            "<trkpt lat=\"2.345\" lon=\"1.234\"></trkpt>",
            GpxVersion::Gpx11
        );

        assert!(waypoint.is_ok());
        let waypoint = waypoint.unwrap();

        assert_eq!(waypoint.point(), Point::new(1.234, 2.345));
        assert_eq!(waypoint.point().lng(), 1.234);
        assert_eq!(waypoint.point().lat(), 2.345);
    }

    #[test]
    fn consume_bad_waypoint() {
        let waypoint = consume!(
            "<wpt lat=\"32.4\" lon=\"notanumber\"></wpt>",
            GpxVersion::Gpx11
        );

        assert!(waypoint.is_err());
    }
}
