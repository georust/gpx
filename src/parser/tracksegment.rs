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


/// consume consumes a GPX track segment from the `reader` until it ends.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<TrackSegment> {
    let mut segment: TrackSegment = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "trkseg" => {}
                    "trkpt" => segment.points.push(waypoint::consume(reader)?),
                    _ => {
                        return Err("bad child element".into());
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(segment);
            }

            _ => {}
        }
    }

    return Err("no end tag for track segment".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_full_trkseg() {
        let segment = consume!(
            "
            <trkseg>
                <trkpt lon=\"-77.0365\" lat=\"38.8977\">
                    <name>The White House</name>
                </trkpt>
                <trkpt lon=\"-77.0465\" lat=\"38.8877\">
                    <name>The White House</name>
                </trkpt>
                <trkpt lon=\"-77.0565\" lat=\"38.8777\">
                    <name>The White House</name>
                </trkpt>
            </trkseg>"
        );

        assert!(segment.is_ok());
        let segment = segment.unwrap();

        assert_eq!(segment.points.len(), 3);

        // TODO. Calculates the length of the entire segment.
        // let linestring = segment.linestring();
    }

    #[test]
    fn consume_empty() {
        let segment = consume!("<trkseg></trkseg>");

        assert!(segment.is_ok());
        let segment = segment.unwrap();

        assert_eq!(segment.points.len(), 0);
    }
}
