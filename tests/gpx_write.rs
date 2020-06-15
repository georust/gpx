use std::fs::File;
use std::io::BufReader;

use gpx::{read, write};
use gpx::{Gpx, Link, Waypoint};

#[test]
fn gpx_writer_write_unknown_gpx_version() {
    let gpx: Gpx = Default::default();
    let mut writer: Vec<u8> = Vec::new();
    // Should fail with unknown version.
    let result = write(&gpx, &mut writer);

    assert!(result.is_err());
}

#[test]
fn gpx_writer_write_test_wikipedia() {
    check_write_for_example_file("tests/fixtures/wikipedia_example.gpx");
}

#[test]
fn gpx_writer_write_test_garmin_activity() {
    check_write_for_example_file("tests/fixtures/garmin-activity.gpx");
}

#[test]
fn gpx_writer_write_test_with_accuracy() {
    check_write_for_example_file("tests/fixtures/with_accuracy.gpx");
}

fn check_write_for_example_file(filename: &str) {
    let reference_gpx = read_test_gpx_file(filename);
    let written_gpx = write_and_reread_gpx(&reference_gpx);

    check_metadata_equal(&reference_gpx, &written_gpx);
    check_points_equal(&reference_gpx, &written_gpx);
}

fn read_test_gpx_file(filename: &str) -> Gpx {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let result = read(reader);
    assert!(result.is_ok());

    result.unwrap()
}

fn write_and_reread_gpx(reference_gpx: &Gpx) -> Gpx {
    let mut buffer: Vec<u8> = Vec::new();
    let result = write(&reference_gpx, &mut buffer);
    assert!(result.is_ok());

    let written_gpx = read(buffer.as_slice()).unwrap();
    written_gpx
}

fn check_metadata_equal(reference_gpx: &Gpx, written_gpx: &Gpx) {
    let reference = &reference_gpx.metadata;
    let written = &written_gpx.metadata;
    if reference.is_some() {
        assert!(written.is_some());
    } else {
        assert!(written.is_none());
        return;
    }
    let reference = reference.as_ref().unwrap();
    let written = written.as_ref().unwrap();
    assert_eq!(reference.name, written.name);
    assert_eq!(reference.time, written.time);
    check_links_equal(&reference.links, &written.links);
}

fn check_links_equal(reference: &Vec<Link>, written: &Vec<Link>) {
    assert_eq!(reference.len(), written.len());
    for (r, w) in reference.iter().zip(written) {
        assert_eq!(r.href, w.href);
        assert_eq!(r.text, w.text);
    }
}

fn check_points_equal(reference: &Gpx, written: &Gpx) {
    check_waypoints_equal(&reference.waypoints, &written.waypoints);
    assert_eq!(reference.tracks.len(), written.tracks.len());
    for (r_track, w_track) in reference.tracks.iter().zip(written.tracks.iter()) {
        assert_eq!(r_track.name, w_track.name);
        assert_eq!(r_track.segments.len(), w_track.segments.len());
        for (r_seg, w_seg) in r_track.segments.iter().zip(w_track.segments.iter()) {
            check_waypoints_equal(&r_seg.points, &w_seg.points);
        }
    }
}

fn check_waypoints_equal(reference: &Vec<Waypoint>, written: &Vec<Waypoint>) {
    assert_eq!(reference.len(), written.len());
    for (r_wp, w_wp) in reference.iter().zip(written) {
        assert_eq!(r_wp.point(), w_wp.point());
        assert_eq!(r_wp.elevation, w_wp.elevation);
        assert_eq!(r_wp.speed, w_wp.speed);
        assert_eq!(r_wp.time, w_wp.time);
        assert_eq!(r_wp.geoidheight, w_wp.geoidheight);
        assert_eq!(r_wp.name, w_wp.name);
        assert_eq!(r_wp.comment, w_wp.comment);
        assert_eq!(r_wp.description, w_wp.description);
        assert_eq!(r_wp.source, w_wp.source);
        check_links_equal(&r_wp.links, &w_wp.links);
        assert_eq!(r_wp.symbol, w_wp.symbol);
        assert_eq!(r_wp._type, w_wp._type);
        assert_eq!(r_wp.fix, w_wp.fix);
        assert_eq!(r_wp.sat, w_wp.sat);
        assert_eq!(r_wp.hdop, w_wp.hdop);
        assert_eq!(r_wp.vdop, w_wp.vdop);
        assert_eq!(r_wp.pdop, w_wp.pdop);
        assert_eq!(r_wp.age, w_wp.age);
        assert_eq!(r_wp.dgpsid, w_wp.dgpsid);
    }
}
