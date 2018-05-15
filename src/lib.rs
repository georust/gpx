//! gpx is a library for reading and writing GPX (GPS Exchange Format) files.
//! It uses the primitives provided by [rust-geo](https://github.com/georust/rust-geo)
//! to allow for storage of GPS data.
//!
//! # Examples
//! ```
//! use std::io::BufReader;
//! use std::fs::File;
//!
//! use gpx::read;
//! use gpx::{Gpx, Track, TrackSegment};
//!
//! // This XML file actually exists — try it for yourself!
//! let file = File::open("tests/fixtures/wikipedia_example.gpx").unwrap();
//! let reader = BufReader::new(file);
//!
//! // read takes any io::Read and gives a Result<Gpx, Error>.
//! let gpx: Gpx = read(reader).unwrap();
//!
//! // Each GPX file has multiple "tracks", this takes the first one.
//! let track: &Track = &gpx.tracks[0];
//! assert_eq!(track.name, Some(String::from("Example GPX Document")));
//!
//! // Each track will have different segments full of waypoints, where a
//! // waypoint contains info like latitude, longitude, and elevation.
//! let segment: &TrackSegment = &track.segments[0];
//!
//! // This is an example of retrieving the elevation (in meters) at certain points.
//! assert_eq!(segment.points[0].elevation, Some(4.46));
//! assert_eq!(segment.points[1].elevation, Some(4.94));
//! assert_eq!(segment.points[2].elevation, Some(6.87));
//! ```

#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;

extern crate chrono;
extern crate geo;
extern crate xml;

// Export our type structs in the root, along with the read function.
pub use reader::read;
pub use types::*;

mod parser;
mod reader;
mod types;

// Errors should be namespaced away.
pub mod errors;
