//! gpx handles parsing of GPX elements.

use geo_types::Rect;
use std::io::Read;
use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::time::Time;
use crate::parser::{
    bounds, metadata, route, string, time, track, verify_starting_tag, waypoint, Context,
};
use crate::{Gpx, GpxVersion, Link, Metadata, Person};

use super::extensions;

/// Convert the version string to the version enum
fn version_string_to_version(version_str: &str) -> GpxResult<GpxVersion> {
    match version_str {
        "1.0" => Ok(GpxVersion::Gpx10),
        "1.1" => Ok(GpxVersion::Gpx11),
        _ => Err(GpxError::UnknownVersionError(GpxVersion::Unknown)),
    }
}

/// consume consumes an entire GPX element.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Gpx, GpxError> {
    let mut gpx: Gpx = Default::default();

    let mut author: Option<String> = None;
    let mut url: Option<String> = None;
    let mut urlname: Option<String> = None;
    let mut email: Option<String> = None;
    let mut time: Option<Time> = None;
    let mut bounds: Option<Rect<f64>> = None;
    let mut gpx_name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut keywords: Option<String> = None;

    // First we consume the gpx tag and its attributes
    let attributes = verify_starting_tag(context, "gpx")?;
    let version = attributes
        .iter()
        .find(|attr| attr.name.local_name == "version")
        .ok_or(GpxError::InvalidElementLacksAttribute("version", "gpx"))?;
    gpx.version = version_string_to_version(&version.value)?;
    context.version = gpx.version;

    let creator = attributes
        .iter()
        .find(|attr| attr.name.local_name == "creator");
    gpx.creator = creator.map(|c| c.value.to_owned());

    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => return Err(GpxError::EventParsingError("Expecting an event")),
                }
            } else {
                break;
            }
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
                "extensions" => {
                    extensions::consume(context)?;
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
                    let author = if person != Default::default() {
                        Some(person)
                    } else {
                        None
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

                    if metadata != Default::default() {
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

    use super::consume;
    use crate::{errors::GpxError, GpxVersion};

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
            <gpx version=\"1.0\" xmlns:locus=\"http://www.locusmap.eu\" xmlns:ql=\"http://www.qlandkarte.org/xmlschemas/v1.1\">
                <time>2016-03-27T18:57:55Z</time>
                <bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"></bounds>
                <trk>
                  <extensions>
                    <line xmlns=\"http://www.topografix.com/GPX/gpx_style/0/2\">
                      <color>00D7D7</color>
                      <opacity>0.59</opacity>
                      <width>6.0</width>
                      <extensions>
                        <locus:lsColorBase>#9600D7D7</locus:lsColorBase>
                        <locus:lsWidth>6.0</locus:lsWidth>
                        <locus:lsUnits>PIXELS</locus:lsUnits>
                      </extensions>
                    </line>
                    <locus:activity>cycling</locus:activity>
                    <locus:rteComputeType>9</locus:rteComputeType>
                  </extensions>
                  <trkseg>
                    <trkpt lat=\"2.00742\" lon=\"2.286288\">
                      <ele>1375.85</ele>
                    </trkpt>
                  </trkseg>
                </trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
                <rte></rte>
                <extensions>
                    <ql:key>715595d89a4f0d1145703cb1c227bd15</ql:key>
                </extensions>
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

    #[test]
    fn error_on_double_closing_tag() {
        let gpx = consume!(
            "
            <gpx version=\"1.0\" xmlns:locus=\"http://www.locusmap.eu\">
                <time>2016-03-27T18:57:55Z</time>
                <bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"></bounds>
                <trk>
                  <extensions>
                    <line xmlns=\"http://www.topografix.com/GPX/gpx_style/0/2\">
                      <color>00D7D7</color>
                      <opacity>0.59</opacity>
                      <width>6.0</width>
                      <extensions>
                        <locus:lsColorBase>#9600D7D7</locus:lsColorBase>
                        <locus:lsWidth>6.0</locus:lsWidth>
                        <locus:lsUnits>PIXELS</locus:lsUnits>
                      </extensions>
                    </line>
                    <locus:activity>cycling</locus:activity>
                    <locus:rteComputeType>9</locus:rteComputeType>
                  </extensions>
                  </extensions>
                  <trkseg>
                    <trkpt lat=\"2.00742\" lon=\"2.286288\">
                      <ele>1375.85</ele>
                    </trkpt>
                  </trkseg>
                </trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
                <rte></rte>
            </gpx>
            ",
            GpxVersion::Unknown
        );

        assert!(gpx.is_err());
        // the track parser gets an internal "invalid closing tag"-error, and gives back an "EventParsingError("track event")
        if let GpxError::EventParsingError(err) = gpx.unwrap_err() {
            assert_eq!(err, "track event");
        } else {
            panic!("Expected different error.")
        }
    }

    #[test]
    fn fail_on_double_internal_closing_tag() {
        let gpx = consume!(
            "
            <gpx version=\"1.0\" xmlns:locus=\"http://www.locusmap.eu\">
                <time>2016-03-27T18:57:55Z</time>
                <bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"></bounds>
                <trk>
                  <extensions>
                    <line xmlns=\"http://www.topografix.com/GPX/gpx_style/0/2\">
                      <color>00D7D7</color>
                      <opacity>0.59</opacity>
                      <width>6.0</width>
                      <extensions>
                        <locus:lsColorBase>#9600D7D7</locus:lsColorBase>
                        <locus:lsWidth>6.0</locus:lsWidth>
                        <locus:lsUnits>PIXELS</locus:lsUnits>
                      </extensions>
                    </line>
                    <locus:activity>cycling</locus:activity>
                    <locus:rteComputeType>9</locus:rteComputeType>
                  </extensions>
                  </extensions>
                  <trkseg>
                    <trkpt lat=\"2.00742\" lon=\"2.286288\">
                      <ele>1375.85</ele>
                    </trkpt>
                  </trkseg>
                </trk>
                <wpt lat=\"1.23\" lon=\"2.34\"></wpt>
                <wpt lon=\"10.256\" lat=\"-81.324\">
                    <time>2001-10-26T19:32:52+00:00</time>
                </wpt>
                <rte></rte>
            </gpx>
            ",
            GpxVersion::Unknown
        );

        assert!(gpx.is_err());
        // the track parser gets an internal "invalid closing tag"-error, and gives back an "EventParsingError("track event")
        if let GpxError::EventParsingError(err) = gpx.unwrap_err() {
            assert_eq!(err, "track event");
        } else {
            panic!("Expected different error.")
        }
    }
}
