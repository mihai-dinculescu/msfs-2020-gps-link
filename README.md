# MSFS 2020 GPS Link

Transmit GPS data from Microsoft Flight Simulator 2020 to navigation apps.\
Tested with [SkyDemon](https://www.skydemon.aero) and [Garmin Pilot](https://buy.garmin.com/en-US/US/p/115856).

# Using

- download the latest version that's available on the [Releases](https://github.com/mihai-dinculescu/msfs-2020-gps-link/releases) page
- unzip & install
- with MSFS 2020 running, open `MSFS 2020 GPS Link` and hit "Connect"

# Contributing

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
