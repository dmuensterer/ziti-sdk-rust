//! All `extern "C"` function declarations from `zitilib.h`.

use std::os::raw::{c_char, c_int, c_uint, c_ulong};

use crate::enums::ziti_enroll_mode;
use crate::types::{ziti_handle_t, ziti_socket_t};

extern "C" {
    // ── Library lifecycle ───────────────────────────────────────────────

    /// Initialize the Ziti library (starts background thread).
    pub fn Ziti_lib_init();

    /// Shut down the Ziti library and terminate background thread.
    pub fn Ziti_lib_shutdown();

    /// Return error code for the last failed operation.
    pub fn Ziti_last_error() -> c_int;

    /// Return a human-readable error description for a Ziti error code.
    pub fn ziti_errorstr(err: c_int) -> *const c_char;

    // ── Enrollment ─────────────────────────────────────────────────────

    /// Enroll a new Ziti identity from a JWT token.
    ///
    /// * `jwt` - enrollment token string
    /// * `key` - private key (required for 3rd-party CA enrollment, otherwise NULL)
    /// * `cert` - x.509 certificate (required for 3rd-party CA, otherwise NULL)
    /// * `id_json` - (output) identity JSON, caller must free with `libc::free`
    /// * `id_json_len` - (output) length of `id_json`
    pub fn Ziti_enroll_identity(
        jwt: *const c_char,
        key: *const c_char,
        cert: *const c_char,
        id_json: *mut *mut c_char,
        id_json_len: *mut c_ulong,
    ) -> c_int;

    /// Enroll or authenticate a Ziti identity via a controller URL.
    ///
    /// * `url` - controller URL (e.g. `"https://ctrl.example.com:1280"`)
    /// * `jwt` - network JWT string, or NULL to fetch from controller
    /// * `mode` - enrollment mode
    /// * `signer_name` - ext-jwt-signer name, or NULL to auto-select
    /// * `id_json` - (output) identity JSON, caller must free with `libc::free`
    /// * `id_json_len` - (output) length of `id_json`
    pub fn Ziti_enroll_controller(
        url: *const c_char,
        jwt: *const c_char,
        mode: ziti_enroll_mode,
        signer_name: *const c_char,
        id_json: *mut *mut c_char,
        id_json_len: *mut c_ulong,
    ) -> c_int;

    // ── Identity / context loading ─────────────────────────────────────

    /// Load a Ziti identity from a JSON string or file path.
    ///
    /// Returns `ZITI_OK` on success, or informational codes:
    /// `ZITI_EXTERNAL_LOGIN_REQUIRED`, `ZITI_PARTIALLY_AUTHENTICATED`,
    /// `ZITI_MFA_NOT_ENROLLED`.
    pub fn Ziti_load_context(h: *mut ziti_handle_t, identity: *const c_char) -> c_int;

    /// Load a Ziti identity with timeout (milliseconds). 0 = no timeout.
    pub fn Ziti_load_context_with_timeout(
        h: *mut ziti_handle_t,
        identity: *const c_char,
        timeout_ms: c_int,
    ) -> c_int;

    // ── Authentication ─────────────────────────────────────────────────

    /// Get names of external JWT signers available for authentication.
    ///
    /// Returns a NULL-terminated array of string pointers.
    pub fn Ziti_get_ext_signers(ztx: ziti_handle_t) -> *const *const c_char;

    /// Start external login. Returns a URL the user should open in a browser.
    ///
    /// The returned string must be freed with `libc::free`.
    pub fn Ziti_login_external(ztx: ziti_handle_t, signer_name: *const c_char) -> *mut c_char;

    /// Login with a TOTP code.
    pub fn Ziti_login_totp(ztx: ziti_handle_t, code: *const c_char) -> c_int;

    /// Block until authentication completes or timeout. 0 = no timeout.
    pub fn Ziti_wait_for_auth(ztx: ziti_handle_t, timeout_ms: c_int) -> c_int;

    // ── Socket operations ──────────────────────────────────────────────

    /// Create a Ziti socket.
    ///
    /// `sock_type`: `libc::SOCK_STREAM` or `libc::SOCK_DGRAM`.
    pub fn Ziti_socket(sock_type: c_int) -> ziti_socket_t;

    /// Close a Ziti socket.
    pub fn Ziti_close(socket: ziti_socket_t) -> c_int;

    /// Check if a socket is a Ziti socket.
    ///
    /// Returns: 0 = not ziti, 1 = connected, 2 = server socket.
    pub fn Ziti_check_socket(socket: ziti_socket_t) -> c_int;

    // ── Client connect ─────────────────────────────────────────────────

    /// Connect a socket to a Ziti service by name.
    pub fn Ziti_connect(
        socket: ziti_socket_t,
        ztx: ziti_handle_t,
        service: *const c_char,
        terminator: *const c_char,
    ) -> c_int;

    /// Connect a socket to a Ziti service by hostname and port.
    pub fn Ziti_connect_addr(
        socket: ziti_socket_t,
        host: *const c_char,
        port: c_uint,
    ) -> c_int;

    /// Connect a socket using a `sockaddr`. Falls back to standard connect
    /// if the address does not resolve to a Ziti service.
    pub fn Ziti_connect_sockaddr(
        socket: ziti_socket_t,
        addr: *const libc::sockaddr,
        addrlen: c_int,
    ) -> c_int;

    // ── Server bind/listen/accept ──────────────────────────────────────

    /// Bind a socket to a Ziti service (server-side).
    pub fn Ziti_bind(
        socket: ziti_socket_t,
        ztx: ziti_handle_t,
        service: *const c_char,
        terminator: *const c_char,
    ) -> c_int;

    /// Mark a bound socket as listening for incoming connections.
    pub fn Ziti_listen(socket: ziti_socket_t, backlog: c_int) -> c_int;

    /// Accept an incoming Ziti connection.
    ///
    /// Writes the caller's identity name into `caller` (up to `caller_len` bytes).
    /// Returns a new socket fd, or -1 on error.
    pub fn Ziti_accept(
        socket: ziti_socket_t,
        caller: *mut c_char,
        caller_len: c_int,
    ) -> ziti_socket_t;

    // ── DNS resolution ─────────────────────────────────────────────────

    /// Resolve a hostname via Ziti DNS.
    pub fn Ziti_resolve(
        host: *const c_char,
        port: *const c_char,
        hints: *const libc::addrinfo,
        addrlist: *mut *mut libc::addrinfo,
    ) -> c_int;
}
