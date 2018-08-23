use futures::Stream;
use crate::futures::InferErr;

pub trait StreamExt: Sized {
    /// Convert a `Stream` with error type `!` to a `Stream` with an inferred error type.
    fn infer_err<E>(self) -> InferErr<Self, E>
    where
        Self: Stream<Error = !>;
}

impl<S> StreamExt for S {
    fn infer_err<E>(self) -> InferErr<Self, E>
    where
        Self: Stream<Error = !>,
    {
        InferErr::new(self)
    }
}

