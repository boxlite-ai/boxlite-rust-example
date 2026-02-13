# BoxLite Rust Example

Minimal example of using [BoxLite](https://github.com/boxlite-ai/boxlite) from [crates.io](https://crates.io/crates/boxlite) to create a lightweight VM sandbox and run a command inside it.

## Prerequisites

- **Rust** 1.88+
- **Supported platforms**: macOS ARM64 (Apple Silicon), Linux x86_64, Linux ARM64

## Quick Start

```bash
cargo build
cargo run
```

The first build downloads prebuilt native runtime binaries (~50 MB) from GitHub Releases automatically. No additional toolchains (Go, LLVM, meson) are needed.

## What It Does

```rust
use boxlite::{BoxCommand, BoxOptions, BoxliteRuntime, RootfsSpec};

let runtime = BoxliteRuntime::default_runtime();
let options = BoxOptions {
    rootfs: RootfsSpec::Image("alpine:latest".into()),
    ..Default::default()
};

let litebox = runtime.create(options, None).await?;
let mut execution = litebox
    .exec(BoxCommand::new("echo").arg("Hello from BoxLite!"))
    .await?;
```

This creates a lightweight VM running Alpine Linux and executes `echo "Hello from BoxLite!"` inside it, streaming stdout back to the host.

## Project Structure

```
Cargo.toml  - boxlite + tokio + futures dependencies
build.rs    - Sets rpath so the binary can find native libraries at runtime
src/main.rs - Example code
```

## Learn More

- [BoxLite on crates.io](https://crates.io/crates/boxlite)
- [BoxLite on GitHub](https://github.com/boxlite-ai/boxlite)
- [API Documentation](https://docs.rs/boxlite)
