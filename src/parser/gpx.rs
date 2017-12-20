//! gpx handles parsing of GPX elements.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::track;
use parser::metadata;
use parser::waypoint;

use Gpx;

enum ParseEvent {
    StartMetadata,
    StartTrack,
    StartWaypoint,
    Ignore,
    EndGpx,
}

/// consume consumes an entire GPX element.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Gpx> {
    let mut gpx: Gpx = Default::default();

    loop {
        // Peep into the reader and see what type of event is next. Based on
        // that information, we'll either forward the event to a downstream
        // module or take the information for ourselves.
        let event: Result<ParseEvent> = {
            if let Some(next) = reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement { ref name, .. }) => {
                        match name.local_name.as_ref() {
                            "metadata" => Ok(ParseEvent::StartMetadata),
                            "trk" => Ok(ParseEvent::StartTrack),
                            "wpt" => Ok(ParseEvent::StartWaypoint),
                            "gpx" => Ok(ParseEvent::Ignore),
                            _ => Err(Error::from(ErrorKind::InvalidChildElement("gpx")))?,
                        }
                    }

                    &Ok(XmlEvent::EndElement { .. }) => Ok(ParseEvent::EndGpx),

                    _ => Ok(ParseEvent::Ignore),
                }
            } else {
                break;
            }
        };

        match event.chain_err(
            || Error::from("error while parsing gpx event"),
        )? {
            ParseEvent::Ignore => {
                reader.next();
            }

            ParseEvent::StartMetadata => {
                gpx.metadata = Some(metadata::consume(reader)?);
            }

            ParseEvent::StartTrack => {
                gpx.tracks.push(track::consume(reader)?);
            }

            ParseEvent::StartWaypoint => {
                gpx.waypoints.push(waypoint::consume(reader)?);
            }

            ParseEvent::EndGpx => {
                reader.next();

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

    use super::consume;

    #[test]
    fn consume_gpx() {
        let gpx = consume!("<gpx></gpx>");

        assert!(gpx.is_ok());
    }

    #[test]
    fn consume_gpx_full() {
        let gpx = consume!(
            "
            <gpx>
                <trk></trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
            </gpx>
            "
        );

        assert!(gpx.is_ok());
        let gpx = gpx.unwrap();

        assert_eq!(gpx.tracks.len(), 1);

        assert_eq!(gpx.waypoints.len(), 2);

        let wpt = &gpx.waypoints[1];
        assert_eq!(wpt.point(), Point::new(10.256, -81.324));
    }
}
