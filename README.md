# MSFS 2020 GPS Link

[![GitHub release][latest_release_badge]][releases_url]
[![Github all releases][downloads_badge]][releases_url]
[![CI][ci_badge]][ci]

Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.\
Tested with [SkyDemon][sky_demon_url] and [Garmin Pilot][garmin_pilot_url].

## [Quick Download][latest_release]

## Tested on

| App                    | Platform | UDP     | COM     |
| ---------------------- | -------- | ------- | ------- |
| Garmin Pilot           | Android  | &check; | -       |
| Jeppesen FliteDeck Pro | PC       | -       | &check; |
| SkyDemon               | Android  | &check; | -       |
| SkyDemon               | PC       | &check; | &check; |
| ForeFlight             | iOS      | ? \*    | -       |

\* Feedback from the ones with the ability to test will be greatly appreciated.

## Usage

- download the latest version from the [Releases][releases_url] page or by using the link above
- unzip & install
- with MSFS 2020 running, open `MSFS 2020 GPS Link` and press "Connect"

![MSFS 2020 GPS Link Usage][usage]

### SkyDemon

With `MSFS 2020 GPS Link` open and connected on your PC, open SkyDemon on your navigation device and press on `Fly` -> `Use X-Plane`.

![SkyDemon Usage][usage_skydemon]

### Garmin Pilot

With `MSFS 2020 GPS Link` open and connected on your PC, open Garmin Pilot on your navigation device and press on `Settings` -> `GPS`.

Scroll to the bottom and you should see `SIMULATOR STATUS` showing an `amber`status light. Press on `Connect`.

![Garmin Pilot Usage][usage_garmin_pilot]

## Contributing

Contributions are welcome and encouraged! Please read [CONTRIBUTING.md][contributing_url].

[latest_release]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases/download/v0.3.0/msfs-2020-gps-link-v0.3.0.zip
[latest_release_badge]: https://img.shields.io/github/release/mihai-dinculescu/msfs-2020-gps-link.svg
[downloads_badge]: https://img.shields.io/github/downloads/mihai-dinculescu/msfs-2020-gps-link/total.svg
[ci_badge]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/actions
[sky_demon_url]: https://www.skydemon.aero
[garmin_pilot_url]: https://buy.garmin.com/en-US/US/p/115856
[releases_url]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases
[usage]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage.PNG
[usage_skydemon]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage-skydemon.PNG
[usage_garmin_pilot]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage-garmin-pilot.PNG
[contributing_url]: /CONTRIBUTING.md
