//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::Context;

/// consume consumes the entire content of the extension tag and returns as string
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<(String)> {
    let mut extension_xml: Option<String> = None;
    let mut depth = 1; //this consume function is called when the opening extension tag was already read (except in tests)
    for event in context.reader() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                if &name.local_name == "extensions" && depth == 1 {
                    //needed because tests have this tag in, actual parsing has it stripped already
                    continue;
                }
                ensure!(
                    &name.local_name != "extensions",
                    "extensions tag opened twice"
                );
                extension_xml = Some(
                    extension_xml.unwrap_or("".to_string()) + "<" + &name.borrow().to_repr() + ">",
                );
                depth += 1;
            }

            XmlEvent::Characters(content) => {
                extension_xml = Some(extension_xml.unwrap_or("".to_string()) + &content)
            }

            XmlEvent::EndElement { name, .. } => {
                depth -= 1;
                ensure!(depth >= 0, "extensions tag contained invalid xml");
                if &name.local_name == "extensions" {
                    ensure!(depth == 0, "extensions tag contained invalid xml");
                    return extension_xml.ok_or("no content inside extensions".into());
                } else {
                    extension_xml = Some(
                        extension_xml.unwrap_or("".to_string()) + "</" + &name.borrow().to_repr()
                            + ">",
                    );
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

    use GpxVersion;
    use parser::Context;
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
