# sample-static-rust-axum

**One repo: static frontend + Rust (Axum) API.**

## Build & run

**With [OctoPilot pipeline tools](https://github.com/octopilot/octopilot-pipeline-tools) (`op`)** — recommended for local/Mac:

```bash
docker run -d -p 5001:5000 --restart=unless-stopped --name registry registry:2  # once
op build-push
op run context list   # list runnable contexts
op run api            # run API (port 8081)
op run frontend       # run frontend (port 8080)
```

Ports and env are preconfigured in **.run.yaml**. **op run** is for local dev only (not production; use docker-compose, Kubernetes/kind, or your deploy path for that). Or with Skaffold: `skaffold build` then `op run api` / `op run frontend`.

Open http://localhost:8080. Set API URL to `http://localhost:8081`.

## Deploy

`skaffold build --file-output build.json` or `op push` (set `--default-repo` or use a `.registry` file).
