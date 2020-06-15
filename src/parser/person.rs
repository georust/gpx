//! person handles parsing of GPX-spec persons.

use crate::errors::*;
use std::io::Read;
use xml::reader::XmlEvent;
use error_chain::{bail, ensure};

use crate::parser::email;
use crate::parser::link;
use crate::parser::string;
use crate::parser::verify_starting_tag;
use crate::parser::Context;

use crate::Person;

pub fn consume<R: Read>(context: &mut Context<R>, tagname: &'static str) -> Result<Person> {
    let mut person: Person = Default::default();
    verify_starting_tag(context, tagname)?;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => bail!("error while parsing person event"),
                }
            } else {
                break;
            }
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "name" => person.name = Some(string::consume(context, "name", false)?),
                "email" => person.email = Some(email::consume(context)?),
                "link" => person.link = Some(link::consume(context)?),
                child => {
                    bail!(ErrorKind::InvalidChildElement(
                        String::from(child),
                        "person"
                    ));
                }
            },
            XmlEvent::EndElement { ref name } => {
                ensure!(
                    name.local_name == tagname,
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "person")
                );
                context.reader.next(); //consume the end tag
                return Ok(person);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    bail!(ErrorKind::MissingClosingTag("person"));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::consume;
    use crate::GpxVersion;

    #[test]
    fn consume_whole_person() {
        let result = consume!(
            "
                <person>
                    <name>John Doe</name>
                    <email id=\"john.doe\" domain=\"example.com\" />
                    <link href=\"example.com\">
                        <text>hello world</text>
                        <type>some type</type>
                    </link>
                </person>
            ",
            GpxVersion::Gpx11,
            "person"
        );

        assert!(result.is_ok());
    }
}
