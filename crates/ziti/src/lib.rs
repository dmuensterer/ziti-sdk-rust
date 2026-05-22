//! Safe, typed Rust wrapper for the OpenZiti C SDK.
//!
//! This crate provides idiomatic Rust access to the OpenZiti socket API
//! (`zitilib.h`). All unsafe C calls are wrapped with proper error handling,
//! RAII resource management, and typed enums.
//!
//! # Quick Start
//!
//! ```no_run
//! use ziti::{ZitiLib, SocketType, ZitiSocket};
//!
//! let lib = ZitiLib::init();
//! let result = lib.load_context("/path/to/identity.json").unwrap();
//! let ctx = result.into_context();
//!
//! let sock = ZitiSocket::new(SocketType::Stream).unwrap();
//! sock.connect(&ctx, "my-service", None).unwrap();
//! ```

pub mod context;
pub mod enums;
pub mod enrollment;
pub mod error;
pub mod resolve;
pub mod socket;

pub use context::{LoadResult, ZitiContext, ZitiLib};
pub use enums::{CryptoMethod, EnrollMode, RateType, SocketStatus, SocketType};
pub use enrollment::{enroll_controller, enroll_identity};
pub use error::{check, error_str, ZitiError};
pub use resolve::{resolve, ResolvedAddrs};
pub use socket::ZitiSocket;
