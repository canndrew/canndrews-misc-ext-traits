pub trait ResultNeverErrExt<T> {
    /// Unwraps `Result<T, !>`, yielding the contained `Ok` value.
    fn never_err(self) -> T;
    /// Convert a `Result<T, !>` to `Result<T, E>` where `E` is some inferred type.
    fn infer_err<E>(self) -> Result<T, E>;
}

impl<T> ResultNeverErrExt<T> for Result<T, !> {
    fn never_err(self) -> T {
        match self {
            Ok(val) => val,
        }
    }

    fn infer_err<E>(self) -> Result<T, E> {
        match self {
            Ok(val) => Ok(val),
        }
    }
}

