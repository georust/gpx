//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::Context;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<()> {
    let mut started = false;

    for event in context.reader() {
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

    use GpxVersion;
    use super::consume;

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
