#![feature(never_type)]
#![feature(exhaustive_patterns)]

mod vec_deque;
mod result;
mod arc;
mod vec;
mod net;
pub use self::vec_deque::VecDequeExt;
pub use self::result::ResultNeverErrExt;
pub use self::arc::ArcExt;
pub use self::vec::VecBytesExt;

#[cfg(feature = "futures")]
pub mod futures;
#[cfg(feature = "futures")]
pub use self::futures::*;

#[cfg(feature = "tokio")]
pub mod tokio;
#[cfg(feature = "tokio")]
pub use self::tokio::*;

#[cfg(feature = "bytes")]
pub mod bytes;
#[cfg(feature = "bytes")]
pub use self::bytes::*;

