use std::convert::TryInto;
use std::fmt::Debug;

use crate::macros::impl_for_numeric;
use crate::Deserialize;

impl_for_numeric!(
    Deserialize,
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128
);

impl_for_numeric!(Serialize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

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
