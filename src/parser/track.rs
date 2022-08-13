//! track handles parsing of GPX-spec tracks.

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{extensions, link, string, tracksegment, verify_starting_tag, Context};
use crate::Track;

/// consume consumes a GPX track from the `reader` until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Track> {
    let mut track = Track::default();
    verify_starting_tag(context, "trk")?;

    loop {
        let next_event = match context.reader.peek() {
            Some(Err(_)) => return Err(GpxError::EventParsingError("Expecting an event")),
            Some(Ok(event)) => event,
            None => break,
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "name" => {
                    track.name = Some(string::consume(context, "name", true)?);
                }
                "cmt" => {
                    track.comment = Some(string::consume(context, "cmt", true)?);
                }
                "desc" => {
                    track.description = Some(string::consume(context, "desc", true)?);
                }
                "src" => {
                    track.source = Some(string::consume(context, "src", true)?);
                }
                "type" => {
                    track._type = Some(string::consume(context, "type", false)?);
                }
                "trkseg" => {
                    track.segments.push(tracksegment::consume(context)?);
                }
                "link" => {
                    track.links.push(link::consume(context)?);
                }
                "number" => {
                    track.number = Some(string::consume(context, "number", false)?.parse()?);
                }
                "extensions" => {
                    extensions::consume(context)?;
                }
                child => {
                    return Err(GpxError::InvalidChildElement(String::from(child), "track"));
                }
            },
            XmlEvent::EndElement { ref name } => {
                if name.local_name != "trk" {
                    return Err(GpxError::InvalidClosingTag(
                        name.local_name.clone(),
                        "track",
                    ));
                }
                context.reader.next(); //consume the end tag
                return Ok(track);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    Err(GpxError::MissingClosingTag("track"))
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

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
            ",
            GpxVersion::Gpx11
        );

        assert!(track.is_ok());

        let track = track.unwrap();

        assert_eq!(track.name.unwrap(), "track name");
        assert_eq!(track.comment.unwrap(), "track comment");
        assert_eq!(track.description.unwrap(), "track description");
        assert_eq!(track.source.unwrap(), "track source");
        assert_eq!(track._type.unwrap(), "track type");
    }

    #[test]
    fn consume_empty() {
        let track = consume!("<trk></trk>", GpxVersion::Gpx11);
        assert!(track.is_ok());
    }
}
