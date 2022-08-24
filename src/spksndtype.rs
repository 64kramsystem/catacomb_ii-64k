#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
pub struct spksndtype {
    pub start: u16,
    pub priority: u8,
    pub samplerate: u8,
    pub name: [i8; 12],
}
