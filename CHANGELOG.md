# Changelog: aki-unbody
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Include code review report: `docs/reviews/2026-06-01_code_review.3.md`

### Changed
- Organize review reports: move `docs/review.1.md` and `docs/review.2.md` to `docs/reviews/` with date prefix

## [0.2.1] - 2026-05-20

### Changed
- Update dependencies: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2), `runnel` (0.4.2)
- Update minimum supported Rust version (MSRV) to 1.68.0
- Optimize performance of `run_normal` to use `VecDeque` for O(1) tail buffering
- Improve memory efficiency of `run_inverse` using a sliding window approach for large files
- Standardize code style by removing unjustified `#[rustfmt::skip]` and applying standard `rustfmt`

### Fixed
- Address Clippy warning: `needless_borrow`

### Removed
- Delete `memx-cdy` dependency

## [0.2.0] - 2025-09-15

### Added
- Include specification documents in `specs/`
- Expand test suite
- Include test case for invalid UTF-8 input

### Changed
- Implement `IntoIterator` compatibility for arguments in `execute()`
- Update dependencies: `runnel` (0.4.0), `rust-version-info-file` (0.2)
- Update CI to use `actions/checkout@v4` in GitHub Actions

### Fixed
- Resolve capacity overflow in `-t` option
- Correct minimum supported version in documentation
- Set Rust-version to 1.65.0
- Address Clippy warning: `derivable_impls`

## [0.1.19] - 2024-06-19

### Added
- Configure GitHub Actions workflows: Ubuntu, macOS, Windows
- Include test status badges in `README.tpl`
- Enable Miri support for tests
- Configure `tarpaulin` support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Remove `cfg(has_not_matches)`
- Refactor `Makefile`
- Update dependencies: `flood-tide` (0.2.9), `flood-tide-gen` (0.1.20), `memx-cdy` (0.1.11), `runnel` (0.3.16), `exec-target` (0.2.8), `indoc` (2.0.5), `rust-version-info-file` (0.1.8)
- Update MSRV from 1.56.0 to 1.60.0

### Removed
- Delete `COPYING` file

### Fixed
- Correct license files: `LICENSE-APACHE`, `LICENSE-MIT`
- Address Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`

## [0.1.18] - 2023-01-11

### Added
- Include badges in `README.tpl`
- Define `rust-version = "1.56.0"` in `Cargo.toml`

### Changed
- Format `CHANGELOG.md`
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15)

### Fixed
- Address Clippy warnings: `Eq` implementation for types deriving `PartialEq`, `uninlined_format_args`

## [0.1.17] - 2022-06-18

### Changed
- Migrate to Edition 2021
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), `semver` (1.0.10)

## [0.1.16] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `exec-target` (0.2.5), `rust-version-info-file` (0.1.5)

## [0.1.15] - 2021-11-15

### Added
- Include additional documentation

## [0.1.14] - 2021-11-15

### Added
- Include additional documentation

## [0.1.13] - 2021-11-15

### Added
- Include additional documentation

### Changed
- Update minimum supported Rustc to 1.47.0
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `rust-version-info-file` (0.1.3)

## [0.1.12] - 2021-09-11

### Added
- Include `indoc` (1.0.3) dependency

### Changed
- Address Cargo Clippy warnings
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), `exec-target` (0.2.3)
- Improve `TARGET_EXE_PATH` definition using `CARGO_BIN_EXE` environment variable

## [0.1.11] - 2021-06-24

### Added
- Enable fast memory operations via `memx_cdy::memx_init()`

### Changed
- Improve `TARGET_EXE_PATH` definition with `env!("CARGO_BIN_EXE_aki-unbody")`

### Fixed
- Resolve feature gate bug: `#[cfg(feature = "debian_build")]`

## [0.1.10] - 2021-06-03

### Added
- Enable support for `debian_build` feature

### Changed
- Update dependencies: `flood-tide` (0.2.2), `regex` (1.5.4)

### Fixed
- Resolve command option bug: `-X rust-version-info`

## [0.1.9] - 2021-04-23

### Fixed
- Resolve build script issue in `build.rs`

## [0.1.8] - 2021-04-23

### Added
- Implement command option: `-X`

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1), `regex` (1.4.6)

## [0.1.7] - 2021-04-19

### Changed
- Update dependency: `flood-tide-gen` (0.1.10)

## [0.1.6] - 2021-04-07

### Changed
- Update dependencies: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), `runnel` (0.3.6)

## [0.1.5] - 2021-03-22

### Changed
- Update dependency: `anyhow`

## [0.1.4] - 2021-03-08

### Changed
- Update dependencies: `runnel`, `rustc_version` (0.3)

## [0.1.3] - 2021-03-08

### Changed
- Update dependency: `runnel`

## [0.1.2] - 2021-03-06

### Added
- Implement new algorithm to address high memory usage

## [0.1.1] - 2021-03-03

### Added
- Include examples in command help output

## [0.1.0] - 2021-03-02

### Added
- Initial release

[Unreleased]: https://github.com/aki-akaguma/aki-unbody/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-unbody/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.19..v0.2.0
[0.1.19]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-unbody/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-unbody/releases/tag/v0.1.0
