# MSFS 2020 GPS Link

[![GitHub release][latest_release_badge]][releases_url]
[![Github all releases][downloads_badge]][releases_url]

Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.\
Tested with [SkyDemon][sky_demon_url] and [Garmin Pilot][garmin_pilot_url].

### [Quick Download][latest_release]

# Usage

- download the latest version from the [Releases][releases_url] page or by using the link above
- unzip & install
- with MSFS 2020 running, open `MSFS 2020 GPS Link` and press "Connect"

![MSFS 2020 GPS Link Usage][usage]

## SkyDemon

With `MSFS 2020 GPS Link` open and connected on your PC, open SkyDemon on your navigation device and press on `Fly` -> `Use X-Plane`.

![SkyDemon Usage][usage_skydemon]

## Garmin Pilot

With `MSFS 2020 GPS Link` open and connected on your PC, open Garmin Pilot on your navigation device and press on `Settings` -> `GPS`.

Scroll to the bottom and you should see `SIMULATOR STATUS` showing an `amber`status light. Press on `Connect`.

![Garmin Pilot Usage][usage_garmin_pilot]

# Contributing

Contributions are welcome and encouraged!

## Development

Clone the repo

```bash
git clone https://github.com/mihai-dinculescu/msfs-2020-gps-link.git
```

Start msfs-2020-gps-link in dev mode

```bash
cd msfs-2020-gps-link
yarn tauri dev
```

### Troubleshooting

### 1. Tauri `target` path

Tauri doesn't currently handle Rust workspaces very well. The following workaround is needed in order to make it successfully build the release bundle

msfs-2020-gps-link/.cargo/config

```toml
[build]
target-dir = "<full-path-to-project>/msfs-2020-gps-link/src-tauri/target"
```

## Tracing

Spin up Jaeger

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```

Start msfs-2020-gps-link in dev mode

```bash
cd msfs-2020-gps-link
RUST_LOG=info yarn tauri dev
```

Browse the Jaeger traces at http://localhost:16686.

[latest_release_badge]: https://img.shields.io/github/release/mihai-dinculescu/msfs-2020-gps-link.svg
[downloads_badge]: https://img.shields.io/github/downloads/mihai-dinculescu/msfs-2020-gps-link/total.svg
[sky_demon_url]: https://www.skydemon.aero
[garmin_pilot_url]: https://buy.garmin.com/en-US/US/p/115856
[latest_release]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases/download/v0.1.0/msfs-2020-gps-link-v0.1.0.zip
[releases_url]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases
[usage]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage.PNG
[usage_skydemon]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage-skydemon.PNG
[usage_garmin_pilot]: https://github.com/mihai-dinculescu/msfs-2020-gps-link/blob/main/assets/usage-garmin-pilot.PNG
