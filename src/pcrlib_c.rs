use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::{fs, mem, ptr};

use ::libc;
use libc::O_RDONLY;

use crate::catacomb::loadgrfiles;
use crate::cpanel_state::CpanelState;
use crate::input_type::inputtype::*;
use crate::pcrlib_a::{initrnd, initrndt, SetupEmulatedVBL, StartupSound};
use crate::pcrlib_a_state::PcrlibAState;
use crate::pcrlib_c_state::PcrlibCState;
use crate::sound_type::soundtype::{self, *};
use crate::spkr_table::SPKRtable;
use crate::{
    catasm::drawchartile,
    control_struct::ControlStruct,
    demo_enum::demoenum,
    dir_type::dirtype::*,
    extra_constants::{port_temp__extension, O_BINARY, SDL_BUTTON_LEFT, SDL_BUTTON_RIGHT},
    extra_macros::SDL_BUTTON,
    extra_types::boolean,
    global_state::GlobalState,
    gr_type::grtype::{self, *},
    pcrlib_a::{drawchar, PlaySound, ShutdownSound, WaitVBL},
    safe_sdl::*,
    scan_codes::*,
    scores::scores,
};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: u64) -> i64;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
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
    pub _old_offset: i64,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();

pub type SDL_bool = u32;

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
pub const SDL_TEXTUREACCESS_STREAMING: C2RustUnnamed_4 = 1;

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
    pub soundmode: soundtype,
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

pub fn SetupKBD(pcs: &mut PcrlibCState) {
    let mut i: u32 = 0;
    i = 0;
    while i < 128 {
        pcs.keydown[i as usize] = false as boolean;
        i = i.wrapping_add(1);
    }
}

pub unsafe fn ProcessEvents(pcs: &mut PcrlibCState) {
    pcs.mouseEvent = false as boolean;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as i32 as u32 {
            pcs.keydown[event.key.keysym.scancode as usize] = true as boolean;
            pcs.lastkey = event.key.keysym.scancode;
        } else if event.type_0 == SDL_KEYUP as i32 as u32 {
            pcs.keydown[event.key.keysym.scancode as usize] = false as boolean;
        } else if event.type_0 == SDL_MOUSEMOTION as i32 as u32 {
            pcs.mouseEvent = true as boolean;
        }
    }
}

unsafe extern "C" fn WatchUIEvents(userdata: *mut libc::c_void, event: *mut SDL_Event) -> i32 {
    let userdata = &*(userdata as *mut SDLEventPayload);

    if (*event).type_0 == SDL_QUIT as i32 as u32 {
        // We're quitting, so we can deallocate the userdata (although it's not strictly necessary).
        // This approach works because we're not in a multithreaded contenxt, so this function is
        // invoked only once a a time.
        let userdata = Box::from_raw(userdata as *const _ as *mut SDLEventPayload);

        _quit(
            b"\0" as *const u8 as *const i8 as *mut i8,
            &mut *userdata.pas,
            &mut *userdata.pcs,
        );
    } else if (*event).type_0 == SDL_WINDOWEVENT as i32 as u32 {
        let (_, pcs) = (&mut *userdata.pas, &mut *userdata.pcs);

        match (*event).window.event as i32 {
            13 => {
                pcs.hasFocus = false as boolean;
                CheckMouseMode(pcs);
            }
            12 => {
                while safe_SDL_GetMouseFocus() != pcs.window {
                    safe_SDL_PumpEvents();
                    safe_SDL_Delay(10);
                }
                pcs.hasFocus = true as boolean;
                CheckMouseMode(pcs);
            }
            _ => {}
        }
    }
    return 0;
}

