use std::io::Read;

// use error_chain::{bail, ensure};
use geo_types::{Coord, Rect};
use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{verify_starting_tag, Context};

/// consume consumes a bounds element until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Rect<f64>> {
    let attributes = verify_starting_tag(context, "bounds")?;
    // get required bounds
    let minlat = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlat")
        .ok_or(GpxError::InvalidElementLacksAttribute("minlat", "bounds"))?;
    let maxlat = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlat")
        .ok_or(GpxError::InvalidElementLacksAttribute("maxlat", "bounds"))?;

    let minlat: f64 = minlat.value.parse()?;
    let maxlat: f64 = maxlat.value.parse()?;

    let minlon = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlon")
        .ok_or(GpxError::InvalidElementLacksAttribute("minlon", "bounds"))?;
    let maxlon = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlon")
        .ok_or(GpxError::InvalidElementLacksAttribute("maxlon", "bounds"))?;

    let minlon: f64 = minlon.value.parse()?;
    let maxlon: f64 = maxlon.value.parse()?;

    // Verify bounding box first, since Rect::new will panic if these are wrong.
    if minlon > maxlon {
        return Err(GpxError::OutOfBounds("longitude"));
    } else if minlat > maxlat {
        return Err(GpxError::OutOfBounds("latitude"));
    }

    let bounds: Rect<f64> = Rect::new(
        Coord {
            x: minlon,
            y: minlat,
        },
        Coord {
            x: maxlon,
            y: maxlat,
        },
    );

    for event in context.reader() {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                return Err(GpxError::InvalidChildElement(name.local_name, "bounds"));
            }
            XmlEvent::EndElement { name } => {
                if name.local_name != "bounds" {
                    return Err(GpxError::InvalidClosingTag(name.local_name, "bounds"));
                } else {
                    return Ok(bounds);
                }
            }
            _ => {}
        }
    }
    Err(GpxError::MissingClosingTag("bounds"))
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

    #[test]
    fn consume_bounds() {
        let bounds = consume!(
            "
<bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"/>
            ",
            GpxVersion::Gpx11
        );

        assert!(bounds.is_ok());

        let bounds = bounds.unwrap();
        assert_eq!(bounds.min().x, -74.031837463);
        assert_eq!(bounds.min().y, 45.487064362);
        assert_eq!(bounds.max().x, -73.586273193);
        assert_eq!(bounds.max().y, 45.701225281);
    }

    #[test]
    fn consume_bad_bounds() {
        let bounds = consume!(
            "<bounds minlat=\"32.4\" minlon=\"notanumber\"></wpt>",
            GpxVersion::Gpx11
        );

        assert!(bounds.is_err());
    }
}
