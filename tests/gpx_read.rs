extern crate gpx;

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::fs::File;

    use gpx::reader;

    #[test]
    fn gpx_reader_read_test_badxml() {
        // Should fail with badly formatted XML.
        let file = File::open("tests/fixtures/badcharacter.xml").unwrap();
        let reader = BufReader::new(file);

        let result = reader::read(reader);

        assert!(result.is_err());
    }

    #[test]
    fn gpx_reader_read_test_wikipedia() {
        // Should not give an error, and should have all the correct data.
        let file = File::open("tests/fixtures/wikipedia_example.xml").unwrap();
        let reader = BufReader::new(file);

        let result = reader::read(reader);
        assert!(result.is_ok());

        let mut result = result.unwrap();

        assert_eq!(result.tracks.len(), 1);

        let mut track = result.tracks.pop().unwrap();
        assert_eq!(track.name.unwrap(), "Example GPX Document");

        assert_eq!(track.segments.len(), 1);
        let mut points = track.segments.pop().unwrap().points;

        assert_eq!(points.len(), 3);
        assert_eq!(points.pop().unwrap().elevation.unwrap(), 6.87);
        assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.94);
        assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.46);
    }
}
