//! string handles parsing of GPX-spec strings.

use errors::*;
use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<String> {
    let mut element: Option<String> = None;
    let mut string: Option<String> = None;

    for event in reader {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                ensure!(element.is_none(), "cannot start element inside string");

                element = Some(name.local_name);
            }

            XmlEvent::Characters(content) => string = Some(content),

            XmlEvent::EndElement { .. } => {
                return string.ok_or("no content inside string".into());
            }

            _ => {}
        }
    }

    return Err("no end tag for string".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_simple_string() {
        let result = consume!("<string>hello world</string>");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn consume_new_tag() {
        // cannot start new tag inside string
        let result = consume!("<foo>bar<baz></baz></foo>");

        assert!(result.is_err());
    }

    #[test]
    fn consume_start_tag() {
        // must have starting tag
        let result = consume!("bar</foo>");

        assert!(result.is_err());
    }

    #[test]
    fn consume_end_tag() {
        // must have ending tag
        let result = consume!("<foo>bar");

        assert!(result.is_err());
    }

    #[test]
    fn consume_no_body() {
        // must have string content
        let result = consume!("<foo></foo>");

        assert!(result.is_err());
    }

    #[test]
    fn consume_different_ending_tag() {
        // this is just illegal
        let result = consume!("<foo></foobar>");

        assert!(result.is_err());
    }
}
