//! metadata handles parsing of GPX-spec metadata.

extern crate xml;

use errors::*;
use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::link;
use parser::string;
use parser::person;
use parser::time;

use Metadata;

pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Metadata> {
    let mut metadata: Metadata = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => metadata.name = Some(string::consume(reader)?),
                    "description" => metadata.description = Some(string::consume(reader)?),
                    "author" => metadata.author = Some(person::consume(reader)?),
                    "keywords" => metadata.keywords = Some(string::consume(reader)?),
                    "time" => metadata.time = Some(time::consume(reader)?),
                    "link" => metadata.links.push(link::consume(reader)?),
                    "metadata" => {}
                    _ => {
                        return Err("bad child element for metadata".into());
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(metadata);
            }

            _ => {}
        }
    }

    return Err("no end tag for metadata".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;
    use chrono::prelude::*;

    use super::consume;

    #[test]
    fn consume_empty() {
        let result = consume!("<metadata></metadata>");

        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.name.is_none());
        assert!(result.description.is_none());
        assert!(result.author.is_none());
        assert!(result.keywords.is_none());
        assert!(result.time.is_none());
    }

    #[test]
    fn consume_metadata() {
        let result = consume!(
            "
            <metadata>
                <link href=\"example.com\" />
                <name>xxname</name>
                <description>xxdescription</description>
                <author>
                    <name>John Doe</name>
                    <email id=\"john.doe\" domain=\"example.com\" />
                    <link href=\"example.com\">
                        <text>hello world</text>
                        <type>some type</type>
                    </link>
                </author>
                <keywords>some keywords here</keywords>
                <time>2017-08-16T04:03:33.735Z</time>
            </metadata>
            "
        );

        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.name.is_some());
        assert_eq!(result.name.unwrap(), "xxname");

        assert!(result.description.is_some());
        assert_eq!(result.description.unwrap(), "xxdescription");

        assert!(result.author.is_some());
        let author = result.author.unwrap();

        assert_eq!(author.name.unwrap(), "John Doe");
        assert!(author.email.is_some());
        assert!(author.link.is_some());

        assert!(result.keywords.is_some());
        assert_eq!(result.keywords.unwrap(), "some keywords here");

        assert!(result.time.is_some());
        assert_eq!(
            result.time.unwrap(),
            Utc.ymd(2017, 8, 16).and_hms_micro(4, 3, 33, 735_000)
        );

        assert_eq!(result.links.len(), 1);
    }
}
