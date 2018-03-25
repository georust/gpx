//! email handles parsing of GPX-spec emails.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::Context;
use parser::verify_starting_tag;

/// consume consumes a GPX email from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX email
/// tag.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<String> {
    let attributes = verify_starting_tag(context, "email")?;
    // get required id and domain attributes
    let id = attributes
        .iter()
        .filter(|attr| attr.name.local_name == "id")
        .nth(0)
        .ok_or(ErrorKind::InvalidElementLacksAttribute("id", "email"))?;

    let domain = attributes
        .iter()
        .filter(|attr| attr.name.local_name == "domain")
        .nth(0)
        .ok_or(ErrorKind::InvalidElementLacksAttribute("domain", "email"))?;

    let email = format!("{id}@{domain}", id = &id.value, domain = &domain.value);

    while let Some(event) = context.reader.next() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { ref name, .. } => {
                bail!(ErrorKind::InvalidChildElement(
                    name.local_name.clone(),
                    "email"
                ));
            }
            XmlEvent::Characters(content) => {
                bail!(ErrorKind::InvalidChildElement(content, "email"));
            }
            XmlEvent::EndElement { ref name } => {
                ensure!(
                    name.local_name == "email",
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "email")
                );
                return Ok(email);
            }
            _ => {} //consume and ignore other events
        }
    }
    bail!(ErrorKind::MissingClosingTag("email"));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use GpxVersion;
    use parser::Context;
    use super::consume;

    #[test]
    fn consume_simple_email() {
        let email = consume!(
            "<email id=\"me\" domain=\"example.com\" />",
            GpxVersion::Gpx11
        );

        assert!(email.is_ok());

        let email = email.unwrap();

        assert_eq!(email, "me@example.com");
    }

    #[test]
    fn consume_attrs_reversed() {
        let email = consume!(
            "<email domain=\"example.com\" id=\"me\" />",
            GpxVersion::Gpx11
        );

        assert!(email.is_ok());

        let email = email.unwrap();

        assert_eq!(email, "me@example.com");
    }

    #[test]
    fn consume_err_no_id() {
        let err = consume!("<email domain='example.com'/>", GpxVersion::Gpx11).unwrap_err();

        assert_eq!(
            err.description(),
            "invalid element, lacks required attribute"
        );
        assert_eq!(
            err.to_string(),
            "invalid element, email lacks required attribute id"
        );
    }

    #[test]
    fn consume_err_no_domain() {
        let err = consume!("<email id=\"gpx\" />", GpxVersion::Gpx11).unwrap_err();

        assert_eq!(
            err.description(),
            "invalid element, lacks required attribute"
        );
        assert_eq!(
            err.to_string(),
            "invalid element, email lacks required attribute domain"
        );
    }

    #[test]
    fn consume_err_invalid_child_element() {
        let err = consume!(
            "<email id=\"id\" domain=\"domain\"><child /></email>",
            GpxVersion::Gpx11
        ).unwrap_err();

        assert_eq!(err.description(), "invalid child element");
        assert_eq!(err.to_string(), "invalid child element 'child' in email");
    }

    #[test]
    fn consume_err_no_ending_tag() {
        let err = consume!("<email id=\"id\" domain=\"domain\">", GpxVersion::Gpx11).unwrap_err();

        assert_eq!(err.description(), "error while parsing XML");
    }
}
