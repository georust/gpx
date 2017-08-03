//! rust-gpx implements both reading and writing GPX (GPS Exchange Format)
//! formatted data.

#[macro_use]
extern crate error_chain;

extern crate xml;
extern crate chrono;
extern crate geo;

pub mod reader;
pub mod writer;
pub mod errors;

pub mod parser;
