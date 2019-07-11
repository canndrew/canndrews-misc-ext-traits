use std::io::{Cursor, Read};
use std::io;

pub trait CursorExt<T> {
    fn read_slice(&mut self, len: usize) -> io::Result<&[u8]>;
}

impl<T> CursorExt<T> for Cursor<T>
where
    T: AsRef<[u8]>
{
    fn read_slice(&mut self, len: usize) -> io::Result<&[u8]> {
        let bytes = self.get_ref().as_ref();
        let bytes_len = bytes.len();
        let pos = self.position() as usize;
        let next_pos = pos + len;

        if next_pos > bytes_len {
            // Get whatever error Cursor throws when it reads past the end of a byte buffer.
            self.set_position(bytes_len as u64);
            let mut c = [0u8];
            match self.read(&mut c[..]) {
                Ok(..) => panic!("Cursor can read past the end of this buffer?"),
                Err(e) => return Err(e),
            }
        }

        let bytes = self.get_ref().as_ref();
        Ok(&bytes[pos..next_pos])
    }
}

