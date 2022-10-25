# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]

### Fixed

- Fixed an issue that was preventing the SimConnect client from closing on user "Disconnect".

## [v0.2.4] - 2022-08-26

### Changed

- All dependencies are updated to the latest version.

## [v0.2.3] - 2021-05-31

### Changed

- The GPS Track is now calculated by subtracting MAGVAR from GPS GROUND MAGNETIC TRACK

## [v0.2.2] - 2021-05-05

### Changed

- The GPS track is now based on 'GPS GROUND MAGNETIC TRACK' instead of 'GPS GROUND TRUE TRACK'

## [v0.2.1] - 2021-05-01

### Changed

- The GPS track is now based on 'GPS GROUND TRUE TRACK' instead of 'PLANE HEADING DEGREES TRUE'

## [v0.2.0] - 2021-02-21

### Added

- SkyDemon documentation
- Garmin Pilot documentation

### Changed

- Rename "Broadcast subnet mask" to "Broadcast address" and add an explanation for how it works

### Fixed

- Fix the issue that is causing the broadcast to happen on the "255.255.255.255" netmask, irrespective of what is configured in the UI
- Fix the text input mask of "Broadcast address"
- Fix the issue that is causing the check for a new version to happen more often than needed

## [v0.1.0] - 2020-12-26

### Initial Release of MSFS 2020 GPS Link

[unreleased]: https://github.com/mihai-dinculescu/cargo-wipe
[v0.2.4]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.4
[v0.2.3]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.3
[v0.2.2]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.2
[v0.2.1]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.0
[v0.1.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.0
