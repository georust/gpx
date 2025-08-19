# gpx

[![Crates.io](https://img.shields.io/crates/v/gpx.svg)](https://crates.io/crates/gpx)
[![Build Status](https://github.com/georust/gpx/actions/workflows/test.yml/badge.svg)](https://github.com/georust/gpx/actions/workflows/test.yml)
[![docs.rs](https://docs.rs/gpx/badge.svg)](https://docs.rs/gpx)

gpx is a library for reading and writing GPX (GPS Exchange Format) files. It uses the
primitives provided by [geo-types](https://github.com/georust/geo) to allow for storage
of GPS data.

## Examples

### Read a GPX file
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

### Generate a new GPX file
This example only generates tracks. You can add waypoints and routes as well by instantiating new ``Waypoint``s and ``Route``s.

```rust
use geo_types::{coord, Point};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use std::io::BufWriter;
use tempfile::tempfile;

fn main() {
    // Instantiate Gpx struct with an empty single-segment Track
    let mut gpx = Gpx::new(GpxVersion::Gpx11);
    gpx.tracks.push(Track::new());
    gpx.tracks[0].segments.push(TrackSegment::new());

    // Add track point
    let geo_coord = coord! { x: -121.1, y: 38.82 };
    let geo_point: Point = geo_coord.into();
    gpx.tracks[0].segments[0].points.push(Waypoint::new(geo_point));

    // Write to file
    let gpx_file = tempfile().unwrap();
    let buf = BufWriter::new(gpx_file);
    gpx::write(&gpx, buf).unwrap();
}
```

### Write to string
`write` will write the GPX output to anything that implements `std::io::Write`. To save the output to a string, write it to a `u8` vector, and then convert the vector to a string.
```rust
use gpx::{Gpx, GpxVersion};

let gpx = Gpx::new(GpxVersion::Gpx11);
let mut vec = Vec::new();
gpx::write(&gpx, &mut vec).unwrap();
let string = String::from_utf8(vec).unwrap();
```

## Current Status

rust-gpx currently supports reading and writing both GPX 1.1 and 1.0.
GPX extensions are not yet supported.

## Contributing
All contributions are welcome! Please open an issue if you find a bug / have any
questions, and pull requests are always appreciated.

## License
rust-gpx is licensed under the [MIT license](./LICENSE).
