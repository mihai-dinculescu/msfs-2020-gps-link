# Contributing

Contributions are welcome and encouraged!

## Development

Clone the repo

```bash
git clone https://github.com/mihai-dinculescu/msfs-2020-gps-link.git
```

Install `tauri-cli` globally

```bash
cargo install tauri-cli
```

Run the app

```bash
cargo tauri dev
```

## Releases

- Update the version in `Cargo.toml`
- Update the version in `www/package.json`
- Update CHANGELOG.md

- Commit
- Add tag

```bash
git tag -a vX.X.X -m "vX.X.X"
```

- Push

```bash
git push --follow-tags
```

- Build a new installer

```bash
cargo tauri build
```

The new installer can be found in `target/release/bundle/msi`.

- Zip the msi installer and name the archive in the `msfs-2020-gps-link-vX.X.X.zip` format.
- Create the GitHub release, add the change log, and attach the MSI
- Update the `latest_release` URL in README.md
- Update `version.txt`
- Commit & push

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

Browse the Jaeger traces at <http://localhost:16686>.
