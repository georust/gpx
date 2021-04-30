//! generic types for GPX

use geo_types::{Geometry, LineString, MultiLineString, Point, Rect};

use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GpxVersion {
    Unknown,
    Gpx10,
    Gpx11,
}

impl Default for GpxVersion {
    fn default() -> GpxVersion {
        GpxVersion::Unknown
    }
}

/// Gpx is the root element in the XML file.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Gpx {
    /// Version of the Gpx file.
    pub version: GpxVersion,

    /// Creator name or URL of the software that created GPX document
    pub creator: Option<String>,

    /// Metadata about the file.
    pub metadata: Option<Metadata>,

    /// A list of waypoints.
    pub waypoints: Vec<Waypoint>,

    /// A list of tracks.
    pub tracks: Vec<Track>,

    /// A list of routes with a list of point-by-point directions
    pub routes: Vec<Route>,
}

/// Information about the copyright holder and any license governing use of this file.
///
/// By linking to an appropriate license, you may place your data into the
/// public domain or grant additional usage rights.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct GpxCopyright {
    pub author: Option<String>,
    pub year: Option<i32>,
    pub license: Option<String>,
}

/// Metadata is information about the GPX file, author, and copyright restrictions.
///
/// Providing rich, meaningful information about your GPX files allows others to
/// search for and use your GPS data.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Metadata {
    /// The name of the GPX file.
    pub name: Option<String>,

    /// A description of the contents of the GPX file.
    pub description: Option<String>,

    /// The person or organization who created the GPX file.
    pub author: Option<Person>,

    /// URLs associated with the location described in the file.
    pub links: Vec<Link>,

    /// The creation date of the file.
    pub time: Option<DateTime<Utc>>,

    /// Keywords associated with the file. Search engines or databases can use
    /// this information to classify the data.
    pub keywords: Option<String>,

    /// Information about the copyright holder and any license governing use of this file.
    pub copyright: Option<GpxCopyright>,

    /// Bounds for the tracks in the GPX.
    pub bounds: Option<Rect<f64>>,
    /*extensions: GpxExtensionsType,*/
}

/// Route represents an ordered list of waypoints representing a series of turn points leading to a destination.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Route {
    /// GPS name of route.
    pub name: Option<String>,

    /// GPS comment for route.
    pub comment: Option<String>,

    /// User description of route.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability
    /// and accuracy of data.
    pub source: Option<String>,

    /// Links to external information about the route.
    pub links: Vec<Link>,

    /// GPS route number.
    pub number: Option<u32>,

    /// Type (classification) of route.
    pub _type: Option<String>,

    /// Each Waypoint holds the coordinates, elevation, timestamp, and metadata
    /// for a single point in a track.
    pub points: Vec<Waypoint>,
}

impl Route {
    /// Gives the linestring of the segment's points, the sequence of points that
    /// comprises the track segment.
    pub fn linestring(&self) -> LineString<f64> {
        self.points.iter().map(|wpt| wpt.point()).collect()
    }

    /// Creates a new Route with default values.
    ///
    /// ```
    /// extern crate gpx;
    /// extern crate geo_types;
    ///
    /// use gpx::{Route, Waypoint};
    /// use geo_types::Point;
    ///
    /// fn main() {
    ///     let mut route: Route = Route::new();
    ///
    ///     let point = Waypoint::new(Point::new(-121.97, 37.24));
    ///     route.points.push(point);
    /// }
    ///
    pub fn new() -> Route {
        Default::default()
    }
}

impl From<Route> for Geometry<f64> {
    fn from(route: Route) -> Geometry<f64> {
        Geometry::LineString(route.linestring())
    }
}

