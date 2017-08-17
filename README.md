# rust-gpx

[![Crates.io](https://img.shields.io/crates/v/gpx.svg)](https://crates.io/crates/gpx) [![Build Status](https://travis-ci.org/georust/rust-gpx.svg?branch=master)](https://travis-ci.org/georust/rust-gpx) [![docs.rs](https://docs.rs/gpx/badge.svg)](https://docs.rs/gpx)

gpx is a library for reading and writing GPX (GPS Exchange Format) files. It uses the
primitives provided by [rust-geo](https://github.com/georust/rust-geo) to allow for storage
of GPS data.

## Example
```rust
extern crate gpx;

use std::io::BufReader;
use std::fs::File;

use gpx::reader;
use gpx::parser::gpx::Gpx;
use gpx::parser::track::Track;
use gpx::parser::waypoint::Waypoint;

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("tests/fixtures/wikipedia_example.xml").unwrap();
    let reader = BufReader::new(file);

    // reader::read takes any io::Read and gives an Option<Gpx>.
    let mut gpx: Gpx = reader::read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let mut track: Track = gpx.tracks.pop().unwrap();
    assert_eq!(track.name.unwrap(), "Example GPX Document");

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let mut points: Vec<Waypoint> = track.segments.pop().unwrap().points;

    // This is an example of retrieving the elevation (in meters) at certain points.
    assert_eq!(points.pop().unwrap().elevation.unwrap(), 6.87);
    assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.94);
    assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.46);
}
```

## Contributing
All contributions are welcome! Please open an issue if you find a bug / have any
questions, and pull requests are always appreciated.

## License
rust-gpx is licensed under the [MIT license](./LICENSE).
