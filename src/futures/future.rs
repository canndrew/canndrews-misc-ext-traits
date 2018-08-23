use futures::Future;
use crate::futures::InferErr;

pub trait FutureExt: Sized {
    /// Convert a `Future` with error type `!` to a future with an inferred error type.
    fn infer_err<E>(self) -> InferErr<Self, E>
    where
        Self: Future<Error = !>;
}

impl<F> FutureExt for F {
    fn infer_err<E>(self) -> InferErr<Self, E>
    where
        Self: Future<Error = !>,
    {
        InferErr::new(self)
    }
}

