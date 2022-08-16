use crate::{
    extra_types::boolean,
    input_type::inputtype,
    pcrlib_c::SDL_Rect,
    safe_sdl::{SDL_Renderer, SDL_Texture, SDL_Window},
    scan_codes::SDL_Scancode,
};

// Globals previously belonging to pcrlib_c.rs.
//
pub struct PcrlibCState {
    /*
    Private
    */
    pub mouseEvent: boolean,
    pub demobuffer: [u8; 5000],
    pub demoptr: usize,
    pub democount: i32,
    pub lastdemoval: i32,
    pub lastkey: SDL_Scancode,
    pub window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    pub sdltexture: *mut SDL_Texture,
    pub updateRect: SDL_Rect,

    /*
    Public
     */
    pub playermode: [inputtype; 3],
    pub keydown: [boolean; 512],
    pub JoyXlow: [i32; 3],
    pub JoyXhigh: [i32; 3],
    pub JoyYlow: [i32; 3],
    pub JoyYhigh: [i32; 3],
    pub MouseSensitivity: i32,
    pub key: [i32; 8],
    pub keyB1: i32,
    pub keyB2: i32,
}
impl PcrlibCState {
    pub fn new(
        mouseEvent: boolean,
        demobuffer: [u8; 5000],
        demoptr: usize,
        democount: i32,
        lastdemoval: i32,
        lastkey: SDL_Scancode,
        window: *mut SDL_Window,
        renderer: *mut SDL_Renderer,
        sdltexture: *mut SDL_Texture,
        updateRect: SDL_Rect,
        playermode: [inputtype; 3],
        keydown: [boolean; 512],
        JoyXlow: [i32; 3],
        JoyXhigh: [i32; 3],
        JoyYlow: [i32; 3],
        JoyYhigh: [i32; 3],
        MouseSensitivity: i32,
        key: [i32; 8],
        keyB1: i32,
        keyB2: i32,
    ) -> Self {
        Self {
            mouseEvent,
            demobuffer,
            demoptr,
            democount,
            lastdemoval,
            lastkey,
            window,
            renderer,
            sdltexture,
            updateRect,
            playermode,
            keydown,
            JoyXlow,
            JoyXhigh,
            JoyYlow,
            JoyYhigh,
            MouseSensitivity,
            key,
            keyB1,
            keyB2,
        }
    }
}
