use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::{fs, mem, ptr};

use ::libc;
use libc::O_RDONLY;

use crate::{
    catacomb::loadgrfiles,
    catasm::drawchartile,
    control_struct::ControlStruct,
    demo_enum::demoenum,
    dir_type::dirtype::*,
    extra_constants::{port_temp__extension, O_BINARY, SDL_BUTTON_LEFT, SDL_BUTTON_RIGHT},
    extra_macros::SDL_BUTTON,
    extra_types::boolean,
    global_state::GlobalState,
    gr_type::grtype::{self, *},
    indemo,
    pcrlib_a::{
        drawchar, initrnd, initrndt, soundmode, PlaySound, SetupEmulatedVBL, ShutdownSound,
        SoundData, StartupSound, WaitVBL,
    },
    safe_sdl::*,
    scores::scores,
    sdl_scan_codes::*,
    spkr_table::SPKRtable,
};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn SDL_Quit();
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: u64) -> i64;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn strlen(_: *const i8) -> u64;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

pub type SDL_bool = u32;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;

const SDL_PIXELFORMAT_ARGB8888: u32 = 372645892;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayMode {
    pub format: u32,
    pub w: i32,
    pub h: i32,
    pub refresh_rate: i32,
    pub driverdata: *mut libc::c_void,
}

pub const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = 4097;

pub type SDL_Keycode = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub mod_0: u16,
    pub unused: u32,
}

