use bytes::BytesMut;
use rand::RngCore;

pub trait BytesMutExt {
    /// Generate a random `BytesMut` of the given size.
    fn random(size: usize) -> BytesMut;
}

impl BytesMutExt for BytesMut {
    fn random(size: usize) -> BytesMut {
        let mut ret = BytesMut::with_capacity(size);
        unsafe {
            ret.set_len(size);
            rand::thread_rng().fill_bytes(&mut ret[..]);
        }
        ret
    }
}

