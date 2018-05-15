//! metadata handles parsing of GPX-spec metadata.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::bounds;
use parser::link;
use parser::person;
use parser::string;
use parser::time;
use parser::Context;

use Metadata;

enum ParseEvent {
    StartName,
    StartDescription,
    StartAuthor,
    StartKeywords,
    StartTime,
    StartLink,
    StartBounds,
    Ignore,
    EndMetadata,
}

pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Metadata> {
    let mut metadata: Metadata = Default::default();

    loop {
        // Peep into the reader and see what type of event is next. Based on
        // that information, we'll either forward the event to a downstream
        // module or take the information for ourselves.
        let event: Result<ParseEvent> = {
            if let Some(next) = context.reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement { ref name, .. }) => {
                        match name.local_name.as_ref() {
                            "metadata" => Ok(ParseEvent::Ignore),
                            "name" => Ok(ParseEvent::StartName),
                            "description" => Ok(ParseEvent::StartDescription),
                            "author" => Ok(ParseEvent::StartAuthor),
                            "keywords" => Ok(ParseEvent::StartKeywords),
                            "time" => Ok(ParseEvent::StartTime),
                            "link" => Ok(ParseEvent::StartLink),
                            "bounds" => Ok(ParseEvent::StartBounds),
                            child => Err(Error::from(ErrorKind::InvalidChildElement(
                                String::from(child),
                                "metadata",
                            )))?,
                        }
                    }

                    &Ok(XmlEvent::EndElement { .. }) => Ok(ParseEvent::EndMetadata),

                    _ => Ok(ParseEvent::Ignore),
                }
            } else {
                break;
            }
        };

        match event.chain_err(|| Error::from("error while parsing gpx event"))? {
            ParseEvent::Ignore => {
                context.reader.next();
            }

            ParseEvent::StartName => {
                metadata.name = Some(string::consume(context)?);
            }

            ParseEvent::StartDescription => {
                metadata.description = Some(string::consume(context)?);
            }

            ParseEvent::StartAuthor => {
                metadata.author = Some(person::consume(context)?);
            }

            ParseEvent::StartKeywords => {
                metadata.keywords = Some(string::consume(context)?);
            }

            ParseEvent::StartTime => {
                metadata.time = Some(time::consume(context)?);
            }

            ParseEvent::StartLink => {
                metadata.links.push(link::consume(context)?);
            }

            ParseEvent::StartBounds => {
                metadata.bounds = Some(bounds::consume(context)?);
            }

            ParseEvent::EndMetadata => {
                context.reader.next();

                return Ok(metadata);
            }
        }
    }

    return Err("no end tag for metadata".into());
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;
    use parser::Context;
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