pub type SDL_Joystick = _SDL_Joystick;
pub type SDL_JoystickID = i32;
pub type SDL_GameController = _SDL_GameController;
pub type SDL_GameControllerAxis = i32;
pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
pub type SDL_GameControllerButton = i32;
pub const SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = 1;
pub const SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = 0;
pub type SDL_TouchID = i64;
pub type SDL_FingerID = i64;
pub type SDL_GestureID = i64;
pub type C2RustUnnamed_3 = u32;
pub const SDL_MOUSEMOTION: C2RustUnnamed_3 = 1024;
pub const SDL_KEYUP: C2RustUnnamed_3 = 769;
pub const SDL_KEYDOWN: C2RustUnnamed_3 = 768;
pub const SDL_WINDOWEVENT: C2RustUnnamed_3 = 512;
pub const SDL_QUIT: C2RustUnnamed_3 = 256;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_CommonEvent {
    pub type_0: u32,
    pub timestamp: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub display: u32,
    pub event: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub data1: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub event: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub data1: i32,
    pub data2: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub state: u8,
    pub repeat: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub text: [i8; 32],
    pub start: i32,
    pub length: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub text: [i8; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub state: u32,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub button: u8,
    pub state: u8,
    pub clicks: u8,
    pub padding1: u8,
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub which: u32,
    pub x: i32,
    pub y: i32,
    pub direction: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub ball: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub xrel: i16,
    pub yrel: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub hat: u8,
    pub value: u8,
    pub padding1: u8,
    pub padding2: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub button: u8,
    pub state: u8,
    pub padding1: u8,
    pub padding2: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub axis: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub value: i16,
    pub padding4: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: SDL_JoystickID,
    pub button: u8,
    pub state: u8,
    pub padding1: u8,
    pub padding2: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: u32,
    pub iscapture: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub touchId: SDL_TouchID,
    pub fingerId: SDL_FingerID,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub pressure: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub touchId: SDL_TouchID,
    pub dTheta: libc::c_float,
    pub dDist: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub numFingers: u16,
    pub padding: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: u32,
    pub error: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub file: *mut i8,
    pub windowID: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub which: i32,
    pub data: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: u32,
    pub timestamp: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub windowID: u32,
    pub code: i32,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: u32,
    pub timestamp: u32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: u32,
    pub common: SDL_CommonEvent,
    pub display: SDL_DisplayEvent,
    pub window: SDL_WindowEvent,
    pub key: SDL_KeyboardEvent,
    pub edit: SDL_TextEditingEvent,
    pub text: SDL_TextInputEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub wheel: SDL_MouseWheelEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub jdevice: SDL_JoyDeviceEvent,
    pub caxis: SDL_ControllerAxisEvent,
    pub cbutton: SDL_ControllerButtonEvent,
    pub cdevice: SDL_ControllerDeviceEvent,
    pub adevice: SDL_AudioDeviceEvent,
    pub sensor: SDL_SensorEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
    pub tfinger: SDL_TouchFingerEvent,
    pub mgesture: SDL_MultiGestureEvent,
    pub dgesture: SDL_DollarGestureEvent,
    pub drop: SDL_DropEvent,
    pub padding: [u8; 56],
}
pub type SDL_EventFilter = Option<unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> i32>;
pub type C2RustUnnamed_4 = u32;
pub const SDL_TEXTUREACCESS_TARGET: C2RustUnnamed_4 = 2;
pub const SDL_TEXTUREACCESS_STREAMING: C2RustUnnamed_4 = 1;
pub const SDL_TEXTUREACCESS_STATIC: C2RustUnnamed_4 = 0;
pub type soundtype = u32;
pub const sdlib: soundtype = 2;
pub const spkr: soundtype = 1;
pub const off: soundtype = 0;

pub type inputtype = u32;
pub const demo: inputtype = 4;
pub const joystick2: inputtype = 3;
pub const joystick1: inputtype = 2;
pub const mouse: inputtype = 1;
pub const keyboard: inputtype = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct joyinfo_t {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub device: i32,
    pub isgamecontroller: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub controller: *mut SDL_GameController,
    pub joy: *mut SDL_Joystick,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ctlpaneltype {
    pub grmode: grtype,
    pub soundmode: u16,
    pub playermode: [u16; 3],
    pub JoyXlow: [i16; 3],
    pub JoyYlow: [i16; 3],
    pub JoyXhigh: [i16; 3],
    pub JoyYhigh: [i16; 3],
    pub MouseSensitivity: i16,
    pub key: [u8; 8],
    pub keyB1: u8,
    pub keyB2: u8,
}

pub static mut ch: i8 = 0;
pub static mut playermode: [inputtype; 3] = [keyboard, keyboard, joystick1];
pub static mut keydown: [boolean; 512] = [0; 512];
pub static mut JoyXlow: [i32; 3] = [0; 3];
pub static mut JoyXhigh: [i32; 3] = [0; 3];
pub static mut JoyYlow: [i32; 3] = [0; 3];
pub static mut JoyYhigh: [i32; 3] = [0; 3];
pub static mut MouseSensitivity: i32 = 0;
static mut mouseEvent: boolean = 0;
pub static mut key: [i32; 8] = [0; 8];
pub static mut keyB1: i32 = 0;
pub static mut keyB2: i32 = 0;
static mut demobuffer: [i8; 5000] = [0; 5000];
static mut demoptr: *mut i8 = 0 as *const i8 as *mut i8;
static mut democount: i32 = 0;
static mut lastdemoval: i32 = 0;
static mut lastkey: SDL_Scancode = SDL_SCANCODE_UNKNOWN;
static mut window: *mut SDL_Window = 0 as *const SDL_Window as *mut SDL_Window;
static mut renderer: *mut SDL_Renderer = 0 as *const SDL_Renderer as *mut SDL_Renderer;
static mut sdltexture: *mut SDL_Texture = 0 as *const SDL_Texture as *mut SDL_Texture;
static mut updateRect: SDL_Rect = SDL_Rect {
    x: 0,
    y: 0,
    w: 0,
    h: 0,
};
#[no_mangle]
pub static mut mode: SDL_DisplayMode = SDL_DisplayMode {
    format: 0,
    w: 0,
    h: 0,
    refresh_rate: 0,
    driverdata: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut joystick: [joyinfo_t; 3] = [joyinfo_t {
    c2rust_unnamed: C2RustUnnamed_5 {
        controller: 0 as *const SDL_GameController as *mut SDL_GameController,
    },
    device: 0,
    isgamecontroller: 0,
}; 3];

pub unsafe fn SetupKBD() {
    let mut i: u32 = 0;
    i = 0;
    while i < 128 {
        keydown[i as usize] = false as boolean;
        i = i.wrapping_add(1);
    }
}

pub unsafe fn ProcessEvents() {
    mouseEvent = false as boolean;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as i32 as u32 {
            keydown[event.key.keysym.scancode as usize] = true as boolean;
            lastkey = event.key.keysym.scancode;
        } else if event.type_0 == SDL_KEYUP as i32 as u32 {
            keydown[event.key.keysym.scancode as usize] = false as boolean;
        } else if event.type_0 == SDL_MOUSEMOTION as i32 as u32 {
            mouseEvent = true as boolean;
        }
    }
}

static mut hasFocus: boolean = true as boolean;

unsafe extern "C" fn WatchUIEvents(
    mut _userdata: *mut libc::c_void,
    mut event: *mut SDL_Event,
) -> i32 {
    if (*event).type_0 == SDL_QUIT as i32 as u32 {
        _quit(b"\0" as *const u8 as *const i8 as *mut i8);
    } else if (*event).type_0 == SDL_WINDOWEVENT as i32 as u32 {
        match (*event).window.event as i32 {
            13 => {
                hasFocus = false as boolean;
                CheckMouseMode();
            }
            12 => {
                while safe_SDL_GetMouseFocus() != window {
                    safe_SDL_PumpEvents();
                    safe_SDL_Delay(10);
                }
                hasFocus = true as boolean;
                CheckMouseMode();
            }
            _ => {}
        }
    }
    return 0;
}

pub unsafe fn ControlKBD() -> ControlStruct {
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    if keydown[key[north as i32 as usize] as usize] != 0 {
        ymove = -(1);
    }
    if keydown[key[east as i32 as usize] as usize] != 0 {
        xmove = 1;
    }
    if keydown[key[south as i32 as usize] as usize] != 0 {
        ymove = 1;
    }
    if keydown[key[west as i32 as usize] as usize] != 0 {
        xmove = -(1);
    }
    if keydown[key[northeast as i32 as usize] as usize] != 0 {
        ymove = -(1);
        xmove = 1;
    }
    if keydown[key[northwest as i32 as usize] as usize] != 0 {
        ymove = -(1);
        xmove = -(1);
    }
    if keydown[key[southeast as i32 as usize] as usize] != 0 {
        ymove = 1;
        xmove = 1;
    }
    if keydown[key[southwest as i32 as usize] as usize] != 0 {
        ymove = 1;
        xmove = -(1);
    }
    match ymove * 3 + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    action.button1 = keydown[keyB1 as usize];
    action.button2 = keydown[keyB2 as usize];
    return action;
}

pub unsafe fn ControlMouse() -> ControlStruct {
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut buttons: i32 = safe_SDL_GetRelativeMouseState(&mut newx, &mut newy) as i32;
    action.button1 = (buttons & SDL_BUTTON(SDL_BUTTON_LEFT)) as boolean;
    action.button2 = (buttons & SDL_BUTTON(SDL_BUTTON_RIGHT)) as boolean;
    if mouseEvent as i32 == false as i32 {
        action.dir = nodir;
        return action;
    }
    if newx > MouseSensitivity {
        xmove = 1;
    } else if newx < -MouseSensitivity {
        xmove = -(1);
    }
    if newy > MouseSensitivity {
        ymove = 1;
    } else if newy < -MouseSensitivity {
        ymove = -(1);
    }
    match ymove * 3 + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    return action;
}

unsafe fn ShutdownJoysticks() {
    let mut j: u32 = 0;
    j = 1;
    while j < 3 {
        if !(joystick[j as usize].device < 0) {
            if joystick[j as usize].isgamecontroller != 0 {
                safe_SDL_GameControllerClose(joystick[j as usize].c2rust_unnamed.controller);
            } else {
                safe_SDL_JoystickClose(joystick[j as usize].c2rust_unnamed.joy);
            }
            joystick[j as usize].device = -(1);
        }
        j = j.wrapping_add(1);
    }
}

pub unsafe fn ProbeJoysticks() {
    let mut j: i32 = 0;
    if joystick[1].device > 0 || joystick[2].device > 0 {
        ShutdownJoysticks();
    }
    j = 1;
    while j < 3 {
        if j - 1 >= safe_SDL_NumJoysticks() {
            joystick[j as usize].device = -(1);
        } else {
            joystick[j as usize].device = j - 1;
            joystick[j as usize].isgamecontroller = safe_SDL_IsGameController(j - 1) as boolean;
            if safe_SDL_IsGameController(j - 1) as u64 != 0 {
                joystick[j as usize].c2rust_unnamed.controller = safe_SDL_GameControllerOpen(j - 1);
            } else {
                joystick[j as usize].c2rust_unnamed.joy = safe_SDL_JoystickOpen(j - 1);
            }
        }
        j += 1;
    }
}

pub unsafe fn ReadJoystick(mut joynum: i32, mut xcount: *mut i32, mut ycount: *mut i32) {
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    *xcount = 0;
    *ycount = 0;
    safe_SDL_JoystickUpdate();
    if joystick[joynum as usize].isgamecontroller != 0 {
        a1 = safe_SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as i32;
        a2 = safe_SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as i32;
    } else {
        a1 = safe_SDL_JoystickGetAxis(joystick[joynum as usize].c2rust_unnamed.joy, 0) as i32;
        a2 = safe_SDL_JoystickGetAxis(joystick[joynum as usize].c2rust_unnamed.joy, 1) as i32;
    }
    *xcount = a1;
    *ycount = a2;
}

pub unsafe fn ControlJoystick(mut joynum: i32) -> ControlStruct {
    let mut joyx: i32 = 0;
    let mut joyy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    ReadJoystick(joynum, &mut joyx, &mut joyy);
    if joystick[joynum as usize].isgamecontroller != 0 {
        action.button1 = (safe_SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_A,
        ) as i32
            != 0) as i32 as boolean;
        action.button2 = (safe_SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_B,
        ) as i32
            != 0) as i32 as boolean;
    } else {
        action.button1 =
            (safe_SDL_JoystickGetButton(joystick[joynum as usize].c2rust_unnamed.joy, 0) as i32
                != 0) as i32 as boolean;
        action.button2 =
            (safe_SDL_JoystickGetButton(joystick[joynum as usize].c2rust_unnamed.joy, 1) as i32
                != 0) as i32 as boolean;
    }
    if joyx == 0 && joyy == 0 {
        action.dir = nodir;
        return action;
    }
    if joyx > JoyXhigh[joynum as usize] {
        xmove = 1;
    } else if joyx < JoyXlow[joynum as usize] {
        xmove = -(1);
    }
    if joyy > JoyYhigh[joynum as usize] {
        ymove = 1;
    } else if joyy < JoyYlow[joynum as usize] {
        ymove = -(1);
    }
    match ymove * 3 + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    return action;
}

