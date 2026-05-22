use ziti::{check, ZitiError};

#[test]
fn all_known_codes_roundtrip() {
    let codes: &[(i32, ZitiError)] = &[
        (-1, ZitiError::ConfigNotFound),
        (-2, ZitiError::JwtNotFound),
        (-3, ZitiError::JwtInvalid),
        (-4, ZitiError::JwtInvalidFormat),
        (-5, ZitiError::Pkcs7Asn1ParsingFailed),
        (-6, ZitiError::JwtSigningAlgUnsupported),
        (-7, ZitiError::JwtVerificationFailed),
        (-8, ZitiError::EnrollmentMethodUnsupported),
        (-9, ZitiError::EnrollmentCertificateRequired),
        (-10, ZitiError::KeyGenerationFailed),
        (-11, ZitiError::KeyLoadFailed),
        (-12, ZitiError::CsrGenerationFailed),
        (-13, ZitiError::InvalidConfig),
        (-14, ZitiError::AuthenticationFailed),
        (-15, ZitiError::NotAuthorized),
        (-16, ZitiError::ControllerUnavailable),
        (-17, ZitiError::GatewayUnavailable),
        (-18, ZitiError::ServiceUnavailable),
        (-19, ZitiError::Eof),
        (-20, ZitiError::Timeout),
        (-21, ZitiError::ConnAbort),
        (-22, ZitiError::InvalidState),
        (-23, ZitiError::CryptoFail),
        (-24, ZitiError::ConnClosed),
        (-25, ZitiError::InvalidPosture),
        (-26, ZitiError::MfaExists),
        (-27, ZitiError::MfaInvalidToken),
        (-28, ZitiError::MfaNotEnrolled),
        (-29, ZitiError::NotFound),
        (-30, ZitiError::Disabled),
        (-31, ZitiError::PartiallyAuthenticated),
        (-32, ZitiError::InvalidAuthenticatorType),
        (-33, ZitiError::InvalidAuthenticatorCert),
        (-34, ZitiError::InvalidCertKeyPair),
        (-35, ZitiError::CertInUse),
        (-36, ZitiError::CertFailedValidation),
        (-37, ZitiError::MissingCertClaim),
        (-38, ZitiError::AllocFailed),
        (-39, ZitiError::ExternalLoginRequired),
        (-40, ZitiError::AlreadyEnrolled),
        (-41, ZitiError::EnrollmentNotAllowed),
        (-42, ZitiError::ApiSessionInvalid),
        (-111, ZitiError::Wtf),
    ];

    for &(code, expected) in codes {
        let err = ZitiError::from(code);
        assert_eq!(err, expected, "from({code}) did not match");
        assert_eq!(err.code(), code, "{err:?}.code() did not match");
    }
}

#[test]
fn unknown_code() {
    let err = ZitiError::from(-999);
    assert_eq!(err, ZitiError::Unknown(-999));
    assert_eq!(err.code(), -999);
}

#[test]
fn display_is_non_empty_for_known_codes() {
    let _lib = ziti::ZitiLib::init();
    for code in -42..=-1 {
        let err = ZitiError::from(code);
        let msg = format!("{err}");
        assert!(!msg.is_empty(), "Display for code {code} was empty");
    }
}

#[test]
fn check_ok() {
    assert!(check(0).is_ok());
}

#[test]
fn check_error() {
    let result = check(-14);
    assert_eq!(result, Err(ZitiError::AuthenticationFailed));
}

#[test]
fn error_implements_std_error() {
    let err = ZitiError::Timeout;
    let _: &dyn std::error::Error = &err;
}
