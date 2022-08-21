use crate::{
    extra_types::boolean,
    gr_type::grtype,
    input_type::inputtype,
    pcrlib_c::{joyinfo_t, SDL_DisplayMode, SDL_Rect},
    safe_sdl::{SDL_Renderer, SDL_Texture, SDL_Window},
    scan_codes::SDL_Scancode,
    scores::scores,
};

// Globals previously belonging to pcrlib_c.rs.
//
#[rustfmt::skip]
pub struct PcrlibCState {
    // //////////////////////////////////////////////////////////
    // Rust port: shared
    // //////////////////////////////////////////////////////////

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
    pub grmode: grtype,
    pub picfile: Vec<u8>,             // Rust port: Added
    pub charptr: usize,               // 8*8 tileset; Rust port: refers to `picfile`
    // pub tileptr: usize,            // 16*16 tileset; Rust port: unused
    pub picptr: *mut libc::c_void,    // any size picture set
    pub spriteptr: *mut libc::c_void, // any size masked and hit rect sprites
    pub egaplaneofs: [u32; 4],
    pub sx: i32,
    pub sy: i32,
    pub leftedge: i32,
    pub highscores: [scores; 5],
    pub score: i32,
    pub level: i16,

    // //////////////////////////////////////////////////////////
    // Rust port: private to pcrlib_c.rs
    // //////////////////////////////////////////////////////////

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
    pub mode: SDL_DisplayMode,
    pub joystick: [joyinfo_t; 3],
    pub hasFocus: boolean,
    pub win_xl: i32,
    pub win_yl: i32,
    pub win_xh: i32,
    pub win_yh: i32,
    pub conv: [u32; 64000],
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
        mode: SDL_DisplayMode,
        joystick: [joyinfo_t; 3],
        hasFocus: boolean,
        win_xl: i32,
        win_yl: i32,
        win_xh: i32,
        win_yh: i32,
        conv: [u32; 64000],
        grmode: grtype,
        picfile: Vec<u8>,
        charptr: usize,
        picptr: *mut libc::c_void,
        spriteptr: *mut libc::c_void,
        egaplaneofs: [u32; 4],
        sx: i32,
        sy: i32,
        leftedge: i32,
        highscores: [scores; 5],
        score: i32,
        level: i16,
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
            mode,
            joystick,
            hasFocus,
            win_xl,
            win_yl,
            win_xh,
            win_yh,
            conv,
            grmode,
            picfile,
            charptr,
            picptr,
            spriteptr,
            egaplaneofs,
            sx,
            sy,
            leftedge,
            highscores,
            score,
            level,
        }
    }
}