pub unsafe fn ControlPlayer(mut player: i32) -> ControlStruct {
    let mut ret: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut val: i32 = 0;
    ProcessEvents();
    if indemo == demoenum::notdemo || indemo == demoenum::recording {
        match playermode[player as usize] as u32 {
            1 => {
                ret = ControlMouse();
            }
            2 => {
                ret = ControlJoystick(1);
            }
            3 => {
                ret = ControlJoystick(2);
            }
            0 | _ => {
                ret = ControlKBD();
            }
        }
        if indemo == demoenum::recording {
            val = ((ret.dir as u32) << 2 | ((ret.button2 as i32) << 1) as u32 | ret.button1 as u32)
                as i32;
            let fresh0 = demoptr;
            demoptr = demoptr.offset(1);
            *fresh0 = val as i8;
        }
    } else {
        let fresh1 = demoptr;
        demoptr = demoptr.offset(1);
        val = *fresh1 as i32;
        ret.button1 = (val & 1) as boolean;
        ret.button2 = ((val & 2) >> 1) as boolean;
        ret.dir = ((val & 4 + 8 + 16 + 32) >> 2).into();
    }
    return ret;
}

pub unsafe fn RecordDemo() {
    demobuffer[0] = level as i8;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1) as *mut i8;
    indemo = demoenum::recording;
}

pub fn LoadDemo(mut demonum: i32) {
    let filename = format!("DEMO{demonum}.{port_temp__extension}");
    let mut temp_port_demobuffer = [0; 5000];
    port_temp_LoadFile(&filename, &mut temp_port_demobuffer);
    unsafe {
        demobuffer.copy_from_slice(&temp_port_demobuffer.map(|b| b as i8));
        level = demobuffer[0] as i16;
        demoptr = &mut *demobuffer.as_mut_ptr().offset(1) as *mut i8;
        indemo = demoenum::demoplay;
    }
}

pub unsafe fn SaveDemo(mut demonum: i32) {
    let str = CString::new(format!("DEMO{demonum}.{port_temp__extension}")).unwrap();

    SaveFile(
        str.as_ptr(),
        demobuffer.as_mut_ptr(),
        demoptr.offset_from(&mut *demobuffer.as_mut_ptr().offset(0) as *mut i8) as i64,
    );
    indemo = demoenum::notdemo;
}

pub unsafe fn clearkeys() {
    let mut i: i32 = 0;
    while bioskey(1) != 0 {
        bioskey(0);
    }
    i = 0;
    while i < 128 {
        keydown[i as usize] = 0;
        i += 1;
    }
}

unsafe fn filelength(mut fd: i32) -> i64 {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &mut s) != 0 {
        return -(1) as i64;
    }
    return s.st_size;
}

pub unsafe fn LoadFile(mut filename: *const i8, mut buffer: *const i8) -> u64 {
    let mut fd: i32 = 0;
    fd = open(filename, 0o400 as i32);
    if fd < 0 {
        return 0;
    }
    let mut len: i64 = filelength(fd);
    let mut bytesRead: i64 = read(fd, buffer as *mut libc::c_void, len as u64);
    close(fd);
    return bytesRead as u64;
}

/// Using a Vec as dest buffer would be more convenient and idiomatic, however, routines may rely on
/// a certain buffer length.
/// An alternative is to pass the intended destination length, but there isn't a significant difference.
pub fn port_temp_LoadFile(filename: &str, dest: &mut [u8]) -> usize {
    if let Ok(mut file) = File::open(filename) {
        let mut buffer = Vec::new();
        let bytes_read = file.read_to_end(&mut buffer).unwrap();
        dest[..bytes_read].copy_from_slice(&buffer[..bytes_read]);
        bytes_read
    } else {
        0
    }
}

pub unsafe fn SaveFile(mut filename: *const i8, mut buffer: *const i8, mut size: i64) {
    let mut fd: i32 = 0;
    fd = open(
        filename,
        0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
        0o400 as i32 | 0o200 as i32,
    );
    if fd < 0 {
        return;
    }
    write(fd, buffer as *const libc::c_void, size as u64);
    close(fd);
}

