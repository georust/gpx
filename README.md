# gpx

[![Crates.io](https://img.shields.io/crates/v/gpx.svg)](https://crates.io/crates/gpx) [![Build Status](https://travis-ci.org/georust/gpx.svg?branch=master)](https://travis-ci.org/georust/gpx) [![docs.rs](https://docs.rs/gpx/badge.svg)](https://docs.rs/gpx)

gpx is a library for reading and writing GPX (GPS Exchange Format) files. It uses the
primitives provided by [rust-geo](https://github.com/georust/rust-geo) to allow for storage
of GPS data.

## Example
```rust
extern crate gpx;

use std::io::BufReader;
use std::fs::File;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("tests/fixtures/wikipedia_example.gpx").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    assert_eq!(track.name, Some(String::from("Example GPX Document")));

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];

    // This is an example of retrieving the elevation (in meters) at certain points.
    assert_eq!(segment.points[0].elevation, Some(4.46));
    assert_eq!(segment.points[1].elevation, Some(4.94));
    assert_eq!(segment.points[2].elevation, Some(6.87));
}
```

## Current Status

rust-gpx currently supports reading both GPX 1.1 and 1.0. GPX extensions and
writing to files are not yet supported.

## Contributing
All contributions are welcome! Please open an issue if you find a bug / have any
questions, and pull requests are always appreciated.

## License
rust-gpx is licensed under the [MIT license](./LICENSE).
