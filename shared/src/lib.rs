#[macro_use]
extern crate log;

pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(172, 16, 127, 108), 19864),
);

pub mod chess;
pub mod error;
pub mod file;
pub mod game;
pub mod id;
pub mod maths;
pub mod message;
