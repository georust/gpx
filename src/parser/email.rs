//! email handles parsing of GPX-spec emails.

extern crate xml;

use errors::*;
use std::io::Read;
use std::iter::Peekable;
use xml::reader::Events;
use xml::reader::XmlEvent;

use Email;

/// consume consumes a GPX email from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX email
/// tag.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Email> {
    let mut email: Email = Default::default();

    while let Some(event) = reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, attributes, .. } => {
                match name.local_name.as_ref() {
                    "email" => {
                        // get required id and domain attributes
                        let id = attributes
                            .iter()
                            .filter(|attr| attr.name.local_name == "id")
                            .nth(0)
                            .ok_or("no id attribute on email tag".to_owned())?;

                        email.id = id.clone().value;

                        let domain = attributes
                            .iter()
                            .filter(|attr| attr.name.local_name == "domain")
                            .nth(0)
                            .ok_or("no domain attribute on email tag".to_owned())?;

                        email.domain = domain.clone().value;
                    }
                    _ => {
                        return Err("cannot have child element in email tag".into());
                    }
                }
            }

            XmlEvent::EndElement { .. } => {
                return Ok(email);
            }

            _ => {}
        }
    }

    return Err("no end tag for email".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_simple_email() {
        let email = consume!("<email id=\"me\" domain=\"example.com\" />");

        assert!(email.is_ok());

        let email = email.unwrap();

        assert_eq!(email.id, "me");
        assert_eq!(email.domain, "example.com");
    }

    #[test]
    fn consume_attrs_reversed() {
        let email = consume!("<email domain=\"example.com\" id=\"me\" />");

        assert!(email.is_ok());

        let email = email.unwrap();

        assert_eq!(email.id, "me");
        assert_eq!(email.domain, "example.com");
    }

    #[test]
    fn consume_err_no_id() {
        let email = consume!("<email domain='example.com'/>");

        assert!(email.is_err());
    }

    #[test]
    fn consume_err_no_domain() {
        let email = consume!("<email id=\"gpx\" />");

        assert!(email.is_err());
    }
}
