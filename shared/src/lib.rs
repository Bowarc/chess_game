#[macro_use]
extern crate log;

pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 1, 111), 19864),
);

pub mod chess;
pub mod file;
pub mod id;
pub mod maths;
pub mod message;
