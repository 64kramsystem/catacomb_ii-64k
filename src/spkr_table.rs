use crate::spksndtype::spksndtype;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SPKRtable {
    pub id: [i8; 4],
    pub filelength: u16,
    pub filler: [u16; 5],
    pub sounds: [spksndtype; 63],
    pub freqdata: [u16; 0],
}
