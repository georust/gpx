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
use std::path::Path;
use gpx::{Gpx, Track, TrackSegment, Waypoint, Route};
use geo_types::{Point, coord};

pub fn to_gpx<P: AsRef<Path>>(out_path: P) -> Result<String, Box<dyn Error>> {
    // Instantiate Gpx struct
    let mut gpx: Gpx = Gpx {
        version: GpxVersion::Gpx11, // or Gpx10
        creator: None, // Option<String>
        metadata: None, // Option<Metadata>
        waypoints: vec![], // Vec<Waypoint>
        tracks: vec![], // Vec<Track>
        routes: vec![], // Vec<Route>
    };
    gpx.tracks.push(Track {
        name: Some("Track 1".to_string()), // Option<String>
        comment: None, // Option<String>
        description: None, // Option<String>
        source: None, // Option<String>
        links: vec![], // Vec<Link>
        type_: None, // Option<String>
        number: None, // Option<u32>
        segments: vec![], // Vec<TrackSegment>
    });
    gpx.tracks[0].segments.push(TrackSegment { 
        points: vec![] // Vec<Waypoint>
    });

    // Create file at path
    let gpx_file = File::create(out_path);
    
    // Add track point
    let geo_coord = coord! { x: 38.82, y: -121.1 };
    let geo_point: Point = geo_coord.into();
    gpx.tracks[0].segments[0].points.push(Waypoint::new(geo_point));

    // Write to file
    write(&gpx, gpx_file?)?;

    Ok(());
}
```

### Write to string
`write` will write the GPX output to anything that implements `std::io::Write`. To save the output to a string, use a mutable `BufWriter` to write it to a vector, and then convert the vector to a string.
```rust
let mut buf = BufWriter::new(Vec::new());
write(&gpx, &mut buf)?;
let bytes = buf.into_inner()?;
let string = String::from_utf8(bytes)?;
```

## Current Status

rust-gpx currently supports reading and writing both GPX 1.1 and 1.0.
GPX extensions are not yet supported.

## Contributing
All contributions are welcome! Please open an issue if you find a bug / have any
questions, and pull requests are always appreciated.

## License
rust-gpx is licensed under the [MIT license](./LICENSE).
