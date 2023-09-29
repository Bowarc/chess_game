#[macro_use]
extern crate log;

pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 19864),
);

pub mod error;
pub mod chess;
pub mod file;
pub mod id;
pub mod maths;
pub mod message;
pub mod game;