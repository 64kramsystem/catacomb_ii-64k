// Macros hardcoded by the transpiler.

pub(crate) fn SDL_BUTTON(x: u8) -> libc::c_int {
    1 << (x - 1)
}
