use crate::errors::*;

use geo_types::{Coordinate, Rect};
use std::io::Read;
use xml::reader::XmlEvent;
use error_chain::{bail, ensure};

use crate::parser::verify_starting_tag;
use crate::parser::Context;

/// consume consumes a bounds element until it ends.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<Rect<f64>> {
    let attributes = verify_starting_tag(context, "bounds")?;
    // get required bounds
    let minlat = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlat")
        .ok_or(ErrorKind::InvalidElementLacksAttribute("minlat", "bounds"))?;
    let maxlat = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlat")
        .ok_or(ErrorKind::InvalidElementLacksAttribute("maxlat", "bounds"))?;

    let minlat: f64 = minlat
        .value
        .parse()
        .chain_err(|| "error while casting min latitude to f64")?;
    let maxlat: f64 = maxlat
        .value
        .parse()
        .chain_err(|| "error while casting max latitude to f64")?;

    let minlon = attributes
        .iter()
        .find(|attr| attr.name.local_name == "minlon")
        .ok_or(ErrorKind::InvalidElementLacksAttribute("minlon", "bounds"))?;
    let maxlon = attributes
        .iter()
        .find(|attr| attr.name.local_name == "maxlon")
        .ok_or(ErrorKind::InvalidElementLacksAttribute("maxlon", "bounds"))?;

    let minlon: f64 = minlon
        .value
        .parse()
        .chain_err(|| "error while casting min longitude to f64")?;
    let maxlon: f64 = maxlon
        .value
        .parse()
        .chain_err(|| "error while casting max longitude to f64")?;

    // Verify bounding box first, since Rect::new will panic if these are wrong.
    if minlon > maxlon {
        bail!("Minimum longitude larger than maximum longitude");
    } else if minlat > maxlat {
        bail!("Minimum latitude larger than maximum latitude");
    }

    let bounds: Rect<f64> = Rect::new(
        Coordinate {
            x: minlon,
            y: minlat,
        },
        Coordinate {
            x: maxlon,
            y: maxlat,
        },
    );

    for event in context.reader() {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement { name, .. } => {
                bail!(ErrorKind::InvalidChildElement(
                    name.local_name,
                    "bounds"
                ));
            }
            XmlEvent::EndElement { name } => {
                ensure!(
                    name.local_name == "bounds",
                    ErrorKind::InvalidClosingTag(name.local_name, "bounds")
                );
                return Ok(bounds);
            }
            _ => {}
        }
    }
    bail!(ErrorKind::MissingClosingTag("bounds"));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

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
