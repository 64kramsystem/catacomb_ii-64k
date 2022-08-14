use std::convert::TryFrom;

use num::FromPrimitive;
use num_derive::FromPrimitive;

#[repr(u32)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum dirtype {
    nodir = 8,
    northwest = 7,
    southwest = 6,
    southeast = 5,
    northeast = 4,
    west = 3,
    south = 2,
    east = 1,
    north = 0,
}

// For readability. Possibly, only a reference one will be needed once/if the data types are fully
// cleaned up.

impl TryFrom<i32> for dirtype {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        FromPrimitive::from_i32(value).ok_or(())
    }
}

impl TryFrom<u16> for dirtype {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        FromPrimitive::from_u16(value).ok_or(())
    }
}
