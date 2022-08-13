//! gpx handles parsing of GPX elements.

use std::io::Read;

use geo_types::Rect;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::time::Time;
use crate::parser::{
    bounds, metadata, route, string, time, track, verify_starting_tag, waypoint, Context,
};
use crate::{Gpx, GpxVersion, Link, Metadata, Person};

/// Convert the version string to the version enum
fn version_string_to_version(version_str: &str) -> GpxResult<GpxVersion> {
    match version_str {
        "1.0" => Ok(GpxVersion::Gpx10),
        "1.1" => Ok(GpxVersion::Gpx11),
        _ => Err(GpxError::UnknownVersionError(GpxVersion::Unknown)),
    }
}

/// Try to create a [`Gpx`] from an attribute list
fn try_from_attributes(attributes: &[OwnedAttribute]) -> GpxResult<Gpx> {
    let version = attributes
        .iter()
        .find(|attr| attr.name.local_name == "version")
        .ok_or(GpxError::InvalidElementLacksAttribute("version", "gpx"))
        .map(|v| version_string_to_version(&v.value))??;
    let creator = attributes
        .iter()
        .find(|attr| attr.name.local_name == "creator")
        .map(|c| c.value.clone());

    Ok(Gpx {
        version,
        creator,
        ..Default::default()
    })
}

/// consume consumes an entire GPX element.
#[allow(clippy::too_many_lines)]
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Gpx, GpxError> {
    // First we consume the gpx tag and its attributes
    let attributes = verify_starting_tag(context, "gpx")?;

    let mut gpx = try_from_attributes(&attributes)?;
    let mut author: Option<String> = None;
    let mut url: Option<String> = None;
    let mut urlname: Option<String> = None;
    let mut email: Option<String> = None;
    let mut time: Option<Time> = None;
    let mut bounds: Option<Rect<f64>> = None;
    let mut gpx_name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut keywords: Option<String> = None;

    context.version = gpx.version;

    loop {
        let next_event = match context.reader.peek() {
            Some(Err(_)) => return Err(GpxError::EventParsingError("Expecting an event")),
            Some(Ok(event)) => event,
            None => break,
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => match name.local_name.as_ref() {
                "metadata" if context.version != GpxVersion::Gpx10 => {
                    gpx.metadata = Some(metadata::consume(context)?);
                }
                "trk" => {
                    gpx.tracks.push(track::consume(context)?);
                }
                "rte" => {
                    gpx.routes.push(route::consume(context)?);
                }
                "wpt" => {
                    gpx.waypoints.push(waypoint::consume(context, "wpt")?);
                }
                "time" if context.version == GpxVersion::Gpx10 => {
                    time = Some(time::consume(context)?);
                }
                "bounds" if context.version == GpxVersion::Gpx10 => {
                    bounds = Some(bounds::consume(context)?);
                }
                "author" if context.version == GpxVersion::Gpx10 => {
                    author = Some(string::consume(context, "author", false)?);
                }
                "email" if context.version == GpxVersion::Gpx10 => {
                    email = Some(string::consume(context, "email", false)?);
                }
                "url" if context.version == GpxVersion::Gpx10 => {
                    url = Some(string::consume(context, "url", false)?);
                }
                "urlname" if context.version == GpxVersion::Gpx10 => {
                    urlname = Some(string::consume(context, "urlname", false)?);
                }
                "name" if context.version == GpxVersion::Gpx10 => {
                    gpx_name = Some(string::consume(context, "name", false)?);
                }
                "desc" if context.version == GpxVersion::Gpx10 => {
                    description = Some(string::consume(context, "desc", true)?);
                }
                "keywords" if context.version == GpxVersion::Gpx10 => {
                    keywords = Some(string::consume(context, "keywords", true)?);
                }
                child => {
                    return Err(GpxError::InvalidChildElement(String::from(child), "gpx"));
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name != "gpx" {
                    return Err(GpxError::InvalidClosingTag(name.local_name.clone(), "gpx"));
                }
                if gpx.version == GpxVersion::Gpx10 {
                    let link = url.map(|url| Link {
                        href: url,
                        text: urlname,
                        ..Default::default()
                    });
                    let person: Person = Person {
                        name: author,
                        email,
                        link,
                    };
                    let author = if person == Person::default() {
                        None
                    } else {
                        Some(person)
                    };
                    let metadata: Metadata = Metadata {
                        name: gpx_name,
                        time,
                        bounds,
                        keywords,
                        description,
                        author,
                        ..Default::default()
                    };

                    if metadata != Metadata::default() {
                        gpx.metadata = Some(metadata);
                    }
                }
                context.reader.next();

                return Ok(gpx);
            }
            _ => {
                context.reader.next(); //consume and ignore this event
            }
        }
    }

    Err(GpxError::MissingClosingTag("gpx"))
}

#[cfg(test)]
mod tests {
    use geo_types::Point;

    use crate::GpxVersion;

    use super::consume;

    #[test]
    fn consume_gpx() {
        let gpx = consume!("<gpx version=\"1.1\"></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_ok());
    }

    #[test]
    fn consume_gpx_no_version() {
        let gpx = consume!("<gpx></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_err());
    }

    #[test]
    fn consume_gpx_version_error() {
        let gpx = consume!("<gpx version=\"1.2\"></gpx>", GpxVersion::Unknown);

        assert!(gpx.is_err());
    }

    #[test]
    fn consume_gpx_creator() {
        let gpx = consume!(
            "<gpx version=\"1.1\" creator=\"unit test\"></gpx>",
            GpxVersion::Unknown
        );

        assert!(gpx.is_ok());
        assert_eq!(gpx.unwrap().creator, Some("unit test".into()));
    }

    #[test]
    fn consume_gpx_full() {
        let gpx = consume!(
            "
            <gpx version=\"1.0\">
                <time>2016-03-27T18:57:55Z</time>
                <bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"></bounds>
                <trk></trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
                <rte></rte>
            </gpx>
            ",
            GpxVersion::Unknown
        );

        assert!(gpx.is_ok());
        let gpx = gpx.unwrap();

        assert_eq!(gpx.version, GpxVersion::Gpx10);
        assert_eq!(gpx.tracks.len(), 1);

        assert_eq!(gpx.waypoints.len(), 2);

        let wpt = &gpx.waypoints[1];
        assert_eq!(wpt.point(), Point::new(10.256, -81.324));
    }
}
