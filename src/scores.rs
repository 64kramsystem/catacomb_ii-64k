#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct scores {
    pub score: i32,
    pub level: i16,
    pub initials: [u8; 3], // Rust port: 3-letters string; originally 4 chars (null terminated)
}
