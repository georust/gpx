//! metadata handles parsing of GPX-spec metadata.

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{
    bounds, copyright, extensions, link, person, string, time, verify_starting_tag, Context,
};
use crate::Metadata;

pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Metadata> {
    let mut metadata: Metadata = Default::default();
    verify_starting_tag(context, "metadata")?;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => return Err(GpxError::MetadataParsingError()),
                }
            } else {
                break;
            }
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "name" => {
                    metadata.name = Some(string::consume(context, "name", false)?);
                }
                "desc" => {
                    metadata.description = Some(string::consume(context, "desc", true)?);
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
                "copyright" => {
                    metadata.copyright = Some(copyright::consume(context)?);
                }
                "extensions" => {
                    extensions::consume(context)?;
                }
                child => {
                    return Err(GpxError::InvalidChildElement(
                        String::from(child),
                        "metadata",
                    ));
                }
            },
            XmlEvent::EndElement { ref name } => {
                if name.local_name != "metadata" {
                    return Err(GpxError::InvalidClosingTag(
                        name.local_name.clone(),
                        "metadata",
                    ));
                }
                context.reader.next(); //consume the end tag
                return Ok(metadata);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    Err(GpxError::MissingClosingTag("metadata"))
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "time"))]
    use chrono::{TimeZone, Utc};

    use super::consume;
    use crate::GpxVersion;
    #[cfg(feature = "time")]
    use time::{Date, Month, PrimitiveDateTime, Time};

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
                <desc>xxdescription</desc>
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

        #[cfg(not(feature = "time"))]
        let expect = Utc.ymd(2017, 8, 16).and_hms_micro(4, 3, 33, 735_000);
        #[cfg(feature = "time")]
        let expect = PrimitiveDateTime::new(
            Date::from_calendar_date(2017, Month::August, 16).unwrap(),
            Time::from_hms_milli(4, 3, 33, 735).unwrap(),
        )
        .assume_utc();

        assert_eq!(result.time.unwrap(), expect);

        assert_eq!(result.links.len(), 1);
    }
}
