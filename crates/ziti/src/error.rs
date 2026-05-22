//! Typed error handling for the OpenZiti C SDK.

use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_int;

/// All error codes from the OpenZiti C SDK (`errors.h`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ZitiError {
    ConfigNotFound,
    JwtNotFound,
    JwtInvalid,
    JwtInvalidFormat,
    Pkcs7Asn1ParsingFailed,
    JwtSigningAlgUnsupported,
    JwtVerificationFailed,
    EnrollmentMethodUnsupported,
    EnrollmentCertificateRequired,
    KeyGenerationFailed,
    KeyLoadFailed,
    CsrGenerationFailed,
    InvalidConfig,
    AuthenticationFailed,
    NotAuthorized,
    ControllerUnavailable,
    GatewayUnavailable,
    ServiceUnavailable,
    Eof,
    Timeout,
    ConnAbort,
    InvalidState,
    CryptoFail,
    ConnClosed,
    InvalidPosture,
    MfaExists,
    MfaInvalidToken,
    MfaNotEnrolled,
    NotFound,
    Disabled,
    PartiallyAuthenticated,
    InvalidAuthenticatorType,
    InvalidAuthenticatorCert,
    InvalidCertKeyPair,
    CertInUse,
    CertFailedValidation,
    MissingCertClaim,
    AllocFailed,
    ExternalLoginRequired,
    AlreadyEnrolled,
    EnrollmentNotAllowed,
    ApiSessionInvalid,
    Wtf,
    Unknown(i32),
}

impl ZitiError {
    /// Get the raw C error code for this error.
    pub fn code(self) -> i32 {
        match self {
            Self::ConfigNotFound => -1,
            Self::JwtNotFound => -2,
            Self::JwtInvalid => -3,
            Self::JwtInvalidFormat => -4,
            Self::Pkcs7Asn1ParsingFailed => -5,
            Self::JwtSigningAlgUnsupported => -6,
            Self::JwtVerificationFailed => -7,
            Self::EnrollmentMethodUnsupported => -8,
            Self::EnrollmentCertificateRequired => -9,
            Self::KeyGenerationFailed => -10,
            Self::KeyLoadFailed => -11,
            Self::CsrGenerationFailed => -12,
            Self::InvalidConfig => -13,
            Self::AuthenticationFailed => -14,
            Self::NotAuthorized => -15,
            Self::ControllerUnavailable => -16,
            Self::GatewayUnavailable => -17,
            Self::ServiceUnavailable => -18,
            Self::Eof => -19,
            Self::Timeout => -20,
            Self::ConnAbort => -21,
            Self::InvalidState => -22,
            Self::CryptoFail => -23,
            Self::ConnClosed => -24,
            Self::InvalidPosture => -25,
            Self::MfaExists => -26,
            Self::MfaInvalidToken => -27,
            Self::MfaNotEnrolled => -28,
            Self::NotFound => -29,
            Self::Disabled => -30,
            Self::PartiallyAuthenticated => -31,
            Self::InvalidAuthenticatorType => -32,
            Self::InvalidAuthenticatorCert => -33,
            Self::InvalidCertKeyPair => -34,
            Self::CertInUse => -35,
            Self::CertFailedValidation => -36,
            Self::MissingCertClaim => -37,
            Self::AllocFailed => -38,
            Self::ExternalLoginRequired => -39,
            Self::AlreadyEnrolled => -40,
            Self::EnrollmentNotAllowed => -41,
            Self::ApiSessionInvalid => -42,
            Self::Wtf => -111,
            Self::Unknown(code) => code,
        }
    }
}

impl From<i32> for ZitiError {
    fn from(code: i32) -> Self {
        match code {
            -1 => Self::ConfigNotFound,
            -2 => Self::JwtNotFound,
            -3 => Self::JwtInvalid,
            -4 => Self::JwtInvalidFormat,
            -5 => Self::Pkcs7Asn1ParsingFailed,
            -6 => Self::JwtSigningAlgUnsupported,
            -7 => Self::JwtVerificationFailed,
            -8 => Self::EnrollmentMethodUnsupported,
            -9 => Self::EnrollmentCertificateRequired,
            -10 => Self::KeyGenerationFailed,
            -11 => Self::KeyLoadFailed,
            -12 => Self::CsrGenerationFailed,
            -13 => Self::InvalidConfig,
            -14 => Self::AuthenticationFailed,
            -15 => Self::NotAuthorized,
            -16 => Self::ControllerUnavailable,
            -17 => Self::GatewayUnavailable,
            -18 => Self::ServiceUnavailable,
            -19 => Self::Eof,
            -20 => Self::Timeout,
            -21 => Self::ConnAbort,
            -22 => Self::InvalidState,
            -23 => Self::CryptoFail,
            -24 => Self::ConnClosed,
            -25 => Self::InvalidPosture,
            -26 => Self::MfaExists,
            -27 => Self::MfaInvalidToken,
            -28 => Self::MfaNotEnrolled,
            -29 => Self::NotFound,
            -30 => Self::Disabled,
            -31 => Self::PartiallyAuthenticated,
            -32 => Self::InvalidAuthenticatorType,
            -33 => Self::InvalidAuthenticatorCert,
            -34 => Self::InvalidCertKeyPair,
            -35 => Self::CertInUse,
            -36 => Self::CertFailedValidation,
            -37 => Self::MissingCertClaim,
            -38 => Self::AllocFailed,
            -39 => Self::ExternalLoginRequired,
            -40 => Self::AlreadyEnrolled,
            -41 => Self::EnrollmentNotAllowed,
            -42 => Self::ApiSessionInvalid,
            -111 => Self::Wtf,
            other => Self::Unknown(other),
        }
    }
}

impl fmt::Display for ZitiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        let ptr = unsafe { ziti_sys::ziti_errorstr(code) };
        if !ptr.is_null() {
            let msg = unsafe { CStr::from_ptr(ptr) };
            if let Ok(s) = msg.to_str() {
                return write!(f, "{s}");
            }
        }
        write!(f, "ziti error {code}")
    }
}

impl std::error::Error for ZitiError {}

/// Convert a C SDK return code to `Result<(), ZitiError>`.
///
/// Returns `Ok(())` for `ZITI_OK` (0), `Err` for any non-zero code.
pub fn check(rc: c_int) -> Result<(), ZitiError> {
    if rc == ziti_sys::ZITI_OK {
        Ok(())
    } else {
        Err(ZitiError::from(rc))
    }
}

/// Get a human-readable error description from the C SDK.
pub fn error_str(code: c_int) -> &'static str {
    if code == ziti_sys::ZITI_OK {
        return "OK";
    }
    let ptr = unsafe { ziti_sys::ziti_errorstr(code) };
    if ptr.is_null() {
        return "unknown error";
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .unwrap_or("unknown error")
}
