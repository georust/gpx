//! Writes an activity to GPX format.

use std::io::Write;

use geo_types::Rect;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

use crate::errors::{GpxError, GpxResult};
use crate::parser::time::Time;
use crate::types::*;
use crate::{Gpx, GpxVersion};

/// Writes an activity to GPX format.
///
/// Takes any `std::io::Write` as its writer, and returns a
/// [`Result<(), GpxError>`].
///
/// [`Result<(), GpxError>`]: std::result::Result<T>
///
/// ```
/// use gpx::write;
/// use gpx::Gpx;
/// use gpx::GpxVersion;
///
/// let mut data : Gpx = Default::default();
/// data.version = GpxVersion::Gpx11;
///
/// // You can give it anything that implements `std::io::Write`.
/// write(&data, std::io::stdout()).unwrap();
/// ```
pub fn write<W: Write>(gpx: &Gpx, writer: W) -> GpxResult<()> {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(writer);
    write_with_event_writer(gpx, &mut writer)
}

/// Writes an activity to GPX format.
///
/// Takes [EventWriter](xml::writer::EventWriter) as its writer, and returns a
/// [`Result<(), GpxError>`].
///
/// [`Result<(), GpxError>`]: std::result::Result<T>
///
/// ```
/// use gpx::write_with_event_writer;
/// use gpx::Gpx;
/// use gpx::GpxVersion;
/// use xml::writer::EmitterConfig;
///
/// let mut data : Gpx = Default::default();
/// data.version = GpxVersion::Gpx11;
/// let mut writer = EmitterConfig::new()
///         .perform_indent(false)
///         .create_writer(std::io::stdout());
///
/// write_with_event_writer(&data, &mut writer).unwrap();
/// ```
pub fn write_with_event_writer<W: Write>(gpx: &Gpx, writer: &mut EventWriter<W>) -> GpxResult<()> {
    let creator: &str = gpx
        .creator
        .as_deref()
        .unwrap_or("https://github.com/georust/gpx");
    write_xml_event(
        XmlEvent::start_element("gpx")
            .attr("version", version_to_version_string(gpx.version)?)
            .attr("xmlns", version_to_xml_url(gpx.version)?)
            .attr("creator", creator),
        writer,
    )?;
    write_metadata(gpx, writer)?;
    for point in &gpx.waypoints {
        write_waypoint("wpt", point, writer)?;
    }
    for track in &gpx.tracks {
        write_track(track, writer)?;
    }
    for route in &gpx.routes {
        write_route(route, writer)?;
    }
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_xml_event<'a, W, E>(event: E, writer: &mut EventWriter<W>) -> GpxResult<()>
where
    W: Write,
    E: Into<XmlEvent<'a>>,
{
    Ok(writer.write(event)?)
}

fn version_to_version_string(version: GpxVersion) -> GpxResult<&'static str> {
    match version {
        GpxVersion::Gpx10 => Ok("1.0"),
        GpxVersion::Gpx11 => Ok("1.1"),
        version => Err(GpxError::UnknownVersionError(version)),
    }
}

fn version_to_xml_url(version: GpxVersion) -> GpxResult<&'static str> {
    match version {
        GpxVersion::Gpx10 => Ok("http://www.topografix.com/GPX/1/0"),
        GpxVersion::Gpx11 => Ok("http://www.topografix.com/GPX/1/1"),
        version => Err(GpxError::UnknownVersionError(version)),
    }
}

fn write_metadata<W: Write>(gpx: &Gpx, writer: &mut EventWriter<W>) -> GpxResult<()> {
    match gpx.version {
        GpxVersion::Gpx10 => write_gpx10_metadata(gpx, writer),
        GpxVersion::Gpx11 => write_gpx11_metadata(gpx, writer),
        version => Err(GpxError::UnknownVersionError(version)),
    }
}

