//! Core type aliases matching the OpenZiti C SDK.

use std::os::raw::c_int;

/// Opaque handle to a loaded Ziti identity context.
pub type ziti_handle_t = u32;

/// Socket file descriptor returned by `Ziti_socket` / `Ziti_accept`.
pub type ziti_socket_t = c_int;

/// Invalid handle sentinel - equivalent to `(ziti_handle_t)-1` in C.
pub const ZITI_INVALID_HANDLE: ziti_handle_t = u32::MAX;
