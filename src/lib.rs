//! gpx is a library for reading and writing GPX (GPS Exchange Format) files.
//! It uses the primitives provided by [rust-geo](https://github.com/georust/rust-geo)
//! to allow for storage of GPS data.
//!
//! # Examples
//! ```
//! use std::io::BufReader;
//! use std::fs::File;
//!
//! use gpx::reader;
//! use gpx::{Gpx, Track, Waypoint};
//!
//! // This XML file actually exists — try it for yourself!
//! let file = File::open("tests/fixtures/wikipedia_example.xml").unwrap();
//! let reader = BufReader::new(file);
//!
//! // reader::read takes any io::Read and gives an Option<Gpx>.
//! let mut gpx: Gpx = reader::read(reader).unwrap();
//!
//! // Each GPX file has multiple "tracks", this takes the first one.
//! let mut track: Track = gpx.tracks.pop().unwrap();
//! assert_eq!(track.name.unwrap(), "Example GPX Document");
//!
//! // Each track will have different segments full of waypoints, where a
//! // waypoint contains info like latitude, longitude, and elevation.
//! let mut points: Vec<Waypoint> = track.segments.pop().unwrap().points;
//!
//! // This is an example of retrieving the elevation (in meters) at certain points.
//! assert_eq!(points.pop().unwrap().elevation.unwrap(), 6.87);
//! assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.94);
//! assert_eq!(points.pop().unwrap().elevation.unwrap(), 4.46);
//! ```

#[macro_use]
extern crate error_chain;

// TODO, this might be a bug, try and remove this unused imports tag.
#[allow(unused_imports)]
#[macro_use]
extern crate assert_approx_eq;

extern crate xml;
extern crate chrono;
extern crate geo;

pub use types::*;

mod types;

pub mod reader;
pub mod errors;

pub mod parser;
