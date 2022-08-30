// Safe wrappers around unsafe SDL calls and related routines. This will also make it easy to use a
// Rust media library.

use std::ffi::CStr;

use crate::pcrlib_a::SDL_AudioSpec;

extern "C" {
    fn SDL_GetError() -> *const i8;
    fn SDL_OpenAudioDevice(
        device: *const i8,
        iscapture: i32,
        desired: *const SDL_AudioSpec,
        obtained: *mut SDL_AudioSpec,
        allowed_changes: i32,
    ) -> u32;
    fn SDL_PauseAudioDevice(dev: u32, pause_on: i32);
}

// //////////////////////////////////////////////////////////////////////////////////////////////////
// DIRECT SDL APIS
// //////////////////////////////////////////////////////////////////////////////////////////////////

pub fn safe_SDL_GetError() -> String {
    unsafe {
        let raw_str = SDL_GetError();
        // Assume that the string is valid UTF-8
        CStr::from_ptr(raw_str).to_string_lossy().to_string()
    }
}

pub fn safe_SDL_OpenAudioDevice(
    device: *const i8,
    iscapture: i32,
    desired: *const SDL_AudioSpec,
    obtained: *mut SDL_AudioSpec,
    allowed_changes: i32,
) -> u32 {
    unsafe { SDL_OpenAudioDevice(device, iscapture, desired, obtained, allowed_changes) }
}

pub fn safe_SDL_PauseAudioDevice(dev: u32, pause_on: i32) {
    unsafe { SDL_PauseAudioDevice(dev, pause_on) }
}
