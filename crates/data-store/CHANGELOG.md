# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0](https://github.com/fluencelabs/aquavm/compare/avm-data-store-v0.5.0...avm-data-store-v0.6.0) (2023-02-27)


### ⚠ BREAKING CHANGES

* **data_store:** use particle_id + current_peer_id as prev_data key in DataStore ([#485](https://github.com/fluencelabs/aquavm/issues/485))

### Bug Fixes

* **data_store:** use particle_id + current_peer_id as prev_data key in DataStore ([#485](https://github.com/fluencelabs/aquavm/issues/485)) ([36e1c87](https://github.com/fluencelabs/aquavm/commit/36e1c8762c1888f375adacc21907d98a811d28d9))

## [0.5.0](https://github.com/fluencelabs/aquavm/compare/avm-data-store-v0.4.1...avm-data-store-v0.5.0) (2023-02-21)


### ⚠ BREAKING CHANGES

* **avm:** improve anomaly detection

### Features

* **avm:** improve anomaly detection ([5e6863d](https://github.com/fluencelabs/aquavm/commit/5e6863d4d59684d4f2b509ece6e597831e648f05))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * avm-interface bumped from 0.28.1 to 0.28.2

## [Unreleased]

## [0.4.1] - 2022-09-13

### Other
- Introduce length functor (#314)
- Update all non-major Rust dependencies (#309)
- Get rid of unsafe code in the interpreter (#303)
- make clippy happy (#291)
