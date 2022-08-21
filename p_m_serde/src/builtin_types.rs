use std::convert::TryInto;
use std::fmt::Debug;

use super::Deserialize;

impl Deserialize for u16 {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let mut buffer = [0; 2];
        r.read_exact(&mut buffer).unwrap();
        u16::from_le_bytes(buffer)
    }
}

impl Deserialize for u32 {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let mut buffer = [0; 4];
        r.read_exact(&mut buffer).unwrap();
        u32::from_le_bytes(buffer)
    }
}

impl Deserialize for i8 {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let mut buffer = [0; 1];
        r.read_exact(&mut buffer).unwrap();
        i8::from_le_bytes(buffer)
    }
}

impl Deserialize for i16 {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let mut buffer = [0; 2];
        r.read_exact(&mut buffer).unwrap();
        i16::from_le_bytes(buffer)
    }
}

impl<T, const N: usize> Deserialize for [T; N]
where
    T: Deserialize + Debug,
{
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        // Optimization (e.g. via `arr_macro` crate) is insignificant in this context, and it should
        // be measured first, even if it was.
        //
        std::iter::repeat_with(|| T::deserialize(&mut r))
            .take(N)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
