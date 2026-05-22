//! Raw FFI bindings to the OpenZiti C SDK socket API (`zitilib.h`).
//!
//! This crate provides direct, unsafe bindings to all functions, constants,
//! and enums from the OpenZiti C SDK's socket-based API. For a safe, typed
//! wrapper, use the `ziti` crate instead.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod enums;
pub mod errors;
pub mod functions;
pub mod types;

pub use enums::*;
pub use errors::*;
pub use functions::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // The C SDK is not safe to init/shutdown from multiple threads concurrently.
    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn can_init_and_shutdown() {
        let _guard = LOCK.lock().unwrap();
        unsafe {
            Ziti_lib_init();
            Ziti_lib_shutdown();
        }
    }

    #[test]
    fn error_constants_match_expected_values() {
        assert_eq!(ZITI_OK, 0);
        assert_eq!(ZITI_CONFIG_NOT_FOUND, -1);
        assert_eq!(ZITI_AUTHENTICATION_FAILED, -14);
        assert_eq!(ZITI_TIMEOUT, -20);
        assert_eq!(ZITI_CONN_CLOSED, -24);
        assert_eq!(ZITI_API_SESSION_INVALID, -42);
        assert_eq!(ZITI_WTF, -111);
    }

    #[test]
    fn invalid_handle_is_max() {
        assert_eq!(ZITI_INVALID_HANDLE, u32::MAX);
    }

    #[test]
    fn errorstr_returns_non_null_for_known_codes() {
        let _guard = LOCK.lock().unwrap();
        unsafe {
            Ziti_lib_init();
            for code in -42..=0 {
                let ptr = ziti_errorstr(code);
                assert!(!ptr.is_null(), "ziti_errorstr({code}) returned null");
            }
            let ptr = ziti_errorstr(ZITI_WTF);
            assert!(!ptr.is_null());
            Ziti_lib_shutdown();
        }
    }
}
