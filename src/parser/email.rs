//! email handles parsing of GPX-spec emails.

use std::io::Read;

use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{verify_starting_tag, Context};

/// consume consumes a GPX email from the `reader` until it ends.
/// When it returns, the reader will be at the element after the end GPX email
/// tag.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<String> {
    let attributes = verify_starting_tag(context, "email")?;
    // get required id and domain attributes
    let id = attributes
        .iter()
        .find(|attr| attr.name.local_name == "id")
        .ok_or(GpxError::InvalidElementLacksAttribute("id", "email"))?;

    let domain = attributes
        .iter()
        .find(|attr| attr.name.local_name == "domain")
        .ok_or(GpxError::InvalidElementLacksAttribute("domain", "email"))?;

    let email = format!("{id}@{domain}", id = &id.value, domain = &domain.value);

    for event in &mut context.reader {
        match event? {
            XmlEvent::StartElement { ref name, .. } => {
                return Err(GpxError::InvalidChildElement(
                    name.local_name.clone(),
                    "email",
                ));
            }
            XmlEvent::Characters(characters) => {
                return Err(GpxError::InvalidChildElement(characters, "email"));
            }
            XmlEvent::EndElement { ref name } => {
                if name.local_name != "email" {
                    return Err(GpxError::InvalidClosingTag(
                        name.local_name.clone(),
                        "email",
                    ));
                }
                return Ok(email);
            }
            _ => {} //consume and ignore other events
        }
    }
    Err(GpxError::MissingClosingTag("email"))
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

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
            err.to_string(),
            "invalid element, `email` lacks required attribute `id`"
        );
    }

    #[test]
    fn consume_err_no_domain() {
        let err = consume!("<email id=\"gpx\" />", GpxVersion::Gpx11).unwrap_err();

        assert_eq!(
            err.to_string(),
            "invalid element, `email` lacks required attribute `domain`"
        );
    }

    #[test]
    fn consume_err_invalid_child_element() {
        let err = consume!(
            "<email id=\"id\" domain=\"domain\"><child /></email>",
            GpxVersion::Gpx11
        )
        .unwrap_err();

        assert_eq!(err.to_string(), "invalid child element `child` in `email`");
    }

    #[test]
    fn consume_err_no_ending_tag() {
        let err = consume!("<email id=\"id\" domain=\"domain\">", GpxVersion::Gpx11).unwrap_err();

        assert_eq!(err.to_string(), "error while parsing XML");
    }
}
