# Changelog: aki-unbody

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Code review report: `docs/reviews/2026-06-01_code_review.3.md`

### Changed
- Organize review reports: move `docs/review.1.md` and `docs/review.2.md` to `docs/reviews/` with date prefix

## [0.2.1] - 2026-05-20

### Changed
- Dependency updates: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2)
- Dependency update: `runnel` (0.4.2)
- Minimum supported Rust version (MSRV) to 1.68.0
- Performance: optimize `run_normal` to use `VecDeque` for O(1) tail buffering
- Memory efficiency: optimize `run_inverse` using a sliding window approach for large files
- Code style: remove unjustified `#[rustfmt::skip]` and apply standard `rustfmt`

### Fixed
- Clippy warning: `needless_borrow`

### Removed
- `memx-cdy` dependency

## [0.2.0] - 2025-09-15

### Added
- Specification documents in `specs/`
- Expanded test suite
- Test case for invalid UTF-8 input

### Changed
- `IntoIterator` compatibility for arguments in `execute()`
- Dependency updates: `runnel` (0.4.0), `rust-version-info-file` (0.2)
- CI: use `actions/checkout@v4` in GitHub Actions

### Fixed
- Capacity overflow in `-t` option
- Minimum supported version in documentation
- Rust-version setting: 1.65.0
- Clippy warning: `derivable_impls`

## [0.1.19] - 2024-06-19

### Added
- GitHub Actions workflows: Ubuntu, macOS, Windows
- Test status badges in `README.tpl`
- Miri support for tests
- `tarpaulin` support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Remove `cfg(has_not_matches)`
- Refactor `Makefile`
- Dependency updates: `flood-tide` (0.2.9), `flood-tide-gen` (0.1.20)
- Dependency updates: `memx-cdy` (0.1.11), `runnel` (0.3.16)
- Dependency updates: `exec-target` (0.2.8), `indoc` (2.0.5), `rust-version-info-file` (0.1.8)

### Removed
- `COPYING` file

### Fixed
- License files: `LICENSE-APACHE`, `LICENSE-MIT`
- Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
- Clippy warnings: `uninlined_format_args`, `unused_imports`
- MSRV: update from 1.56.0 to 1.60.0

## [0.1.18] - 2023-01-11

### Added
- Badges in `README.tpl`
- `rust-version = "1.56.0"` in `Cargo.toml`

### Changed
- Format `CHANGELOG.md`
- Dependency update: `anyhow` (1.0.68)
- Dependency updates: `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19)
- Dependency updates: `memx-cdy` (0.1.10), `runnel` (0.3.15)

### Fixed
- Clippy warnings: `Eq` implementation for types deriving `PartialEq`
- Clippy warning: `uninlined_format_args`

## [0.1.17] - 2022-06-18

### Changed
- Migrate to Edition 2021
- Dependency updates: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11)
- Dependency updates: `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6)
- Dependency update: `semver` (1.0.10)

## [0.1.16] - 2022-05-22

### Changed
- Dependency updates: `runnel` (0.3.10), `memx` (0.1.20)
- Dependency updates: `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6)
- Dependency updates: `exec-target` (0.2.5), `rust-version-info-file` (0.1.5)

## [0.1.15] - 2021-11-15

### Added
- Additional documentation

## [0.1.14] - 2021-11-15

### Added
- Additional documentation

## [0.1.13] - 2021-11-15

### Added
- Additional documentation

### Changed
- Minimum supported Rustc to 1.47.0
- Dependency updates: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9)
- Dependency updates: `anyhow` (1.0.45), `libc` (0.2.107)
- Dependency updates: `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), `rust-version-info-file` (0.1.3)

## [0.1.12] - 2021-09-11

### Added
- Dependency: `indoc` (1.0.3)

### Changed
- Address Cargo Clippy warnings
- Dependency updates: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8)
- Improve `TARGET_EXE_PATH` definition using `CARGO_BIN_EXE` environment variable
- Dependency update: `exec-target` (0.2.3)

## [0.1.11] - 2021-06-24

### Added
- Fast memory operations via `memx_cdy::memx_init()`

### Changed
- Improve `TARGET_EXE_PATH` definition with `env!("CARGO_BIN_EXE_aki-unbody")`

### Fixed
- Feature gate bug: `#[cfg(feature = "debian_build")]`

## [0.1.10] - 2021-06-03

### Added
- Support for `debian_build` feature

### Changed
- Dependency updates: `flood-tide` (0.2.2), `regex` (1.5.4)

### Fixed
- Command option bug: `-X rust-version-info`

## [0.1.9] - 2021-04-23

### Fixed
- Build script issue in `build.rs`

## [0.1.8] - 2021-04-23

### Added
- Command option: `-X`

### Changed
- Dependency updates: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1)
- Dependency update: `regex` (1.4.6)

## [0.1.7] - 2021-04-19

### Changed
- Dependency update: `flood-tide-gen` (0.1.10)

## [0.1.6] - 2021-04-07

### Changed
- Dependency updates: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), `runnel` (0.3.6)

## [0.1.5] - 2021-03-22

### Changed
- Dependency update: `anyhow`

## [0.1.4] - 2021-03-08

### Changed
- Dependency updates: `runnel`, `rustc_version` (0.3)

## [0.1.3] - 2021-03-08

### Changed
- Dependency update: `runnel`

## [0.1.2] - 2021-03-06

### Added
- New algorithm to address high memory usage

## [0.1.1] - 2021-03-03

### Added
- Examples to command help output

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
