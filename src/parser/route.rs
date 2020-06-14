//! route handles parsing of GPX-spec routes.

use errors::*;
use std::io::Read;
use xml::reader::XmlEvent;

use parser::link;
use parser::string;
use parser::verify_starting_tag;
use parser::waypoint;
use parser::Context;

use Route;

/// consume consumes a GPX route from the `reader` until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Route> {
    let mut route: Route = Default::default();
    verify_starting_tag(context, "rte")?;

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => bail!("error while parsing route event"),
                }
            } else {
                break;
            }
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "name" => {
                    route.name = Some(string::consume(context, "name", false)?);
                }
                "cmt" => {
                    route.comment = Some(string::consume(context, "cmt", true)?);
                }
                "desc" => {
                    route.description = Some(string::consume(context, "desc", true)?);
                }
                "src" => {
                    route.source = Some(string::consume(context, "src", true)?);
                }
                "type" => {
                    route._type = Some(string::consume(context, "type", false)?);
                }
                "rtept" => {
                    route.points.push(waypoint::consume(context, "rtept")?);
                }
                "link" => {
                    route.links.push(link::consume(context)?);
                }
                child => {
                    bail!(ErrorKind::InvalidChildElement(String::from(child), "route"));
                }
            },
            XmlEvent::EndElement { ref name } => {
                ensure!(
                    name.local_name == "rte",
                    ErrorKind::InvalidClosingTag(name.local_name.clone(), "route")
                );
                context.reader.next(); //consume the end tag
                return Ok(route);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    bail!(ErrorKind::MissingClosingTag("route"));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::consume;
    use GpxVersion;

    #[test]
    fn consume_full_route() {
        let route = consume!(
            "
            <rte>
                <name>route name</name>
                <cmt>route comment</cmt>
                <desc>route description</desc>
                <src>route source</src>
                <type>route type</type>
            </rte>
            ",
            GpxVersion::Gpx11
        );

        assert!(route.is_ok());

        let route = route.unwrap();

        assert_eq!(route.name.unwrap(), "route name");
        assert_eq!(route.comment.unwrap(), "route comment");
        assert_eq!(route.description.unwrap(), "route description");
        assert_eq!(route.source.unwrap(), "route source");
        assert_eq!(route._type.unwrap(), "route type");
    }

    #[test]
    fn consume_empty() {
        let route = consume!("<rte></rte>", GpxVersion::Gpx11);
        assert!(route.is_ok());
    }
}
