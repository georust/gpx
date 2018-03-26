//! gpx handles parsing of GPX elements.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;
use geo::Bbox;
use chrono::{DateTime, Utc};

use parser::bounds;
use parser::time;
use parser::string;
use parser::track;
use parser::metadata;
use parser::waypoint;
use parser::Context;

use Gpx;
use GpxVersion;
use Link;
use Person;
use Metadata;

enum ParseEvent {
    StartAuthor,      // GPX 1.0
    StartBounds,      // GPX 1.0
    StartDescription, // GPX 1.0
    StartEmail,       // GPX 1.0
    StartKeywords,    // GPX 1.0
    StartMetadata,
    StartName, // GPX 1.0
    StartTime, // GPX 1.0
    StartTrack,
    StartUrl,     // GPX 1.0
    StartUrlname, // GPX 1.0
    StartWaypoint,
    Ignore,
    EndGpx,
}

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
    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut keywords: Option<String> = None;

    loop {
        // Peep into the reader and see what type of event is next. Based on
        // that information, we'll either forward the event to a downstream
        // module or take the information for ourselves.
        let event: Result<ParseEvent> = {
            if let Some(next) = context.reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    }) => match name.local_name.as_ref() {
                        "metadata" if context.version != GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartMetadata)
                        }
                        "trk" => Ok(ParseEvent::StartTrack),
                        "wpt" => Ok(ParseEvent::StartWaypoint),
                        "gpx" => {
                            if let Ok(version) = attributes
                                .iter()
                                .filter(|attr| attr.name.local_name == "version")
                                .nth(0)
                                .ok_or("no version found".to_owned())
                            {
                                gpx.version = version_string_to_version(&version.value)?;
                                context.version = gpx.version;
                                Ok(ParseEvent::Ignore)
                            } else {
                                Err(Error::from(ErrorKind::InvalidElementLacksAttribute(
                                    "version",
                                    "gpx",
                                )))
                            }
                        }
                        "time" if context.version == GpxVersion::Gpx10 => Ok(ParseEvent::StartTime),
                        "bounds" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartBounds)
                        }
                        "author" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartAuthor)
                        }
                        "email" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartEmail)
                        }
                        "url" if context.version == GpxVersion::Gpx10 => Ok(ParseEvent::StartUrl),
                        "urlname" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartUrlname)
                        }
                        "name" if context.version == GpxVersion::Gpx10 => Ok(ParseEvent::StartName),
                        "description" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartDescription)
                        }
                        "keywords" if context.version == GpxVersion::Gpx10 => {
                            Ok(ParseEvent::StartKeywords)
                        }
                        child => Err(Error::from(ErrorKind::InvalidChildElement(
                            String::from(child),
                            "gpx",
                        )))?,
                    },

                    &Ok(XmlEvent::EndElement { .. }) => Ok(ParseEvent::EndGpx),

                    _ => Ok(ParseEvent::Ignore),
                }
            } else {
                break;
            }
        };

        match event.chain_err(|| Error::from("error while parsing gpx event"))? {
            ParseEvent::Ignore => {
                context.reader.next();
            }

            ParseEvent::StartAuthor => {
                author = Some(string::consume(context)?);
            }

            ParseEvent::StartBounds => {
                bounds = Some(bounds::consume(context)?);
            }

            ParseEvent::StartEmail => {
                email = Some(string::consume(context)?);
            }

            ParseEvent::StartMetadata => {
                gpx.metadata = Some(metadata::consume(context)?);
            }

            ParseEvent::StartName => {
                name = Some(string::consume(context)?);
            }

            ParseEvent::StartTime => {
                time = Some(time::consume(context)?);
            }

            ParseEvent::StartUrl => {
                url = Some(string::consume(context)?);
            }

            ParseEvent::StartUrlname => {
                urlname = Some(string::consume(context)?);
            }

            ParseEvent::StartDescription => {
                description = Some(string::consume(context)?);
            }

            ParseEvent::StartKeywords => {
                keywords = Some(string::consume(context)?);
            }

            ParseEvent::StartTrack => {
                gpx.tracks.push(track::consume(context)?);
            }

            ParseEvent::StartWaypoint => {
                gpx.waypoints.push(waypoint::consume(context)?);
            }

            ParseEvent::EndGpx => {
                if gpx.version == GpxVersion::Gpx10 {
                    let mut metadata: Metadata = Default::default();
                    metadata.name = name;
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
        }
    }

    return Err("no end tag for gpx".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;
    use geo::Point;

    use GpxVersion;
    use parser::Context;
    use super::consume;

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