pub fn bloadin(filename: &str) -> *const u8 {
    let file_meta = fs::metadata(filename);

    if let Ok(file_meta) = file_meta {
        let mut buffer = vec![0; file_meta.len() as usize];

        port_temp_LoadFile(filename, &mut buffer);

        let buffer_p = buffer.as_mut_ptr();

        mem::forget(buffer);

        buffer_p
    } else {
        ptr::null_mut()
    }
}

pub static mut grmode: grtype = text;
pub static mut charptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut tileptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut picptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut spriteptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut egaplaneofs: [u32; 4] = [0; 4];
pub static mut sx: i32 = 0;
pub static mut sy: i32 = 0;
pub static mut leftedge: i32 = 0;
static mut win_xl: i32 = 0;
static mut win_yl: i32 = 0;
static mut win_xh: i32 = 0;
static mut win_yh: i32 = 0;

pub unsafe fn drawwindow(mut xl: i32, mut yl: i32, mut xh: i32, mut yh: i32, gs: &mut GlobalState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    win_xl = xl;
    win_yl = yl;
    win_xh = xh;
    win_yh = yh;
    drawchar(xl, yl, 1, gs);
    x = xl + 1;
    while x < xh {
        drawchar(x, yl, 2, gs);
        x += 1;
    }
    drawchar(xh, yl, 3, gs);
    y = yl + 1;
    while y < yh {
        drawchar(xl, y, 4, gs);
        x = xl + 1;
        while x < xh {
            drawchar(x, y, ' ' as i32, gs);
            x += 1;
        }
        drawchar(xh, y, 5, gs);
        y += 1;
    }
    drawchar(xl, yh, 6, gs);
    x = xl + 1;
    while x < xh {
        drawchar(x, yh, 7, gs);
        x += 1;
    }
    drawchar(xh, yh, 8, gs);
    leftedge = xl + 1;
    sx = leftedge;
    sy = yl + 1;
}

pub unsafe fn bar(
    mut xl: i32,
    mut yl: i32,
    mut xh: i32,
    mut yh: i32,
    mut ch_0: i32,
    gs: &mut GlobalState,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    y = yl;
    while y <= yh {
        x = xl;
        while x <= xh {
            drawchar(x, y, ch_0, gs);
            x += 1;
        }
        y += 1;
    }
}

pub unsafe fn erasewindow(gs: &mut GlobalState) {
    bar(win_xl, win_yl, win_xh, win_yh, ' ' as i32, gs);
}

pub unsafe fn centerwindow(mut width: i32, mut height: i32, gs: &mut GlobalState) {
    let mut xl: i32 = gs.screencenter.x - width / 2;
    let mut yl: i32 = gs.screencenter.y - height / 2;
    drawwindow(xl, yl, xl + width + 1, yl + height + 1, gs);
}

pub unsafe fn expwin(mut width: i32, mut height: i32, gs: &mut GlobalState) {
    if width > 2 {
        if height > 2 {
            expwin(width - 2, height - 2, gs);
        } else {
            expwinh(width - 2, height, gs);
        }
    } else if height > 2 {
        expwinv(width, height - 2, gs);
    }
    UpdateScreen(gs);
    WaitVBL();
    centerwindow(width, height, gs);
}

unsafe fn expwinh(mut width: i32, mut height: i32, gs: &mut GlobalState) {
    if width > 2 {
        expwinh(width - 2, height, gs);
    }
    UpdateScreen(gs);
    WaitVBL();
    centerwindow(width, height, gs);
}

unsafe fn expwinv(mut width: i32, mut height: i32, gs: &mut GlobalState) {
    if height > 2 {
        expwinv(width, height - 2, gs);
    }
    UpdateScreen(gs);
    WaitVBL();
    centerwindow(width, height, gs);
}

