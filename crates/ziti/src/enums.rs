//! Rust-idiomatic enums wrapping C SDK enum types.

/// Socket type for `Ziti_socket`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Dgram,
}

impl SocketType {
    pub fn to_raw(self) -> i32 {
        match self {
            Self::Stream => libc::SOCK_STREAM,
            Self::Dgram => libc::SOCK_DGRAM,
        }
    }
}

/// Status returned by `Ziti_check_socket`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketStatus {
    /// Not a Ziti socket.
    NotZiti,
    /// Connected Ziti socket.
    Connected,
    /// Ziti server (listening) socket.
    Server,
}

impl From<i32> for SocketStatus {
    fn from(val: i32) -> Self {
        match val {
            1 => Self::Connected,
            2 => Self::Server,
            _ => Self::NotZiti,
        }
    }
}

/// Enrollment mode for `Ziti_enroll_controller`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrollMode {
    /// No enrollment, identity pre-created by admin.
    None,
    /// Generate CSR, exchange for client certificate.
    Cert,
    /// Auto-create identity, authenticate via external JWT.
    Token,
}

impl EnrollMode {
    pub fn to_raw(self) -> ziti_sys::ziti_enroll_mode {
        match self {
            Self::None => ziti_sys::ziti_enroll_mode::ziti_enroll_none,
            Self::Cert => ziti_sys::ziti_enroll_mode::ziti_enroll_cert,
            Self::Token => ziti_sys::ziti_enroll_mode::ziti_enroll_token,
        }
    }
}

impl From<ziti_sys::ziti_enroll_mode> for EnrollMode {
    fn from(raw: ziti_sys::ziti_enroll_mode) -> Self {
        match raw {
            ziti_sys::ziti_enroll_mode::ziti_enroll_none => Self::None,
            ziti_sys::ziti_enroll_mode::ziti_enroll_cert => Self::Cert,
            ziti_sys::ziti_enroll_mode::ziti_enroll_token => Self::Token,
        }
    }
}

/// Cryptographic method for end-to-end encryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoMethod {
    Invalid,
    None,
    Libsodium,
    AesGcm,
}

impl From<ziti_sys::ziti_crypto_method> for CryptoMethod {
    fn from(raw: ziti_sys::ziti_crypto_method) -> Self {
        match raw {
            ziti_sys::ziti_crypto_method::ziti_crypto_invalid => Self::Invalid,
            ziti_sys::ziti_crypto_method::ziti_crypto_none => Self::None,
            ziti_sys::ziti_crypto_method::ziti_crypto_libsodium => Self::Libsodium,
            ziti_sys::ziti_crypto_method::ziti_crypto_aes_gcm => Self::AesGcm,
        }
    }
}

/// Metric rate type for transfer rate calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateType {
    Ewma1m,
    Ewma5m,
    Ewma15m,
    Mma1m,
    Cma1m,
    Ewma5s,
    Instant,
}

impl From<ziti_sys::rate_type> for RateType {
    fn from(raw: ziti_sys::rate_type) -> Self {
        match raw {
            ziti_sys::rate_type::EWMA_1m => Self::Ewma1m,
            ziti_sys::rate_type::EWMA_5m => Self::Ewma5m,
            ziti_sys::rate_type::EWMA_15m => Self::Ewma15m,
            ziti_sys::rate_type::MMA_1m => Self::Mma1m,
            ziti_sys::rate_type::CMA_1m => Self::Cma1m,
            ziti_sys::rate_type::EWMA_5s => Self::Ewma5s,
            ziti_sys::rate_type::INSTANT => Self::Instant,
        }
    }
}
