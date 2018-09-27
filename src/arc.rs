use std::sync::Arc;

pub trait ArcExt<T> {
    fn take_or_clone(self) -> T
    where
        T: Clone;
}

impl<T> ArcExt<T> for Arc<T> {
    fn take_or_clone(self) -> T
    where
        T: Clone
    {
        Arc::try_unwrap(self).unwrap_or_else(|arc| T::clone(&arc))
    }
}