pub unsafe fn bioskey(mut cmd: i32) -> i32 {
    if lastkey as u64 != 0 {
        let mut oldkey: i32 = lastkey as i32;
        if cmd != 1 {
            lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as i32 as u32 {
            if cmd == 1 {
                lastkey = event.key.keysym.scancode;
                return lastkey as i32;
            }
            return event.key.keysym.scancode as i32;
        }
    }
    return lastkey as i32;
}

pub unsafe fn UpdateScreen(gs: &mut GlobalState) {
    static mut EGAPalette: [u32; 16] = [
        0, 0xaa, 0xaa00, 0xaaaa, 0xaa0000, 0xaa00aa, 0xaa5500, 0xaaaaaa, 0x555555, 0x5555ff,
        0x55ff55, 0x55ffff, 0xff5555, 0xff55ff, 0xffff55, 0xffffff,
    ];
    static mut CGAPalette: [u32; 4] = [0, 0x55ffff, 0xff55ff, 0xffffff];
    static mut conv: [u32; 64000] = [0; 64000];
    let mut i: u64 = 0;
    if grmode as u32 == EGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            conv[i as usize] = EGAPalette[gs.screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if grmode as u32 == CGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            conv[i as usize] = CGAPalette[gs.screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else {
        panic!("VGA Palette conversion not implemented.");
    }
    safe_SDL_UpdateTexture(
        sdltexture,
        0 as *const SDL_Rect,
        conv.as_mut_ptr() as *const libc::c_void,
        (320 as i32 as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as i32,
    );
    safe_SDL_RenderClear(renderer);
    safe_SDL_RenderCopy(renderer, sdltexture, 0 as *const SDL_Rect, &mut updateRect);
    safe_SDL_RenderPresent(renderer);
}

pub unsafe fn get(gs: &mut GlobalState) -> i32 {
    let mut cycle: i32 = 0;
    let mut key_0: i32 = 0;
    loop {
        cycle = 9;
        loop {
            key_0 = bioskey(0);
            if !(key_0 == 0 && cycle < 13) {
                break;
            }
            let fresh2 = cycle;
            cycle = cycle + 1;
            drawchar(sx, sy, fresh2, gs);
            UpdateScreen(gs);
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
        }
        if !(key_0 == 0) {
            break;
        }
    }
    drawchar(sx, sy, ' ' as i32, gs);
    UpdateScreen(gs);
    return safe_SDL_GetKeyFromScancode(key_0 as SDL_Scancode);
}

pub unsafe fn print(mut str_0: *const i8, gs: &mut GlobalState) {
    let mut ch_0: i8 = 0;
    loop {
        let fresh3 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh3;
        if !(ch_0 as i32 != 0) {
            break;
        }
        if ch_0 as i32 == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as i32 == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh4 = sx;
            sx = sx + 1;
            drawchar(fresh4, sy, ch_0 as u8 as i32, gs);
        }
    }
}

pub unsafe fn printchartile(mut str_0: *const i8, gs: &mut GlobalState) {
    let mut ch_0: i8 = 0;
    loop {
        let fresh5 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh5;
        if !(ch_0 as i32 != 0) {
            break;
        }
        if ch_0 as i32 == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as i32 == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh6 = sx;
            sx = sx + 1;
            drawchartile(fresh6, sy, ch_0 as u8 as i32, gs);
        }
    }
}

pub unsafe fn printint(mut val: i32, gs: &mut GlobalState) {
    let str = CString::new(format!("{val}")).unwrap();
    print(str.as_ptr(), gs);
}

pub unsafe fn printlong(mut val: i64, gs: &mut GlobalState) {
    let str = CString::new(format!("{val}")).unwrap();
    print(str.as_ptr(), gs);
}

pub unsafe fn _Verify(mut filename: *const i8) -> i64 {
    let mut handle: i32 = 0;
    let mut size: i64 = 0;
    handle = open(filename, 0);
    if handle == -(1) {
        return 0;
    }
    size = filelength(handle);
    close(handle);
    return size;
}

unsafe fn _printhexb(mut value: libc::c_uchar, gs: &mut GlobalState) {
    let mut loop_0: i32 = 0;
    let mut hexstr: [i8; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [i8; 16]>(b"0123456789ABCDEF");
    let mut str_0: [i8; 2] = *::std::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"\0\0");
    loop_0 = 0;
    while loop_0 < 2 {
        str_0[0] = hexstr[(value as i32 >> (1 - loop_0) * 4 & 15) as usize];
        print(str_0.as_mut_ptr(), gs);
        loop_0 += 1;
    }
}

unsafe fn _printhex(mut value: u32, gs: &mut GlobalState) {
    print(b"$\0" as *const u8 as *const i8, gs);
    _printhexb((value >> 8) as libc::c_uchar, gs);
    _printhexb((value & 0xff as i32 as u32) as libc::c_uchar, gs);
}

unsafe fn _printbin(mut value: u32, gs: &mut GlobalState) {
    let mut loop_0: i32 = 0;
    print(b"%\0" as *const u8 as *const i8, gs);
    loop_0 = 0;
    while loop_0 < 16 {
        if value >> 15 - loop_0 & 1 != 0 {
            print(b"1\0" as *const u8 as *const i8, gs);
        } else {
            print(b"0\0" as *const u8 as *const i8, gs);
        }
        loop_0 += 1;
    }
}

unsafe fn _printc(mut string: *const i8, gs: &mut GlobalState) {
    sx = 1 + gs.screencenter.x - (strlen(string)).wrapping_div(2) as i32;
    print(string, gs);
}

// Rust port: Avoids importing strlen, and also, works on u8.
//
pub fn port_temp_strlen(string: &[u8]) -> usize {
    string.iter().position(|c| *c == 0).unwrap()
}

////////////////////////////////////////////////////////////////////
//
// input unsigned
//
////////////////////////////////////////////////////////////////////
pub unsafe fn _inputint(gs: &mut GlobalState) -> u32 {
    let mut string = vec![0; 18];
    let hexstr = b"0123456789ABCDEF";
    let mut value = 0;

    _input(&mut string, 17, gs);

    if string[0] == b'$' {
        let digits = port_temp_strlen(&string) as isize - 2;
        if digits < 0 {
            return 0;
        }
        for loop1 in 0..=digits {
            let digit = string[loop1 as usize + 1].to_ascii_uppercase();

            for loop_0 in 0..16 {
                if digit == hexstr[loop_0 as usize] {
                    value |= (loop_0 as u8) << (digits - loop1 as isize) * 4;
                    break;
                }
            }
        }
    } else if string[0] == b'%' {
        let digits_0 = (port_temp_strlen(&string)) as isize - 2;
        if digits_0 < 0 {
            return 0;
        }
        for loop1 in 0..=(digits_0 as usize) {
            if (string[loop1 + 1]) < b'0' || string[loop1 + 1] > b'1' {
                return 0;
            }
            value |= (string[loop1 + 1] - b'0') << digits_0 - loop1 as isize;
        }
    } else {
        value = String::from_utf8(string).unwrap().parse().unwrap();
    }
    value as u32
}

////////////////////////////////////////////////////////////////////
//
// line input routine
//
////////////////////////////////////////////////////////////////////
unsafe fn _input(string: &mut [u8], max: usize, gs: &mut GlobalState) -> i32 {
    let mut key_ = 0;
    let mut count = 0;

    loop {
        key_ = (get(gs) as u8).to_ascii_uppercase();
        if (key_ == 127 || key_ == 8) && count > 0 {
            count -= 1;
            drawchar(sx, sy, ' ' as i32, gs);
            sx -= 1;
        }

        if key_ >= b' ' && key_ <= b'z' && count < max {
            string[count as usize] = key_;
            count += 1;
            drawchar(sx, sy, key_ as i32, gs);
            sx += 1;
        }

        if key_ == 27 || key_ == 13 {
            break;
        }
    }
    for loop_ in count..max {
        string[loop_] = 0;
    }
    if key_ == 13 {
        return 1;
    }
    0
}

#[no_mangle]
pub static mut scoreswap: scores = scores {
    score: 0,
    level: 0,
    initials: [0; 4],
};
#[no_mangle]
pub static mut highscores: [scores; 5] = [scores {
    score: 0,
    level: 0,
    initials: [0; 4],
}; 5];
#[no_mangle]
pub static mut score: i32 = 0;
#[no_mangle]
pub static mut level: i16 = 0;

// There seems to be a (harmless) wrong definition in the original project:
//
//     char *_extension = "PCR";
//
// which is overwritten with "CA2" in `CATACOMB.C`.

#[no_mangle]
pub static mut _cgaok: boolean = true as boolean;
#[no_mangle]
pub static mut _egaok: boolean = true as boolean;
#[no_mangle]
pub static mut _vgaok: boolean = false as boolean;
static mut DOSScanCodeMap: [SDL_Scancode; 128] = [
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_ESCAPE,
    SDL_SCANCODE_1,
    SDL_SCANCODE_2,
    SDL_SCANCODE_3,
    SDL_SCANCODE_4,
    SDL_SCANCODE_5,
    SDL_SCANCODE_6,
    SDL_SCANCODE_7,
    SDL_SCANCODE_8,
    SDL_SCANCODE_9,
    SDL_SCANCODE_0,
    SDL_SCANCODE_MINUS,
    SDL_SCANCODE_EQUALS,
    SDL_SCANCODE_BACKSPACE,
    SDL_SCANCODE_TAB,
    SDL_SCANCODE_Q,
    SDL_SCANCODE_W,
    SDL_SCANCODE_E,
    SDL_SCANCODE_R,
    SDL_SCANCODE_T,
    SDL_SCANCODE_Y,
    SDL_SCANCODE_U,
    SDL_SCANCODE_I,
    SDL_SCANCODE_O,
    SDL_SCANCODE_P,
    SDL_SCANCODE_LEFTBRACKET,
    SDL_SCANCODE_RIGHTBRACKET,
    SDL_SCANCODE_RETURN,
    SDL_SCANCODE_LCTRL,
    SDL_SCANCODE_A,
    SDL_SCANCODE_S,
    SDL_SCANCODE_D,
    SDL_SCANCODE_F,
    SDL_SCANCODE_G,
    SDL_SCANCODE_H,
    SDL_SCANCODE_J,
    SDL_SCANCODE_K,
    SDL_SCANCODE_L,
    SDL_SCANCODE_SEMICOLON,
    SDL_SCANCODE_APOSTROPHE,
    SDL_SCANCODE_GRAVE,
    SDL_SCANCODE_LSHIFT,
    SDL_SCANCODE_BACKSLASH,
    SDL_SCANCODE_Z,
    SDL_SCANCODE_X,
    SDL_SCANCODE_C,
    SDL_SCANCODE_V,
    SDL_SCANCODE_B,
    SDL_SCANCODE_N,
    SDL_SCANCODE_M,
    SDL_SCANCODE_COMMA,
    SDL_SCANCODE_PERIOD,
    SDL_SCANCODE_SLASH,
    SDL_SCANCODE_RSHIFT,
    SDL_SCANCODE_SYSREQ,
    SDL_SCANCODE_LALT,
    SDL_SCANCODE_SPACE,
    SDL_SCANCODE_CAPSLOCK,
    SDL_SCANCODE_F1,
    SDL_SCANCODE_F2,
    SDL_SCANCODE_F3,
    SDL_SCANCODE_F4,
    SDL_SCANCODE_F5,
    SDL_SCANCODE_F6,
    SDL_SCANCODE_F7,
    SDL_SCANCODE_F8,
    SDL_SCANCODE_F9,
    SDL_SCANCODE_F10,
    SDL_SCANCODE_NUMLOCKCLEAR,
    SDL_SCANCODE_SCROLLLOCK,
    SDL_SCANCODE_HOME,
    SDL_SCANCODE_UP,
    SDL_SCANCODE_PAGEUP,
    SDL_SCANCODE_KP_MINUS,
    SDL_SCANCODE_LEFT,
    SDL_SCANCODE_KP_5,
    SDL_SCANCODE_RIGHT,
    SDL_SCANCODE_KP_PLUS,
    SDL_SCANCODE_END,
    SDL_SCANCODE_DOWN,
    SDL_SCANCODE_PAGEDOWN,
    SDL_SCANCODE_INSERT,
    SDL_SCANCODE_DELETE,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_F11,
    SDL_SCANCODE_F12,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
];

pub unsafe fn ScancodeToDOS(mut sc: SDL_Scancode) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < 128 {
        if DOSScanCodeMap[i as usize] as u32 == sc as u32 {
            return i;
        }
        i += 1;
    }
    return 0;
}

pub unsafe fn CheckMouseMode() {
    safe_SDL_SetRelativeMouseMode(
        (hasFocus as i32 != 0
            && (playermode[1] as u32 == mouse as i32 as u32
                || playermode[2] as u32 == mouse as i32 as u32)) as i32 as SDL_bool,
    );
}

pub unsafe fn _loadctrls() {
    let str = CString::new(format!("CTLPANEL.{port_temp__extension}")).unwrap();
    // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
    // project.
    let handle = open(
        str.as_ptr(),
        O_RDONLY | O_BINARY,
        0o200 as i32 | 0o400 as i32,
    );
    if handle == -1 {
        grmode = VGAgr;
        soundmode = spkr;
        playermode[1] = keyboard;
        playermode[2] = joystick1;
        JoyXlow[2] = 20;
        JoyXlow[1] = JoyXlow[2];
        JoyXhigh[2] = 60;
        JoyXhigh[1] = JoyXhigh[2];
        JoyYlow[2] = 20;
        JoyYlow[1] = JoyYlow[2];
        JoyYhigh[2] = 60;
        JoyYhigh[1] = JoyYhigh[2];
        MouseSensitivity = 5;
        key[north as usize] = SDL_SCANCODE_UP as i32;
        key[northeast as usize] = SDL_SCANCODE_PAGEUP as i32;
        key[east as usize] = SDL_SCANCODE_RIGHT as i32;
        key[southeast as usize] = SDL_SCANCODE_PAGEDOWN as i32;
        key[south as usize] = SDL_SCANCODE_DOWN as i32;
        key[southwest as usize] = SDL_SCANCODE_END as i32;
        key[west as usize] = SDL_SCANCODE_LEFT as i32;
        key[northwest as usize] = SDL_SCANCODE_HOME as i32;
        keyB1 = SDL_SCANCODE_LCTRL as i32;
        keyB2 = SDL_SCANCODE_LALT as i32;
    } else {
        let mut ctlpanel: ctlpaneltype = ctlpaneltype {
            grmode: text,
            soundmode: 0,
            playermode: [0; 3],
            JoyXlow: [0; 3],
            JoyYlow: [0; 3],
            JoyXhigh: [0; 3],
            JoyYhigh: [0; 3],
            MouseSensitivity: 0,
            key: [0; 8],
            keyB1: 0,
            keyB2: 0,
        };
        read(
            handle,
            &mut ctlpanel as *mut ctlpaneltype as *mut libc::c_void,
            ::std::mem::size_of::<ctlpaneltype>() as u64,
        );
        close(handle);
        grmode = ctlpanel.grmode as grtype;
        soundmode = ctlpanel.soundmode as soundtype;
        let mut i: u32 = 0;
        i = 0;
        while i < 3 {
            playermode[i as usize] = ctlpanel.playermode[i as usize] as inputtype;
            JoyXlow[i as usize] = ctlpanel.JoyXlow[i as usize] as i32;
            JoyYlow[i as usize] = ctlpanel.JoyYlow[i as usize] as i32;
            JoyXhigh[i as usize] = ctlpanel.JoyXhigh[i as usize] as i32;
            JoyYhigh[i as usize] = ctlpanel.JoyYhigh[i as usize] as i32;
            if playermode[i as usize] as u32 == mouse as i32 as u32 {
                CheckMouseMode();
            }
            if playermode[i as usize] as u32 == joystick1 as i32 as u32
                || playermode[i as usize] as u32 == joystick2 as i32 as u32
            {
                ProbeJoysticks();
                if playermode[i as usize] as u32 == joystick1 as i32 as u32
                    && joystick[1].device < 0
                    || playermode[i as usize] as u32 == joystick2 as i32 as u32
                        && joystick[2].device < 0
                {
                    playermode[i as usize] = keyboard;
                }
            }
            i = i.wrapping_add(1);
        }
        MouseSensitivity = ctlpanel.MouseSensitivity as i32;
        i = 0;
        while i < 8 {
            key[i as usize] = DOSScanCodeMap[ctlpanel.key[i as usize] as usize] as i32;
            i = i.wrapping_add(1);
        }
        keyB1 = DOSScanCodeMap[ctlpanel.keyB1 as usize] as i32;
        keyB2 = DOSScanCodeMap[ctlpanel.keyB2 as usize] as i32;
    };
}

pub unsafe fn _savectrls() {
    let mut ctlpanel: ctlpaneltype = ctlpaneltype {
        grmode: text,
        soundmode: 0,
        playermode: [0; 3],
        JoyXlow: [0; 3],
        JoyYlow: [0; 3],
        JoyXhigh: [0; 3],
        JoyYhigh: [0; 3],
        MouseSensitivity: 0,
        key: [0; 8],
        keyB1: 0,
        keyB2: 0,
    };
    let str = CString::new(format!("CTLPANEL.{port_temp__extension}")).unwrap();
    let handle = open(
        str.as_ptr(),
        0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
        0o400 as i32 | 0o200 as i32,
    );
    if handle == -1 {
        return;
    }
    ctlpanel.grmode = grmode;
    ctlpanel.soundmode = soundmode as u16;
    let mut i = 0;
    while i < 3 {
        ctlpanel.playermode[i] = playermode[i] as u16;
        ctlpanel.JoyXlow[i] = JoyXlow[i] as i16;
        ctlpanel.JoyYlow[i] = JoyYlow[i] as i16;
        ctlpanel.JoyXhigh[i] = JoyXhigh[i] as i16;
        ctlpanel.JoyYhigh[i] = JoyYhigh[i] as i16;
        i = i.wrapping_add(1);
    }
    ctlpanel.MouseSensitivity = MouseSensitivity as i16;
    i = 0;
    while i < 8 {
        ctlpanel.key[i as usize] = ScancodeToDOS(key[i as usize] as SDL_Scancode) as u8;
        i = i.wrapping_add(1);
    }
    ctlpanel.keyB1 = ScancodeToDOS(keyB1 as SDL_Scancode) as u8;
    ctlpanel.keyB2 = ScancodeToDOS(keyB2 as SDL_Scancode) as u8;
    write(
        handle,
        &mut ctlpanel as *mut ctlpaneltype as *const libc::c_void,
        ::std::mem::size_of::<ctlpaneltype>() as u64,
    );
    close(handle);
}

pub unsafe fn _loadhighscores() {
    let str = CString::new(format!("SCORES.{port_temp__extension}")).unwrap();
    if LoadFile(str.as_ptr(), highscores.as_mut_ptr() as *mut i8) == 0 {
        let mut i = 0;
        while i < 5 {
            highscores[i as usize].score = 100;
            highscores[i as usize].level = 1;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"PCR\0" as *const u8 as *const i8,
            );
            i += 1;
        }
    }
}

