use candid::Principal;

mod error;
mod http;
mod version;

pub use error::*;
pub use http::*;
pub use version::*;

pub type CanisterId = Principal;
pub type Cycles = u128;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
