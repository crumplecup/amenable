//! `RustStdType` registrations for `std::net`.
//!
//! `IntoIncoming` is deliberately not covered here — unstable
//! (`tcp_into_incoming`).

use std::net::{Incoming, Shutdown, TcpListener, TcpStream, UdpSocket};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type_lifetime0!(
    Incoming,
    "std",
    "std::net",
    "https://doc.rust-lang.org/std/net/struct.Incoming.html",
    "The Incoming carrier lazily yields each connection accepted by a TcpListener."
);

impl_rust_std_type!(
    Shutdown,
    "std",
    "std::net",
    "https://doc.rust-lang.org/std/net/enum.Shutdown.html",
    "The Shutdown carrier names which half (or both) of a TCP connection to shut down."
);

impl_rust_std_type!(
    TcpListener,
    "std",
    "std::net",
    "https://doc.rust-lang.org/std/net/struct.TcpListener.html",
    "The TcpListener carrier listens for incoming TCP connections on a bound socket."
);

impl_rust_std_type!(
    TcpStream,
    "std",
    "std::net",
    "https://doc.rust-lang.org/std/net/struct.TcpStream.html",
    "The TcpStream carrier is a connected TCP stream between a local and remote socket."
);

impl_rust_std_type!(
    UdpSocket,
    "std",
    "std::net",
    "https://doc.rust-lang.org/std/net/struct.UdpSocket.html",
    "The UdpSocket carrier is a bound UDP socket for sending and receiving datagrams."
);

register_rust_std_standard_evidence!(
    Incoming<'static>,
    Shutdown,
    TcpListener,
    TcpStream,
    UdpSocket,
);