pub fn ControlKBD(pcs: &mut PcrlibCState) -> ControlStruct {
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    if pcs.keydown[pcs.key[north as i32 as usize] as usize] != 0 {
        ymove = -1;
    }
    if pcs.keydown[pcs.key[east as i32 as usize] as usize] != 0 {
        xmove = 1;
    }
    if pcs.keydown[pcs.key[south as i32 as usize] as usize] != 0 {
        ymove = 1;
    }
    if pcs.keydown[pcs.key[west as i32 as usize] as usize] != 0 {
        xmove = -1;
    }
    if pcs.keydown[pcs.key[northeast as i32 as usize] as usize] != 0 {
        ymove = -1;
        xmove = 1;
    }
    if pcs.keydown[pcs.key[northwest as i32 as usize] as usize] != 0 {
        ymove = -1;
        xmove = -1;
    }
    if pcs.keydown[pcs.key[southeast as i32 as usize] as usize] != 0 {
        ymove = 1;
        xmove = 1;
    }
    if pcs.keydown[pcs.key[southwest as i32 as usize] as usize] != 0 {
        ymove = 1;
        xmove = -1;
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
    action.button1 = pcs.keydown[pcs.keyB1 as usize];
    action.button2 = pcs.keydown[pcs.keyB2 as usize];
    return action;
}

pub fn ControlMouse(pcs: &mut PcrlibCState) -> ControlStruct {
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let buttons: i32 = safe_SDL_GetRelativeMouseState(&mut newx, &mut newy) as i32;
    action.button1 = (buttons & SDL_BUTTON(SDL_BUTTON_LEFT)) as boolean;
    action.button2 = (buttons & SDL_BUTTON(SDL_BUTTON_RIGHT)) as boolean;
    if pcs.mouseEvent as i32 == false as i32 {
        action.dir = nodir;
        return action;
    }
    if newx > pcs.MouseSensitivity {
        xmove = 1;
    } else if newx < -pcs.MouseSensitivity {
        xmove = -1;
    }
    if newy > pcs.MouseSensitivity {
        ymove = 1;
    } else if newy < -pcs.MouseSensitivity {
        ymove = -1;
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

unsafe fn ShutdownJoysticks(pcs: &mut PcrlibCState) {
    let mut j: u32 = 0;
    j = 1;
    while j < 3 {
        if !(pcs.joystick[j as usize].device < 0) {
            if pcs.joystick[j as usize].isgamecontroller != 0 {
                safe_SDL_GameControllerClose(pcs.joystick[j as usize].c2rust_unnamed.controller);
            } else {
                safe_SDL_JoystickClose(pcs.joystick[j as usize].c2rust_unnamed.joy);
            }
            pcs.joystick[j as usize].device = -1;
        }
        j = j.wrapping_add(1);
    }
}

pub unsafe fn ProbeJoysticks(pcs: &mut PcrlibCState) {
    let mut j: i32 = 0;
    if pcs.joystick[1].device > 0 || pcs.joystick[2].device > 0 {
        ShutdownJoysticks(pcs);
    }
    j = 1;
    while j < 3 {
        if j - 1 >= safe_SDL_NumJoysticks() {
            pcs.joystick[j as usize].device = -1;
        } else {
            pcs.joystick[j as usize].device = j - 1;
            pcs.joystick[j as usize].isgamecontroller = safe_SDL_IsGameController(j - 1) as boolean;
            if safe_SDL_IsGameController(j - 1) as u64 != 0 {
                pcs.joystick[j as usize].c2rust_unnamed.controller =
                    safe_SDL_GameControllerOpen(j - 1);
            } else {
                pcs.joystick[j as usize].c2rust_unnamed.joy = safe_SDL_JoystickOpen(j - 1);
            }
        }
        j += 1;
    }
}

pub unsafe fn ReadJoystick(
    joynum: i32,
    xcount: *mut i32,
    ycount: *mut i32,
    pcs: &mut PcrlibCState,
) {
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    *xcount = 0;
    *ycount = 0;
    safe_SDL_JoystickUpdate();
    if pcs.joystick[joynum as usize].isgamecontroller != 0 {
        a1 = safe_SDL_GameControllerGetAxis(
            pcs.joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as i32;
        a2 = safe_SDL_GameControllerGetAxis(
            pcs.joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as i32;
    } else {
        a1 = safe_SDL_JoystickGetAxis(pcs.joystick[joynum as usize].c2rust_unnamed.joy, 0) as i32;
        a2 = safe_SDL_JoystickGetAxis(pcs.joystick[joynum as usize].c2rust_unnamed.joy, 1) as i32;
    }
    *xcount = a1;
    *ycount = a2;
}

pub unsafe fn ControlJoystick(joynum: i32, pcs: &mut PcrlibCState) -> ControlStruct {
    let mut joyx: i32 = 0;
    let mut joyy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    ReadJoystick(joynum, &mut joyx, &mut joyy, pcs);
    if pcs.joystick[joynum as usize].isgamecontroller != 0 {
        action.button1 = (safe_SDL_GameControllerGetButton(
            pcs.joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_A,
        ) as i32
            != 0) as i32 as boolean;
        action.button2 = (safe_SDL_GameControllerGetButton(
            pcs.joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_B,
        ) as i32
            != 0) as i32 as boolean;
    } else {
        action.button1 =
            (safe_SDL_JoystickGetButton(pcs.joystick[joynum as usize].c2rust_unnamed.joy, 0) as i32
                != 0) as i32 as boolean;
        action.button2 =
            (safe_SDL_JoystickGetButton(pcs.joystick[joynum as usize].c2rust_unnamed.joy, 1) as i32
                != 0) as i32 as boolean;
    }
    if joyx == 0 && joyy == 0 {
        action.dir = nodir;
        return action;
    }
    if joyx > pcs.JoyXhigh[joynum as usize] {
        xmove = 1;
    } else if joyx < pcs.JoyXlow[joynum as usize] {
        xmove = -1;
    }
    if joyy > pcs.JoyYhigh[joynum as usize] {
        ymove = 1;
    } else if joyy < pcs.JoyYlow[joynum as usize] {
        ymove = -1;
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

pub unsafe fn ControlPlayer(
    player: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
) -> ControlStruct {
    let mut ret: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    ProcessEvents(pcs);
    if gs.indemo == demoenum::notdemo || gs.indemo == demoenum::recording {
        match pcs.playermode[player as usize] as u32 {
            1 => {
                ret = ControlMouse(pcs);
            }
            2 => {
                ret = ControlJoystick(1, pcs);
            }
            3 => {
                ret = ControlJoystick(2, pcs);
            }
            0 | _ => {
                ret = ControlKBD(pcs);
            }
        }
        if gs.indemo == demoenum::recording {
            let val = ((ret.dir as u32) << 2
                | ((ret.button2 as i32) << 1) as u32
                | ret.button1 as u32) as i32;
            pcs.demobuffer[pcs.demoptr] = val as u8;
            pcs.demoptr += 1;
        }
    } else {
        let val = pcs.demobuffer[pcs.demoptr];
        pcs.demoptr += 1;
        ret.button1 = (val & 1) as boolean;
        ret.button2 = ((val & 2) >> 1) as boolean;
        ret.dir = ((val & (4 + 8 + 16 + 32)) >> 2).into();
    }
    return ret;
}

pub fn RecordDemo(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.demobuffer[0] = pcs.level as u8;
    pcs.demoptr = 1;
    gs.indemo = demoenum::recording;
}

////////////////////////
//
// LoadDemo / SaveDemo
// Loads a demo from disk or
// saves the accumulated demo command string to disk
//
////////////////////////

pub fn LoadDemo(demonum: i32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let filename = format!("DEMO{demonum}.{port_temp__extension}");
    let mut temp_port_demobuffer = [0; 5000];

    port_temp_LoadFile(&filename, &mut temp_port_demobuffer);
    pcs.demobuffer
        .copy_from_slice(&temp_port_demobuffer.map(|b| b as u8));
    pcs.level = pcs.demobuffer[0] as i16;
    pcs.demoptr = 1;
    gs.indemo = demoenum::demoplay;
}

pub fn SaveDemo(demonum: u8, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let str = format!("DEMO{demonum}.{port_temp__extension}");

    port_temp_SaveFile(&str, &pcs.demobuffer[..pcs.demoptr]);

    gs.indemo = demoenum::notdemo;
}

////////////////////////
//
// StartDemo
//
////////////////////////

/*=========================================================================*/

pub unsafe fn clearkeys(pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    while bioskey(1, pcs) != 0 {
        bioskey(0, pcs);
    }
    i = 0;
    while i < 128 {
        pcs.keydown[i as usize] = 0;
        i += 1;
    }
}

unsafe fn filelength(fd: i32) -> i64 {
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
        return -1 as i64;
    }
    return s.st_size;
}

pub unsafe fn LoadFile(filename: *const i8, buffer: *const i8) -> u64 {
    let mut fd: i32 = 0;
    fd = open(filename, 0o400 as i32);
    if fd < 0 {
        return 0;
    }
    let len: i64 = filelength(fd);
    let bytesRead: i64 = read(fd, buffer as *mut libc::c_void, len as u64);
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

//===========================================================================

/*
==============================================
=
= Save a *LARGE* file far a FAR buffer!
= by John Romero (C) 1990 PCRcade
=
==============================================
*/

pub unsafe fn SaveFile(filename: *const i8, buffer: *const i8, size: i64) {
    let mut fd: i32 = 0;
    // Flags: O_WRONLY | O_BINARY | O_CREAT | O_TRUNC, S_IREAD | S_IWRITE
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

pub fn port_temp_SaveFile(filename: &str, buffer: &[u8]) {
    // Flags originally used: O_WRONLY | O_BINARY | O_CREAT | O_TRUNC, S_IREAD | S_IWRITE
    //
    // Rust port: In the original project, this is written in ASM (https://github.com/64kramsystem/catacomb_ii-64k/blob/db8017c1aba84823cb5116ca2f819e5c77636c9e/original_project/PCRLIB_C.C#L649).
    // Errors are swallowed; it's not clear if this is intentional, but we leave this behavior.
    // The file is truncated if existing (http://spike.scu.edu.au/~barry/interrupts.html#ah3c), so we just use corresponding flags.
    // Permissions are ignored (they're set in the SDL port).

    let file = OpenOptions::new() //
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename);

    if let Ok(mut file) = file {
        file.write_all(buffer).ok();
    }
}

//==========================================================================

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

pub fn port_temp_bloadin(filename: &str) -> Result<Vec<u8>, io::Error> {
    let file_meta = fs::metadata(filename);

    let mut buffer = vec![0; file_meta?.len() as usize];

    port_temp_LoadFile(filename, &mut buffer);

    Ok(buffer)
}

pub fn drawwindow(
    xl: i32,
    yl: i32,
    xh: i32,
    yh: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    pcs.win_xl = xl;
    pcs.win_yl = yl;
    pcs.win_xh = xh;
    pcs.win_yh = yh;
    drawchar(xl, yl, 1, gs, pcs);
    x = xl + 1;
    while x < xh {
        drawchar(x, yl, 2, gs, pcs);
        x += 1;
    }
    drawchar(xh, yl, 3, gs, pcs);
    y = yl + 1;
    while y < yh {
        drawchar(xl, y, 4, gs, pcs);
        x = xl + 1;
        while x < xh {
            drawchar(x, y, ' ' as i32, gs, pcs);
            x += 1;
        }
        drawchar(xh, y, 5, gs, pcs);
        y += 1;
    }
    drawchar(xl, yh, 6, gs, pcs);
    x = xl + 1;
    while x < xh {
        drawchar(x, yh, 7, gs, pcs);
        x += 1;
    }
    drawchar(xh, yh, 8, gs, pcs);
    pcs.leftedge = xl + 1;
    pcs.sx = pcs.leftedge;
    pcs.sy = yl + 1;
}

pub fn bar(
    xl: i32,
    yl: i32,
    xh: i32,
    yh: i32,
    ch_0: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    y = yl;
    while y <= yh {
        x = xl;
        while x <= xh {
            drawchar(x, y, ch_0, gs, pcs);
            x += 1;
        }
        y += 1;
    }
}

pub fn erasewindow(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    bar(
        pcs.win_xl, pcs.win_yl, pcs.win_xh, pcs.win_yh, ' ' as i32, gs, pcs,
    );
}

pub fn centerwindow(width: i32, height: i32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let xl: i32 = gs.screencenter.x - width / 2;
    let yl: i32 = gs.screencenter.y - height / 2;
    drawwindow(xl, yl, xl + width + 1, yl + height + 1, gs, pcs);
}

pub unsafe fn expwin(
    width: i32,
    height: i32,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    if width > 2 {
        if height > 2 {
            expwin(width - 2, height - 2, gs, pas, pcs);
        } else {
            expwinh(width - 2, height, gs, pas, pcs);
        }
    } else if height > 2 {
        expwinv(width, height - 2, gs, pas, pcs);
    }
    UpdateScreen(gs, pcs);
    WaitVBL(pas);
    centerwindow(width, height, gs, pcs);
}

unsafe fn expwinh(
    width: i32,
    height: i32,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    if width > 2 {
        expwinh(width - 2, height, gs, pas, pcs);
    }
    UpdateScreen(gs, pcs);
    WaitVBL(pas);
    centerwindow(width, height, gs, pcs);
}

unsafe fn expwinv(
    width: i32,
    height: i32,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    if height > 2 {
        expwinv(width, height - 2, gs, pas, pcs);
    }
    UpdateScreen(gs, pcs);
    WaitVBL(pas);
    centerwindow(width, height, gs, pcs);
}

pub unsafe fn bioskey(cmd: i32, pcs: &mut PcrlibCState) -> i32 {
    if pcs.lastkey as u64 != 0 {
        let oldkey: i32 = pcs.lastkey as i32;
        if cmd != 1 {
            pcs.lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as i32 as u32 {
            if cmd == 1 {
                pcs.lastkey = event.key.keysym.scancode;
                return pcs.lastkey as i32;
            }
            return event.key.keysym.scancode as i32;
        }
    }
    return pcs.lastkey as i32;
}

const EGAPalette: [u32; 16] = [
    0, 0xaa, 0xaa00, 0xaaaa, 0xaa0000, 0xaa00aa, 0xaa5500, 0xaaaaaa, 0x555555, 0x5555ff, 0x55ff55,
    0x55ffff, 0xff5555, 0xff55ff, 0xffff55, 0xffffff,
];
const CGAPalette: [u32; 4] = [0, 0x55ffff, 0xff55ff, 0xffffff];

pub fn UpdateScreen(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: u64 = 0;
    if pcs.grmode as u32 == EGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            pcs.conv[i as usize] = EGAPalette[gs.screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if pcs.grmode as u32 == CGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            pcs.conv[i as usize] = CGAPalette[gs.screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else {
        panic!("VGA Palette conversion not implemented.");
    }
    safe_SDL_UpdateTexture(
        pcs.sdltexture,
        0 as *const SDL_Rect,
        pcs.conv.as_mut_ptr() as *const libc::c_void,
        (320 as i32 as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as i32,
    );
    safe_SDL_RenderClear(pcs.renderer);
    safe_SDL_RenderCopy(pcs.renderer, pcs.sdltexture, ptr::null(), &pcs.updateRect);
    safe_SDL_RenderPresent(pcs.renderer);
}

pub unsafe fn get(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) -> i32 {
    let mut cycle: i32 = 0;
    let mut key_0: i32 = 0;
    loop {
        cycle = 9;
        loop {
            key_0 = bioskey(0, pcs);
            if !(key_0 == 0 && cycle < 13) {
                break;
            }
            let fresh2 = cycle;
            cycle += 1;
            drawchar(pcs.sx, pcs.sy, fresh2, gs, pcs);
            UpdateScreen(gs, pcs);
            WaitVBL(pas);
            WaitVBL(pas);
            WaitVBL(pas);
            WaitVBL(pas);
            WaitVBL(pas);
        }
        if !(key_0 == 0) {
            break;
        }
    }
    drawchar(pcs.sx, pcs.sy, ' ' as i32, gs, pcs);
    UpdateScreen(gs, pcs);
    return safe_SDL_GetKeyFromScancode(key_0 as SDL_Scancode);
}

/////////////////////////
//
// print
// Prints a string at sx,sy.  No clipping!!!
//
/////////////////////////

pub unsafe fn _print(mut str_0: *const i8, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    loop {
        let ch_0 = *str_0;
        str_0 = str_0.offset(1);
        match ch_0 as u8 {
            0 => break,
            b'\n' => {
                pcs.sy += 1;
                pcs.sx = pcs.leftedge;
            }
            b'\r' => {
                pcs.sx = pcs.leftedge;
            }
            _ => {
                drawchar(pcs.sx, pcs.sy, ch_0 as i32, gs, pcs);
                pcs.sx += 1;
            }
        }
    }
}

/// To be the used when printing from memory.
///
pub fn port_temp_print_cstr(str_0: &CStr, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    port_temp_print_str(&str_0.to_string_lossy(), gs, pcs)
}

/// Convenience.
///
pub fn port_temp_print_str(str_0: &str, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    port_temp_print_arr(str_0.as_bytes(), gs, pcs);
}

/// Reference print routine. &[u8] is used, because this in not necessarily a textual string.
///
pub fn port_temp_print_arr(str_0: &[u8], gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    for ch_0 in str_0 {
        match ch_0 {
            0 => break,
            b'\n' => {
                pcs.sy += 1;
                pcs.sx = pcs.leftedge;
            }
            b'\r' => {
                pcs.sx = pcs.leftedge;
            }
            _ => {
                drawchar(pcs.sx, pcs.sy, *ch_0 as i32, gs, pcs);
                pcs.sx += 1;
            }
        }
    }
}

pub unsafe fn printchartile(mut str_0: *const i8, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut ch_0: i8 = 0;
    loop {
        let fresh5 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh5;
        if !(ch_0 as i32 != 0) {
            break;
        }
        if ch_0 as i32 == '\n' as i32 {
            pcs.sy += 1;
            pcs.sx = pcs.leftedge;
        } else if ch_0 as i32 == '\r' as i32 {
            pcs.sx = pcs.leftedge;
        } else {
            let fresh6 = pcs.sx;
            pcs.sx += 1;
            drawchartile(fresh6, pcs.sy, ch_0 as u8 as i32, gs, pcs);
        }
    }
}

////////////////////////////////////////////////////////////////////
//
// Verify a file's existence
//
////////////////////////////////////////////////////////////////////
/// Rust port: returns 0 if the file doesn't exist, otherwise its length.
#[allow(dead_code)]
pub fn _Verify(filename: &str) -> u64 {
    let filepath = Path::new(filename);

    if filepath.exists() {
        let file_meta = fs::metadata(filename);
        // If the file exists, assume that it can be read correctly.
        file_meta.unwrap().len()
    } else {
        0
    }
}

////////////////////////////////////////////////////////////////////
//
// print hex byte
//
////////////////////////////////////////////////////////////////////
/// Rust port: Prints a byte in padded hex.
fn _printhexb(value: libc::c_uchar, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let fmt_value = format!("{:02X}", value);
    port_temp_print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// print hex
//
////////////////////////////////////////////////////////////////////
/// Rust port: Prints a word in padded hex, prefixed by `$`.
fn _printhex(value: u32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let fmt_value = format!("${:04X}", value);
    port_temp_print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// print bin
//
////////////////////////////////////////////////////////////////////
/// Rust port: Prints a word in padded binary, prefixed by `%`; dead code.
fn _printbin(value: u32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let fmt_value = format!("%{:016b}", value);
    port_temp_print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// center print
//
////////////////////////////////////////////////////////////////////
fn _printc(string: &CString, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 1 + gs.screencenter.x - string.as_bytes().len() as i32;
    port_temp_print_cstr(string, gs, pcs);
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
pub unsafe fn _inputint(
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) -> u32 {
    let mut string = vec![0; 18];
    let hexstr = b"0123456789ABCDEF";
    let mut value = 0;

    _input(&mut string, 17, gs, pas, pcs);

    if string[0] == b'$' {
        let digits = port_temp_strlen(&string) as isize - 2;
        if digits < 0 {
            return 0;
        }
        for loop1 in 0..=digits {
            let digit = string[loop1 as usize + 1].to_ascii_uppercase();

            for loop_0 in 0..16 {
                if digit == hexstr[loop_0 as usize] {
                    value |= (loop_0 as u8) << ((digits - loop1 as isize) * 4);
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
            value |= (string[loop1 + 1] - b'0') << (digits_0 - loop1 as isize);
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
unsafe fn _input(
    string: &mut [u8],
    max: usize,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) -> i32 {
    let mut key_ = 0;
    let mut count = 0;

    loop {
        key_ = (get(gs, pas, pcs) as u8).to_ascii_uppercase();
        if (key_ == 127 || key_ == 8) && count > 0 {
            count -= 1;
            drawchar(pcs.sx, pcs.sy, ' ' as i32, gs, pcs);
            pcs.sx -= 1;
        }

        if key_ >= b' ' && key_ <= b'z' && count < max {
            string[count as usize] = key_;
            count += 1;
            drawchar(pcs.sx, pcs.sy, key_ as i32, gs, pcs);
            pcs.sx += 1;
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

// Rust port: dead code!
//
// const scoreswap: scores = scores {
//     score: 0,
//     level: 0,
//     initials: [0; 4],
// };

// Rust port: there seems to be a (harmless) wrong definition in the original project:
//
//     char *_extension = "PCR";
//
// which is overwritten with "CA2" in `CATACOMB.C`.

const _cgaok: boolean = true as boolean;

pub const _egaok: boolean = true as boolean;

pub const _vgaok: boolean = false as boolean;

pub fn ScancodeToDOS(sc: SDL_Scancode) -> i32 {
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

pub fn CheckMouseMode(pcs: &mut PcrlibCState) {
    safe_SDL_SetRelativeMouseMode(
        (pcs.hasFocus as i32 != 0
            && (pcs.playermode[1] as u32 == mouse as i32 as u32
                || pcs.playermode[2] as u32 == mouse as i32 as u32)) as i32 as SDL_bool,
    );
}

pub unsafe fn _loadctrls(pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let str = CString::new(format!("CTLPANEL.{port_temp__extension}")).unwrap();
    // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
    // project.
    let handle = open(
        str.as_ptr(),
        O_RDONLY | O_BINARY,
        0o200 as i32 | 0o400 as i32,
    );
    if handle == -1 {
        pcs.grmode = VGAgr;
        pas.soundmode = spkr;
        pcs.playermode[1] = keyboard;
        pcs.playermode[2] = joystick1;
        pcs.JoyXlow[2] = 20;
        pcs.JoyXlow[1] = pcs.JoyXlow[2];
        pcs.JoyXhigh[2] = 60;
        pcs.JoyXhigh[1] = pcs.JoyXhigh[2];
        pcs.JoyYlow[2] = 20;
        pcs.JoyYlow[1] = pcs.JoyYlow[2];
        pcs.JoyYhigh[2] = 60;
        pcs.JoyYhigh[1] = pcs.JoyYhigh[2];
        pcs.MouseSensitivity = 5;
        pcs.key[north as usize] = SDL_SCANCODE_UP as i32;
        pcs.key[northeast as usize] = SDL_SCANCODE_PAGEUP as i32;
        pcs.key[east as usize] = SDL_SCANCODE_RIGHT as i32;
        pcs.key[southeast as usize] = SDL_SCANCODE_PAGEDOWN as i32;
        pcs.key[south as usize] = SDL_SCANCODE_DOWN as i32;
        pcs.key[southwest as usize] = SDL_SCANCODE_END as i32;
        pcs.key[west as usize] = SDL_SCANCODE_LEFT as i32;
        pcs.key[northwest as usize] = SDL_SCANCODE_HOME as i32;
        pcs.keyB1 = SDL_SCANCODE_LCTRL as i32;
        pcs.keyB2 = SDL_SCANCODE_LALT as i32;
    } else {
        let mut ctlpanel: ctlpaneltype = ctlpaneltype {
            grmode: text,
            soundmode: off,
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
        pcs.grmode = ctlpanel.grmode as grtype;
        pas.soundmode = ctlpanel.soundmode as soundtype;
        let mut i: u32 = 0;
        i = 0;
        while i < 3 {
            pcs.playermode[i as usize] = ctlpanel.playermode[i as usize].into();
            pcs.JoyXlow[i as usize] = ctlpanel.JoyXlow[i as usize] as i32;
            pcs.JoyYlow[i as usize] = ctlpanel.JoyYlow[i as usize] as i32;
            pcs.JoyXhigh[i as usize] = ctlpanel.JoyXhigh[i as usize] as i32;
            pcs.JoyYhigh[i as usize] = ctlpanel.JoyYhigh[i as usize] as i32;
            if pcs.playermode[i as usize] as u32 == mouse as i32 as u32 {
                CheckMouseMode(pcs);
            }
            if pcs.playermode[i as usize] as u32 == joystick1 as i32 as u32
                || pcs.playermode[i as usize] as u32 == joystick2 as i32 as u32
            {
                ProbeJoysticks(pcs);
                if pcs.playermode[i as usize] as u32 == joystick1 as i32 as u32
                    && pcs.joystick[1].device < 0
                    || pcs.playermode[i as usize] as u32 == joystick2 as i32 as u32
                        && pcs.joystick[2].device < 0
                {
                    pcs.playermode[i as usize] = keyboard;
                }
            }
            i = i.wrapping_add(1);
        }
        pcs.MouseSensitivity = ctlpanel.MouseSensitivity as i32;
        i = 0;
        while i < 8 {
            pcs.key[i as usize] = DOSScanCodeMap[ctlpanel.key[i as usize] as usize] as i32;
            i = i.wrapping_add(1);
        }
        pcs.keyB1 = DOSScanCodeMap[ctlpanel.keyB1 as usize] as i32;
        pcs.keyB2 = DOSScanCodeMap[ctlpanel.keyB2 as usize] as i32;
    };
}

pub unsafe fn _savectrls(pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut ctlpanel: ctlpaneltype = ctlpaneltype {
        grmode: text,
        soundmode: off,
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
    ctlpanel.grmode = pcs.grmode;
    ctlpanel.soundmode = pas.soundmode;
    let mut i = 0;
    while i < 3 {
        ctlpanel.playermode[i] = pcs.playermode[i] as u16;
        ctlpanel.JoyXlow[i] = pcs.JoyXlow[i] as i16;
        ctlpanel.JoyYlow[i] = pcs.JoyYlow[i] as i16;
        ctlpanel.JoyXhigh[i] = pcs.JoyXhigh[i] as i16;
        ctlpanel.JoyYhigh[i] = pcs.JoyYhigh[i] as i16;
        i = i.wrapping_add(1);
    }
    ctlpanel.MouseSensitivity = pcs.MouseSensitivity as i16;
    i = 0;
    while i < 8 {
        ctlpanel.key[i as usize] = ScancodeToDOS(pcs.key[i as usize] as SDL_Scancode) as u8;
        i = i.wrapping_add(1);
    }
    ctlpanel.keyB1 = ScancodeToDOS(pcs.keyB1 as SDL_Scancode) as u8;
    ctlpanel.keyB2 = ScancodeToDOS(pcs.keyB2 as SDL_Scancode) as u8;
    write(
        handle,
        &mut ctlpanel as *mut ctlpaneltype as *const libc::c_void,
        ::std::mem::size_of::<ctlpaneltype>() as u64,
    );
    close(handle);
}

pub unsafe fn _loadhighscores(pcs: &mut PcrlibCState) {
    let str = CString::new(format!("SCORES.{port_temp__extension}")).unwrap();
    if LoadFile(str.as_ptr(), pcs.highscores.as_mut_ptr() as *mut i8) == 0 {
        for i in 0..5 {
            pcs.highscores[i].score = 100;
            pcs.highscores[i].level = 1;
            pcs.highscores[i].initials = "PCR".as_bytes().try_into().unwrap();
        }
    }
}

pub unsafe fn _savehighscores(pcs: &mut PcrlibCState) {
    let str = CString::new(format!("SCORES.{port_temp__extension}")).unwrap();
    SaveFile(
        str.as_ptr(),
        pcs.highscores.as_mut_ptr() as *mut i8,
        ::std::mem::size_of::<[scores; 5]>() as u64 as i64,
    );
}

pub fn _showhighscores(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    let mut h: i64 = 0;
    centerwindow(17, 17, gs, pcs);
    port_temp_print_str("\n   HIGH SCORES\n\n", gs, pcs);
    port_temp_print_str(" #  SCORE LV  BY\n", gs, pcs);
    port_temp_print_str(" - ------ -- ---\n", gs, pcs);
    i = 0;
    while i < 5 {
        pcs.sx += 1;
        drawchar(pcs.sx, pcs.sy, '1' as i32 + i, gs, pcs);
        pcs.sx += 2;
        h = pcs.highscores[i as usize].score as i64;
        if h < 100000 {
            pcs.sx += 1;
        }
        if h < 10000 {
            pcs.sx += 1;
        }
        if h < 1000 {
            pcs.sx += 1;
        }
        if h < 100 {
            pcs.sx += 1;
        }
        if h < 10 {
            pcs.sx += 1;
        }
        port_temp_print_str(&h.to_string(), gs, pcs);
        pcs.sx += 1;
        if (pcs.highscores[i as usize].level as i32) < 10 {
            pcs.sx += 1;
        }
        let str = { pcs.highscores[i as usize].level }.to_string();
        port_temp_print_str(&str, gs, pcs);
        pcs.sx += 1;
        // Rust port: Watch out! Entries includes the cstring terminator, which we must skip!
        let highscore_bytes = &pcs.highscores[i as usize].initials.map(|f| f as u8)[0..=2];
        port_temp_print_arr(highscore_bytes, gs, pcs);
        port_temp_print_str("\n\n", gs, pcs);
        i += 1;
    }
    let str = CString::new(format!("SCORE:{}", pcs.score)).unwrap();
    _printc(&str, gs, pcs);
}

pub unsafe fn _checkhighscore(
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    i = 0;
    while i < 5 {
        if pcs.score > pcs.highscores[i as usize].score {
            j = 4;
            while i < j {
                k = j - 1;
                pcs.highscores[j as usize] = pcs.highscores[k as usize];
                j -= 1;
            }
            pcs.highscores[i as usize].score = pcs.score;
            pcs.highscores[i as usize].level = pcs.level;
            pcs.highscores[i as usize].initials = b"   ".to_owned();
            break;
        } else {
            i += 1;
        }
    }
    _showhighscores(gs, pcs);
    UpdateScreen(gs, pcs);
    if i < 5 {
        PlaySound(16, pas);
        clearkeys(pcs);
        pcs.sx = gs.screencenter.x - 17 / 2 + 14;
        pcs.sy = gs.screencenter.y - 17 / 2 + 6 + i * 2;
        j = 0;
        loop {
            k = get(gs, pas, pcs);
            let ch = k as i8;
            if ch >= ' ' as i8 && j < 3 {
                drawchar(pcs.sx, pcs.sy, ch as i32, gs, pcs);
                pcs.sx += 1;
                pcs.highscores[i as usize].initials[j as usize] = ch as u8;
                j += 1;
            }
            if ch as i32 == 8 || k == 19200 {
                if j > 0 {
                    pcs.sx -= 1;
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

pub struct SDLEventPayload {
    pub pas: *mut PcrlibAState,
    pub pcs: *mut PcrlibCState,
}

pub fn _setupgame(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    if safe_SDL_Init(0x20 as u32 | 0x1 as u32 | 0x200 as u32 | 0x2000 as u32) < 0 {
        eprintln!("Failed to initialize SDL: {}", safe_SDL_GetError());
        std::process::exit(1);
    }
    safe_register_sdl_quit_on_exit();

    let userdata = Box::into_raw(Box::new(SDLEventPayload { pas, pcs }));

    safe_SDL_AddEventWatch(
        Some(WatchUIEvents as unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> i32),
        userdata as *mut libc::c_void,
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
        if safe_SDL_GetCurrentDisplayMode(displayindex, &mut pcs.mode) < -1
            || safe_SDL_GetDisplayBounds(displayindex, &mut bounds) < 0
        {
            eprintln!("Could not get display mode: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        if windowed {
            bounds.x = (0x1fff0000 as u32 | 0) as i32;
            bounds.y = (0x1fff0000 as u32 | 0) as i32;
            pcs.mode.w = winWidth as i32;
            pcs.mode.h = winHeight as i32;
        }
        pcs.window = safe_SDL_CreateWindow(
            b"The Catacomb\0" as *const u8 as *const i8,
            bounds.x,
            bounds.y,
            pcs.mode.w,
            pcs.mode.h,
            (if windowed as i32 != 0 {
                0
            } else {
                SDL_WINDOW_FULLSCREEN_DESKTOP as i32
            }) as u32,
        );
        if pcs.window.is_null() || {
            pcs.renderer = safe_SDL_CreateRenderer(pcs.window, -1, 0);
            pcs.renderer.is_null()
        } {
            eprintln!("Failed to create SDL window: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        pcs.sdltexture = safe_SDL_CreateTexture(
            pcs.renderer,
            SDL_PIXELFORMAT_ARGB8888,
            SDL_TEXTUREACCESS_STREAMING as i32,
            320,
            200,
        );
        if pcs.sdltexture.is_null() {
            eprintln!("Could not create video buffer: {}", safe_SDL_GetError());
            std::process::exit(1);
        }
        if pcs.mode.w == 320 && pcs.mode.h == 200 || pcs.mode.w == 640 && pcs.mode.h == 400 {
            pcs.updateRect.w = pcs.mode.w;
            pcs.updateRect.h = pcs.mode.h;
            pcs.updateRect.y = 0;
            pcs.updateRect.x = pcs.updateRect.y;
        } else {
            pcs.updateRect.h = pcs.mode.h;
            pcs.updateRect.w = pcs.mode.h * 4 / 3;
            pcs.updateRect.x = (pcs.mode.w - pcs.updateRect.w) >> 1;
            pcs.updateRect.y = 0;
        }
        gs.screenseg.fill(0);
        pcs.grmode = EGAgr;
        pcs.joystick[2].device = -1;
        pcs.joystick[1].device = pcs.joystick[2].device;
        _loadctrls(pas, pcs);
        if pcs.grmode as u32 == VGAgr as i32 as u32 && _vgaok as i32 != 0 {
            pcs.grmode = VGAgr;
        } else if pcs.grmode as u32 >= EGAgr as i32 as u32 && _egaok as i32 != 0 {
            pcs.grmode = EGAgr;
        } else {
            pcs.grmode = CGAgr;
        }
        let filename = format!("SOUNDS.{port_temp__extension}");
        pas.SoundData = bloadin(&filename) as *mut SPKRtable;
        StartupSound(pas);
        SetupKBD(pcs);
        initrndt(1, pas);
        initrnd(1, pas);
        _loadhighscores(pcs);
        loadgrfiles(gs, cps, pcs);
        SetupEmulatedVBL(pas);
    }
}

pub unsafe fn _quit(error: *const i8, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    if *error == 0 {
        _savehighscores(pcs);
        _savectrls(pas, pcs);
    } else {
        puts(error);
        puts(b"\n\0" as *const u8 as *const i8);
        puts(b"\n\0" as *const u8 as *const i8);
        puts(b"For techinical assistance with running this software\n\0" as *const u8 as *const i8);
        puts(b"    call Softdisk Publishing at 1-318-221-8311\n\0" as *const u8 as *const i8);
        puts(b"\n\0" as *const u8 as *const i8);
        std::process::exit(1);
    }
    ShutdownSound(pas);
    ShutdownJoysticks(pcs);
    safe_SDL_DestroyRenderer(pcs.renderer);
    safe_SDL_DestroyWindow(pcs.window);
    pcs.renderer = 0 as *mut SDL_Renderer;
    pcs.window = 0 as *mut SDL_Window;
    std::process::exit(0);
}
