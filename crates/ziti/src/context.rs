//! Ziti library lifecycle and identity context management.

use std::ffi::CString;
use std::sync::Once;

use crate::error::{check, ZitiError};

static INIT: Once = Once::new();

/// RAII guard for the Ziti library lifecycle.
///
/// The underlying C library is initialized once (on first `ZitiLib::init()` call)
/// and remains active for the lifetime of the process. Call [`ZitiLib::shutdown()`]
/// explicitly if you need to tear down the background thread before exit.
pub struct ZitiLib {
    _private: (),
}

impl ZitiLib {
    /// Initialize the Ziti library.
    ///
    /// The underlying C library is initialized exactly once, regardless of how
    /// many times this is called. Subsequent calls return a handle to the same
    /// library state.
    pub fn init() -> Self {
        INIT.call_once(|| {
            unsafe { ziti_sys::Ziti_lib_init() };
        });
        Self { _private: () }
    }

    /// Explicitly shut down the Ziti library.
    ///
    /// This terminates the background processing thread. After calling this,
    /// no further Ziti operations should be performed. This is optional -
    /// the library cleans up on process exit automatically.
    ///
    /// # Safety
    ///
    /// All `ZitiContext` and `ZitiSocket` instances must be dropped before
    /// calling this method.
    pub fn shutdown(self) {
        unsafe { ziti_sys::Ziti_lib_shutdown() };
    }

    /// Load a Ziti identity from a JSON string or file path.
    pub fn load_context(&self, identity: &str) -> Result<LoadResult, ZitiError> {
        let identity_cstr = CString::new(identity).map_err(|_| ZitiError::InvalidConfig)?;

        let mut handle: ziti_sys::ziti_handle_t = ziti_sys::ZITI_INVALID_HANDLE;
        let rc = unsafe { ziti_sys::Ziti_load_context(&mut handle, identity_cstr.as_ptr()) };

        match rc {
            ziti_sys::ZITI_OK => Ok(LoadResult::Ready(ZitiContext { handle })),
            ziti_sys::ZITI_EXTERNAL_LOGIN_REQUIRED => {
                Ok(LoadResult::ExternalLoginRequired(ZitiContext { handle }))
            }
            ziti_sys::ZITI_PARTIALLY_AUTHENTICATED => {
                Ok(LoadResult::PartiallyAuthenticated(ZitiContext { handle }))
            }
            ziti_sys::ZITI_MFA_NOT_ENROLLED => {
                Ok(LoadResult::MfaNotEnrolled(ZitiContext { handle }))
            }
            _ => Err(ZitiError::from(rc)),
        }
    }

    /// Load a Ziti identity with a timeout in milliseconds.
    pub fn load_context_with_timeout(
        &self,
        identity: &str,
        timeout_ms: i32,
    ) -> Result<LoadResult, ZitiError> {
        let identity_cstr = CString::new(identity).map_err(|_| ZitiError::InvalidConfig)?;

        let mut handle: ziti_sys::ziti_handle_t = ziti_sys::ZITI_INVALID_HANDLE;
        let rc = unsafe {
            ziti_sys::Ziti_load_context_with_timeout(
                &mut handle,
                identity_cstr.as_ptr(),
                timeout_ms,
            )
        };

        match rc {
            ziti_sys::ZITI_OK => Ok(LoadResult::Ready(ZitiContext { handle })),
            ziti_sys::ZITI_EXTERNAL_LOGIN_REQUIRED => {
                Ok(LoadResult::ExternalLoginRequired(ZitiContext { handle }))
            }
            ziti_sys::ZITI_PARTIALLY_AUTHENTICATED => {
                Ok(LoadResult::PartiallyAuthenticated(ZitiContext { handle }))
            }
            ziti_sys::ZITI_MFA_NOT_ENROLLED => {
                Ok(LoadResult::MfaNotEnrolled(ZitiContext { handle }))
            }
            _ => Err(ZitiError::from(rc)),
        }
    }

