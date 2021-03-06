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

## Releases

- Update the version in `msfs-2020-gps-link\src-tauri\Cargo.toml`
- Update the version in `msfs-2020-gps-link\package.json`
- Update CHANGELOG.md

- Commit
- Add tag

```bash
git tag -a vX.X.X
```

- Push

```bash
git push --follow-tags
```

- Build a new installer

```bash
yarn tauri build
```

The new installer can be found in `msfs-2020-gps-link\msfs-2020-gps-link\src-tauri\target\release\bundle\msi`.

Zip it up and rename the archive to `msfs-2020-gps-link-vX.X.X.zip`.

- Create the GitHub release, add the change log, and attach the archive
- Update the `latest_release` URL in README.md
- Update `version.txt`
- Commit & push

## Troubleshooting

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
