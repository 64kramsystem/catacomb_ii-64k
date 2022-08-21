use crate::cpanel::farptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct picfiletype {
    pub charptr: farptr,
    pub tileptr: farptr, // Rust port: this is actually unused
    pub picptr: farptr,
    pub spriteptr: farptr,
    pub pictableptr: farptr,
    pub spritetableptr: farptr,
    pub plane: [farptr; 4],
    pub numchars: i16,
    pub numtiles: i16,
    pub numpics: i16,
    pub numsprites: i16,
}
