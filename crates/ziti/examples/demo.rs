//! Demonstrates the ziti crate API.
//!
//! Run with: cargo run -p ziti --example demo [identity.json]
//!
//! Without an identity file, it exercises error handling, socket creation,
//! and enum types. With an identity file, it loads the context and reports
//! the authentication state.

use ziti::{
    error_str, EnrollMode, LoadResult, SocketStatus, SocketType, ZitiError, ZitiLib, ZitiSocket,
};

fn main() {
    println!("=== OpenZiti Rust SDK Demo ===\n");

    // 1. Initialize the library
    let lib = ZitiLib::init();
    println!("[1] Library initialized");

    // 2. Demonstrate error types
    println!("\n[2] Error type system:");
    let errors = [
        ZitiError::AuthenticationFailed,
        ZitiError::Timeout,
        ZitiError::ServiceUnavailable,
        ZitiError::ConnClosed,
    ];
    for err in &errors {
        println!("    {:?} (code={}) => \"{}\"", err, err.code(), err);
    }
    println!(
        "    error_str(-14) => \"{}\"",
        error_str(-14)
    );

    // 3. Unknown error codes are handled gracefully
    let unknown = ZitiError::from(-999);
    println!("    Unknown(-999) => {:?}, display: \"{}\"", unknown, unknown);

    // 4. Socket creation
    println!("\n[3] Socket operations:");
    let sock = ZitiSocket::new(SocketType::Stream).expect("failed to create socket");
    println!(
        "    Created SOCK_STREAM socket (fd={})",
        sock.as_raw_fd()
    );
    println!("    Status: {:?}", sock.check_status());

    let sock2 = ZitiSocket::new(SocketType::Dgram).expect("failed to create socket");
    println!(
        "    Created SOCK_DGRAM socket (fd={})",
        sock2.as_raw_fd()
    );
    drop(sock);
    drop(sock2);
    println!("    Sockets closed (RAII drop)");

    // 5. Enum types
    println!("\n[4] Typed enums:");
    println!("    SocketType::Stream.to_raw() = {}", SocketType::Stream.to_raw());
    println!("    SocketType::Dgram.to_raw()  = {}", SocketType::Dgram.to_raw());
    println!("    SocketStatus::from(0) = {:?}", SocketStatus::from(0));
    println!("    SocketStatus::from(1) = {:?}", SocketStatus::from(1));
    println!("    SocketStatus::from(2) = {:?}", SocketStatus::from(2));
    println!("    EnrollMode::None  => {:?}", EnrollMode::None.to_raw());
    println!("    EnrollMode::Cert  => {:?}", EnrollMode::Cert.to_raw());
    println!("    EnrollMode::Token => {:?}", EnrollMode::Token.to_raw());

    // 6. Error handling for invalid identity
    println!("\n[5] Error handling (invalid identity):");
    match lib.load_context("this-file-does-not-exist.json") {
        Ok(_) => println!("    Unexpected success"),
        Err(e) => println!("    Expected error: {:?} => \"{}\"", e, e),
    }

    // 7. If an identity file was provided, try to load it
    if let Some(identity_path) = std::env::args().nth(1) {
        println!("\n[6] Loading identity: {}", identity_path);
        match lib.load_context(&identity_path) {
            Ok(result) => {
                match &result {
                    LoadResult::Ready(ctx) => {
                        println!("    Identity loaded and ready (handle={})", ctx.raw_handle());
                    }
                    LoadResult::ExternalLoginRequired(ctx) => {
                        println!(
                            "    Identity loaded, external login required (handle={})",
                            ctx.raw_handle()
                        );
                        let signers = ctx.get_ext_signers();
                        if !signers.is_empty() {
                            println!("    Available signers: {:?}", signers);
                        }
                    }
                    LoadResult::PartiallyAuthenticated(ctx) => {
                        println!(
                            "    Identity loaded, TOTP required (handle={})",
                            ctx.raw_handle()
                        );
                    }
                    LoadResult::MfaNotEnrolled(ctx) => {
                        println!(
                            "    Identity loaded, MFA enrollment needed (handle={})",
                            ctx.raw_handle()
                        );
                    }
                }

                // Try creating a socket and connecting
                let ctx = result.into_context();
                let sock = ZitiSocket::new(SocketType::Stream).unwrap();
                println!("    Socket created (fd={})", sock.as_raw_fd());
                println!("    Socket status: {:?}", sock.check_status());

                // Try connecting to a service (will fail if service doesn't exist)
                match sock.connect(&ctx, "demo-service", None) {
                    Ok(()) => println!("    Connected to 'demo-service'!"),
                    Err(e) => println!("    Connect to 'demo-service': {:?}", e),
                }
            }
            Err(e) => {
                println!("    Failed to load: {:?} => \"{}\"", e, e);
            }
        }
    } else {
        println!("\n[6] Skipped identity loading (pass a .json file as argument)");
    }

    println!("\n=== Demo complete ===");
}
