# BoxLite Rust Example

Minimal example of using [BoxLite](https://github.com/boxlite-ai/boxlite) from [crates.io](https://crates.io/crates/boxlite) to create a lightweight VM sandbox and run a command inside it.

## Prerequisites

- **Rust** 1.88+
- **Supported platforms**: macOS ARM64 (Apple Silicon), Linux x86_64, Linux ARM64

## Quick Start

**1. Add dependencies**

```bash
cargo add boxlite tokio --features tokio/rt,tokio/rt-multi-thread,tokio/macros
cargo add futures
```

**2. Create `build.rs`**

BoxLite bundles native libraries (libkrun, libgvproxy). Your binary needs an rpath so the OS can find them at runtime:

```rust
fn main() {
    let runtime_dir = std::env::var("DEP_BOXLITE_RUNTIME_DIR").unwrap();
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{runtime_dir}");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{runtime_dir}");
}
```

**3. Build and run**

```bash
cargo build
cargo run
```

The first build downloads prebuilt runtime binaries (~50 MB) from GitHub Releases automatically. No additional toolchains (Go, LLVM, meson) are needed.

## Example Code

```rust
use boxlite::{BoxCommand, BoxOptions, BoxliteRuntime, RootfsSpec};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = BoxliteRuntime::default_runtime();
    let options = BoxOptions {
        rootfs: RootfsSpec::Image("alpine:latest".into()),
        ..Default::default()
    };

    let litebox = runtime.create(options, None).await?;
    let mut execution = litebox
        .exec(BoxCommand::new("echo").arg("Hello from BoxLite!"))
        .await?;

    let mut stdout = execution.stdout().unwrap();
    while let Some(line) = stdout.next().await {
        println!("{}", line);
    }

    Ok(())
}
```

## Distribution

To ship your product, bundle the runtime files next to your binary:

```bash
# Build release
cargo build --release

# Copy runtime files next to your binary
cp target/release/build/boxlite-*/out/runtime/* dist/
cp target/release/my-app dist/
```

Your `dist/` folder should look like:

```
dist/
  my-app
  libkrun.*.dylib      # (or .so on Linux)
  libkrunfw.*.dylib
  libgvproxy.dylib
  boxlite-shim
  boxlite-guest
  mke2fs
```

BoxLite automatically discovers runtime files next to the binary. You can also set the `BOXLITE_RUNTIME_DIR` environment variable to point to a custom location.

## Learn More

- [BoxLite on crates.io](https://crates.io/crates/boxlite)
- [BoxLite on GitHub](https://github.com/boxlite-ai/boxlite)
- [API Documentation](https://docs.rs/boxlite)
