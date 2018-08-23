use futures::Stream;
use crate::futures::InferErr;

pub trait StreamExt: Sized {
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

