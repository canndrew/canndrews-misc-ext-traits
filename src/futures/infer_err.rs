use std::marker::PhantomData;
use futures::{Future, Stream, Async};

/// Wraps a `Future` or `Stream` with error type `!` and yields an inferred error type.
pub struct InferErr<F, E> {
    future: F,
    _ph: PhantomData<E>,
}

impl<F, E> InferErr<F, E> {
    pub fn new(arg: F) -> InferErr<F, E> {
        InferErr {
            future: arg,
            _ph: PhantomData,
        }
    }
}

impl<F, E> Future for InferErr<F, E>
where
    F: Future<Error = !>,
{
    type Item = F::Item;
    type Error = E;

    fn poll(&mut self) -> Result<Async<F::Item>, E> {
        match self.future.poll() {
            Ok(x) => Ok(x),
        }
    }
}

impl<S, E> Stream for InferErr<S, E>
where
    S: Stream<Error = !>,
{
    type Item = S::Item;
    type Error = E;

    fn poll(&mut self) -> Result<Async<Option<S::Item>>, E> {
        match self.future.poll() {
            Ok(x) => Ok(x),
        }
    }
}


