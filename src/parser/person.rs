//! person handles parsing of GPX-spec persons.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::email;
use parser::link;
use parser::string;
use parser::Context;

use Person;

enum ParseEvent {
    StartName,
    StartEmail,
    StartLink,
    EndPerson,
    Ignore,
}

pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Person> {
    let mut person: Person = Default::default();

    loop {
        // Peep into the reader and see what type of event is next. Based on
        // that information, we'll either forward the event to a downstream
        // module or take the information for ourselves.
        let event: Result<ParseEvent> = {
            if let Some(next) = context.reader.peek() {
                match next {
                    &Ok(XmlEvent::StartElement { ref name, .. }) => {
                        match name.local_name.as_ref() {
                            "name" => Ok(ParseEvent::StartName),
                            "email" => Ok(ParseEvent::StartEmail),
                            "link" => Ok(ParseEvent::StartLink),
                            "person" => Ok(ParseEvent::Ignore),
                            "author" => Ok(ParseEvent::Ignore),
                            child => Err(Error::from(ErrorKind::InvalidChildElement(
                                String::from(child),
                                "person",
                            ))),
                        }
                    }

                    &Ok(XmlEvent::EndElement { .. }) => Ok(ParseEvent::EndPerson),

                    _ => Ok(ParseEvent::Ignore),
                }
            } else {
                break;
            }
        };

        match event.chain_err(|| Error::from("error while parsing person event"))? {
            ParseEvent::Ignore => {
                context.reader.next();
            }

            ParseEvent::StartName => {
                person.name = Some(string::consume(context)?);
            }

            ParseEvent::StartEmail => {
                person.email = Some(email::consume(context)?);
            }

            ParseEvent::StartLink => {
                person.link = Some(link::consume(context)?);
            }

            ParseEvent::EndPerson => {
                context.reader.next();

                return Ok(person);
            }
        }
    }

    unreachable!("should return by now");
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use GpxVersion;
    use parser::Context;
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
                ",
            GpxVersion::Gpx11
        );

        assert!(result.is_ok());
    }
}
