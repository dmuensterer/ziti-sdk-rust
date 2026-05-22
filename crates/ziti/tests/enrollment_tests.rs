//! Enrollment tests require network access and are gated behind the `e2e` feature.

#[cfg(feature = "e2e")]
mod tests {
    use ziti::{enroll_identity, ZitiLib};

    #[test]
    fn enroll_identity_invalid_jwt_returns_error() {
        let _lib = ZitiLib::init();
        let result = enroll_identity("not-a-valid-jwt", None, None);
        assert!(result.is_err());
    }
}
