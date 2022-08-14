// Macros hardcoded by the transpiler.

pub(crate) fn SDL_BUTTON(x: u8) -> i32 {
    1 << (x - 1)
}
