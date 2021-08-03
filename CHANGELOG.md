# CHANGELOG

## 0.8.3

- [#55](https://github.com/georust/gpx/pull/55): Allow `name` tags inside of `trk`s to be empty

## 0.8.2

- [#49](https://github.com/georust/gpx/pull/49): Use correct XML tag "desc" instead of "description"
- [#48](https://github.com/georust/gpx/pull/48): Support parsing copyright tag in metadata

## 0.8.1

- [allow empty fields: "desc", "cmt", "description", "keywords", "src"](https://github.com/georust/gpx/pull/25)
- [Add support for route tag](https://github.com/georust/gpx/pull/26)

## 0.8.0

- [#24](https://github.com/georust/gpx/pull/24): Parse `link` elements inside `trk` tags, `extensions` inside `metadata`

## 0.7.0

Just different dependency updates.

## 0.6.0

- [#22](https://github.com/georust/gpx/pull/22): Support writing GPX files

## 0.5.0

- [#20](https://github.com/georust/gpx/pull/20): Switch from `geo` to `geo-types`

## 0.4.1

- [`d7fec64`](https://github.com/georust/gpx/commit/d7fec646469c820a299d32f8b09daa2c7f4525a3): Support geoidheight waypoint tag

## 0.4.0

- [`5869643`](https://github.com/georust/gpx/commit/5869643a4c6021882dffca37ee02d4f2ab9b8ecf): Bump dependencies: `geo`, `xml-rs`
- [`78ce583`](https://github.com/georust/gpx/commit/78ce583906920ebfd832c5b6a03ae1bc72f3fde1): Rework parsing: More strict and (hopefully) cleaner

## 0.3.0

The 0.3.0 release contains added support for GPX 1.0, "bounds" support for tracks, and improved error reporting.

- [`385ca1c`](https://github.com/georust/gpx/commit/385ca1c04c115a5bffa19d1606839f28ecffce5c): Support GPX 1.0 ([#6](https://github.com/georust/gpx/pull/6))
- [`9680234`](https://github.com/georust/gpx/commit/9680234a8f47da0c2559ed5769d0f533cffb4eab): Handle the GPX version attribute ([#6](https://github.com/georust/gpx/pull/6))
- [`6e07049`](https://github.com/georust/gpx/commit/6e07049401fbc99de0220fa796a4f5e94ab6282a): Handle bounds attribute ([#6](https://github.com/georust/gpx/pull/6))
- [`92dbb56`](https://github.com/georust/gpx/commit/92dbb56564cfd9defdc9a655d0cda84af5c3ec64): Include the child tag name into 'InvalidChildElement' error. ([#7](https://github.com/georust/gpx/pull/7))

## 0.2.0

The 0.2.0 release contains new changes that add GPX waypoint accuracy information and `Clone` for public types.

- [`74d5132`](https://github.com/georust/gpx/commit/74d5132162f206886454365c5ecfa3facffa21ce): Derive clone for public types ([#3](https://github.com/georust/gpx/pull/3))
- [`13ca700`](https://github.com/georust/gpx/commit/13ca700b8c70837f2656e0e6fbf4c03650f0ac23): Add GPX waypoint accuracy information ([#2](https://github.com/georust/gpx/pull/2))
