//! `RustStdType` registrations for `core::net`.
//!
//! `Ipv6MulticastScope` is deliberately not covered here — it sits behind
//! the unstable `ip` feature (rust-lang/rust#27709), not nameable from a
//! stable toolchain.

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    core::net::IpAddr,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/enum.IpAddr.html",
    "The IpAddr carrier holds either a V4 or V6 IP address."
);

impl_rust_std_type!(
    core::net::Ipv4Addr,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/struct.Ipv4Addr.html",
    "The Ipv4Addr carrier stores a 32-bit IPv4 address."
);

impl_rust_std_type!(
    core::net::Ipv6Addr,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/struct.Ipv6Addr.html",
    "The Ipv6Addr carrier stores a 128-bit IPv6 address."
);

impl_rust_std_type!(
    core::net::SocketAddr,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/enum.SocketAddr.html",
    "The SocketAddr carrier holds either a V4 or V6 IP address paired with a port."
);

impl_rust_std_type!(
    core::net::SocketAddrV4,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/struct.SocketAddrV4.html",
    "The SocketAddrV4 carrier stores an IPv4 address paired with a port."
);

impl_rust_std_type!(
    core::net::SocketAddrV6,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/struct.SocketAddrV6.html",
    "The SocketAddrV6 carrier stores an IPv6 address paired with a port, flow info, and scope id."
);

impl_rust_std_type!(
    core::net::AddrParseError,
    "core",
    "core::net",
    "https://doc.rust-lang.org/core/net/struct.AddrParseError.html",
    "The AddrParseError carrier reports that a string could not be parsed as an IP address or socket address."
);
