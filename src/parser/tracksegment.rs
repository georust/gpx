//! tracksegment handles parsing of GPX-spec track segments.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use geo::{ToGeo, Geometry};
use geo::LineString;

use parser::waypoint;

/// TrackSegment represents a list of track points.

/// This TrackSegment holds a list of Track Points which are logically
/// connected in order. To represent a single GPS track where GPS reception
/// was lost, or the GPS receiver was turned off, start a new Track Segment
/// for each continuous span of track data.
#[derive(Default, Debug)]
pub struct TrackSegment {
    /// Each Waypoint holds the coordinates, elevation, timestamp, and metadata
    /// for a single point in a track.
    pub points: Vec<waypoint::Waypoint>,
    /* extensions */
}

impl TrackSegment {
    /// Gives the linestring of the segment's points, the sequence of points that
    /// comprises the track segment.
    pub fn linestring(&self) -> LineString<f64> {
        self.points.iter().map(|wpt| wpt.point()).collect()
    }
}

impl ToGeo<f64> for TrackSegment {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::LineString(self.linestring())
    }
}


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
        let event: Option<TrackSegmentEvent> = {
            if let Some(next) = reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement { ref name, .. }) => {
                        match name.local_name.as_ref() {
                            "trkseg" => Some(TrackSegmentEvent::StartTrkSeg),
                            "trkpt" => Some(TrackSegmentEvent::StartTrkPt),
                            _ => None,
                        }
                    }

                    &Ok(XmlEvent::EndElement { .. }) => {
                        Some(TrackSegmentEvent::EndTrkSeg)
                    }

                    _ => Some(TrackSegmentEvent::Ignore),
                }
            } else {
                break
            }
        };

        if event.is_none() {
            return Err("error while parsing track segment".into());
        }

        match event.unwrap() {
            TrackSegmentEvent::StartTrkSeg => {
                reader.next();
            },

            TrackSegmentEvent::StartTrkPt => {
                segment.points.push(waypoint::consume(reader)?);
            },

            TrackSegmentEvent::EndTrkSeg => {
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
