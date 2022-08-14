use std::convert::TryFrom;

use num::FromPrimitive;
use num_derive::FromPrimitive;

/// In the original, this is an enum, stored as u16.
#[repr(u16)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum grtype {
    VGAgr = 3,
    EGAgr = 2,
    CGAgr = 1,
    text = 0,
}

impl TryFrom<i32> for grtype {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        FromPrimitive::from_i32(value).ok_or(())
    }
}
