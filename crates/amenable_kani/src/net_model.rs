//! Kani-only accommodation model for TCP/UDP socket semantics.
//!
//! This module is where Amenable stops asking Kani to execute libc-backed
//! socket creation directly and instead proves against a small package of
//! explicit connection/datagram laws that the real implementation is
//! expected to refine.
//!
//! The direct `TcpListener::bind()` path remains preserved in the proof
//! gallery as an unsupported `socket` boundary
//! (`gallery::replace_recommendations::
//! socket_construction_reaches_an_unsupported_socket_syscall_boundary`,
//! confirmed identical across `TcpListener`/`TcpStream`/`UdpSocket`/
//! `Incoming`/shutdown). Production proofs that use this model are
//! therefore conditional:
//!
//! - if the real std/libc path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::KaniModelError;

#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn loopback(port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
}

/// One modeled TCP connection, queued at `connect()` and readable/
/// writable once `accept()`ed -- shared state behind the client/server
/// handles that operate on it, the same shape `pipe_model::KaniPipe`
/// uses for its reader/writer pair.
#[derive(Debug, Clone, PartialEq, Eq)]
struct KaniTcpConnection {
    resource_id: u64,
    server_addr: SocketAddr,
    client_addr: SocketAddr,
    buffered: Vec<u8>,
    client_write_open: bool,
    accepted: bool,
}

/// Modeled listening TCP socket. Holds at most one pending/accepted
/// connection at a time -- sufficient for every claim this crate's
/// production proofs make; none needs a multi-connection backlog.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniTcpListener {
    /// The listener's own bound address.
    #[getter(copy)]
    local_addr: SocketAddr,
    #[getter(skip)]
    resource_id: u64,
    #[getter(skip)]
    connection: Option<KaniTcpConnection>,
}

/// Modeled client-side handle to one TCP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniTcpClient {
    #[getter(copy)]
    resource_id: u64,
    /// The client's own (ephemeral) address.
    #[getter(copy)]
    local_addr: SocketAddr,
    /// The listener address this client connected to.
    #[getter(copy)]
    peer_addr: SocketAddr,
}

/// Modeled server-side handle to one accepted TCP connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniTcpServer {
    #[getter(copy)]
    resource_id: u64,
    /// The listener's own bound address.
    #[getter(copy)]
    local_addr: SocketAddr,
    /// The connecting client's address.
    #[getter(copy)]
    peer_addr: SocketAddr,
}

impl KaniTcpListener {
    /// Construct a fixed, deterministic bound listener at loopback port
    /// 0, resource id 0 -- no non-deterministic construction, so (like
    /// `KaniPipe::minimal`) this stays available outside `cfg(kani)`.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn minimal() -> Self {
        Self {
            local_addr: loopback(0),
            resource_id: 0,
            connection: None,
        }
    }

    /// Model a client connecting: queues one connection in the backlog
    /// and returns the client's own handle to it. Mirrors real socket
    /// backlog behavior -- a `connect()` queues before any `accept()`
    /// runs, which is exactly what
    /// `verify_incoming_yields_an_already_queued_connection` checks.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self)))]
    pub fn connect(&mut self, client_port: u16) -> KaniTcpClient {
        assert!(
            self.connection.is_none(),
            "modeled listener supports one pending/accepted connection at a time"
        );
        let client_addr = loopback(client_port);
        self.connection = Some(KaniTcpConnection {
            resource_id: self.resource_id,
            server_addr: self.local_addr,
            client_addr,
            buffered: Vec::new(),
            client_write_open: true,
            accepted: false,
        });
        KaniTcpClient {
            resource_id: self.resource_id,
            local_addr: client_addr,
            peer_addr: self.local_addr,
        }
    }

    /// Report whether a connection is queued and not yet accepted.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn has_pending(&self) -> bool {
        self.connection
            .as_ref()
            .is_some_and(|connection| !connection.accepted)
    }

    /// Model `.accept()`: pop the queued connection and hand back its
    /// server-side handle plus the client's address.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self)))]
    pub fn accept(&mut self) -> (KaniTcpServer, SocketAddr) {
        let connection = self
            .connection
            .as_mut()
            .expect("accept requires a queued connection");
        assert!(!connection.accepted, "connection already accepted");
        connection.accepted = true;
        (
            KaniTcpServer {
                resource_id: connection.resource_id,
                local_addr: connection.server_addr,
                peer_addr: connection.client_addr,
            },
            connection.client_addr,
        )
    }

    /// Model `.incoming().next()`: the same underlying operation as
    /// [`Self::accept`] -- real `Incoming::next()` never returns `None`,
    /// it loops until a connection is ready, so this asserts one is
    /// already queued rather than modeling the non-blocking case.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self)))]
    pub fn incoming_next(&mut self) -> KaniTcpServer {
        self.accept().0
    }

    /// Append bytes written by the given client handle, failing once
    /// that client's write half has been shut down.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "info", skip(self, client, payload))
    )]
    pub fn client_write(
        &mut self,
        client: KaniTcpClient,
        payload: Vec<u8>,
    ) -> Result<(), KaniModelError> {
        let connection = self
            .connection
            .as_mut()
            .filter(|connection| connection.resource_id == client.resource_id)
            .expect("client handle must target this listener's modeled connection");
        if !connection.client_write_open {
            return Err(KaniModelError::write_half_closed());
        }
        connection.buffered.extend(payload);
        Ok(())
    }

    /// Close the given client handle's write half.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, client)))]
    pub fn client_shutdown_write(&mut self, client: KaniTcpClient) {
        let connection = self
            .connection
            .as_mut()
            .filter(|connection| connection.resource_id == client.resource_id)
            .expect("client handle must target this listener's modeled connection");
        connection.client_write_open = false;
    }

    /// Drain all bytes buffered for the given accepted server handle.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self, server)))]
    pub fn server_read(&mut self, server: KaniTcpServer) -> Vec<u8> {
        let connection = self
            .connection
            .as_mut()
            .filter(|connection| connection.resource_id == server.resource_id)
            .expect("server handle must target this listener's modeled connection");
        assert!(
            connection.accepted,
            "server handle requires an accepted connection"
        );
        let delivered = connection.buffered.clone();
        connection.buffered.clear();
        delivered
    }
}

