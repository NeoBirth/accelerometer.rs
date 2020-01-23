# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.11.0 (2020-01-23)
### Changed
- Split `Accelerometer` and `RawAccelerometer` ([#53], [#54])
- Move vector types under `accelerometer::vector::*` ([#54])
- Use `accelerometer::Error` in (Raw)`Accelerometer` API ([#54])

[#53]: https://github.com/NeoBirth/accelerometer.rs/pull/53
[#54]: https://github.com/NeoBirth/accelerometer.rs/pull/54

## 0.10.0 (2019-12-02)
### Changed
- Upgrade to `micromath` v1.0 ([#48])

[#48]: https://github.com/NeoBirth/accelerometer.rs/pull/48

## 0.9.0 (2019-11-14)
### Changed
- Upgrade to `micromath` v0.5 ([#45])

[#45]: https://github.com/NeoBirth/accelerometer.rs/pull/45

## 0.8.1 (2019-11-01)
### Fixed
- Fix doc link resolution failure ([#40])

[#40]: https://github.com/NeoBirth/accelerometer.rs/pull/40

## 0.8.0 (2019-11-01)
### Changed
- Decouple `Tracker` from `Accelerometer` ([#38])

[#38]: https://github.com/NeoBirth/accelerometer.rs/pull/38

## 0.7.0 (2019-10-02)
### Changed
- Upgrade to `micromath` v0.4 ([#34])

[#34]: https://github.com/NeoBirth/accelerometer.rs/pull/34

## 0.6.1 (2019-09-26)
### Changed
- README updates ([#30], [#32])

[#30]: https://github.com/NeoBirth/accelerometer.rs/pull/30
[#32]: https://github.com/NeoBirth/accelerometer.rs/pull/32

## 0.6.0 (2019-05-04)
### Changed
- Update to micromath v0.3 ([#28])

[#28]: https://github.com/NeoBirth/accelerometer.rs/pull/28

## 0.5.2 (2019-05-03)
### Added
- `lib.rs`: Add docs on writing an accelerometer driver ([#24])

### Fixed
- `lib.rs`: Fix `html_logo_url` to point at `develop` branch ([#23])

[#23]: https://github.com/NeoBirth/accelerometer.rs/pull/23
[#24]: https://github.com/NeoBirth/accelerometer.rs/pull/24

## 0.5.1 (2019-05-03)
### Changed
- Update to micromath v0.2 ([#20])

[#20]: https://github.com/NeoBirth/accelerometer.rs/pull/20

## 0.5.0 (2019-05-03)
### Changed
- Use the `micromath` crate ([#18])

[#18]: https://github.com/NeoBirth/accelerometer.rs/pull/18

## 0.4.0 (2019-04-04)
### Added
- Add a `Component` trait for vector components ([#16])

### Changed
- Simplified orientation tracker ([#15])
- Refactor `vector` module ([#13])

[#13]: https://github.com/NeoBirth/accelerometer.rs/pull/13
[#15]: https://github.com/NeoBirth/accelerometer.rs/pull/15
[#16]: https://github.com/NeoBirth/accelerometer.rs/pull/16

## 0.3.0 (2019-03-31)
### Added
- `Orientation` enum ([#11])
- Basic device tracking support ([#6])

[#6]: https://github.com/NeoBirth/accelerometer.rs/pull/6
[#11]: https://github.com/NeoBirth/accelerometer.rs/pull/11

## 0.2.0 (2019-03-26)
### Added
- Vector types ([#7])
- Error type ([#5])

[#5]: https://github.com/NeoBirth/accelerometer.rs/pull/5
[#7]: https://github.com/NeoBirth/accelerometer.rs/pull/7

## 0.1.0 (2019-03-24)
### Added
- Initial implementation ([#1])

[#1]: https://github.com/NeoBirth/accelerometer.rs/pull/1
