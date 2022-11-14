# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]

## [v0.3.1] - 2022-11-14

### Fixed

- GPS Ground Speed is now correctly sent in knots instead of meters per second over COM.

## [v0.3.0] - 2022-11-07

### Added

- The option to broadcast to a virtual COM port, emulating an RS232 GPS device, has been added.

### Changed

- When a new release is available, the notification now includes its version number.

## [v0.3.0-alpha.2] - 2022-11-06

### Changed

- The **Baud rate** field is now a dropdown with a fixed set of options.

### Fixed

- The values for **Course Over Ground** and **Magnetic Variation** sent over COM should now be correct.

## [v0.3.0-alpha.1] - 2022-11-06

### Added

- The option to broadcast to a virtual COM port, emulating an RS232 GPS device, has been added.

### Changed

- When a new release is available, the notification now includes its version number.

## [v0.2.5] - 2022-10-29

### Changed

- The "Connected" status will now be reported much quicker (up to 6x times faster).
- Error handling and tracing have been improved.

### Fixed

- Fixed an issue that was causing the UI to occasionally think it's still connected after MSFS2020 has been closed.
- Fixed an issue that was preventing the SimConnect Client from properly closing on user "Disconnect".
- Fixed an issue that was causing SimConnect Client to unnecessarily connect a second time on user "Connect".

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

[unreleased]: https://github.com/mihai-dinculescu/msfs-2020-gps-link
[v0.3.0]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.3.0
[v0.3.0-alpha.2]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.3.0-alpha.2
[v0.3.0-alpha.1]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.3.0-alpha.1
[v0.2.5]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.5
[v0.2.4]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.4
[v0.2.3]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.3
[v0.2.2]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.2
[v0.2.1]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.2.0
[v0.1.0]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/tree/v0.1.0
