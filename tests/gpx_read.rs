// This is a pretty complete functional test of the library.
// Feel free to read through these tests and their accompanying
// .gpx files to see how usage might be.

#[macro_use]
extern crate assert_approx_eq;

extern crate chrono;
extern crate geo;
extern crate geo_types;
extern crate gpx;

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use geo::algorithm::haversine_distance::HaversineDistance;
    use geo::euclidean_length::EuclideanLength;
    use geo_types::{Geometry, Point};
    use std::fs::File;
    use std::io::BufReader;

    use gpx::read;
    use gpx::Fix;

    #[test]
    fn gpx_reader_read_test_badxml() {
        // Should fail with badly formatted XML.
        let file = File::open("tests/fixtures/badcharacter.xml").unwrap();
        let reader = BufReader::new(file);

        let result = read(reader);

        assert!(result.is_err());
    }

    #[test]
    fn gpx_reader_read_test_wikipedia() {
        // Should not give an error, and should have all the correct data.
        let file = File::open("tests/fixtures/wikipedia_example.gpx").unwrap();
        let reader = BufReader::new(file);

        let result = read(reader);
        assert!(result.is_ok());

        let result = result.unwrap();

        // Check the metadata, of course; here it has a time.
        let metadata = result.metadata.unwrap();
        assert_eq!(
            metadata.time.unwrap(),
            Utc.ymd(2009, 10, 17).and_hms(22, 58, 43)
        );

        assert_eq!(metadata.links.len(), 1);
        let link = &metadata.links[0];
        assert_eq!(link.href, "http://www.garmin.com");
        assert_eq!(link.text, Some(String::from("Garmin International")));

        // There should just be one track, "example gpx document".
        assert_eq!(result.tracks.len(), 1);
        let track = &result.tracks[0];

        assert_eq!(track.name, Some(String::from("Example GPX Document")));

        // Each point has its own information; test elevation.
        assert_eq!(track.segments.len(), 1);
        let points = &track.segments[0].points;

        assert_eq!(points.len(), 3);
        assert_eq!(points[0].elevation, Some(4.46));
        assert_eq!(points[1].elevation, Some(4.94));
        assert_eq!(points[2].elevation, Some(6.87));
    }

    #[test]
    fn gpx_reader_read_test_garmin_activity() {
        let file = File::open("tests/fixtures/garmin-activity.gpx").unwrap();
        let reader = BufReader::new(file);

        let result = read(reader);
        assert!(result.is_ok());
        let res = result.unwrap();

        // Check the info on the metadata.
        let metadata = res.metadata.unwrap();
        assert_eq!(
            metadata.time.unwrap(),
            Utc.ymd(2017, 7, 29).and_hms_micro(14, 46, 35, 000_000)
        );

        assert_eq!(metadata.links.len(), 1);
        let link = &metadata.links[0];
        assert_eq!(link.text, Some(String::from("Garmin Connect")));
        assert_eq!(link.href, String::from("connect.garmin.com"));

        // Check the main track.
        assert_eq!(res.tracks.len(), 1);
        let track = &res.tracks[0];

        assert_eq!(track.name, Some(String::from("casual stroll")));
        assert_eq!(track._type, Some(String::from("running")));

        // Check some Geo operations on the track.
        let mls = track.multilinestring();
        assert_approx_eq!(mls.euclidean_length(), 0.12704048);

        // Get the first track segment.
        assert_eq!(track.segments.len(), 1);
        let segment = &track.segments[0];

        // Test for every single point in the file.
        for point in segment.points.iter() {
            // Elevation is between 90 and 220.
            let elevation = point.elevation.unwrap();
            assert!(elevation > 90. && elevation < 220.);

            // All the points should be close (5000 units, its closer than you think).
            let reference_point = Point::new(-121.97, 37.24);
            let distance = reference_point.haversine_distance(&point.point());
            assert!(distance < 5000.);

            // Time is between a day before and after.
            let time = point.time.unwrap();
            assert!(time > Utc.ymd(2017, 7, 28).and_hms_micro(0, 0, 0, 000_000));
            assert!(time < Utc.ymd(2017, 7, 30).and_hms_micro(0, 0, 0, 000_000));

            // Should coerce to Point.
            let geo: Geometry<f64> = point.clone().into();
            match geo {
                Geometry::Point(_) => {} // ok
                _ => panic!("point.into() gave bad geometry"),
            }

            // It's missing almost all fields, actually.
            assert!(point.name.is_none());
            assert!(point.comment.is_none());
            assert!(point.description.is_none());
            assert!(point.source.is_none());
            assert!(point.symbol.is_none());
            assert!(point._type.is_none());
            assert_eq!(point.links.len(), 0);
        }
    }

    #[test]
    fn gpx_reader_read_test_with_accuracy() {
        let file = File::open("tests/fixtures/with_accuracy.gpx").unwrap();
        let reader = BufReader::new(file);

        let result = read(reader);
        assert!(result.is_ok());
        let res = result.unwrap();

        // Check the info on the metadata.
        let metadata = res.metadata.unwrap();
        assert_eq!(metadata.name.unwrap(), "20170412_CARDIO.gpx");

        assert_eq!(metadata.links.len(), 0);

        // Check the main track.
        assert_eq!(res.tracks.len(), 1);
        let track = &res.tracks[0];

        assert_eq!(track.name, Some(String::from("Cycling")));

        // Get the first track segment.
        assert_eq!(track.segments.len(), 1);
        let segment = &track.segments[0];

        // Get the first point
        assert_eq!(segment.points.len(), 3);
        let points = &segment.points;

        assert_eq!(points[0].fix, Some(Fix::DGPS));
        assert_eq!(points[0].sat.unwrap(), 4);
        assert_eq!(points[0].hdop.unwrap(), 5.);
        assert_eq!(points[0].vdop.unwrap(), 6.2);
        assert_eq!(points[0].pdop.unwrap(), 728.);
        assert_eq!(points[0].age.unwrap(), 1.);
        assert_eq!(points[0].dgpsid.unwrap(), 3);

        assert_eq!(points[1].fix, Some(Fix::ThreeDimensional));
        assert_eq!(points[1].sat.unwrap(), 5);
        assert_eq!(points[1].hdop.unwrap(), 3.6);
        assert_eq!(points[1].vdop.unwrap(), 5.);
        assert_eq!(points[1].pdop.unwrap(), 619.1);
        assert_eq!(points[1].age.unwrap(), 2.01);
        assert_eq!(points[1].dgpsid.unwrap(), 4);

        assert_eq!(
            points[2].fix,
            Some(Fix::Other("something_not_in_the_spec".to_string()))
        );
    }
}
