#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct scores {
    pub score: i32,
    pub level: i16,
    pub initials: [i8; 4],
}
