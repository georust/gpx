//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::Context;

use super::verify_starting_tag;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<()> {
    verify_starting_tag(context, "extensions")?;

    let mut depth = 1;
    for event in &mut context.reader {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                // I think its bad to hardcode the check on name == "extensions", because it is not a generic approach
                // and treats inner tags that are called "extensions" differently from any other inner tags, like "a", "foo", "bar"
                // It is correct, but feels wrong, maybe only a personal feeling
                if name.local_name == "extensions" {
                    depth += 1;
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "extensions" {
                    // pop one
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
            }
            _ => {}
        }
    }

    Err(GpxError::MissingClosingTag("extensions"))
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::consume;
    use crate::{errors::GpxError, GpxVersion};

    #[test]
    fn consume_arbitrary_extensions() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
            </extensions>",
            GpxVersion::Gpx11
        );

        assert!(result.is_ok());
    }

    #[test]
    fn consume_arbitrary_nested_extensions() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
                <extensions>
                    hello world
                    <a><b cond=\"no\"><c>derp</c></b></a>
                    <tag>yadda yadda we dont care</tag>
                </extensions>
            </extensions>",
            GpxVersion::Gpx11
        );
        assert!(result.is_ok());
    }

    #[test]
    fn error_on_nested_extensions_with_too_many_opening_tags() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
                <extensions>
                    hello world
                    <a><b cond=\"no\"><c>derp</c></b></a>
                    <tag>yadda yadda we dont care</tag>
                </extensions>
                <extensions>
                <extensions>
              <extensions>",
            GpxVersion::Gpx11
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            GpxError::XmlParseError(err) => match err.kind() {
                xml::reader::ErrorKind::Syntax(err) => {
                    assert_eq!(
                        err,
                        "Unexpected end of stream: still inside the root element"
                    )
                }
                _ => {
                    panic!("expected other error")
                }
            },
            _ => {
                panic!("expected other error")
            }
        };
    }

    #[test]
    fn error_on_invalid_internal_structure() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
                <extensions>
                    hello world
                    <a></extensions><b cond=\"no\"><c>derp</c></b></a>
                    <tag>yadda yadda we dont care</tag>
                </extensions>
              </extensions>",
            GpxVersion::Gpx11
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            GpxError::XmlParseError(err) => match err.kind() {
                xml::reader::ErrorKind::Syntax(err) => {
                    assert_eq!(err, "Unexpected closing tag: extensions, expected a")
                }
                _ => {
                    panic!("expected other error")
                }
            },
            _ => {
                panic!("expected other error")
            }
        };
    }
}
