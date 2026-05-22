# ziti-sdk-rust

Unofficial Rust bindings for the [OpenZiti](https://openziti.io) C SDK.

## Why this exists

OpenZiti has official SDKs for C, Go, Swift, Python, .NET, and others - but no official Rust bindings. This crate fills that gap by wrapping the C SDK's socket API (`zitilib.h`) with safe, typed Rust.

## What is OpenZiti?

[OpenZiti](https://github.com/openziti) is an open-source zero-trust networking platform. It provides secure, identity-based connectivity between applications without exposing ports or using traditional VPNs. Applications embed the SDK directly and communicate over an encrypted overlay network.

## Crate Structure

| Crate | Description |
|-------|-------------|
| **`ziti-sys`** | Raw `extern "C"` FFI bindings - types, constants, enums, and all 18 functions from `zitilib.h` |
| **`ziti`** | Safe typed wrapper - `Result<T, ZitiError>`, RAII resource management, idiomatic Rust API |

## Prerequisites

- **Rust** toolchain (edition 2021)
- **CMake** (3.x)
- **vcpkg** with the following packages installed:
  - `libuv`, `libsodium`, `json-c`, `protobuf-c`, `llhttp`, `openssl`, `zlib`
- **Git** (for the C SDK submodule)
- **macOS** or **Linux** (no Windows support currently)

## Getting Started

### 1. Clone with submodules

```sh
git clone --recurse-submodules https://github.com/dmuensterer/ziti-sdk-rust.git
cd ziti-sdk-rust
```

If you already cloned without `--recurse-submodules`:

```sh
git submodule update --init
```

### 2. Install vcpkg dependencies

```sh
# If you don't have vcpkg yet:
git clone https://github.com/microsoft/vcpkg.git ~/vcpkg
~/vcpkg/bootstrap-vcpkg.sh

# The C SDK's CMake build handles vcpkg integration automatically.
# Just make sure VCPKG_ROOT is set or vcpkg lives at ~/vcpkg.
export VCPKG_ROOT=~/vcpkg
```

### 3. Build and test

```sh
cargo build
cargo test -- --test-threads=1
```

### Skip C recompilation

The C SDK is compiled via CMake on every `cargo build`. To build it once and reuse:

```sh
cd vendor/ziti-sdk-c
cmake -B build -DBUILD_SHARED_LIBS=OFF \
  -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --target ziti -j$(nproc)
```

Then set `ZITI_BUILD_DIR` to skip CMake on subsequent builds:

```sh
export ZITI_BUILD_DIR=$(pwd)/build
cargo build   # links pre-built libs, no cmake
```

Add the export to your shell profile to make it permanent.

## Usage

### Initialize the library

```rust
use ziti::ZitiLib;

let lib = ZitiLib::init();
```

The C SDK starts a background thread on init. Call `lib.shutdown()` explicitly if you need to tear it down before process exit.

### Load an identity

```rust
use ziti::{ZitiLib, LoadResult};

let lib = ZitiLib::init();

match lib.load_context("/path/to/identity.json") {
    Ok(LoadResult::Ready(ctx)) => {
        // Fully authenticated, ready to use
    }
    Ok(LoadResult::ExternalLoginRequired(ctx)) => {
        // OIDC login needed - see Authentication section
    }
    Ok(LoadResult::PartiallyAuthenticated(ctx)) => {
        // TOTP code required
    }
    Ok(LoadResult::MfaNotEnrolled(ctx)) => {
        // MFA enrollment needed
    }
    Err(e) => eprintln!("Failed: {e}"),
}
```

### Client: connect to a service

```rust
use ziti::{ZitiLib, ZitiSocket, SocketType};

let lib = ZitiLib::init();
let ctx = lib.load_context("identity.json")?.into_context();

let sock = ZitiSocket::new(SocketType::Stream)?;
sock.connect(&ctx, "my-service", None)?;

// Use sock.as_raw_fd() for read/write via standard I/O,
// or wrap with tokio::io::unix::AsyncFd for async.
```

### Server: bind, listen, accept

```rust
use ziti::{ZitiLib, ZitiSocket, SocketType};

let lib = ZitiLib::init();
let ctx = lib.load_context("identity.json")?.into_context();

let sock = ZitiSocket::new(SocketType::Stream)?;
sock.bind(&ctx, "my-service", None)?;
sock.listen(10)?;

let (client_sock, caller_identity) = sock.accept()?;
println!("Connection from: {caller_identity}");
```

### Enrollment

```rust
use ziti::{enroll_identity, enroll_controller, EnrollMode, ZitiLib};

let _lib = ZitiLib::init();

// Enroll from a JWT token
let identity_json = enroll_identity("eyJ0eX...", None, None)?;

// Or enroll via controller URL
let identity_json = enroll_controller(
    "https://ctrl.example.com:1280",
    None,                // fetch JWT from controller
    EnrollMode::Cert,    // generate CSR
    None,                // auto-select signer
)?;
```

### Authentication (OIDC / TOTP)

```rust
use ziti::{ZitiLib, LoadResult};

let lib = ZitiLib::init();

match lib.load_context("identity.json")? {
    LoadResult::ExternalLoginRequired(ctx) => {
        let signers = ctx.get_ext_signers();
        let url = ctx.login_external(&signers[0])?;
        println!("Open in browser: {url}");
        ctx.wait_for_auth(Some(60_000))?; // wait up to 60s
    }
    LoadResult::PartiallyAuthenticated(ctx) => {
        ctx.login_totp("123456")?;
    }
    _ => {}
}
```

### DNS resolution

```rust
use ziti::resolve;

let addrs = resolve("my-service.ziti", "80")?;
for addr in addrs.addrs() {
    println!("{addr}");
}
```

## API Overview

| Module | Key Types | Description |
|--------|-----------|-------------|
| `context` | `ZitiLib`, `ZitiContext`, `LoadResult` | Library lifecycle and identity management |
| `socket` | `ZitiSocket` | RAII socket wrapper with connect, bind, listen, accept |
| `error` | `ZitiError`, `check()`, `error_str()` | 43 typed error variants mapped from C SDK |
| `enums` | `SocketType`, `SocketStatus`, `EnrollMode`, `CryptoMethod`, `RateType` | Type-safe enum wrappers |
| `enrollment` | `enroll_identity()`, `enroll_controller()` | Identity enrollment from JWT or controller URL |
| `resolve` | `resolve()`, `ResolvedAddrs` | Ziti DNS resolution |

## Error Handling

All C SDK error codes are mapped to a `ZitiError` enum with 43 named variants plus `Unknown(i32)` for forward compatibility. The `Display` implementation delegates to the C SDK's `ziti_errorstr()` for human-readable messages:

```rust
use ziti::ZitiError;

let err = ZitiError::from(-14);
assert_eq!(err, ZitiError::AuthenticationFailed);
println!("{err}"); // "failed to authenticate"
```

## Limitations

- **Socket API only** - wraps `zitilib.h` (blocking, BSD-socket style). The advanced callback-based API (`ziti.h`) with libuv integration is not exposed.
- **No async integration** - all C SDK calls are blocking. For async usage, wrap calls with `tokio::task::spawn_blocking` or use `tokio::io::unix::AsyncFd` on socket fds.
- **No Windows support** - macOS and Linux only.
- **Process-global init** - `ZitiLib::init()` uses `std::sync::Once`. The C SDK's background thread runs for the lifetime of the process.
- **Enrollment may block** - `enroll_identity()` and `enroll_controller()` perform network I/O and may block for extended periods.
- **Hand-written bindings** - not auto-generated with bindgen. The API surface is small (18 functions) and stable.

## Running Tests

Tests must run single-threaded because the C SDK uses global state:

```sh
cargo test -- --test-threads=1
```

## License

MIT
