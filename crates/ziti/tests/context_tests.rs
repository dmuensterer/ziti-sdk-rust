use ziti::{ZitiError, ZitiLib};

#[test]
fn init_and_drop() {
    let _lib = ZitiLib::init();
    // Drop should call Ziti_lib_shutdown without panicking
}

#[test]
fn load_context_invalid_identity_returns_error() {
    let lib = ZitiLib::init();
    let result = lib.load_context("not-a-valid-identity-json-or-path");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // The C SDK may return ConfigNotFound, InvalidState, or InvalidConfig
    // depending on whether the input is treated as a missing file vs bad JSON
    assert!(
        err == ZitiError::ConfigNotFound
            || err == ZitiError::InvalidConfig
            || err == ZitiError::InvalidState,
        "unexpected error: {err:?}"
    );
}

#[test]
fn load_context_with_timeout_invalid_returns_error() {
    let lib = ZitiLib::init();
    let result = lib.load_context_with_timeout("bogus", 1000);
    assert!(result.is_err());
}

#[test]
fn last_error_returns_something_after_failure() {
    let lib = ZitiLib::init();
    // Trigger a failure
    let _ = lib.load_context("invalid");
    // last_error should reflect the failure
    let err = lib.last_error();
    // We don't assert the exact code since it depends on the C SDK's thread-local state,
    // but it should not be a zero/OK
    assert_ne!(err.code(), 0);
}
