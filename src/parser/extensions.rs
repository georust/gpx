//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

extern crate xml;

use errors::*;
use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<()> {
    let mut started = false;

    for event in reader {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                // flip started depending on conditions
                if &name.local_name == "extensions" {
                    ensure!(!started, "extensions tag opened twice");

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

    return Err("no end tag for extensions".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_arbitrary_extensions() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
            </extensions>"
        );

        assert!(result.is_ok());
    }
}