/// Track represents an ordered list of points describing a path.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Track {
    /// GPS name of track.
    pub name: Option<String>,

    /// GPS comment for track.
    pub comment: Option<String>,

    /// User description of track.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability
    /// and accuracy of data.
    pub source: Option<String>,

    /// Links to external information about the track.
    pub links: Vec<Link>,

    /// Type (classification) of track.
    pub _type: Option<String>,

    /// A Track Segment holds a list of Track Points which are logically
    /// connected in order. To represent a single GPS track where GPS reception
    /// was lost, or the GPS receiver was turned off, start a new Track Segment
    /// for each continuous span of track data.
    pub segments: Vec<TrackSegment>,
    /* pub number: u8,*/
    /* extensions */
    /* trkSeg */
}

impl Track {
    /// Gives the multi-linestring that this track represents, which is multiple
    /// linestrings.
    pub fn multilinestring(&self) -> MultiLineString<f64> {
        self.segments.iter().map(|seg| seg.linestring()).collect()
    }

    /// Creates a new Track with default values.
    ///
    /// ```
    /// use gpx::{Track, TrackSegment};
    ///
    /// let mut track: Track = Track::new();
    ///
    /// let segment = TrackSegment::new();
    /// track.segments.push(segment);
    pub fn new() -> Track {
        Default::default()
    }
}

impl From<Track> for Geometry<f64> {
    fn from(track: Track) -> Geometry<f64> {
        Geometry::MultiLineString(track.multilinestring())
    }
}

/// TrackSegment represents a list of track points.
///
/// This TrackSegment holds a list of Track Points which are logically
/// connected in order. To represent a single GPS track where GPS reception
/// was lost, or the GPS receiver was turned off, start a new Track Segment
/// for each continuous span of track data.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TrackSegment {
    /// Each Waypoint holds the coordinates, elevation, timestamp, and metadata
    /// for a single point in a track.
    pub points: Vec<Waypoint>,
    /* extensions */
}

impl TrackSegment {
    /// Gives the linestring of the segment's points, the sequence of points that
    /// comprises the track segment.
    pub fn linestring(&self) -> LineString<f64> {
        self.points.iter().map(|wpt| wpt.point()).collect()
    }

    /// Creates a new TrackSegment with default values.
    ///
    /// ```
    /// extern crate gpx;
    /// extern crate geo_types;
    ///
    /// use gpx::{TrackSegment, Waypoint};
    /// use geo_types::Point;
    ///
    /// fn main() {
    ///     let mut trkseg: TrackSegment = TrackSegment::new();
    ///
    ///     let point = Waypoint::new(Point::new(-121.97, 37.24));
    ///     trkseg.points.push(point);
    /// }
    pub fn new() -> TrackSegment {
        Default::default()
    }
}

impl From<TrackSegment> for Geometry<f64> {
    fn from(track_segment: TrackSegment) -> Geometry<f64> {
        Geometry::LineString(track_segment.linestring())
    }
}

// A Version of geo_types::Point that has the Default trait implemented, which
// allows us to initialise the GpxPoint with default values compactly
// in the Waypoint::new function below
#[derive(Clone, Debug, PartialEq)]
struct GpxPoint(Point<f64>);

impl Default for GpxPoint {
    fn default() -> GpxPoint {
        GpxPoint(Point::new(0 as f64, 0 as f64))
    }
}

