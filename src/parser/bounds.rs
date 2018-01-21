use errors::*;

use std::iter::Peekable;
use std::io::Read;
use xml::reader::Events;
use xml::reader::XmlEvent;
use geo::Bbox;

/// consume consumes an element as a nothing.
pub fn consume<R: Read>(reader: &mut Peekable<Events<R>>) -> Result<Bbox<f64>> {
    let mut element: Option<String> = None;
    let mut bounds: Bbox<f64> = Bbox {
        xmin: 0.,
        xmax: 0.,
        ymin: 0.,
        ymax: 0.,
    };
    for event in reader {
        match event.chain_err(|| "error while parsing XML")? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                ensure!(element.is_none(), "cannot start element inside bounds");
                // get required bounds
                let minlat = attributes
                    .iter()
                    .filter(|attr| attr.name.local_name == "minlat")
                    .nth(0)
                    .ok_or("no min latitude attribute on bounds tag".to_owned())?;
                let maxlat = attributes
                    .iter()
                    .filter(|attr| attr.name.local_name == "maxlat")
                    .nth(0)
                    .ok_or("no max latitude attribute on bounds tag".to_owned())?;

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
                    .filter(|attr| attr.name.local_name == "minlon")
                    .nth(0)
                    .ok_or("no min longitude attribute on bounds tag".to_owned())?;
                let maxlon = attributes
                    .iter()
                    .filter(|attr| attr.name.local_name == "maxlon")
                    .nth(0)
                    .ok_or("no max longitude attribute on bounds tag".to_owned())?;

                let minlon: f64 = minlon
                    .value
                    .parse()
                    .chain_err(|| "error while casting min longitude to f64")?;
                let maxlon: f64 = maxlon
                    .value
                    .parse()
                    .chain_err(|| "error while casting max longitude to f64")?;

                bounds.xmin = minlon;
                bounds.xmax = maxlon;
                bounds.ymin = minlat;
                bounds.ymax = maxlat;

                element = Some(name.local_name);
            }

            XmlEvent::EndElement { .. } => {
                return Ok(bounds);
            }

            _ => {}
        }
    }

    return Err("no end tag for bounds".into());
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use xml::reader::EventReader;

    use super::consume;

    #[test]
    fn consume_bounds() {
        let bounds = consume!(
            "
<bounds minlat=\"45.487064362\" minlon=\"-74.031837463\" maxlat=\"45.701225281\" maxlon=\"-73.586273193\"/>
            "
        );

        assert!(bounds.is_ok());

        let bounds = bounds.unwrap();
        assert_eq!(bounds.xmin, -74.031837463);
        assert_eq!(bounds.ymin, 45.487064362);
        assert_eq!(bounds.xmax, -73.586273193);
        assert_eq!(bounds.ymax, 45.701225281);
    }

    #[test]
    fn consume_bad_bounds() {
        let bounds = consume!("<bounds minlat=\"32.4\" minlon=\"notanumber\"></wpt>");

        assert!(bounds.is_err());
    }
}
