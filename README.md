<img src="https://raw.githubusercontent.com/NeoBirth/accelerometer.rs/develop/img/cartesian-ferris.png" width="150" height="110">

# accelerometer.rs

[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![LGPL 3.0 licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Generic Rust accelerometer support, including traits and types for taking
readings from 2 or 3-axis accelerometers, intended for use with
platform-agnostic accelerometer drivers written using [embedded-hal].

Uses `no_std`-oriented 2D and 3D vector types from the [micromath.rs] crate.

Additionally includes support for tracking device orientations using
accelerometer data.

[Documentation][docs-link]

## Requirements

- Rust 1.47+

## Supported Crates

The following dependent crates providing platform-agnostic accelerometer
drivers built on `embedded-hal` which also implement the [`Accelerometer`]
and/or the [`RawAccelerometer`] traits from this crate:

| Device Name | Description | Crate + Docs |
|-------------|-------------|--------------|
| [ADXL313]   | Analog Devices 3-axis accelerometer | [![crates.io][adxl313-crate-img]][adxl313] [![docs.rs][adxl313-docs-img]][adxl313-docs] |
| [ADXL343]   | Analog Devices 3-axis accelerometer | [![crates.io][adxl343-crate-img]][adxl343] [![docs.rs][adxl343-docs-img]][adxl343-docs] |
| [ADXL355]   | Analog Devices 3-axis accelerometer | [![crates.io][adxl355-crate-img]][adxl355] [![docs.rs][adxl355-docs-img]][adxl355-docs] |
| [LIS2DH12]  | ST 3-Axis MEMS Accelerometer | [![crates.io][lis2dh12-crate-img]][lis2dh12] [![docs.rs][lis2dh12-docs-img]][lis2dh12-docs] |
| [LIS3DH]    | High Performance ST 3-Axis MEMS "nano" Accelerometer | [![crates.io][lis3dh-crate-img]][lis3dh] [![docs.rs][lis3dh-docs-img]][lis3dh-docs] |
| [MMA7660FC] | Grove 3-Axis Digital Accelerometer  | [![crates.io][mma7660fc-crate-img]][mma7660fc] [![docs.rs][mma7660fc-docs-img]][mma7660fc-docs] |
| [STK8BA58] | SENSORTEK 3-Axis MEMS Accelerometer | [![crates.io][stk8ba58-crate-img]][stk8ba58] [![docs.rs][stk8ba58-docs-img]][stk8ba58-docs] |
| [KXTJ31057]   | Silicon Micromachined  3-axis accelerometer | [![crates.io][kxtj31057-crate-img]][kxtj31057] [![docs.rs][kxtj31057-docs-img]][kxtj31057-docs] |


[adxl313]: https://github.com/mvniekerk/adxl313.rs
[adxl313-crate-img]: https://img.shields.io/crates/v/adxl313.svg
[adxl313-docs-img]: https://docs.rs/adxl313/badge.svg
[adxl313-docs]: https://docs.rs/adxl313/

[adxl343]: https://github.com/NeoBirth/ADXL343.rs
[adxl343-crate-img]: https://img.shields.io/crates/v/adxl343.svg
[adxl343-docs-img]: https://docs.rs/adxl343/badge.svg
[adxl343-docs]: https://docs.rs/adxl343/

[adxl355]: https://github.com/JitterCompany/adxl355-rs
[adxl355-crate-img]: https://img.shields.io/crates/v/adxl355.svg
[adxl355-docs-img]: https://docs.rs/adxl355/badge.svg
[adxl355-docs]: https://docs.rs/adxl355/

[lis2dh12]: https://github.com/tkeksa/lis2dh12
[lis2dh12-crate-img]: https://img.shields.io/crates/v/lis2dh12.svg
[lis2dh12-docs-img]: https://docs.rs/lis2dh12/badge.svg
[lis2dh12-docs]: https://docs.rs/lis2dh12/

[lis3dh]: https://github.com/BenBergman/lis3dh-rs
[lis3dh-crate-img]: https://img.shields.io/crates/v/lis3dh.svg
[lis3dh-docs-img]: https://docs.rs/lis3dh/badge.svg
[lis3dh-docs]: https://docs.rs/lis3dh/

[mma7660fc]: https://github.com/rahul-thakoor/mma7660fc/
[mma7660fc-crate-img]: https://img.shields.io/crates/v/mma7660fc.svg
[mma7660fc-docs-img]: https://docs.rs/mma7660fc/badge.svg
[mma7660fc-docs]: https://docs.rs/mma7660fc/

[STK8BA58]: https://gitlab.com/slusheea/stk8ba58/
[stk8ba58-crate-img]: https://img.shields.io/crates/v/stk8ba58.svg
[stk8ba58-docs-img]: https://docs.rs/stk8ba58/badge.svg
[stk8ba58-docs]: https://docs.rs/stk8ba58/

[kxtj31057]: https://github.com/p3zhy/kxtj3-1057/
[kxtj31057-crate-img]: https://img.shields.io/crates/v/kxtj3-1057.svg
[kxtj31057-docs-img]: https://docs.rs/kxtj3-1057/badge.svg
[kxtj31057-docs]: https://docs.rs/kxtj3-1057/


## Orientation Tracking (3-axis accelerometers only)

![ezgif-1-16e98d9b86ad](https://user-images.githubusercontent.com/797/55564522-ebaf2b00-56ac-11e9-808f-9809e85c1bd2.gif)

One of the main features of this crate is device orientation tracking for
3-axis accelerometers, which is gated under the `orientation` cargo feature
(enabled-by-default). This provides smartphone-like device position sensing
using an accelerometer alone, returned as one of the variants of the
[`accelerometer::Orientation`] enum.

See the [`accelerometer::Tracker`] documentation for more information.

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019-2020 NeoBirth Developers

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[//]: # (badges)

[crate-img]: https://img.shields.io/crates/v/accelerometer.svg
[crate-link]: https://crates.io/crates/accelerometer
[docs-img]: https://docs.rs/accelerometer/badge.svg
[docs-link]: https://docs.rs/accelerometer/
[build-image]: https://github.com/neobirth/accelerometer.rs/workflows/Rust/badge.svg
[build-link]: https://github.com/neobirth/accelerometer.rs/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[msrv-image]: https://img.shields.io/badge/rustc-1.31+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/NeoBirth/accelerometer.rs/blob/develop/LICENSE-APACHE
[gitter-image]: https://badges.gitter.im/NeoBirth/accelerometer.rs.svg
[gitter-link]: https://gitter.im/NeoBirth/community

[//]: # (general links)

[embedded-hal]: https://docs.rs/embedded-hal/
[micromath.rs]: https://github.com/NeoBirth/micromath
[`Accelerometer`]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html
[`RawAccelerometer`]: https://docs.rs/accelerometer/latest/accelerometer/trait.RawAccelerometer.html
[`accelerometer::Orientation`]: https://docs.rs/accelerometer/latest/accelerometer/orientation/enum.Orientation.html
[`accelerometer::Tracker`]: https://docs.rs/accelerometer/latest/accelerometer/orientation/struct.Tracker.html
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/accelerometer.rs/blob/develop/CODE_OF_CONDUCT.md