//! person handles parsing of GPX-spec persons.

extern crate xml;

use errors::*;
use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;

use parser::email;
use parser::link;
use parser::string;

/// Person represents a person or organization.
#[derive(Default)]
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email address.
    pub email: Option<email::Email>,

    /// Link to Web site or other external information about person.
    pub link: Option<link::Link>,
}

pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Person> {
    let mut person: Person = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_ref() {
                    "name" => person.name = Some(string::consume(reader)?),
                    "email" => person.email = Some(email::consume(reader)?),
                    "link" => person.link = Some(link::consume(reader)?),
                    "person" => {}
                    _ => {
                        return Err(
                            "cannot have child element besides name, email, and link".into(),
                        );
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(person);
            }

            _ => {}
        }
    }

    return Err("no end tag for person".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

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
                "
        );

        assert!(result.is_ok());
    }
}
