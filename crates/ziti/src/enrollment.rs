//! Ziti identity enrollment functions.

use std::ffi::CString;
use std::os::raw::{c_char, c_ulong};

use crate::enums::EnrollMode;
use crate::error::ZitiError;

/// Enroll a new Ziti identity from a JWT token.
///
/// Returns the identity JSON string on success.
///
/// * `jwt` - enrollment token string
/// * `key` - private key PEM (required for 3rd-party CA enrollment, otherwise `None`)
/// * `cert` - x.509 certificate PEM (required for 3rd-party CA, otherwise `None`)
pub fn enroll_identity(
    jwt: &str,
    key: Option<&str>,
    cert: Option<&str>,
) -> Result<String, ZitiError> {
    let jwt_cstr = CString::new(jwt).map_err(|_| ZitiError::InvalidConfig)?;

    let key_cstr = key
        .map(|k| CString::new(k).map_err(|_| ZitiError::InvalidConfig))
        .transpose()?;
    let cert_cstr = cert
        .map(|c| CString::new(c).map_err(|_| ZitiError::InvalidConfig))
        .transpose()?;

    let key_ptr = key_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
    let cert_ptr = cert_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

    let mut id_json: *mut c_char = std::ptr::null_mut();
    let mut id_json_len: c_ulong = 0;

    let rc = unsafe {
        ziti_sys::Ziti_enroll_identity(
            jwt_cstr.as_ptr(),
            key_ptr,
            cert_ptr,
            &mut id_json,
            &mut id_json_len,
        )
    };

    if rc != ziti_sys::ZITI_OK {
        return Err(ZitiError::from(rc));
    }

    let result = if !id_json.is_null() && id_json_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(id_json as *const u8, id_json_len as usize) };
        String::from_utf8_lossy(slice).into_owned()
    } else {
        String::new()
    };

    if !id_json.is_null() {
        unsafe { libc::free(id_json as *mut libc::c_void) };
    }

    Ok(result)
}

/// Enroll or authenticate a Ziti identity via a controller URL.
///
/// Returns the identity JSON string on success.
///
/// * `url` - controller URL (e.g. `"https://ctrl.example.com:1280"`)
/// * `jwt` - network JWT string, or `None` to fetch from controller
/// * `mode` - enrollment mode
/// * `signer_name` - ext-jwt-signer name, or `None` to auto-select
pub fn enroll_controller(
    url: &str,
    jwt: Option<&str>,
    mode: EnrollMode,
    signer_name: Option<&str>,
) -> Result<String, ZitiError> {
    let url_cstr = CString::new(url).map_err(|_| ZitiError::InvalidConfig)?;

    let jwt_cstr = jwt
        .map(|j| CString::new(j).map_err(|_| ZitiError::InvalidConfig))
        .transpose()?;
    let signer_cstr = signer_name
        .map(|s| CString::new(s).map_err(|_| ZitiError::InvalidConfig))
        .transpose()?;

    let jwt_ptr = jwt_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
    let signer_ptr = signer_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

    let mut id_json: *mut c_char = std::ptr::null_mut();
    let mut id_json_len: c_ulong = 0;

    let rc = unsafe {
        ziti_sys::Ziti_enroll_controller(
            url_cstr.as_ptr(),
            jwt_ptr,
            mode.to_raw(),
            signer_ptr,
            &mut id_json,
            &mut id_json_len,
        )
    };

    if rc != ziti_sys::ZITI_OK {
        return Err(ZitiError::from(rc));
    }

    let result = if !id_json.is_null() && id_json_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(id_json as *const u8, id_json_len as usize) };
        String::from_utf8_lossy(slice).into_owned()
    } else {
        String::new()
    };

    if !id_json.is_null() {
        unsafe { libc::free(id_json as *mut libc::c_void) };
    }

    Ok(result)
}
