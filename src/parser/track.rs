//! track handles parsing of GPX-spec tracks.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use geo::{ToGeo, Geometry};
use geo::MultiLineString;

use parser::string;
use parser::link;
use parser::tracksegment;

/// Track represents an ordered list of points describing a path.
#[derive(Default, Debug)]
pub struct Track {
    /// GPS name of track.
    pub name: Option<String>,

    /// GPS comment for track.
    pub cmt: Option<String>,

    /// User description of track.
    pub desc: Option<String>,

    /// Source of data. Included to give user some idea of reliability
    /// and accuracy of data.
    pub src: Option<String>,

    /// Links to external information about the track.
    pub links: Vec<link::Link>,

    /// Type (classification) of track.
    pub _type: Option<String>,

    /// A Track Segment holds a list of Track Points which are logically
    /// connected in order. To represent a single GPS track where GPS reception
    /// was lost, or the GPS receiver was turned off, start a new Track Segment
    /// for each continuous span of track data.
    pub segments: Vec<tracksegment::TrackSegment>,

    /* pub number: u8,*/
    /* extensions */
    /* trkSeg */
}

impl Track {
    /// Gives the multi-linestring that this track represents, which is multiple
    /// linestrings.
    pub fn multilinestring(&self) -> MultiLineString<f64> {
        self.segments.iter().map(|seg| seg.linestring()).collect()
    }
}

impl ToGeo<f64> for Track {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::MultiLineString(self.multilinestring())
    }
}


/// consume consumes a GPX track from the `reader` until it ends.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Track> {
    let mut track: Track = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "trk" => {}
                    "name" => track.name = Some(string::consume(reader)?),
                    "cmt" => track.cmt = Some(string::consume(reader)?),
                    "desc" => track.desc = Some(string::consume(reader)?),
                    "src" => track.src = Some(string::consume(reader)?),
                    "type" => track._type = Some(string::consume(reader)?),
                    "trkseg" => track.segments.push(tracksegment::consume(reader)?),
                    _ => {
                        return Err("bad child element".into());
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(track);
            }

            _ => {}
        }
    }

    return Err("no end tag for track".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_full_track() {
        let track = consume!(
            "
            <trk>
                <name>track name</name>
                <cmt>track comment</cmt>
                <desc>track description</desc>
                <src>track source</src>
                <type>track type</type>
            </trk>
            "
        );

        assert!(track.is_ok());

        let track = track.unwrap();

        assert_eq!(track.name.unwrap(), "track name");
        assert_eq!(track.cmt.unwrap(), "track comment");
        assert_eq!(track.desc.unwrap(), "track description");
        assert_eq!(track.src.unwrap(), "track source");
        assert_eq!(track._type.unwrap(), "track type");
    }

    #[test]
    fn consume_empty() {
        let track = consume!("<trk></trk>");

        assert!(track.is_ok());
    }
}
