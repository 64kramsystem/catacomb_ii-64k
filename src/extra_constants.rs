// Constants hardcoded by the transpiler.

pub(crate) const SDL_BUTTON_LEFT: u8 = 1;
pub(crate) const SDL_BUTTON_RIGHT: u8 = 3;

pub const O_BINARY: i32 = 0;

pub const NUM_DEMOS: u16 = 1;

pub const PC_BASE_TIMER: u32 = 1193181;

pub const maxpics: usize = 2047;
// pub const NUMTILES: usize = 24 * 24; /*number of tiles displayed on screen*/
pub const numlevels: i16 = 30;
// pub const maxobj: usize = 200; /*maximum possible active objects*/
pub const solidwall: i32 = 129;
pub const blankfloor: usize = 128;
pub const leftoff: usize = 11;
pub const topoff: usize = 11;
pub const tile2s: usize = 256; /*tile number where the 2*2 pictures start*/
// pub const tile3s: usize = tile2s + 67 * 4;
// pub const tile4s: usize = tile3s + 35 * 9;
// pub const tile5s: usize = tile4s + 19 * 16;
// pub const lasttile: usize = tile5s + 19 * 25;

pub const _extension: *const i8 = b"CA2\0" as *const _ as *const i8;
