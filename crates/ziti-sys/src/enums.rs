//! C enum representations from `enums.h`.

/// Metric rate type for transfer rate calculations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rate_type {
    EWMA_1m = 0,
    EWMA_5m = 1,
    EWMA_15m = 2,
    MMA_1m = 3,
    CMA_1m = 4,
    EWMA_5s = 5,
    INSTANT = 6,
}

/// Enrollment mode for identity creation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ziti_enroll_mode {
    ziti_enroll_none = 0,
    ziti_enroll_cert = 1,
    ziti_enroll_token = 2,
}

/// Cryptographic method for end-to-end encryption.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ziti_crypto_method {
    ziti_crypto_invalid = -1,
    ziti_crypto_none = 0,
    ziti_crypto_libsodium = 1,
    ziti_crypto_aes_gcm = 2,
}
