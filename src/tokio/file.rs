use std::io;
use tokio::fs::File;
use futures::{Future, Async};
use unwrap::unwrap;

pub struct SetLenFuture {
    file: Option<File>,
    size: u64,
}

impl Future for SetLenFuture {
    type Item = File;
    type Error = io::Error;

    fn poll(&mut self) -> io::Result<Async<File>> {
        let mut file = unwrap!(self.file.take());
        match file.poll_set_len(self.size)? {
            Async::Ready(()) => Ok(Async::Ready(file)),
            Async::NotReady => {
                self.file = Some(file);
                Ok(Async::NotReady)
            },
        }
    }
}

pub trait FileExt {
    fn set_len(self, size: u64) -> SetLenFuture;
}

impl FileExt for File {
    fn set_len(self, size: u64) -> SetLenFuture {
        SetLenFuture {
            file: Some(self),
            size,
        }
    }
}

