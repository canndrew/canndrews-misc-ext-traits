#![feature(never_type)]
#![feature(exhaustive_patterns)]

mod vec_deque;
mod result;
pub use self::vec_deque::VecDequeExt;
pub use self::result::ResultNeverErrExt;

#[cfg(feature = "futures")]
pub mod futures;
#[cfg(feature = "futures")]
pub use self::futures::*;

#[cfg(feature = "bytes")]
pub mod bytes;
#[cfg(feature = "bytes")]
pub use self::bytes::*;

