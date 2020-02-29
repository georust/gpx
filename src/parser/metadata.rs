//! metadata handles parsing of GPX-spec metadata.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::bounds;
use parser::extensions;
use parser::link;
use parser::person;
use parser::string;
use parser::time;
use parser::verify_starting_tag;
use parser::Context;

use Metadata;

pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Metadata> {
    let mut metadata: Metadata = Default::default();
    verify_starting_tag(context, "metadata")?;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                next.clone()
            } else {
                break;
            }
        };

        match next_event.chain_err(|| Error::from("error while parsing metadata event"))? {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "name" => {
                    metadata.name = Some(string::consume(context, "name", false)?);
                }
                "description" => {
                    metadata.description = Some(string::consume(context, "description", true)?);
                }
                "author" => {
                    metadata.author = Some(person::consume(context, "author")?);
                }
                "keywords" => {
                    metadata.keywords = Some(string::consume(context, "keywords", true)?);
                }
                "time" => {
                    metadata.time = Some(time::consume(context)?);
                }
                "link" => {
                    metadata.links.push(link::consume(context)?);
                }
                "bounds" => {
                    metadata.bounds = Some(bounds::consume(context)?);
                }
                "extensions" => {
                    extensions::consume(context)?;
                }
                child => {
                    bail!(ErrorKind::InvalidChildElement(
                        String::from(child),
                        "metadata"
                    ));
                }
            },
            XmlEvent::EndElement { ref name } => {
                ensure!(
                    name.local_name == "metadata",
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "metadata")
                );
                context.reader.next(); //consume the end tag
                return Ok(metadata);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    bail!(ErrorKind::MissingClosingTag("metadata"));
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use std::io::BufReader;

    use super::consume;
    use GpxVersion;

    #[test]
    fn consume_empty() {
        let result = consume!("<metadata></metadata>", GpxVersion::Gpx11);

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
            ",
            GpxVersion::Gpx11
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
        assert_eq!(author.email.unwrap(), "john.doe@example.com");
        assert_eq!(author.link.unwrap().href, "example.com");

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
