# Changelog for v0.x Series

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

## v0.3.4
_28 April 2022_

* Fix a subtle bug in converting to modern function prototypes.

## v0.3.3
_28 April 2022_

* Update zlib to v1.2.12. (#38)
* Add Rust 1.60.0 to CI build matrix.
* Remove Rust 1.59.0 from CI build matrix.

## v0.3.2
_08 March 2022_

* Update XMP Toolkit to February 2022 release. (#35)
* Update libexpat to v2.4.6. (#36)
* Add Rust 1.59.0 to CI build matrix.
* Remove Rust 1.58.0 from CI build matrix.

## v0.3.1
_28 January 2022_

* Implement `Display` trait in `XmpFileError` enum. (#32)
* Fix Clippy warning about `assert_eq!` with a bool value. (#33)
* Add Rust 1.58.0 to CI build matrix. (#30)
* Remove Rust 1.56.0 from CI build matrix.

## v0.3.0
_27 October 2021_

* Add Rust 1.56.0 to CI build matrix. (#29)
* Remove Rust 1.53.0 from CI build matrix.
* Update XMP Toolkit submodule to October 2021 Release. (#28)

## v0.2.0
_12 October 2021_

* Update XMP Toolkit submodule to August 2021 Release.
* Bumped MSRV to 1.46.0 due to new dependency from bitflags crate.
* Add Rust 1.55.0 to CI build matrix.
* Remove Rust 1.53.0 from CI build matrix.
* Update version references in Cargo.toml.

## v0.1.8
_23 June 2021_

* Include libexpat via git submodule and update to version 2.4.1. (#18)
* Add Rust 1.53.0 to CI build matrix.
* Remove Rust 1.51.0 from CI build matrix.

## v0.1.7
_29 March 2021_

* Add Rust 1.51.0 to CI build matrix.
* Remove Rust 1.48.0 from CI build matrix.

## v0.1.6
_25 November 2020_

* Handle exceptions on OpenFile. (#13)
* Add Rust 1.48.0 to CI build matrix.
* Remove Rust 1.47.0 from CI build matrix.

## v0.1.5
_12 October 2020_

* Add support for building on Windows.
* Add Rust 1.47.0 to CI build matrix.
* Remove Rust 1.46.0 from CI build matrix.

## v0.1.4
_04 September 2020_

* Add Mac OS builds to CI build matrix.
* Add Rust 1.46.0 to CI build matrix.

## v0.1.3
_02 September 2020_

* Improve API documentation.

## v0.1.2
_01 September 2020_

* Fix another build issue that occurs only within the docs.rs environment.

## v0.1.1
_01 September 2020_

* Fix build issue that occurs within the docs.rs environment.

## v0.1.0
_01 September 2020_

* **Initial public release.**
  * Supports Rust 1.44.0 and higher.
  * Includes the 24 July 2020 release of C++ XMP Toolkit SDK.
