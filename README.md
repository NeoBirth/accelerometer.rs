# accelerometer.rs

[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![Build Status][build-image]][build-link]
[![LGPL 3.0 licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Generic Rust accelerometer support, including traits and types for taking
readings from 2 or 3-axis accelerometers

[Documentation][docs-link]

## Supported Crates

The following dependent crates providing platform-agnostic accelerometer
drivers which use the types and traits from this library:

| Device Name | Description | Crate + Docs |
|-------------|-------------|--------------|
| [ADXL343]   | Analog Devices 3-axis accelerometer | [![crates.io][adxl343-crate-img]][adxl343] [![docs.rs][adxl343-docs-img]][adxl343-docs] |

[adxl343]: https://github.com/NeoBirth/ADXL343.rs
[adxl343-crate-img]: https://img.shields.io/crates/v/adxl343.svg
[adxl343-docs-img]: https://docs.rs/adxl343/badge.svg
[adxl343-docs]: https://docs.rs/adxl343/

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 NeoBirth Developers

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crate-img]: https://img.shields.io/crates/v/accelerometer.svg
[crate-link]: https://crates.io/crates/accelerometer
[docs-img]: https://docs.rs/accelerometer/badge.svg
[docs-link]: https://docs.rs/accelerometer/
[build-image]: https://secure.travis-ci.org/NeoBirth/accelerometer.rs.svg?branch=master
[build-link]: https://travis-ci.org/NeoBirth/accelerometer.rs
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/NeoBirth/accelerometer.rs/blob/master/LICENSE-APACHE
[gitter-image]: https://badges.gitter.im/NeoBirth/accelerometer.rs.svg
[gitter-link]: https://gitter.im/NeoBirth/community
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/accelerometer.rs/blob/master/CODE_OF_CONDUCT.md
