//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::GpxError;
use crate::parser::Context;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<(), GpxError> {
    let mut started = false;

    for event in context.reader() {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                // flip started depending on conditions
                if &name.local_name == "extensions" {
                    if started {
                        return Err(GpxError::TagOpenedTwice("extensions"));
                    }

                    started = true;
                }
            }

            XmlEvent::EndElement { name, .. } => {
                if &name.local_name == "extensions" {
                    return Ok(());
                }
            }

            _ => {}
        }
    }

    Err(GpxError::MissingClosingTag("extensions"))
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

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
}
