use std::convert::TryFrom;

use num::FromPrimitive;
use num_derive::FromPrimitive;

/// In the original, this is an enum, (automatically) narrowed down to u16 on assignment.
#[repr(u16)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum classtype {
    lastclass = 23,
    guns = 22,
    gune = 21,
    secretgate = 20,
    torch = 19,
    teleporter = 18,
    dead6 = 17,
    dead5 = 16,
    dead4 = 15,
    dead3 = 14,
    dead2 = 13,
    dead1 = 12,
    rock = 11,
    bigshot = 10,
    shot = 9,
    wallhit = 8,
    turbogre = 7,
    dragon = 6,
    gargoyle = 5,
    ogre = 4,
    skeleton = 3,
    goblin = 2,
    player = 1,
    nothing = 0,
}

impl TryFrom<u16> for classtype {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        FromPrimitive::from_u16(value).ok_or(())
    }
}
