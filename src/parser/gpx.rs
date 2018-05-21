//! gpx handles parsing of GPX elements.

use chrono::{DateTime, Utc};
use errors::*;
use geo::Bbox;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::bounds;
use parser::metadata;
use parser::string;
use parser::time;
use parser::track;
use parser::verify_starting_tag;
use parser::waypoint;
use parser::Context;

use Gpx;
use GpxVersion;
use Link;
use Metadata;
use Person;

/// Convert the version string to the version enum
fn version_string_to_version(version_str: &str) -> Result<GpxVersion> {
    match version_str {
        "1.0" => Ok(GpxVersion::Gpx10),
        "1.1" => Ok(GpxVersion::Gpx11),
        version => Err(Error::from(format!("Unknown version {}", version))),
    }
}

/// consume consumes an entire GPX element.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Gpx> {
    let mut gpx: Gpx = Default::default();

    let mut author: Option<String> = None;
    let mut url: Option<String> = None;
    let mut urlname: Option<String> = None;
    let mut email: Option<String> = None;
    let mut time: Option<DateTime<Utc>> = None;
    let mut bounds: Option<Bbox<f64>> = None;
    let mut gpx_name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut keywords: Option<String> = None;

    // First we consume the gpx tag and its attributes
    let attributes = verify_starting_tag(context, "gpx")?;
    let version = attributes
        .iter()
        .filter(|attr| attr.name.local_name == "version")
        .nth(0)
        .ok_or(ErrorKind::InvalidElementLacksAttribute("version", "gpx"))?;
    gpx.version = version_string_to_version(&version.value)?;
    context.version = gpx.version;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                next.clone()
            } else {
                break;
            }
        };

        match next_event.chain_err(|| Error::from("error while parsing gpx event"))? {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "metadata" if context.version != GpxVersion::Gpx10 => {
                    gpx.metadata = Some(metadata::consume(context)?);
                }
                "trk" => {
                    gpx.tracks.push(track::consume(context)?);
                }
                "wpt" => {
                    gpx.waypoints.push(waypoint::consume(context, "wpt")?);
                }
                "time" if context.version == GpxVersion::Gpx10 => {
                    time = Some(time::consume(context)?);
                }
                "bounds" if context.version == GpxVersion::Gpx10 => {
                    bounds = Some(bounds::consume(context)?);
                }
                "author" if context.version == GpxVersion::Gpx10 => {
                    author = Some(string::consume(context, "author")?);
                }
                "email" if context.version == GpxVersion::Gpx10 => {
                    email = Some(string::consume(context, "email")?);
                }
                "url" if context.version == GpxVersion::Gpx10 => {
                    url = Some(string::consume(context, "url")?);
                }
                "urlname" if context.version == GpxVersion::Gpx10 => {
                    urlname = Some(string::consume(context, "urlname")?);
                }
                "name" if context.version == GpxVersion::Gpx10 => {
                    gpx_name = Some(string::consume(context, "name")?);
                }
                "description" if context.version == GpxVersion::Gpx10 => {
                    description = Some(string::consume(context, "description")?);
                }
                "keywords" if context.version == GpxVersion::Gpx10 => {
                    keywords = Some(string::consume(context, "keywords")?);
                }
                child => {
                    bail!(ErrorKind::InvalidChildElement(String::from(child), "gpx"));
                }
            },
            XmlEvent::EndElement { name } => {
                ensure!(
                    name.local_name == "gpx",
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "gpx")
                );
                if gpx.version == GpxVersion::Gpx10 {
                    let mut metadata: Metadata = Default::default();
                    metadata.name = gpx_name;
                    metadata.time = time;
                    metadata.bounds = bounds;
                    let mut person: Person = Default::default();
                    person.name = author;
                    if let Some(url) = url {
                        let mut link: Link = Default::default();
                        link.href = url;
                        link.text = urlname;
                        person.link = Some(link);
                    }
                    person.email = email;
                    metadata.author = Some(person);
                    metadata.keywords = keywords;
                    metadata.description = description;
                    gpx.metadata = Some(metadata);
                }
                context.reader.next();

                return Ok(gpx);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    bail!(ErrorKind::MissingClosingTag("gpx"));
}

#[cfg(test)]
mod tests {
    use geo::Point;
    use std::io::BufReader;

    use super::consume;
    use GpxVersion;

    #[test]
    fn consume_gpx() {
        let gpx = consume!("<gpx version=\"1.1\"></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_ok());
    }

    #[test]
    fn consume_gpx_no_version() {
        let gpx = consume!("<gpx></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_err());
    }

    #[test]
    fn consume_gpx_version_error() {
        let gpx = consume!("<gpx version=\"1.2\"></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_err());
    }

    #[test]
    fn consume_gpx_full() {
        let gpx = consume!(
            "
            <gpx version=\"1.0\">
                <time>2016-03-27T18:57:55Z</time>
                <bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"></bounds>
                <trk></trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
            </gpx>
            ",
            GpxVersion::Unknown
        );

        assert!(gpx.is_ok());
        let gpx = gpx.unwrap();

        assert_eq!(gpx.version, GpxVersion::Gpx10);
        assert_eq!(gpx.tracks.len(), 1);

        assert_eq!(gpx.waypoints.len(), 2);

        let wpt = &gpx.waypoints[1];
        assert_eq!(wpt.point(), Point::new(10.256, -81.324));
    }
}
