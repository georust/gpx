# CHANGELOG

## 0.3.0

The 0.3.0 release contains added support for GPX 1.0, "bounds" support for tracks, and improved error reporting.

- [`385ca1c`](https://github.com/georust/rust-gpx/commit/385ca1c04c115a5bffa19d1606839f28ecffce5c): Support GPX 1.0 ([#6](https://github.com/georust/rust-gpx/pull/6))
- [`9680234`](https://github.com/georust/rust-gpx/commit/9680234a8f47da0c2559ed5769d0f533cffb4eab): Handle the GPX version attribute ([#6](https://github.com/georust/rust-gpx/pull/6))
- [`6e07049`](https://github.com/georust/rust-gpx/commit/6e07049401fbc99de0220fa796a4f5e94ab6282a): Handle bounds attribute ([#6](https://github.com/georust/rust-gpx/pull/6))
- [`92dbb56`](https://github.com/georust/rust-gpx/commit/92dbb56564cfd9defdc9a655d0cda84af5c3ec64): Include the child tag name into 'InvalidChildElement' error. ([#7](https://github.com/georust/rust-gpx/pull/7))

## 0.2.0

The 0.2.0 release contains new changes that add GPX waypoint accuracy information and `Clone` for public types.

- [`74d5132`](https://github.com/georust/rust-gpx/commit/74d5132162f206886454365c5ecfa3facffa21ce): Derive clone for public types ([#3](https://github.com/georust/rust-gpx/pull/3))
- [`13ca700`](https://github.com/georust/rust-gpx/commit/13ca700b8c70837f2656e0e6fbf4c03650f0ac23): Add GPX waypoint accuracy information ([#2](https://github.com/georust/rust-gpx/pull/2))
