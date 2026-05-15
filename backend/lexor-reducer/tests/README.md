The oracle tests are ignored by default. Run them with either of
```bash
cargo test -- --ignored
```
or
```bash
cargo nextest run --run-ignored only
```
at your convenience. Podman is required to run the tests.