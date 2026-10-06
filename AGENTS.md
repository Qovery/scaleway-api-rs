# AGENTS.md

## Releasing

Every push to `main` runs `.github/workflows/publish-crate.yml`. That workflow publishes the
crate to crates.io under the version in `Cargo.toml`. It skips the upload if that version is
already on crates.io.

- Bump `version` in `Cargo.toml` (and refresh `Cargo.lock`) in any PR that should ship.
- The new version must be higher than the latest one on crates.io, not just the one in git:
  `curl -s https://crates.io/api/v1/crates/scaleway_api_rs | jq -r .crate.max_version`
- Publish only through CI. Versions 0.1.4 to 0.1.8 were published by hand and never committed.
  The repo stayed at 0.1.2, so a later bump published 0.1.3 after 0.1.8.

## Code generation

`src/` and `docs/` are generated. See `UPDATE.md` to regenerate them from the Scaleway OpenAPI specs.
