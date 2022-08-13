use std::io::Read;

// use error_chain::{bail, ensure};
use geo_types::{Coordinate, Rect};
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use crate::errors::{GpxError, GpxResult};
use crate::parser::{verify_starting_tag, Context};

/// Try to create a [`Rect<f64>`] from an attribute list
fn try_from_attributes(attributes: &[OwnedAttribute]) -> GpxResult<Rect<f64>> {
    // get required bounds
    let lat_min = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlat")
        .ok_or(GpxError::InvalidElementLacksAttribute("minlat", "bounds"))
        .and_then(|attr| attr.value.parse::<f64>().map_err(GpxError::from))?;
    let lat_max = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlat")
        .ok_or(GpxError::InvalidElementLacksAttribute("maxlat", "bounds"))
        .and_then(|attr| attr.value.parse::<f64>().map_err(GpxError::from))?;
    let lon_min = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlon")
        .ok_or(GpxError::InvalidElementLacksAttribute("minlon", "bounds"))
        .and_then(|attr| attr.value.parse::<f64>().map_err(GpxError::from))?;
    let lon_max = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlon")
        .ok_or(GpxError::InvalidElementLacksAttribute("maxlon", "bounds"))
        .and_then(|attr| attr.value.parse::<f64>().map_err(GpxError::from))?;

    // Verify bounding box first, since Rect::new will panic if these are wrong.
    if lon_min > lon_max {
        return Err(GpxError::OutOfBounds("longitude"));
    } else if lat_min > lat_max {
        return Err(GpxError::OutOfBounds("latitude"));
    }

    Ok(Rect::new(
        Coordinate {
            x: lon_min,
            y: lat_min,
        },
        Coordinate {
            x: lon_max,
            y: lat_max,
        },
    ))
}

/// consume consumes a bounds element until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> GpxResult<Rect<f64>> {
    let attributes = verify_starting_tag(context, "bounds")?;
    let bounds = try_from_attributes(&attributes)?;

    for event in context.reader() {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                return Err(GpxError::InvalidChildElement(name.local_name, "bounds"));
            }
            XmlEvent::EndElement { name } => {
                return if name.local_name == "bounds" {
                    Ok(bounds)
                } else {
                    Err(GpxError::InvalidClosingTag(name.local_name, "bounds"))
                }
            }
            _ => {}
        }
    }

    Err(GpxError::MissingClosingTag("bounds"))
}

#[cfg(test)]
mod tests {
    use crate::GpxVersion;

    use super::consume;

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
