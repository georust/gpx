//! track handles parsing of GPX-spec tracks.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::string;
use parser::link;
use parser::tracksegment;

/// Track represents a `trkType`.
///
/// > An ordered list of points describing a path.
///
/// ```xml
/// <...>
///   <name> xsd:string </name> [0..1] ?
///   <cmt> xsd:string </cmt> [0..1] ?
///   <desc> xsd:string </desc> [0..1] ?
///   <src> xsd:string </src> [0..1] ?
///   <link> linkType </link> [0..*] ?
///   <number> xsd:nonNegativeInteger </number> [0..1] ?
///   <type> xsd:string </type> [0..1] ?
///   <extensions> extensionsType </extensions> [0..1] ?
///   <trkseg> trksegType </trkseg> [0..*] ?
/// </...>
/// ```
#[derive(Default)]
pub struct Track {
    pub name: Option<String>,
    pub cmt: Option<String>,
    pub desc: Option<String>,
    pub src: Option<String>,
    pub links: Vec<link::Link>,
    /* pub number: u8,*/
    pub _type: Option<String>,
    /* extesions */
    pub segments: Vec<tracksegment::TrackSegment>,
    /* trkSeg */
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
