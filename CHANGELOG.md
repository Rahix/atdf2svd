# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.3.2] - 2022-09-20
### Changed
- Errors during interrupt generation have been converted to warnings.  This
  means you can generate an SVD even without interrupts ([#31]).
- Updated dependencies.

[#31]: https://github.com/Rahix/atdf2svd/pull/31


## [0.3.1] - 2021-08-01
### Fixed
- Added missing `<addressBlock>` elements to be more in line with the SVD spec
  ([#26]).
- Fixed a few SVDConv errors and warnings ([#28]).

[#26]: https://github.com/Rahix/atdf2svd/pull/26
[#28]: https://github.com/Rahix/atdf2svd/pull/28



## [0.3.0] - 2021-03-18
### Fixed
- Fixed bitmask calculation not being correct for register sizes greater than
  1 byte ([#21]).
- Fixed use of the wrong attribute for access mode. `rw` should be used instead
  of `ocd-rw` ([#24]).

[#21]: https://github.com/Rahix/atdf2svd/pull/21
[#24]: https://github.com/Rahix/atdf2svd/pull/24


## [0.2.0] - 2020-11-25
### Added
- `--version` commandline argument.
- Support for write-only register fields ([#9]).
- Support for newer AVR MCUs in the `signals_to_port_fields` patch ([#10]).

### Changed
- Fall back to module caption if instance caption is missing ([#15]).
- Improved naming of interrupts for newer AVR MCUs ([#19]).
- When multiple interrupts with the same vector exist, their names are merged
  into a single interrupt definition ([#20]).

### Fixed
- Properly handle empty `caption` attribute for enumerated values ([#12]).
- Fixed empty `caption` for peripherals ([`3f0003c75350`]).
- Enumerated values which don't actually fit into a field are now dropped with
  a warning ([#14]).

[#9]: https://github.com/Rahix/atdf2svd/pull/9
[#10]: https://github.com/Rahix/atdf2svd/pull/10
[#12]: https://github.com/Rahix/atdf2svd/pull/12
[#14]: https://github.com/Rahix/atdf2svd/pull/14
[#15]: https://github.com/Rahix/atdf2svd/pull/15
[#19]: https://github.com/Rahix/atdf2svd/pull/19
[#20]: https://github.com/Rahix/atdf2svd/pull/20
[`3f0003c75350`]: https://github.com/Rahix/atdf2svd/commit/3f0003c753506618d8da1bd9e2995e9d88b0d878


## [0.1.4] - 2020-10-02
### Changed
- Switch to gumdrop instead of structopt ([`6b21b7ac3f91`]).

### Fixed
- Filter out peripherals with no registers instead of erroring in this case ([#8]).

[#8]: https://github.com/Rahix/atdf2svd/pull/8
[`6b21b7ac3f91`]: https://github.com/Rahix/atdf2svd/commit/6b21b7ac3f910f9a497bbd70cdd1b64771a799d8


## [0.1.3] - 2020-07-26
### Fixed
- Allow missing caption for peripherals.
- Make sure child nodes in register-group have the correct name.
- Only parse `interrupt` children and ignore any other named ones.
- Allow `signals_to_port_fields` patch to fail.


[Unreleased]: https://github.com/Rahix/atdf2svd/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/Rahix/atdf2svd/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/Rahix/atdf2svd/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Rahix/atdf2svd/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Rahix/atdf2svd/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/Rahix/atdf2svd/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/Rahix/atdf2svd/releases/tag/v0.1.3
