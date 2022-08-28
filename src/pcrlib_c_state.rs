use sdl2::{render::WindowCanvas, video::DisplayMode};

use crate::{
    gr_type::grtype::{self, *},
    input_type::inputtype::{self, *},
    pcrlib_c::{joyinfo_t, SDL_Rect},
    safe_sdl::SDL_Texture,
    scan_codes::{SDL_Scancode, SDL_SCANCODE_UNKNOWN},
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
    pub keydown: [bool; 512],
    pub JoyXlow: [i32; 3],
    pub JoyXhigh: [i32; 3],
    pub JoyYlow: [i32; 3],
    pub JoyYhigh: [i32; 3],
    pub MouseSensitivity: i32,
    pub key: [u32; 8],
    pub keyB1: u32,
    pub keyB2: u32,
    pub grmode: grtype,
    pub picfile_data: Vec<u8>,        // Rust port: Added
    pub charptr: usize,               // 8*8 tileset; Rust port: refers to `picfile`
    // pub tileptr: usize,            // 16*16 tileset; Rust port: unused
    pub picptr: usize,                // any size picture set; Rust port: refers to `picfile`
    // pub spriteptr: usize,          // any size masked and hit rect sprites; Rust port: unused
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

    pub mouseEvent: bool,
    pub demobuffer: [u8; 5000],
    pub demoptr: usize,
    pub democount: i32,
    pub lastdemoval: i32,
    pub lastkey: SDL_Scancode,
    // pub window: Window, // Rust port: not needed, as we can get the ref from the renderer
    pub renderer: WindowCanvas,
    pub sdltexture: *mut SDL_Texture,
    pub updateRect: SDL_Rect,
    pub mode: DisplayMode,
    pub joystick: [joyinfo_t; 3],
    pub hasFocus: bool,
    pub win_xl: i32,
    pub win_yl: i32,
    pub win_xh: i32,
    pub win_yh: i32,
    pub conv: [u32; 64000],
}

impl PcrlibCState {
    pub fn new(
        // mouseEvent: bool,
        // demobuffer: [u8; 5000],
        // demoptr: usize,
        // democount: i32,
        // lastdemoval: i32,
        // lastkey: SDL_Scancode,
        renderer: WindowCanvas,
        sdltexture: *mut SDL_Texture,
        updateRect: SDL_Rect,
        // playermode: [inputtype; 3],
        // keydown: [bool; 512],
        // JoyXlow: [i32; 3],
        // JoyXhigh: [i32; 3],
        // JoyYlow: [i32; 3],
        // JoyYhigh: [i32; 3],
        // MouseSensitivity: i32,
        // key: [u32; 8],
        // keyB1: u32,
        // keyB2: u32,
        mode: DisplayMode,
        joystick: [joyinfo_t; 3],
        // hasFocus: bool,
        // win_xl: i32,
        // win_yl: i32,
        // win_xh: i32,
        // win_yh: i32,
        // conv: [u32; 64000],
        // grmode: grtype,
        // picfile: Vec<u8>,
        // charptr: usize,
        // picptr: usize,
        // egaplaneofs: [u32; 4],
        // sx: i32,
        // sy: i32,
        // leftedge: i32,
        // highscores: [scores; 5],
        // score: i32,
        // level: i16,
    ) -> Self {
        Self {
            mouseEvent: false,
            demobuffer: [0; 5000],
            demoptr: 0,
            democount: 0,
            lastdemoval: 0,
            lastkey: SDL_SCANCODE_UNKNOWN,
            renderer,
            sdltexture,
            updateRect,
            playermode: [keyboard, keyboard, joystick1],
            keydown: [false; 512],
            JoyXlow: [0; 3],
            JoyXhigh: [0; 3],
            JoyYlow: [0; 3],
            JoyYhigh: [0; 3],
            MouseSensitivity: 0,
            key: [0; 8],
            keyB1: 0,
            keyB2: 0,
            mode,
            joystick,
            hasFocus: true,
            win_xl: 0,
            win_yl: 0,
            win_xh: 0,
            win_yh: 0,
            conv: [0; 64000],
            grmode: text,
            picfile_data: vec![],
            charptr: usize::MAX,
            picptr: usize::MAX,
            egaplaneofs: [0; 4],
            sx: 0,
            sy: 0,
            leftedge: 0,
            highscores: [scores {
                score: 0,
                level: 0,
                initials: [0; 3],
            }; 5],
            score: 0,
            level: 0,
        }
    }
}