/// Modeled error for a write against a connection whose write half has
/// been shut down. Not `PartialEq`/`Eq`/`Hash`/`PartialOrd`/`Ord`:
/// location tracking makes comparison confusing (this workspace's own
/// error-type exception, `CLAUDE.md`), and not `Copy`: owned `file` is
/// a `String`.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_getters::Getters)]
#[display("the modeled connection's write half is closed")]
pub struct KaniWriteHalfClosed {
    /// Source line of the call site that produced this error.
    #[getter(copy)]
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl KaniWriteHalfClosed {
    /// Construct the error, recording the caller's location.
    ///
    /// `Location::caller()` is itself an unsupported construct under
    /// Kani (see `fs_model::KaniAlreadyExists::new`'s doc comment for
    /// the confirming detail) -- a Kani-reachable panic is its own
    /// failure signal regardless of what file/line this carries.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn new() -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { line, file }
    }
}

impl Default for KaniWriteHalfClosed {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

/// Modeled UDP socket: a bound address plus an inbox of datagrams
/// delivered by [`Self::send_to`], each tagged with its real sender
/// address. Two distinct sockets are already told apart by their
/// (distinct) bound ports, so unlike [`KaniTcpListener`] there is no
/// separate resource-identity field to check.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniUdpSocket {
    /// The socket's own bound address.
    #[getter(copy)]
    local_addr: SocketAddr,
    #[getter(skip)]
    inbox: Option<(SocketAddr, Vec<u8>)>,
}

impl KaniUdpSocket {
    /// Construct a fixed, deterministic bound socket at the given
    /// loopback port -- no non-deterministic construction, so this
    /// stays available outside `cfg(kani)`.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn bind(port: u16) -> Self {
        Self {
            local_addr: loopback(port),
            inbox: None,
        }
    }

    /// Report whether a datagram is currently queued for delivery.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn has_pending(&self) -> bool {
        self.inbox.is_some()
    }

    /// Model `.send_to()`: deliver a datagram directly into the
    /// target's modeled inbox, tagged with this socket's own address as
    /// the reported sender -- the address `.recv_from()` reports back.
    /// Holds at most one queued datagram at a time -- sufficient for
    /// every claim this crate's production proofs make; none needs a
    /// multi-datagram inbox.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "info", skip(self, target, payload))
    )]
    pub fn send_to(&self, target: &mut KaniUdpSocket, payload: Vec<u8>) {
        assert!(
            target.inbox.is_none(),
            "modeled socket supports one queued datagram at a time"
        );
        target.inbox = Some((self.local_addr, payload));
    }

    /// Model `.recv_from()`: take the queued datagram along with its
    /// real sender address.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self)))]
    pub fn recv_from(&mut self) -> (Vec<u8>, SocketAddr) {
        let (from, payload) = self
            .inbox
            .take()
            .expect("recv_from requires a queued datagram");
        (payload, from)
    }
}
