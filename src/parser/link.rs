//! link handles parsing of GPX-spec links.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::string;
use parser::verify_starting_tag;
use parser::Context;

use Link;

/// consume consumes a GPX link from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX link
/// tag.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Link> {
    let mut link: Link = Default::default();
    let attributes = verify_starting_tag(context, "link")?;
    let attr = attributes
        .into_iter()
        .filter(|attr| attr.name.local_name == "href")
        .nth(0);

    let attr = attr.ok_or(ErrorKind::InvalidElementLacksAttribute("href", "link"))?;

    link.href = attr.value;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                next.clone()
            } else {
                break;
            }
        };

        match next_event.chain_err(|| Error::from("error while parsing link event"))? {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "text" => link.text = Some(string::consume(context, "text", false)?),
                "type" => link._type = Some(string::consume(context, "type", false)?),
                child => {
                    bail!(ErrorKind::InvalidChildElement(String::from(child), "link"));
                }
            },
            XmlEvent::EndElement { ref name } => {
                ensure!(
                    name.local_name == "link",
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "link")
                );
                context.reader.next();
                return Ok(link);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    bail!(ErrorKind::MissingClosingTag("link"));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::consume;
    use GpxVersion;

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

        assert!(link._type.is_some());
        assert_eq!(link._type.unwrap(), "world");
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
        assert!(link._type.is_none());
    }

    #[test]
    fn consume_no_href() {
        let link = consume!("<link></link>", GpxVersion::Gpx11);

        assert!(link.is_err());
    }
}
