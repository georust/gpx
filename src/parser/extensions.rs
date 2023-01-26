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

    // helper to keep track of inner tags until generic parser exists for extensions-content
    let mut inner_tag_stack = Vec::<String>::new();

    for event in &mut context.reader {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                // push every opening-element on the stack
                // we treat inner "extensions"-tags as any other tag
                let child = name.local_name.clone();
                inner_tag_stack.push(child);
            },
            XmlEvent::EndElement { name } => {
                // as long as there is an inner tag open..
                if let Some(current_inner_tag) = inner_tag_stack.pop() {
                    // the closing tag has to match the current open tag, this makes it also impossible to close the "extensions" while an inner tag is open
                    // handling this here is optional, xml-reader will return an XMLParseError("Unexpected closing tag: extensions, expected ..") before, so this code might be never reached
                    if name.local_name != current_inner_tag {
                      return Err(GpxError::InvalidClosingTag(name.local_name.clone(), "'inner-extensions-tag'"));
                    }                  
                  } else {
                    // otherwise it has to be the "extensions" closing tag
                    if name.local_name != "extensions" {
                      return Err(GpxError::InvalidClosingTag(name.local_name.clone(), "extensions"));
                    }
                    return Ok(());
                }
            },
            _ => { }
        }
    }

    Err(GpxError::MissingClosingTag("extensions"))

}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::consume;
    use crate::{GpxVersion, errors::GpxError};

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
                  assert_eq!(err, "Unexpected end of stream: still inside the root element")
                },
                _ => { panic!("expected other error")}
            },
            _ => { panic!("expected other error") }
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
                },
                _ => { panic!("expected other error")}
            },
            _ => { panic!("expected other error") }
        };
    }
}
