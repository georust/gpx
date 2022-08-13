//! person handles parsing of GPX-spec persons.

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{email, link, string, verify_starting_tag, Context};
use crate::Person;

pub fn consume<R: Read>(context: &mut Context<R>, tagname: &'static str) -> GpxResult<Person> {
    let mut person = Person::default();
    verify_starting_tag(context, tagname)?;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => return Err(GpxError::EventParsingError("person")),
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
                    return Err(GpxError::InvalidChildElement(String::from(child), "person"));
                }
            },
            XmlEvent::EndElement { ref name } => {
                // TODO: revisit this (and similar) when https://github.com/rust-lang/rfcs/pull/3137
                // has been on stable for a few versions
                if name.local_name != tagname {
                    return Err(GpxError::InvalidClosingTag(
                        name.local_name.clone(),
                        "person",
                    ));
                }
                context.reader.next(); //consume the end tag
                return Ok(person);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    Err(GpxError::MissingClosingTag("person"))
}

#[cfg(test)]
mod tests {
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