pub unsafe fn _savehighscores() {
    let str = CString::new(format!("SCORES.{port_temp__extension}")).unwrap();
    SaveFile(
        str.as_ptr(),
        highscores.as_mut_ptr() as *mut i8,
        ::std::mem::size_of::<[scores; 5]>() as u64 as i64,
    );
}

pub unsafe fn _showhighscores(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    let mut h: i64 = 0;
    centerwindow(17, 17, gs);
    print(b"\n   HIGH SCORES\n\n\0" as *const u8 as *const i8, gs);
    print(b" #  SCORE LV  BY\n\0" as *const u8 as *const i8, gs);
    print(b" - ------ -- ---\n\0" as *const u8 as *const i8, gs);
    i = 0;
    while i < 5 {
        sx += 1;
        drawchar(sx, sy, '1' as i32 + i, gs);
        sx += 2;
        h = highscores[i as usize].score as i64;
        if h < 100000 {
            sx += 1;
        }
        if h < 10000 {
            sx += 1;
        }
        if h < 1000 {
            sx += 1;
        }
        if h < 100 {
            sx += 1;
        }
        if h < 10 {
            sx += 1;
        }
        let str = CString::new(format!("{h}")).unwrap();
        print(str.as_ptr(), gs);
        sx += 1;
        if (highscores[i as usize].level as i32) < 10 {
            sx += 1;
        }
        // Rust port: Interesting, if this is passed as format! parameter, it will cause a warning.
        let highscore = highscores[i as usize].level;
        let str = CString::new(format!("{highscore}")).unwrap();
        print(str.as_ptr(), gs);
        sx += 1;
        let str = CString::new(highscores[i as usize].initials.map(|f| f as u8)).unwrap();
        print(str.as_ptr(), gs);
        let str = CString::new("\n\n").unwrap();
        print(str.as_ptr(), gs);
        i += 1;
    }
    let str = CString::new(format!("SCORE:{score}")).unwrap();
    _printc(str.as_ptr(), gs);
}

