#[macro_use]
extern crate log;

pub mod file;
pub mod logger;
pub mod threading;
pub mod time;

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};
}