/// Waypoint represents a waypoint, point of interest, or named feature on a
/// map.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Waypoint {
    /// The geographical point.
    point: GpxPoint,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Speed (in meters per second) (only in GPX 1.0)
    pub speed: Option<f64>,

    /// Creation/modification timestamp for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time representation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<DateTime<Utc>>,

    /// The GPS name of the waypoint. This field will be transferred to and
    /// from the GPS. GPX does not place restrictions on the length of this
    /// field or the characters contained in it. It is up to the receiving
    /// application to validate the field before sending it to the GPS.
    pub name: Option<String>,

    /// GPS waypoint comment. Sent to GPS as comment.
    pub comment: Option<String>,

    /// A text description of the element. Holds additional information about
    /// the element intended for the user, not the GPS.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data. "Garmin eTrex", "USGS quad Boston North", e.g.
    pub source: Option<String>,

    /// Links to additional information about the waypoint.
    pub links: Vec<Link>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS. If the GPS
    /// abbreviates words, spell them out.
    pub symbol: Option<String>,

    /// Type (classification) of the waypoint.
    pub _type: Option<String>,

    // <magvar> degreesType </magvar> [0..1] ?
    /// Height of geoid in meters above WGS 84. This correspond to the sea level.
    pub geoidheight: Option<f64>,

    /// Type of GPS fix. `none` means GPS had no fix. To signify "the fix info
    /// is unknown", leave out `fix` entirely. Value comes from the list
    /// `{'none'|'2d'|'3d'|'dgps'|'pps'}`, where `pps` means that the military
    /// signal was used.
    pub fix: Option<Fix>,

    /// Number of satellites used to calculate the GPX fix.
    pub sat: Option<u64>,

    /// Horizontal dilution of precision.
    pub hdop: Option<f64>,

    /// Vertical dilution of precision.
    pub vdop: Option<f64>,

    /// Positional dilution of precision.
    pub pdop: Option<f64>,

    #[deprecated = "Prior to gpx 0.9.0 version crate used incorrect field to parse and emit DGPS age. Use `dgps_age` instead. See https://github.com/georust/gpx/issues/21"]
    pub age: Option<f64>,

    /// Number of seconds since last DGPS update, from the <ageofdgpsdata> element.
    pub dgps_age: Option<f64>,

    /// ID of DGPS station used in differential correction, in the range [0, 1023].
    pub dgpsid: Option<u16>,
    // <extensions> extensionsType </extensions> [0..1] ?
}

impl Waypoint {
    /// Gives the geographical point of the waypoint.
    ///
    /// ```
    /// extern crate geo_types;
    /// extern crate gpx;
    ///
    /// use gpx::Waypoint;
    /// use geo_types::Point;
    ///
    /// fn main() {
    ///     // Kind of useless, but it shows the point.
    ///     let wpt = Waypoint::new(Point::new(-121.97, 37.24));
    ///     let point = wpt.point();
    ///
    ///     println!("waypoint latitude: {}, longitude: {}", point.lat(), point.lng());
    /// }
    /// ```
    pub fn point(&self) -> Point<f64> {
        self.point.0 //.0 to extract the geo_types::Point from the tuple struct GpxPoint
    }

    /// Creates a new Waypoint from a given geographical point.
    ///
    /// ```
    /// extern crate geo_types;
    /// extern crate gpx;
    ///
    /// use gpx::Waypoint;
    /// use geo_types::Point;
    ///
    /// fn main() {
    ///     let point = Point::new(-121.97, 37.24);
    ///
    ///     let mut wpt = Waypoint::new(point);
    ///     wpt.elevation = Some(553.21);
    /// }
    /// ```
    pub fn new(point: Point<f64>) -> Waypoint {
        Waypoint {
            point: GpxPoint(point),
            ..Default::default()
        }
    }
}

impl From<Waypoint> for Geometry<f64> {
    fn from(waypoint: Waypoint) -> Geometry<f64> {
        Geometry::Point(waypoint.point())
    }
}

/// Person represents a person or organization.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email address.
    pub email: Option<String>,

    /// Link to Web site or other external information about person.
    pub link: Option<Link>,
}

/// Link represents a link to an external resource.
///
/// An external resource could be a web page, digital photo,
/// video clip, etc., with additional information.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Link {
    /// URL of hyperlink.
    pub href: String,

    /// Text of hyperlink.
    pub text: Option<String>,

    /// Mime type of content (image/jpeg)
    pub _type: Option<String>,
}

/// Type of the GPS fix.
#[derive(Clone, Debug, PartialEq)]
pub enum Fix {
    /// The GPS had no fix. To signify "the fix info is unknown", leave out the Fix entirely.
    None,
    /// 2D fix gives only longitude and latitude. It needs a minimum of 3 satellites.
    TwoDimensional,
    /// 3D fix gives longitude, latitude and altitude. It needs a minimum of 4 satellites.
    ThreeDimensional,
    /// Differential Global Positioning System.
    DGPS,
    /// Military signal.
    PPS,
    /// Other values that are not in the specification.
    Other(String),
}
