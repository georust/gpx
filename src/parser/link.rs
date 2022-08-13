//! link handles parsing of GPX-spec links.

use std::io::Read;

use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{string, verify_starting_tag, Context};
use crate::Link;

/// Try to create a [`Link`] from an attribute list
fn try_from_attributes(attributes: &[OwnedAttribute]) -> GpxResult<Link> {
    let href_attribute = attributes
        .iter()
        .find(|attr| attr.name.local_name == "href")
        .ok_or(GpxError::InvalidElementLacksAttribute("href", "link"))?;

    Ok(Link {
        href: href_attribute.value.clone(),
        ..Default::default()
    })
}

/// consume consumes a GPX link from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX link
/// tag.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Link> {
    let attributes = verify_starting_tag(context, "link")?;
    let mut link = try_from_attributes(&attributes)?;

    loop {
        let next_event = match context.reader.peek() {
            Some(Err(_)) => return Err(GpxError::EventParsingError("Expecting an event")),
            Some(Ok(event)) => event,
            None => break,
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "text" => link.text = Some(string::consume(context, "text", false)?),
                "type" => link.type_ = Some(string::consume(context, "type", false)?),
                child => {
                    return Err(GpxError::InvalidChildElement(String::from(child), "link"));
                }
            },
            XmlEvent::EndElement { ref name } => {
                if name.local_name != "link" {
                    return Err(GpxError::InvalidClosingTag(name.local_name.clone(), "link"));
                }
                context.reader.next();
                return Ok(link);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    Err(GpxError::MissingClosingTag("link"))
}

#[cfg(test)]
mod tests {
    use crate::GpxVersion;

    use super::consume;

    #[test]
    fn consume_simple_link() {
        let link = consume!(
            "<link href='http://example.com'><text>hello</text><type>world</type></link>",
            GpxVersion::Gpx11
        );

        assert!(link.is_ok());

        let link = link.unwrap();

        assert_eq!(link.href, "http://example.com");

        assert!(link.text.is_some());
        assert_eq!(link.text.unwrap(), "hello");

        assert!(link.type_.is_some());
        assert_eq!(link.type_.unwrap(), "world");
    }

    #[test]
    fn consume_barebones() {
        let link = consume!(
            "<link href='http://topografix.com'></link>",
            GpxVersion::Gpx11
        );

        assert!(link.is_ok());

        let link = link.unwrap();

        assert_eq!(link.href, "http://topografix.com");

        assert!(link.text.is_none());
        assert!(link.type_.is_none());
    }

    #[test]
    fn consume_no_href() {
        let link = consume!("<link></link>", GpxVersion::Gpx11);

        assert!(link.is_err());
    }
}