    /// Get the error code for the last failed operation.
    pub fn last_error(&self) -> ZitiError {
        let rc = unsafe { ziti_sys::Ziti_last_error() };
        ZitiError::from(rc)
    }
}

/// Result of loading a Ziti identity context.
///
/// The C SDK returns informational codes that indicate the context was loaded
/// but requires additional authentication steps.
#[derive(Debug)]
pub enum LoadResult {
    /// Identity is fully authenticated and ready to use.
    Ready(ZitiContext),
    /// External login (OIDC) is required. Call `login_external` on the context.
    ExternalLoginRequired(ZitiContext),
    /// Identity is partially authenticated. TOTP code required.
    PartiallyAuthenticated(ZitiContext),
    /// MFA is not enrolled but is required.
    MfaNotEnrolled(ZitiContext),
}

impl LoadResult {
    /// Get a reference to the context regardless of auth state.
    pub fn context(&self) -> &ZitiContext {
        match self {
            Self::Ready(ctx)
            | Self::ExternalLoginRequired(ctx)
            | Self::PartiallyAuthenticated(ctx)
            | Self::MfaNotEnrolled(ctx) => ctx,
        }
    }

    /// Consume and return the inner context regardless of auth state.
    pub fn into_context(self) -> ZitiContext {
        match self {
            Self::Ready(ctx)
            | Self::ExternalLoginRequired(ctx)
            | Self::PartiallyAuthenticated(ctx)
            | Self::MfaNotEnrolled(ctx) => ctx,
        }
    }
}

/// A loaded Ziti identity context.
#[derive(Debug)]
pub struct ZitiContext {
    pub(crate) handle: ziti_sys::ziti_handle_t,
}

impl ZitiContext {
    /// Get the raw handle value (for advanced use cases).
    pub fn raw_handle(&self) -> u32 {
        self.handle
    }

    /// Get names of external JWT signers available for authentication.
    pub fn get_ext_signers(&self) -> Vec<String> {
        let ptr = unsafe { ziti_sys::Ziti_get_ext_signers(self.handle) };
        if ptr.is_null() {
            return Vec::new();
        }

        let mut signers = Vec::new();
        let mut i = 0;
        loop {
            let entry = unsafe { *ptr.add(i) };
            if entry.is_null() {
                break;
            }
            let s = unsafe { std::ffi::CStr::from_ptr(entry) };
            if let Ok(name) = s.to_str() {
                signers.push(name.to_owned());
            }
            i += 1;
        }
        signers
    }

    /// Start external login. Returns a URL the user should open in a browser.
    pub fn login_external(&self, signer_name: &str) -> Result<String, ZitiError> {
        let signer_cstr = CString::new(signer_name).map_err(|_| ZitiError::InvalidState)?;

        let ptr =
            unsafe { ziti_sys::Ziti_login_external(self.handle, signer_cstr.as_ptr()) };
        if ptr.is_null() {
            let err = unsafe { ziti_sys::Ziti_last_error() };
            return Err(ZitiError::from(err));
        }

        let url = unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_str()
            .map_err(|_| ZitiError::InvalidState)?
            .to_owned();

        unsafe { libc::free(ptr as *mut libc::c_void) };

        Ok(url)
    }

    /// Login with a TOTP code.
    pub fn login_totp(&self, code: &str) -> Result<(), ZitiError> {
        let code_cstr = CString::new(code).map_err(|_| ZitiError::InvalidState)?;
        let rc = unsafe { ziti_sys::Ziti_login_totp(self.handle, code_cstr.as_ptr()) };
        check(rc)
    }

    /// Block until authentication completes or timeout.
    ///
    /// `timeout_ms`: timeout in milliseconds, `None` or `Some(0)` means no timeout.
    pub fn wait_for_auth(&self, timeout_ms: Option<i32>) -> Result<(), ZitiError> {
        let rc =
            unsafe { ziti_sys::Ziti_wait_for_auth(self.handle, timeout_ms.unwrap_or(0)) };
        check(rc)
    }
}
