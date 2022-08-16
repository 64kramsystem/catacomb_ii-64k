use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive)]
#[repr(u16)]
pub enum soundtype {
    sdlib = 2,
    spkr = 1,
    off = 0,
}

// For readability. Possibly, only a reference one will be needed once/if the data types are fully
// cleaned up.

impl From<i32> for soundtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}
