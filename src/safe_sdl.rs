// Safe wrappers around unsafe SDL calls and related routines. This will also make it easy to use a
// Rust media library.

use std::ffi::CStr;

use crate::{
    pcrlib_a::{SDL_AudioSpec, SDL_TimerCallback},
    pcrlib_c::*,
    scan_codes::SDL_Scancode,
};

extern "C" {
    pub type SDL_Window;
    pub type _SDL_Joystick;
    pub type _SDL_GameController;
    pub type SDL_SysWMmsg;
    pub type SDL_Renderer;
    pub type SDL_Texture;
    pub type SDL_semaphore;

    fn SDL_Delay(ms: u32);
    fn SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer);
    fn SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> i32;
    fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const libc::c_void,
        pitch: i32,
    ) -> i32;
    fn SDL_PollEvent(event: *mut SDL_Event) -> i32;
    fn SDL_PumpEvents();
    fn SDL_DestroyWindow(window_0: *mut SDL_Window);
    fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
    fn SDL_GetMouseFocus() -> *mut SDL_Window;
    fn SDL_GetRelativeMouseState(x: *mut i32, y: *mut i32) -> u32;
    fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> i32;
    fn SDL_NumJoysticks() -> i32;
    fn SDL_JoystickOpen(device_index: i32) -> *mut SDL_Joystick;
    fn SDL_JoystickUpdate();
    fn SDL_JoystickGetAxis(joystick_0: *mut SDL_Joystick, axis: i32) -> i16;
    fn SDL_JoystickGetButton(joystick_0: *mut SDL_Joystick, button: i32) -> u8;
    fn SDL_JoystickClose(joystick_0: *mut SDL_Joystick);
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
    fn SDL_AddTimer(interval: u32, callback: SDL_TimerCallback, param: *mut libc::c_void) -> i32;
}

// //////////////////////////////////////////////////////////////////////////////////////////////////
// DIRECT SDL APIS
// //////////////////////////////////////////////////////////////////////////////////////////////////

pub fn safe_SDL_Delay(ms: u32) {
    unsafe { SDL_Delay(ms) }
}

pub fn safe_SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer) {
    unsafe { SDL_DestroyRenderer(renderer_0) }
}

pub fn safe_SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> i32 {
    unsafe { SDL_RenderClear(renderer_0) }
}

pub fn safe_SDL_UpdateTexture(
    texture: *mut SDL_Texture,
    rect: *const SDL_Rect,
    pixels: *const libc::c_void,
    pitch: i32,
) -> i32 {
    unsafe { SDL_UpdateTexture(texture, rect, pixels, pitch) }
}

pub fn safe_SDL_PollEvent(event: *mut SDL_Event) -> i32 {
    unsafe { SDL_PollEvent(event) }
}

pub fn safe_SDL_PumpEvents() {
    unsafe { SDL_PumpEvents() }
}

pub fn safe_SDL_DestroyWindow(window_0: *mut SDL_Window) {
    unsafe { SDL_DestroyWindow(window_0) }
}

pub fn safe_SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode {
    unsafe { SDL_GetKeyFromScancode(scancode) }
}

pub fn safe_SDL_GetMouseFocus() -> *mut SDL_Window {
    unsafe { SDL_GetMouseFocus() }
}

pub fn safe_SDL_GetRelativeMouseState(x: *mut i32, y: *mut i32) -> u32 {
    unsafe { SDL_GetRelativeMouseState(x, y) }
}

pub fn safe_SDL_SetRelativeMouseMode(enabled: SDL_bool) -> i32 {
    unsafe { SDL_SetRelativeMouseMode(enabled) }
}

pub fn safe_SDL_NumJoysticks() -> i32 {
    unsafe { SDL_NumJoysticks() }
}

pub fn safe_SDL_JoystickOpen(device_index: i32) -> *mut SDL_Joystick {
    unsafe { SDL_JoystickOpen(device_index) }
}

pub fn safe_SDL_JoystickUpdate() {
    unsafe { SDL_JoystickUpdate() }
}

pub fn safe_SDL_JoystickGetAxis(joystick_0: *mut SDL_Joystick, axis: i32) -> i16 {
    unsafe { SDL_JoystickGetAxis(joystick_0, axis) }
}

pub fn safe_SDL_JoystickGetButton(joystick_0: *mut SDL_Joystick, button: i32) -> u8 {
    unsafe { SDL_JoystickGetButton(joystick_0, button) }
}

pub fn safe_SDL_JoystickClose(joystick_0: *mut SDL_Joystick) {
    unsafe { SDL_JoystickClose(joystick_0) }
}

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

pub fn safe_SDL_AddTimer(
    interval: u32,
    callback: SDL_TimerCallback,
    param: *mut libc::c_void,
) -> i32 {
    unsafe { SDL_AddTimer(interval, callback, param) }
}
