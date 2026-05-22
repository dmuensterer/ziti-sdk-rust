//! DNS resolution via the Ziti network.

use std::ffi::CString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use crate::error::{check, ZitiError};

/// Result of a Ziti DNS resolution.
#[derive(Debug)]
pub struct ResolvedAddrs {
    addrs: Vec<SocketAddr>,
}

impl ResolvedAddrs {
    /// Get the resolved socket addresses.
    pub fn addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }

    /// Consume into a Vec of socket addresses.
    pub fn into_addrs(self) -> Vec<SocketAddr> {
        self.addrs
    }
}

/// Resolve a hostname via Ziti DNS.
///
/// Returns the resolved addresses on success.
pub fn resolve(host: &str, port: &str) -> Result<ResolvedAddrs, ZitiError> {
    let host_cstr = CString::new(host).map_err(|_| ZitiError::InvalidState)?;
    let port_cstr = CString::new(port).map_err(|_| ZitiError::InvalidState)?;

    let mut addrlist: *mut libc::addrinfo = std::ptr::null_mut();

    let rc = unsafe {
        ziti_sys::Ziti_resolve(
            host_cstr.as_ptr(),
            port_cstr.as_ptr(),
            std::ptr::null(),
            &mut addrlist,
        )
    };

    check(rc)?;

    let mut addrs = Vec::new();
    let mut current = addrlist;

    while !current.is_null() {
        let ai = unsafe { &*current };

        if let Some(addr) = sockaddr_to_socket_addr(ai) {
            addrs.push(addr);
        }

        current = ai.ai_next;
    }

    if !addrlist.is_null() {
        unsafe { libc::freeaddrinfo(addrlist) };
    }

    Ok(ResolvedAddrs { addrs })
}

fn sockaddr_to_socket_addr(ai: &libc::addrinfo) -> Option<SocketAddr> {
    if ai.ai_addr.is_null() {
        return None;
    }

    unsafe {
        match (*ai.ai_addr).sa_family as i32 {
            libc::AF_INET => {
                let addr = &*(ai.ai_addr as *const libc::sockaddr_in);
                let ip = Ipv4Addr::from(u32::from_be(addr.sin_addr.s_addr));
                let port = u16::from_be(addr.sin_port);
                Some(SocketAddr::new(IpAddr::V4(ip), port))
            }
            libc::AF_INET6 => {
                let addr = &*(ai.ai_addr as *const libc::sockaddr_in6);
                let ip = Ipv6Addr::from(addr.sin6_addr.s6_addr);
                let port = u16::from_be(addr.sin6_port);
                Some(SocketAddr::new(IpAddr::V6(ip), port))
            }
            _ => None,
        }
    }
}