fn write_gpx10_metadata<W: Write>(gpx: &Gpx, writer: &mut EventWriter<W>) -> GpxResult<()> {
    if gpx.metadata.is_none() {
        return Ok(());
    }
    let metadata = gpx.metadata.as_ref().unwrap();
    write_string_if_exists("name", &metadata.name, writer)?;
    write_string_if_exists("desc", &metadata.description, writer)?;
    if let Some(author) = metadata.author.as_ref() {
        write_string_if_exists("author", &author.name, writer)?;
        write_email_if_exists(&author.email, writer)?;
        if let Some(link) = author.link.as_ref() {
            write_string("url", &link.href, writer)?;
            write_string_if_exists("urlname", &link.text, writer)?;
        }
    }
    write_string_if_exists("keywords", &metadata.keywords, writer)?;
    write_time_if_exists(&metadata.time, writer)?;
    write_bounds_if_exists(&metadata.bounds, writer)?;
    Ok(())
}

fn write_gpx11_metadata<W: Write>(gpx: &Gpx, writer: &mut EventWriter<W>) -> GpxResult<()> {
    if gpx.metadata.is_none() {
        return Ok(());
    }
    let metadata = gpx.metadata.as_ref().unwrap();
    write_xml_event(XmlEvent::start_element("metadata"), writer)?;
    write_string_if_exists("name", &metadata.name, writer)?;
    write_string_if_exists("desc", &metadata.description, writer)?;
    write_person_if_exists("author", &metadata.author, writer)?;
    write_string_if_exists("keywords", &metadata.keywords, writer)?;
    write_time_if_exists(&metadata.time, writer)?;
    for link in &metadata.links {
        write_link(link, writer)?;
    }
    write_bounds_if_exists(&metadata.bounds, writer)?;
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_string<W: Write>(key: &str, value: &str, writer: &mut EventWriter<W>) -> GpxResult<()> {
    write_xml_event(XmlEvent::start_element(key), writer)?;
    write_xml_event(XmlEvent::characters(value), writer)?;
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_string_if_exists<W: Write>(
    key: &str,
    value: &Option<String>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref value) = value {
        write_string(key, value, writer)?;
    }
    Ok(())
}

fn write_value_if_exists<W: Write, T: ToString>(
    key: &str,
    value: &Option<T>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref value) = value {
        write_xml_event(XmlEvent::start_element(key), writer)?;
        let value = &value.to_string();
        write_xml_event(XmlEvent::characters(value), writer)?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_email_if_exists<W: Write>(
    email: &Option<String>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref email) = email {
        let mut parts = email.split('@');
        let id = parts.next().ok_or(GpxError::MissingEmailPartError("id"))?;
        let domain = parts
            .next()
            .ok_or(GpxError::MissingEmailPartError("domain"))?;
        if parts.next().is_some() {
            return Err(GpxError::TooManyAtsError);
        }
        write_xml_event(
            XmlEvent::start_element("email")
                .attr("id", id)
                .attr("domain", domain),
            writer,
        )?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_link<W: Write>(link: &Link, writer: &mut EventWriter<W>) -> GpxResult<()> {
    write_xml_event(
        XmlEvent::start_element("link").attr("href", &link.href),
        writer,
    )?;
    write_string_if_exists("text", &link.text, writer)?;
    write_string_if_exists("type", &link._type, writer)?;
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_link_if_exists<W: Write>(
    link: &Option<Link>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref link) = link {
        write_link(link, writer)?;
    }
    Ok(())
}

fn write_person_if_exists<W: Write>(
    key: &str,
    value: &Option<Person>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref value) = value {
        write_xml_event(XmlEvent::start_element(key), writer)?;
        write_string_if_exists("name", &value.name, writer)?;
        write_email_if_exists(&value.email, writer)?;
        write_link_if_exists(&value.link, writer)?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_time_if_exists<W: Write>(
    time: &Option<Time>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref time) = time {
        write_xml_event(XmlEvent::start_element("time"), writer)?;
        write_xml_event(XmlEvent::characters(&time.format()?), writer)?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_bounds_if_exists<W: Write>(
    bounds: &Option<Rect<f64>>,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    if let Some(ref bounds) = bounds {
        write_xml_event(
            XmlEvent::start_element("bounds")
                .attr("minlat", &bounds.min().y.to_string())
                .attr("maxlat", &bounds.max().y.to_string())
                .attr("minlon", &bounds.min().x.to_string())
                .attr("maxlon", &bounds.max().x.to_string()),
            writer,
        )?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_fix_if_exists<W: Write>(fix: &Option<Fix>, writer: &mut EventWriter<W>) -> GpxResult<()> {
    if let Some(ref fix) = fix {
        write_xml_event(XmlEvent::start_element("fix"), writer)?;
        let fix_str = match fix {
            Fix::None => "none",
            Fix::TwoDimensional => "2d",
            Fix::ThreeDimensional => "3d",
            Fix::DGPS => "dgps",
            Fix::PPS => "pps",
            Fix::Other(string) => string,
        };
        write_xml_event(XmlEvent::characters(fix_str), writer)?;
        write_xml_event(XmlEvent::end_element(), writer)?;
    }
    Ok(())
}

fn write_track<W: Write>(track: &Track, writer: &mut EventWriter<W>) -> GpxResult<()> {
    write_xml_event(XmlEvent::start_element("trk"), writer)?;
    write_string_if_exists("name", &track.name, writer)?;
    write_string_if_exists("cmt", &track.comment, writer)?;
    write_string_if_exists("desc", &track.description, writer)?;
    write_string_if_exists("src", &track.source, writer)?;
    for link in &track.links {
        write_link(link, writer)?;
    }
    write_string_if_exists("type", &track._type, writer)?;
    for segment in &track.segments {
        write_track_segment(segment, writer)?;
    }
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_route<W: Write>(route: &Route, writer: &mut EventWriter<W>) -> GpxResult<()> {
    write_xml_event(XmlEvent::start_element("rte"), writer)?;
    write_string_if_exists("name", &route.name, writer)?;
    write_string_if_exists("cmt", &route.comment, writer)?;
    write_string_if_exists("desc", &route.description, writer)?;
    write_string_if_exists("src", &route.source, writer)?;
    for link in &route.links {
        write_link(link, writer)?;
    }
    write_value_if_exists("number", &route.number, writer)?;
    write_string_if_exists("type", &route._type, writer)?;
    for point in &route.points {
        write_waypoint("rtept", point, writer)?;
    }
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_track_segment<W: Write>(
    segment: &TrackSegment,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    write_xml_event(XmlEvent::start_element("trkseg"), writer)?;
    for point in &segment.points {
        write_waypoint("trkpt", point, writer)?;
    }
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}

fn write_waypoint<W: Write>(
    tagname: &str,
    waypoint: &Waypoint,
    writer: &mut EventWriter<W>,
) -> GpxResult<()> {
    write_xml_event(
        XmlEvent::start_element(tagname)
            .attr("lat", &waypoint.point().lat().to_string())
            .attr("lon", &waypoint.point().lng().to_string()),
        writer,
    )?;
    write_value_if_exists("ele", &waypoint.elevation, writer)?;
    // TODO: write speed if GPX version == 1.0
    write_time_if_exists(&waypoint.time, writer)?;
    write_value_if_exists("geoidheight", &waypoint.geoidheight, writer)?;
    write_string_if_exists("name", &waypoint.name, writer)?;
    write_string_if_exists("cmt", &waypoint.comment, writer)?;
    write_string_if_exists("desc", &waypoint.description, writer)?;
    write_string_if_exists("src", &waypoint.source, writer)?;
    for link in &waypoint.links {
        write_link(link, writer)?;
    }
    write_string_if_exists("sym", &waypoint.symbol, writer)?;
    write_string_if_exists("type", &waypoint._type, writer)?;
    write_fix_if_exists(&waypoint.fix, writer)?;
    write_value_if_exists("sat", &waypoint.sat, writer)?;
    write_value_if_exists("hdop", &waypoint.hdop, writer)?;
    write_value_if_exists("vdop", &waypoint.vdop, writer)?;
    write_value_if_exists("pdop", &waypoint.pdop, writer)?;
    write_value_if_exists("ageofdgpsdata", &waypoint.dgps_age, writer)?;
    write_value_if_exists("dgpsid", &waypoint.dgpsid, writer)?;
    write_xml_event(XmlEvent::end_element(), writer)?;
    Ok(())
}
