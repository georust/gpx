//! link handles parsing of GPX-spec links.

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::string;

use Link;

/// consume consumes a GPX link from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX link
/// tag.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Link> {
    let mut link: Link = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                match name.local_name.as_ref() {
                    "text" => link.text = Some(string::consume(reader)?),
                    "type" => link._type = Some(string::consume(reader)?),
                    "link" => {
                        // retrieve mandatory href attribute
                        let attr = attributes
                            .into_iter()
                            .filter(|attr| attr.name.local_name == "href")
                            .nth(0);

                        let attr = attr.ok_or("no href attribute on link tag".to_owned())?;

                        link.href = attr.value;
                    }
                    child => Err(Error::from(ErrorKind::InvalidChildElement(
                        String::from(child),
                        "link",
                    )))?,
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(link);
            }

            _ => {}
        }
    }

    return Err("no end tag for link".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_simple_link() {
        let link =
            consume!("<link href='http://example.com'><text>hello</text><type>world</type></link>");

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
        let link = consume!("<link href='http://topografix.com'></link>");

        assert!(link.is_ok());

        let link = link.unwrap();

        assert_eq!(link.href, "http://topografix.com");

        assert!(link.text.is_none());
        assert!(link._type.is_none());
    }

    #[test]
    fn consume_no_href() {
        let link = consume!("<link></link>");

        assert!(link.is_err());
    }
}
