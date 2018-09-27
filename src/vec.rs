pub trait VecBytesExt {
    fn zeros(len: usize) -> Vec<u8>;
}

impl VecBytesExt for Vec<u8> {
    fn zeros(len: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(len);
        ret.extend((0..len).map(|_| 0));
        ret
    }
}

