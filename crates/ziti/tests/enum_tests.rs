use ziti::{CryptoMethod, EnrollMode, RateType, SocketStatus, SocketType};

#[test]
fn socket_type_raw_values() {
    assert_eq!(SocketType::Stream.to_raw(), libc::SOCK_STREAM);
    assert_eq!(SocketType::Dgram.to_raw(), libc::SOCK_DGRAM);
}

#[test]
fn socket_status_from_int() {
    assert_eq!(SocketStatus::from(0), SocketStatus::NotZiti);
    assert_eq!(SocketStatus::from(1), SocketStatus::Connected);
    assert_eq!(SocketStatus::from(2), SocketStatus::Server);
    assert_eq!(SocketStatus::from(99), SocketStatus::NotZiti);
}

#[test]
fn enroll_mode_roundtrip() {
    use ziti_sys::ziti_enroll_mode as Raw;

    assert_eq!(EnrollMode::from(Raw::ziti_enroll_none), EnrollMode::None);
    assert_eq!(EnrollMode::from(Raw::ziti_enroll_cert), EnrollMode::Cert);
    assert_eq!(EnrollMode::from(Raw::ziti_enroll_token), EnrollMode::Token);

    assert_eq!(EnrollMode::None.to_raw(), Raw::ziti_enroll_none);
    assert_eq!(EnrollMode::Cert.to_raw(), Raw::ziti_enroll_cert);
    assert_eq!(EnrollMode::Token.to_raw(), Raw::ziti_enroll_token);
}

#[test]
fn crypto_method_from_raw() {
    use ziti_sys::ziti_crypto_method as Raw;

    assert_eq!(
        CryptoMethod::from(Raw::ziti_crypto_invalid),
        CryptoMethod::Invalid
    );
    assert_eq!(
        CryptoMethod::from(Raw::ziti_crypto_none),
        CryptoMethod::None
    );
    assert_eq!(
        CryptoMethod::from(Raw::ziti_crypto_libsodium),
        CryptoMethod::Libsodium
    );
    assert_eq!(
        CryptoMethod::from(Raw::ziti_crypto_aes_gcm),
        CryptoMethod::AesGcm
    );
}

#[test]
fn rate_type_from_raw() {
    use ziti_sys::rate_type as Raw;

    assert_eq!(RateType::from(Raw::EWMA_1m), RateType::Ewma1m);
    assert_eq!(RateType::from(Raw::EWMA_5m), RateType::Ewma5m);
    assert_eq!(RateType::from(Raw::EWMA_15m), RateType::Ewma15m);
    assert_eq!(RateType::from(Raw::MMA_1m), RateType::Mma1m);
    assert_eq!(RateType::from(Raw::CMA_1m), RateType::Cma1m);
    assert_eq!(RateType::from(Raw::EWMA_5s), RateType::Ewma5s);
    assert_eq!(RateType::from(Raw::INSTANT), RateType::Instant);
}
