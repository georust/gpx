//! Handles parsing GPX format.

// Just a shared macro for testing 'consume'.
#[cfg(test)]
#[macro_export]
macro_rules! consume {
    ($xml:expr, $version:expr) => {{
        use std::io::BufReader;
        use $crate::parser::create_context;
        consume(&mut create_context(
            BufReader::new($xml.as_bytes()),
            $version,
        ))
    }};
    ($xml:expr, $version:expr, $tagname:expr) => {{
        use std::io::BufReader;
        use $crate::parser::create_context;
        consume(
            &mut create_context(BufReader::new($xml.as_bytes()), $version),
            $tagname,
        )
    }};
    ($xml:expr, $version:expr, $tagname:expr, $allow_empty:expr) => {{
        use std::io::BufReader;
        use $crate::parser::create_context;
        consume(
            &mut create_context(BufReader::new($xml.as_bytes()), $version),
            $tagname,
            $allow_empty,
        )
    }};
}

pub mod bounds;
pub mod copyright;
pub mod email;
pub mod extensions;
pub mod fix;
pub mod gpx;
pub mod link;
pub mod metadata;
pub mod person;
pub mod route;
pub mod string;
pub mod time;
pub mod track;
pub mod tracksegment;
pub mod waypoint;

use std::io::Read;
use std::iter::Peekable;

use xml::attribute::OwnedAttribute;
use xml::reader::{Events, XmlEvent};
use xml::{EventReader, ParserConfig};

use crate::errors::GpxError;
use crate::types::GpxVersion;

pub struct Context<R: Read> {
    reader: Peekable<Events<R>>,
    version: GpxVersion,
}

impl<R: Read> Context<R> {
    pub fn new(reader: Peekable<Events<R>>, version: GpxVersion) -> Context<R> {
        Context { reader, version }
    }

    pub fn reader(&mut self) -> &mut Peekable<Events<R>> {
        &mut self.reader
    }
}

pub fn verify_starting_tag<R: Read>(
    context: &mut Context<R>,
    local_name: &'static str,
) -> Result<Vec<OwnedAttribute>, GpxError> {
    //makes sure the specified starting tag is the next tag on the stream
    //we ignore and skip all xmlevents except StartElement, Characters and EndElement
    loop {
        let next = context.reader.next();
        match next {
            Some(Ok(XmlEvent::StartElement {
                name, attributes, ..
            })) => {
                if name.local_name != local_name {
                    return Err(GpxError::InvalidChildElement(name.local_name, local_name));
                } else {
                    return Ok(attributes);
                }
            }
            Some(Ok(XmlEvent::EndElement { name, .. })) => {
                return Err(GpxError::InvalidChildElement(name.local_name, local_name));
            }
            Some(Ok(XmlEvent::Characters(chars))) => {
                return Err(GpxError::InvalidChildElement(chars, local_name));
            }
            Some(_) => {} //ignore other elements
            None => return Err(GpxError::MissingOpeningTag(local_name)),
        }
    }
}

pub(crate) fn create_context<R: Read>(reader: R, version: GpxVersion) -> Context<R> {
    let parser_config = ParserConfig {
        whitespace_to_characters: true, //convert Whitespace event to Characters
        cdata_to_characters: true,      //convert CData event to Characters
        ..ParserConfig::new()
    };
    let parser = EventReader::new_with_config(reader, parser_config);
    let events = parser.into_iter().peekable();
    Context::new(events, version)
}
