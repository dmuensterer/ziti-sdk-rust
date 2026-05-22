use ziti::{SocketStatus, SocketType, ZitiLib, ZitiSocket};

#[test]
fn create_stream_socket() {
    let _lib = ZitiLib::init();
    let sock = ZitiSocket::new(SocketType::Stream);
    assert!(sock.is_ok());
    let sock = sock.unwrap();
    assert!(sock.as_raw_fd() >= 0);
}

#[test]
fn create_dgram_socket() {
    let _lib = ZitiLib::init();
    let sock = ZitiSocket::new(SocketType::Dgram);
    assert!(sock.is_ok());
}

#[test]
fn check_unbound_socket_status() {
    let _lib = ZitiLib::init();
    let sock = ZitiSocket::new(SocketType::Stream).unwrap();
    // A freshly created socket is not yet connected or bound
    let status = sock.check_status();
    assert_eq!(status, SocketStatus::NotZiti);
}

#[test]
fn close_socket_explicitly() {
    let _lib = ZitiLib::init();
    let mut sock = ZitiSocket::new(SocketType::Stream).unwrap();
    sock.close(); // should not panic
}

#[test]
fn double_close_is_ok() {
    let _lib = ZitiLib::init();
    let mut sock = ZitiSocket::new(SocketType::Stream).unwrap();
    sock.close();
    sock.close(); // second close is a no-op
}
