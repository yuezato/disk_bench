#[macro_use]
extern crate trackable;
pub mod aligned;
mod error;
pub mod file_builder;
pub mod timer;
pub use error::{Error, ErrorKind};

/// crate固有の`Result`型.
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! track_io {
    ($expr:expr) => {
        $expr.map_err(|e: ::std::io::Error| track!(Error::from(e)))
    };
}
