//! tracksegment handles parsing of GPX-spec track segments.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::waypoint;

use TrackSegment;

enum TrackSegmentEvent {
    StartTrkSeg,
    StartTrkPt,
    EndTrkSeg,
    Ignore,
}


/// consume consumes a GPX track segment from the `reader` until it ends.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<TrackSegment> {
    let mut segment: TrackSegment = Default::default();

    loop {
        // Peep into the reader and see what type of event is next. Based on
        // that information, we'll either forward the event to a downstream
        // module or take the information for ourselves.
        let event: Result<TrackSegmentEvent> = {
            if let Some(next) = reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement { ref name, .. }) => {
                        match name.local_name.as_ref() {
                            "trkseg" => Ok(TrackSegmentEvent::StartTrkSeg),
                            "trkpt" => Ok(TrackSegmentEvent::StartTrkPt),
                            _ => Err("unknown child element".into()),
                        }
                    }

                    &Ok(XmlEvent::EndElement { .. }) => Ok(TrackSegmentEvent::EndTrkSeg),

                    _ => Ok(TrackSegmentEvent::Ignore),
                }
            } else {
                break;
            }
        };

        match event.chain_err(|| {
            Error::from("error while parsing track segment event")
        })? {
            TrackSegmentEvent::StartTrkSeg => {
                reader.next();
            }

            TrackSegmentEvent::StartTrkPt => {
                segment.points.push(waypoint::consume(reader)?);
            }

            TrackSegmentEvent::EndTrkSeg => {
                reader.next();

                return Ok(segment);
            }

            TrackSegmentEvent::Ignore => {
                reader.next();
            }
        }
    }

    return Err("no end tag for track segment".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;
    use geo::length::Length;

    use super::consume;

    #[test]
    fn consume_full_trkseg() {
        let segment = consume!(
            "
            <trkseg>
                <trkpt lon=\"-77.0365\" lat=\"38.8977\">
                    <name>The White House</name>
                </trkpt>
                <trkpt lon=\"-71.063611\" lat=\"42.358056\">
                    <name>Boston, Massachusetts</name>
                </trkpt>
                <trkpt lon=\"-69.7832\" lat=\"44.31055\">
                    <name>Augusta, Maine</name>
                </trkpt>
            </trkseg>"
        );

        assert!(segment.is_ok());
        let segment = segment.unwrap();

        assert_eq!(segment.points.len(), 3);

        let linestring = segment.linestring();
        assert_approx_eq!(linestring.length(), 9.2377437);
    }

    #[test]
    fn consume_empty() {
        let segment = consume!("<trkseg></trkseg>");

        assert!(segment.is_ok());
        let segment = segment.unwrap();

        assert_eq!(segment.points.len(), 0);
    }
}
