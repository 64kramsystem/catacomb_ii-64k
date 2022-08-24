use num::FromPrimitive;
use num_derive::FromPrimitive;

/// In the original, this is an enum, stored as u16.
#[repr(u16)]
#[derive(Clone, Copy, Default, FromPrimitive, PartialEq)]
pub enum grtype {
    VGAgr = 3,
    EGAgr = 2,
    CGAgr = 1,
    #[default]
    text = 0,
}

impl From<i32> for grtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}
