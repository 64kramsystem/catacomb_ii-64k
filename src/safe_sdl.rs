// Safe wrappers around unsafe SDL calls and related routines. This will also make it easy to use a
// Rust media library.

use std::ffi::CStr;

use crate::{pcrlib_a::SDL_AudioSpec, pcrlib_c::*};

extern "C" {
    pub type _SDL_Joystick;
    pub type _SDL_GameController;
    pub type SDL_semaphore;

    fn SDL_IsGameController(joystick_index: i32) -> SDL_bool;
    fn SDL_GameControllerOpen(joystick_index: i32) -> *mut SDL_GameController;
    fn SDL_GameControllerGetAxis(
        gamecontroller: *mut SDL_GameController,
        axis: SDL_GameControllerAxis,
    ) -> i16;
    fn SDL_GameControllerGetButton(
        gamecontroller: *mut SDL_GameController,
        button: SDL_GameControllerButton,
    ) -> u8;
    fn SDL_GameControllerClose(gamecontroller: *mut SDL_GameController);
    fn SDL_GetError() -> *const i8;
    fn SDL_OpenAudioDevice(
        device: *const i8,
        iscapture: i32,
        desired: *const SDL_AudioSpec,
        obtained: *mut SDL_AudioSpec,
        allowed_changes: i32,
    ) -> u32;
    fn SDL_PauseAudioDevice(dev: u32, pause_on: i32);
    fn SDL_CloseAudio();
}

// //////////////////////////////////////////////////////////////////////////////////////////////////
// DIRECT SDL APIS
// //////////////////////////////////////////////////////////////////////////////////////////////////

pub fn safe_SDL_IsGameController(joystick_index: i32) -> SDL_bool {
    unsafe { SDL_IsGameController(joystick_index) }
}

pub fn safe_SDL_GameControllerOpen(joystick_index: i32) -> *mut SDL_GameController {
    unsafe { SDL_GameControllerOpen(joystick_index) }
}

pub fn safe_SDL_GameControllerGetAxis(
    gamecontroller: *mut SDL_GameController,
    axis: SDL_GameControllerAxis,
) -> i16 {
    unsafe { SDL_GameControllerGetAxis(gamecontroller, axis) }
}

pub fn safe_SDL_GameControllerGetButton(
    gamecontroller: *mut SDL_GameController,
    button: SDL_GameControllerButton,
) -> u8 {
    unsafe { SDL_GameControllerGetButton(gamecontroller, button) }
}

pub fn safe_SDL_GameControllerClose(gamecontroller: *mut SDL_GameController) {
    unsafe { SDL_GameControllerClose(gamecontroller) }
}

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

pub fn safe_SDL_CloseAudio() {
    unsafe { SDL_CloseAudio() }
}
