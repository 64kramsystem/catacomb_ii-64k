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

impl Default for SPKRtable {
    fn default() -> Self {
        Self {
            id: Default::default(),
            filelength: Default::default(),
            filler: Default::default(),
            sounds: [spksndtype::default(); 63],
            freqdata: Default::default(),
        }
    }
}
