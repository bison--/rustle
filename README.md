# RUSTLE

A simple Wordl clone in Rust.

### RUN

```bash
cargo build --release
./target/release/tustle
```

### RUN DOCKER

```bash
docker run --rm -e USER=user -v ${PWD}:/usr/src/myapp -w /usr/src/myapp rust:1.59-alpine3.15 cargo build --release
./target/release/tustle
```
