use std::{convert::TryInto, io::Read};

use serdine::derive::Deserialize;

// Rust port: WATCH OUT! On disk, this has a different structure (see #initials and related methods).
#[derive(Copy, Clone, Deserialize)]
#[repr(C, packed)]
pub struct scores {
    pub score: i32,
    pub level: i16,
    #[deserialize = "deserialize_initials"]
    pub initials: [u8; 3],
}

fn deserialize_initials<R: Read>(mut r: R) -> [u8; 3] {
    let mut buffer = [0; 4];
    r.read_exact(&mut buffer).unwrap();
    buffer[0..3].try_into().unwrap()
}

impl scores {
    pub const fn ondisk_struct_size() -> usize {
        10
    }
}