pub unsafe fn _checkhighscore(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    i = 0;
    while i < 5 {
        if score > highscores[i as usize].score {
            j = 4;
            while i < j {
                k = j - 1;
                highscores[j as usize] = highscores[k as usize];
                j -= 1;
            }
            highscores[i as usize].score = score;
            highscores[i as usize].level = level;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"   \0" as *const u8 as *const i8,
            );
            break;
        } else {
            i += 1;
        }
    }
    _showhighscores(gs);
    UpdateScreen(gs);
    if i < 5 {
        PlaySound(16);
        clearkeys();
        sx = gs.screencenter.x - 17 / 2 + 14;
        sy = gs.screencenter.y - 17 / 2 + 6 + i * 2;
        j = 0;
        loop {
            k = get(gs);
            ch = k as i8;
            if ch as i32 >= ' ' as i32 && j < 3 {
                drawchar(sx, sy, ch as i32, gs);
                sx += 1;
                highscores[i as usize].initials[j as usize] = ch;
                j += 1;
            }
            if ch as i32 == 8 || k == 19200 {
                if j > 0 {
                    sx -= 1;
                    j -= 1;
                }
            }
            if !(ch as i32 != 13) {
                break;
            }
        }
    }
}

const VIDEO_PARAM_WINDOWED: &str = "windowed";
const VIDEO_PARAM_FULLSCREEN: &str = "screen";

