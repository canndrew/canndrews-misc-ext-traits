pub trait ResultNeverErrExt<T> {
    fn never_err(self) -> T;
}

impl<T> ResultNeverErrExt<T> for Result<T, !> {
    fn never_err(self) -> T {
        match self {
            Ok(val) => val,
        }
    }
}

