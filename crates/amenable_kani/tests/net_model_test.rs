use amenable_kani::{KaniTcpListener, KaniUdpSocket};

#[test]
fn connect_then_accept_pairs_client_and_server_by_address() {
    let mut listener = KaniTcpListener::minimal();
    let addr = listener.local_addr();
    let client = listener.connect(1);

    assert!(listener.has_pending());
    let (server, peer_addr) = listener.accept();
    assert!(!listener.has_pending());
    assert_eq!(peer_addr, client.local_addr());
    assert_eq!(server.local_addr(), addr);
    assert_eq!(server.peer_addr(), client.local_addr());
}

#[test]
fn incoming_next_yields_an_already_queued_connection() {
    let mut listener = KaniTcpListener::minimal();
    let client = listener.connect(1);

    let server = listener.incoming_next();
    assert_eq!(server.peer_addr(), client.local_addr());
}

#[test]
fn write_then_read_round_trips_buffered_bytes() {
    let mut listener = KaniTcpListener::minimal();
    let client = listener.connect(1);
    let (server, _peer_addr) = listener.accept();

    listener.client_write(client, b"hello".to_vec()).unwrap();
    assert_eq!(listener.server_read(server), b"hello");
}

#[test]
fn shutdown_write_rejects_a_further_write() {
    let mut listener = KaniTcpListener::minimal();
    let client = listener.connect(1);
    let _server = listener.accept();

    listener.client_shutdown_write(client);
    assert!(listener.client_write(client, b"more".to_vec()).is_err());
}

#[test]
fn send_to_then_recv_from_round_trips_a_datagram_with_sender_address() {
    let mut socket_a = KaniUdpSocket::bind(0);
    let socket_b = KaniUdpSocket::bind(1);
    let addr_b = socket_b.local_addr();

    socket_b.send_to(&mut socket_a, b"ping".to_vec());
    assert!(socket_a.has_pending());

    let (payload, from) = socket_a.recv_from();
    assert!(!socket_a.has_pending());
    assert_eq!(payload, b"ping");
    assert_eq!(from, addr_b);
}