pub fn _setupgame(gs: &mut GlobalState) {
    if safe_SDL_Init(0x20 as u32 | 0x1 as u32 | 0x200 as u32 | 0x2000 as u32) < 0 {
        eprintln!("Failed to initialize SDL: {}", safe_SDL_GetError());
        std::process::exit(1);
    }
    safe_register_sdl_quit_on_exit();
    safe_SDL_AddEventWatch(
        Some(WatchUIEvents as unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> i32),
        0 as *mut libc::c_void,
    );
    let mut windowed = false;
    let mut winWidth = 640;
    let mut winHeight = 480;
    let mut displayindex = 0;

    // It's possible to iterate `Args`, although it doesn't get much cleaner.
    let args = std::env::args().into_iter().collect::<Vec<_>>();

    if let Some(screen_mode) = args.get(1) {
        match screen_mode.as_str() {
            VIDEO_PARAM_WINDOWED => {
                if args.len() == 4 {
                    winWidth = args[2]
                        .parse()
                        .expect(&format!("Invalid width parameter: {}", args[2]));
                    winHeight = args[3]
                        .parse()
                        .expect(&format!("Invalid height parameter: {}", args[3]));
                } else {
                    panic!("Incorrect number of windowed mode parameters");
                }

                windowed = true;
            }
            VIDEO_PARAM_FULLSCREEN => {
                if args.len() == 3 {
                    displayindex = args[2]
                        .parse()
                        .expect(&format!("Invalid screen parameter: {}", args[2]));
                } else {
                    panic!("Incorrect number of screen mode parameters");
                }
            }
            _ => panic!("Unexpected screen mode parameter"),
        }
    }

    let mut bounds: SDL_Rect = SDL_Rect {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
    unsafe {
        if safe_SDL_GetCurrentDisplayMode(displayindex, &mut mode) < -(1)
            || safe_SDL_GetDisplayBounds(displayindex, &mut bounds) < 0
        {
            eprintln!("Could not get display mode: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        if windowed {
            bounds.x = (0x1fff0000 as u32 | 0) as i32;
            bounds.y = (0x1fff0000 as u32 | 0) as i32;
            mode.w = winWidth as i32;
            mode.h = winHeight as i32;
        }
        window = safe_SDL_CreateWindow(
            b"The Catacomb\0" as *const u8 as *const i8,
            bounds.x,
            bounds.y,
            mode.w,
            mode.h,
            (if windowed as i32 != 0 {
                0
            } else {
                SDL_WINDOW_FULLSCREEN_DESKTOP as i32
            }) as u32,
        );
        if window.is_null() || {
            renderer = safe_SDL_CreateRenderer(window, -(1), 0);
            renderer.is_null()
        } {
            eprintln!("Failed to create SDL window: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        sdltexture = safe_SDL_CreateTexture(
            renderer,
            SDL_PIXELFORMAT_ARGB8888,
            SDL_TEXTUREACCESS_STREAMING as i32,
            320,
            200,
        );
        if sdltexture.is_null() {
            eprintln!("Could not create video buffer: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        if mode.w == 320 && mode.h == 200 || mode.w == 640 && mode.h == 400 {
            updateRect.w = mode.w;
            updateRect.h = mode.h;
            updateRect.y = 0;
            updateRect.x = updateRect.y;
        } else {
            updateRect.h = mode.h;
            updateRect.w = mode.h * 4 / 3;
            updateRect.x = mode.w - updateRect.w >> 1;
            updateRect.y = 0;
        }
        gs.screenseg.fill(0);
        grmode = EGAgr;
        joystick[2].device = -(1);
        joystick[1].device = joystick[2].device;
        _loadctrls();
        if grmode as u32 == VGAgr as i32 as u32 && _vgaok as i32 != 0 {
            grmode = VGAgr;
        } else if grmode as u32 >= EGAgr as i32 as u32 && _egaok as i32 != 0 {
            grmode = EGAgr;
        } else {
            grmode = CGAgr;
        }
        let filename = format!("SOUNDS.{port_temp__extension}");
        SoundData = bloadin(&filename) as *mut SPKRtable;
        StartupSound();
        SetupKBD();
        initrndt(1);
        initrnd(1);
        _loadhighscores();
        loadgrfiles();
        SetupEmulatedVBL();
    }
}

pub unsafe fn _quit(mut error: *const i8) {
    if *error == 0 {
        _savehighscores();
        _savectrls();
    } else {
        puts(error);
        puts(b"\n\0" as *const u8 as *const i8);
        puts(b"\n\0" as *const u8 as *const i8);
        puts(b"For techinical assistance with running this software\n\0" as *const u8 as *const i8);
        puts(b"    call Softdisk Publishing at 1-318-221-8311\n\0" as *const u8 as *const i8);
        puts(b"\n\0" as *const u8 as *const i8);
        std::process::exit(1);
    }
    ShutdownSound();
    ShutdownJoysticks();
    safe_SDL_DestroyRenderer(renderer);
    safe_SDL_DestroyWindow(window);
    renderer = 0 as *mut SDL_Renderer;
    window = 0 as *mut SDL_Window;
    std::process::exit(0);
}
