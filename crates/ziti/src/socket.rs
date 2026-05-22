//! Safe wrappers around Ziti socket operations.

use std::ffi::CString;
use std::os::raw::c_int;

use crate::context::ZitiContext;
use crate::enums::{SocketStatus, SocketType};
use crate::error::{check, ZitiError};

/// A Ziti socket with RAII cleanup.
///
/// Wraps a `ziti_socket_t` file descriptor. Automatically closed on drop.
pub struct ZitiSocket {
    fd: ziti_sys::ziti_socket_t,
    closed: bool,
}

impl ZitiSocket {
    /// Create a new Ziti socket of the given type.
    pub fn new(sock_type: SocketType) -> Result<Self, ZitiError> {
        let fd = unsafe { ziti_sys::Ziti_socket(sock_type.to_raw()) };
        if fd < 0 {
            let err = unsafe { ziti_sys::Ziti_last_error() };
            return Err(ZitiError::from(err));
        }
        Ok(Self { fd, closed: false })
    }

    /// Get the raw file descriptor.
    pub fn as_raw_fd(&self) -> c_int {
        self.fd
    }

    /// Check the status of this socket.
    pub fn check_status(&self) -> SocketStatus {
        let rc = unsafe { ziti_sys::Ziti_check_socket(self.fd) };
        SocketStatus::from(rc)
    }

    /// Connect this socket to a Ziti service by name.
    pub fn connect(
        &self,
        ctx: &ZitiContext,
        service: &str,
        terminator: Option<&str>,
    ) -> Result<(), ZitiError> {
        let service_cstr = CString::new(service).map_err(|_| ZitiError::InvalidState)?;
        let terminator_ptr = match terminator {
            Some(t) => CString::new(t)
                .map_err(|_| ZitiError::InvalidState)?
                .into_raw() as *const _,
            None => std::ptr::null(),
        };

        let rc = unsafe {
            ziti_sys::Ziti_connect(self.fd, ctx.handle, service_cstr.as_ptr(), terminator_ptr)
        };

        // Free the terminator CString if we allocated one
        if !terminator_ptr.is_null() {
            unsafe { drop(CString::from_raw(terminator_ptr as *mut _)) };
        }

        check(rc)
    }

    /// Connect this socket to a Ziti service by hostname and port.
    pub fn connect_addr(&self, host: &str, port: u32) -> Result<(), ZitiError> {
        let host_cstr = CString::new(host).map_err(|_| ZitiError::InvalidState)?;
        let rc = unsafe { ziti_sys::Ziti_connect_addr(self.fd, host_cstr.as_ptr(), port) };
        check(rc)
    }

    /// Bind this socket to a Ziti service (server-side).
    pub fn bind(
        &self,
        ctx: &ZitiContext,
        service: &str,
        terminator: Option<&str>,
    ) -> Result<(), ZitiError> {
        let service_cstr = CString::new(service).map_err(|_| ZitiError::InvalidState)?;
        let terminator_ptr = match terminator {
            Some(t) => CString::new(t)
                .map_err(|_| ZitiError::InvalidState)?
                .into_raw() as *const _,
            None => std::ptr::null(),
        };

        let rc = unsafe {
            ziti_sys::Ziti_bind(self.fd, ctx.handle, service_cstr.as_ptr(), terminator_ptr)
        };

        if !terminator_ptr.is_null() {
            unsafe { drop(CString::from_raw(terminator_ptr as *mut _)) };
        }

        check(rc)
    }

    /// Mark this socket as listening for incoming connections.
    pub fn listen(&self, backlog: i32) -> Result<(), ZitiError> {
        let rc = unsafe { ziti_sys::Ziti_listen(self.fd, backlog) };
        check(rc)
    }

    /// Accept an incoming connection.
    ///
    /// Returns the new socket and the caller's identity name.
    pub fn accept(&self) -> Result<(ZitiSocket, String), ZitiError> {
        let mut caller_buf = [0u8; 256];
        let client_fd = unsafe {
            ziti_sys::Ziti_accept(
                self.fd,
                caller_buf.as_mut_ptr() as *mut std::os::raw::c_char,
                caller_buf.len() as c_int,
            )
        };

        if client_fd < 0 {
            let err = unsafe { ziti_sys::Ziti_last_error() };
            return Err(ZitiError::from(err));
        }

        let caller_len = caller_buf.iter().position(|&b| b == 0).unwrap_or(0);
        let peer_identity = String::from_utf8_lossy(&caller_buf[..caller_len]).to_string();

        Ok((
            ZitiSocket {
                fd: client_fd,
                closed: false,
            },
            peer_identity,
        ))
    }

    /// Close the socket explicitly. Also called automatically on drop.
    pub fn close(&mut self) {
        if !self.closed {
            self.closed = true;
            let rc = unsafe { ziti_sys::Ziti_close(self.fd) };
            if rc != 0 {
                // Ziti_close may fail for sockets not yet connected/bound;
                // fall back to standard close to release the fd.
                unsafe { libc::close(self.fd) };
            }
        }
    }
}

impl Drop for ZitiSocket {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(unix)]
impl std::os::unix::io::AsRawFd for ZitiSocket {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.fd
    }
}
