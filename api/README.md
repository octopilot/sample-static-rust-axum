# Rust (Axum) Hello — OctoPilot sample

Minimal Axum web app. Same API as other samples. Built with **Docker** (Rust is not in the Paketo jammy builder).

## API

- **GET** `/greet?name=<name>&birth_year=<year>`
- Returns JSON: `{ "greeting": "Hello, <name>!", "age_confirmation": "You are <age> years old." }`

## Build locally

```bash
cargo build --release
./target/release/rust-axum-hello
```

## Build with Docker (from repo root)

```bash
docker build -t octopilot-samples/rust-axum-hello -f rust-axum/Dockerfile rust-axum
docker run -p 8080:8080 octopilot-samples/rust-axum-hello
```

Skaffold will build this artifact using the Dockerfile when you run `skaffold build`.
