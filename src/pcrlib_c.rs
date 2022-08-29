use std::convert::TryInto;
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::Duration;
use std::{fs, mem, thread};

use ::libc;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{TextureAccess, TextureCreator};
use sdl2::sys::SDL_WindowFlags;
use sdl2::video::WindowContext;
use serdine::{Deserialize, Serialize};

use crate::catacomb::loadgrfiles;
use crate::cpanel_state::CpanelState;
use crate::ctl_panel_type::ctlpaneltype;
use crate::input_type::inputtype::*;
use crate::pcrlib_a::{initrnd, initrndt, SetupEmulatedVBL, StartupSound};
use crate::pcrlib_a_state::PcrlibAState;
use crate::pcrlib_c_state::PcrlibCState;
use crate::rc_sdl::RcSdl;
use crate::sound_type::soundtype::{self, *};
use crate::spkr_table::SPKRtable;
use crate::{
    catasm::drawchartile,
    control_struct::ControlStruct,
    demo_enum::demoenum,
    dir_type::dirtype::*,
    extra_constants::port_temp__extension,
    global_state::GlobalState,
    gr_type::grtype::{self, *},
    pcrlib_a::{drawchar, PlaySound, ShutdownSound, WaitVBL},
    safe_sdl::*,
    scan_codes::*,
    scores::scores,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub type SDL_bool = u32;

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

impl SDL_Event {
    fn is_event_type(&self, event_type: u32) -> bool {
        unsafe { self.type_0 == event_type }
    }

    fn key_scancode(&self) -> SDL_Scancode {
        unsafe { self.key.keysym.scancode }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct joyinfo_t {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub device: i32,
    pub isgamecontroller: bool,
}

impl joyinfo_t {
    fn controller(&self) -> *mut SDL_GameController {
        unsafe { self.c2rust_unnamed.controller }
    }

    fn joy(&self) -> *mut SDL_Joystick {
        unsafe { self.c2rust_unnamed.joy }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub controller: *mut SDL_GameController,
    pub joy: *mut SDL_Joystick,
}

// Rust port: unnecessary in Rust (false is the default)
//
// pub fn SetupKBD(pcs: &mut PcrlibCState) {
//     for i in 0..128 {
//         pcs.keydown[i] = false;
//     }
// }

pub fn ProcessEvents(pcs: &mut PcrlibCState) {
    pcs.mouseEvent = false;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.is_event_type(SDL_KEYDOWN) {
            pcs.keydown[event.key_scancode() as usize] = true;
            pcs.lastkey = event.key_scancode();
        } else if event.is_event_type(SDL_KEYUP) {
            pcs.keydown[event.key_scancode() as usize] = false;
        } else if event.is_event_type(SDL_MOUSEMOTION) {
            pcs.mouseEvent = true;
        }
    }
}

/*
=======================
=
= WatchUIEvents
= Event filter which hooks into the user interface (trying to close the window
= or other windowing events).
=
=======================
*/

pub fn WatchUIEvents(event: Event, userdata: *mut SDLEventPayload, sdl: RcSdl) {
    unsafe {
        let userdata = &*userdata;

        match event {
            Event::Quit { .. } => {
                // We're quitting, so we can deallocate the userdata (although it's not strictly necessary).
                // This approach works because we're not in a multithreaded contenxt, so this function is
                // invoked only once a a time.
                let userdata = Box::from_raw(userdata as *const _ as *mut SDLEventPayload);

                _quit(None, &mut *userdata.pas, &mut *userdata.pcs);
            }
            Event::Window {
                win_event: WindowEvent::FocusLost,
                ..
            } => {
                let pcs = &mut *userdata.pcs;
                pcs.hasFocus = false;
                CheckMouseMode(pcs, &sdl);
            }
            Event::Window {
                win_event: WindowEvent::FocusGained,
                ..
            } => {
                let pcs = &mut *userdata.pcs;

                // Try to wait until the window obtains mouse focus before
                // regrabbing input in order to try to prevent grabbing while
                // the user is trying to move the window around.
                while sdl.mouse().focused_window_id()
                    != Some(pcs.renderer.as_ref().unwrap().window().id())
                {
                    sdl.event_pump().pump_events();
                    // Rust port: in the SDL port, this called `SDL_Delay`, however, the Rust sdl2
                    // crate recommeds to use thread::sleep(). This also simplifies a BCK issue,
                    // because `Timer#delay()` requires a mutable sdl instance, which is a problem
                    // when the timer instance is owned by RcSdl.
                    thread::sleep(Duration::from_millis(10));
                }

                pcs.hasFocus = true;
                CheckMouseMode(pcs, &sdl);
            }
            _ => {}
        }
    }
}

pub fn ControlKBD(pcs: &mut PcrlibCState) -> ControlStruct {
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: false,
        button2: false,
    };
    if pcs.keydown[pcs.key[north as i32 as usize] as usize] {
        ymove = -1;
    }
    if pcs.keydown[pcs.key[east as i32 as usize] as usize] {
        xmove = 1;
    }
    if pcs.keydown[pcs.key[south as i32 as usize] as usize] {
        ymove = 1;
    }
    if pcs.keydown[pcs.key[west as i32 as usize] as usize] {
        xmove = -1;
    }
    if pcs.keydown[pcs.key[northeast as i32 as usize] as usize] {
        ymove = -1;
        xmove = 1;
    }
    if pcs.keydown[pcs.key[northwest as i32 as usize] as usize] {
        ymove = -1;
        xmove = -1;
    }
    if pcs.keydown[pcs.key[southeast as i32 as usize] as usize] {
        ymove = 1;
        xmove = 1;
    }
    if pcs.keydown[pcs.key[southwest as i32 as usize] as usize] {
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

/*
============================
=
= ControlMouse
=
============================
*/

pub fn ControlMouse(pcs: &mut PcrlibCState, sdl: &RcSdl) -> ControlStruct {
    /* mickeys the mouse has moved */

    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: false,
        button2: false,
    };

    /* mouse status */
    let mouse_state = sdl.event_pump().relative_mouse_state();

    let newx = mouse_state.x();
    let newy = mouse_state.y();

    for (button, pressed) in mouse_state.mouse_buttons() {
        match button {
            MouseButton::Left => action.button1 = pressed,
            MouseButton::Right => action.button2 = pressed,
            _ => {}
        }
    }

    if !pcs.mouseEvent {
        action.dir = nodir;

        return action;
    }

    let xmove = if newx > pcs.MouseSensitivity {
        1
    } else if newx < -pcs.MouseSensitivity {
        -1
    } else {
        0
    };
    let ymove = if newy > pcs.MouseSensitivity {
        1
    } else if newy < -pcs.MouseSensitivity {
        -1
    } else {
        0
    };

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

    action
}

fn ShutdownJoysticks(pcs: &mut PcrlibCState) {
    let mut j: u32 = 0;
    j = 1;
    while j < 3 {
        if !(pcs.joystick[j as usize].device < 0) {
            if pcs.joystick[j as usize].isgamecontroller {
                safe_SDL_GameControllerClose(pcs.joystick[j as usize].controller());
            } else {
                safe_SDL_JoystickClose(pcs.joystick[j as usize].joy());
            }
            pcs.joystick[j as usize].device = -1;
        }
        j = j.wrapping_add(1);
    }
}

pub fn ProbeJoysticks(pcs: &mut PcrlibCState, sdl: &RcSdl) {
    if pcs.joystick[1].device > 0 || pcs.joystick[2].device > 0 {
        ShutdownJoysticks(pcs);
    }
    for j in 1..3 {
        if j - 1 >= sdl.joystick().num_joysticks().unwrap() as i32 {
            pcs.joystick[j as usize].device = -1;
        } else {
            pcs.joystick[j as usize].device = j - 1;
            pcs.joystick[j as usize].isgamecontroller = safe_SDL_IsGameController(j - 1) != 0;
            if safe_SDL_IsGameController(j - 1) as u64 != 0 {
                pcs.joystick[j as usize].c2rust_unnamed.controller =
                    safe_SDL_GameControllerOpen(j - 1);
            } else {
                pcs.joystick[j as usize].c2rust_unnamed.joy = safe_SDL_JoystickOpen(j - 1);
            }
        }
    }
}

pub fn ReadJoystick(joynum: i32, xcount: &mut i32, ycount: &mut i32, pcs: &mut PcrlibCState) {
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    *xcount = 0;
    *ycount = 0;
    safe_SDL_JoystickUpdate();
    if pcs.joystick[joynum as usize].isgamecontroller {
        a1 = safe_SDL_GameControllerGetAxis(
            pcs.joystick[joynum as usize].controller(),
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as i32;
        a2 = safe_SDL_GameControllerGetAxis(
            pcs.joystick[joynum as usize].controller(),
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as i32;
    } else {
        a1 = safe_SDL_JoystickGetAxis(pcs.joystick[joynum as usize].joy(), 0) as i32;
        a2 = safe_SDL_JoystickGetAxis(pcs.joystick[joynum as usize].joy(), 1) as i32;
    }
    *xcount = a1;
    *ycount = a2;
}

pub fn ControlJoystick(joynum: i32, pcs: &mut PcrlibCState) -> ControlStruct {
    let mut joyx: i32 = 0;
    let mut joyy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: false,
        button2: false,
    };
    ReadJoystick(joynum, &mut joyx, &mut joyy, pcs);
    if pcs.joystick[joynum as usize].isgamecontroller {
        action.button1 = safe_SDL_GameControllerGetButton(
            pcs.joystick[joynum as usize].controller(),
            SDL_CONTROLLER_BUTTON_A,
        ) != 0;
        action.button2 = safe_SDL_GameControllerGetButton(
            pcs.joystick[joynum as usize].controller(),
            SDL_CONTROLLER_BUTTON_B,
        ) != 0;
    } else {
        action.button1 = safe_SDL_JoystickGetButton(pcs.joystick[joynum as usize].joy(), 0) != 0;
        action.button2 = safe_SDL_JoystickGetButton(pcs.joystick[joynum as usize].joy(), 1) != 0;
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

pub fn ControlPlayer(
    player: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
    sdl: &RcSdl,
) -> ControlStruct {
    let mut ret: ControlStruct = ControlStruct {
        dir: north,
        button1: false,
        button2: false,
    };
    ProcessEvents(pcs);
    if gs.indemo == demoenum::notdemo || gs.indemo == demoenum::recording {
        match pcs.playermode[player as usize] as u32 {
            1 => {
                ret = ControlMouse(pcs, sdl);
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
        ret.button1 = (val & 1) != 0;
        ret.button2 = ((val & 2) >> 1) != 0;
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

    loadFile(&filename, &mut temp_port_demobuffer);
    pcs.demobuffer
        .copy_from_slice(&temp_port_demobuffer.map(|b| b as u8));
    pcs.level = pcs.demobuffer[0] as i16;
    pcs.demoptr = 1;
    gs.indemo = demoenum::demoplay;
}

pub fn SaveDemo(demonum: u8, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let str = format!("DEMO{demonum}.{port_temp__extension}");

    SaveFile(&str, &pcs.demobuffer[..pcs.demoptr]);

    gs.indemo = demoenum::notdemo;
}

////////////////////////
//
// StartDemo
//
////////////////////////

/*=========================================================================*/

pub fn clearkeys(pcs: &mut PcrlibCState) {
    while bioskey(1, pcs) != 0 {
        bioskey(0, pcs);
    }
    for i in 0..128 {
        pcs.keydown[i] = false;
    }
}

/*
==============================================
=
= Load a *LARGE* file into a FAR buffer!
= by John Romero (C) 1990 PCRcade
=
==============================================
*/

/// Using a Vec as dest buffer would be more convenient and idiomatic, however, routines may rely on
/// a certain buffer length.
/// An alternative is to pass the intended destination length, but there isn't a significant difference.
pub fn loadFile(filename: &str, dest: &mut [u8]) -> usize {
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

pub fn SaveFile(filename: &str, buffer: &[u8]) {
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

/*
====================================
=
= bloadin
= Paraligns just enough space and bloads in the
= specified file, returning a pointer to the start
=
====================================
*/

pub fn bloadin(filename: &str) -> Result<Vec<u8>, io::Error> {
    let file_meta = fs::metadata(filename);

    let mut buffer = vec![0; file_meta?.len() as usize];

    loadFile(filename, &mut buffer);

    Ok(buffer)
}

/*==================================================================================*/

/*
** Graphic routines
*/

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

pub fn expwin(
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
    WaitVBL();
    centerwindow(width, height, gs, pcs);
}

fn expwinh(
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
    WaitVBL();
    centerwindow(width, height, gs, pcs);
}

fn expwinv(
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
    WaitVBL();
    centerwindow(width, height, gs, pcs);
}

/////////////////////////
//
// get
// Flash a cursor at sx,sy and waits for a user bioskey
//
/////////////////////////

pub fn bioskey(cmd: i32, pcs: &mut PcrlibCState) -> u32 {
    if pcs.lastkey != 0 {
        let oldkey = pcs.lastkey;
        if cmd != 1 {
            pcs.lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }

    let mut event = SDL_Event { type_0: 0 };
    while safe_SDL_PollEvent(&mut event) != 0 {
        if event.is_event_type(SDL_KEYDOWN) {
            if cmd == 1 {
                pcs.lastkey = event.key_scancode();
                return pcs.lastkey;
            } else {
                return event.key_scancode();
            };
        }
    }
    pcs.lastkey
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

    let pixel_bytes = pcs
        .conv
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect::<Vec<_>>();

    pcs.sdltexture
        .update(None, pixel_bytes.as_slice(), 320 * mem::size_of::<u32>())
        .unwrap();
    pcs.renderer.as_mut().unwrap().clear();
    pcs.renderer
        .as_mut()
        .unwrap()
        .copy(&pcs.sdltexture, None, Some(pcs.updateRect))
        .unwrap();
    pcs.renderer.as_mut().unwrap().present();
}

pub fn get(gs: &mut GlobalState, pcs: &mut PcrlibCState) -> i32 {
    let mut key = 0;

    loop {
        let mut cycle = 9;
        loop {
            key = bioskey(0, pcs);
            if key != 0 || cycle == 13 {
                break;
            }
            drawchar(pcs.sx, pcs.sy, cycle, gs, pcs);
            cycle += 1;
            UpdateScreen(gs, pcs);
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
        }
        if key != 0 {
            break;
        }
    }
    drawchar(pcs.sx, pcs.sy, ' ' as i32, gs, pcs);
    UpdateScreen(gs, pcs);

    let scancode = Scancode::from_i32(key as i32).unwrap();
    Keycode::from_scancode(scancode).unwrap() as i32 // take it out of the buffer
}

/////////////////////////
//
// print
// Prints a string at sx,sy.  No clipping!!!
//
/////////////////////////

/// Reference print routine. &[u8] is used, because this in not necessarily a textual string.
///
pub fn print(str_0: &[u8], gs: &mut GlobalState, pcs: &mut PcrlibCState) {
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

/// Rust port: convenience.
///
pub fn print_str(str_0: &str, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    print(str_0.as_bytes(), gs, pcs);
}

// For help screen
pub fn printchartile(str_0: &[u8], gs: &mut GlobalState, pcs: &mut PcrlibCState) {
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
                drawchartile(pcs.sx, pcs.sy, *ch_0 as i32, gs, pcs);
                pcs.sx += 1;
            }
        }
    }
}

/*========================================================================*/

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
    print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// print hex
//
////////////////////////////////////////////////////////////////////
/// Rust port: Prints a word in padded hex, prefixed by `$`.
fn _printhex(value: u32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let fmt_value = format!("${:04X}", value);
    print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// print bin
//
////////////////////////////////////////////////////////////////////
/// Rust port: Prints a word in padded binary, prefixed by `%`; dead code.
fn _printbin(value: u32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let fmt_value = format!("%{:016b}", value);
    print_str(&fmt_value, gs, pcs);
}

////////////////////////////////////////////////////////////////////
//
// center print
//
////////////////////////////////////////////////////////////////////
fn _printc(string: &CString, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 1 + gs.screencenter.x - string.as_bytes().len() as i32;
    print(string.as_bytes(), gs, pcs);
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
pub fn _inputint(gs: &mut GlobalState, pcs: &mut PcrlibCState) -> u32 {
    let mut string = vec![0; 18];
    let hexstr = b"0123456789ABCDEF";
    let mut value = 0;

    _input(&mut string, 17, gs, pcs);

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
fn _input(string: &mut [u8], max: usize, gs: &mut GlobalState, pcs: &mut PcrlibCState) -> i32 {
    let mut key_ = 0;
    let mut count = 0;

    loop {
        key_ = (get(gs, pcs) as u8).to_ascii_uppercase();
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

const _cgaok: bool = true;

pub const _egaok: bool = true;

pub const _vgaok: bool = false;

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

// Enable and disable mouse grabbing
pub fn CheckMouseMode(pcs: &mut PcrlibCState, sdl: &RcSdl) {
    sdl.mouse().set_relative_mouse_mode(
        pcs.hasFocus && (pcs.playermode[1] == mouse || pcs.playermode[2] == mouse),
    )
}

////////////////////////
//
// _loadctrls
// Tries to load the control panel settings
// creates a default if not present
//
////////////////////////

pub fn _loadctrls(pas: &mut PcrlibAState, pcs: &mut PcrlibCState, sdl: &RcSdl) {
    let str = format!("CTLPANEL.{port_temp__extension}");
    // Rust port: the original flags where O_RDONLY, O_BINARY, S_IRUSR, S_IWUSR.
    // For simplicity, we do a standard file open.
    if let Ok(file) = File::open(&str) {
        let ctlpanel = ctlpaneltype::deserialize(file);

        pcs.grmode = ctlpanel.grmode as grtype;
        pas.soundmode = ctlpanel.soundmode as soundtype;
        for i in 0..3 {
            pcs.playermode[i] = ctlpanel.playermode[i].into();
            pcs.JoyXlow[i] = ctlpanel.JoyXlow[i] as i32;
            pcs.JoyYlow[i] = ctlpanel.JoyYlow[i] as i32;
            pcs.JoyXhigh[i] = ctlpanel.JoyXhigh[i] as i32;
            pcs.JoyYhigh[i] = ctlpanel.JoyYhigh[i] as i32;

            if pcs.playermode[i] == mouse {
                CheckMouseMode(pcs, sdl);
            }

            if pcs.playermode[i] == joystick1 || pcs.playermode[i] == joystick2 {
                ProbeJoysticks(pcs, sdl);
                if (pcs.playermode[i] == joystick1 && pcs.joystick[1].device < 0)
                    || (pcs.playermode[i] == joystick2 && pcs.joystick[2].device < 0)
                {
                    pcs.playermode[i] = keyboard;
                }
            }
        }
        pcs.MouseSensitivity = ctlpanel.MouseSensitivity as i32;
        for i in 0..8 {
            pcs.key[i] = DOSScanCodeMap[ctlpanel.key[i] as usize];
        }
        pcs.keyB1 = DOSScanCodeMap[ctlpanel.keyB1 as usize];
        pcs.keyB2 = DOSScanCodeMap[ctlpanel.keyB2 as usize];
    } else {
        //
        // set up default control panel settings
        //
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

        pcs.key[north as usize] = SDL_SCANCODE_UP;
        pcs.key[northeast as usize] = SDL_SCANCODE_PAGEUP;
        pcs.key[east as usize] = SDL_SCANCODE_RIGHT;
        pcs.key[southeast as usize] = SDL_SCANCODE_PAGEDOWN;
        pcs.key[south as usize] = SDL_SCANCODE_DOWN;
        pcs.key[southwest as usize] = SDL_SCANCODE_END;
        pcs.key[west as usize] = SDL_SCANCODE_LEFT;
        pcs.key[northwest as usize] = SDL_SCANCODE_HOME;
        pcs.keyB1 = SDL_SCANCODE_LCTRL;
        pcs.keyB2 = SDL_SCANCODE_LALT;
    }
}

pub fn _savectrls(pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut ctlpanel = ctlpaneltype::default();
    let str = format!("CTLPANEL.{port_temp__extension}");

    // Rust port: Original flags: (O_WRONLY | O_BINARY | O_CREAT | O_TRUNC, S_IREAD | S_IWRITE); for
    // simplicity, we do a straight create.
    if let Ok(file) = File::create(str) {
        ctlpanel.grmode = pcs.grmode;
        ctlpanel.soundmode = pas.soundmode;
        for i in 0..3 {
            ctlpanel.playermode[i] = pcs.playermode[i] as u16;
            ctlpanel.JoyXlow[i] = pcs.JoyXlow[i] as i16;
            ctlpanel.JoyYlow[i] = pcs.JoyYlow[i] as i16;
            ctlpanel.JoyXhigh[i] = pcs.JoyXhigh[i] as i16;
            ctlpanel.JoyYhigh[i] = pcs.JoyYhigh[i] as i16;
        }
        ctlpanel.MouseSensitivity = pcs.MouseSensitivity as i16;
        for i in 0..8 {
            ctlpanel.key[i as usize] = ScancodeToDOS(pcs.key[i as usize] as SDL_Scancode) as u8;
        }
        ctlpanel.keyB1 = ScancodeToDOS(pcs.keyB1 as SDL_Scancode) as u8;
        ctlpanel.keyB2 = ScancodeToDOS(pcs.keyB2 as SDL_Scancode) as u8;

        ctlpanel.serialize(file);
    }
}

pub fn _loadhighscores(pcs: &mut PcrlibCState) {
    let filename = format!("SCORES.{port_temp__extension}");
    let mut buffer = [0_u8; scores::ondisk_struct_size() * 5];

    let bytes_loaded = loadFile(&filename, &mut buffer);

    if bytes_loaded > 0 {
        // Rust port: there isn't a type for the whole scores data (file), so we deserialize in
        // chunks.
        for (highscore, score_buffer) in pcs
            .highscores
            .iter_mut()
            .zip(buffer.chunks_exact(scores::ondisk_struct_size()))
        {
            *highscore = Deserialize::deserialize(score_buffer);
        }
    } else {
        for i in 0..5 {
            pcs.highscores[i].score = 100;
            pcs.highscores[i].level = 1;
            pcs.highscores[i].initials = "PCR".as_bytes().try_into().unwrap();
        }
    }
}

pub fn _savehighscores(pcs: &mut PcrlibCState) {
    let mut buffer = Vec::new();

    Serialize::serialize(&pcs.highscores, &mut buffer);

    let str = format!("SCORES.{port_temp__extension}");

    SaveFile(&str, &buffer);
}

pub fn _showhighscores(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    let mut h: i64 = 0;
    centerwindow(17, 17, gs, pcs);
    print_str("\n   HIGH SCORES\n\n", gs, pcs);
    print_str(" #  SCORE LV  BY\n", gs, pcs);
    print_str(" - ------ -- ---\n", gs, pcs);
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
        print_str(&h.to_string(), gs, pcs);
        pcs.sx += 1;
        if (pcs.highscores[i as usize].level as i32) < 10 {
            pcs.sx += 1;
        }
        let str = { pcs.highscores[i as usize].level }.to_string();
        print_str(&str, gs, pcs);
        pcs.sx += 1;
        // Rust port: Watch out! Entries includes the cstring terminator, which we must skip!
        let highscore_bytes = &pcs.highscores[i as usize].initials.map(|f| f as u8)[0..=2];
        print(highscore_bytes, gs, pcs);
        print_str("\n\n", gs, pcs);
        i += 1;
    }
    let str = CString::new(format!("SCORE:{}", pcs.score)).unwrap();
    _printc(&str, gs, pcs);
}

pub fn _checkhighscore(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
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
            k = get(gs, pcs);
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

pub struct SDLEventPayload<'t> {
    pub pas: *mut PcrlibAState<'t>,
    pub pcs: *mut PcrlibCState<'t>,
}

////////////////////
//
// _setupgame
//
////////////////////

pub fn _setupgame<'s, 't>(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState<'s>,
    sdl: &'s RcSdl,
    texture_creator: &'t mut Option<TextureCreator<WindowContext>>,
) -> PcrlibCState<'t> {
    let mut windowed = false;
    let mut winWidth = 640;
    let mut winHeight = 480;
    let mut displayindex = 0;

    // Rust port: It's possible to iterate `Args`, although it doesn't get much cleaner.
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

    let mut pcs_mode = sdl
        .video()
        .current_display_mode(displayindex)
        .expect("Could not get display mode");

    let mut bounds = sdl
        .video()
        .display_bounds(displayindex)
        .expect("Could not get display mode");

    let window_flags = if windowed {
        // Rust port: the SDL port intentionally chooses SDL_WINDOWPOS_UNDEFINED; this has different
        // default behavior, depending on the system.
        bounds.x = sdl2::sys::SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        bounds.y = sdl2::sys::SDL_WINDOWPOS_UNDEFINED_MASK as i32;
        pcs_mode.w = winWidth as i32;
        pcs_mode.h = winHeight as i32;
        0
        // Rust port: WindowBuilder's defaults are position:undefined and flags:0.
    } else {
        // Rust port: There's a an explicit API for this, but then we need to separate the conditionals
        // and initialize the window builder in the middle.
        SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32
    };

    let pcs_window = sdl
        .video()
        .window("The Catacomb", pcs_mode.w as u32, pcs_mode.h as u32)
        .set_window_flags(window_flags)
        .position(bounds.x, bounds.y)
        .build()
        .expect("Failed to create SDL window");

    // Rust port: the error message is not exact (copied from the SDL port).
    // The default flags in the Rust library are 0, like the C SDL port.
    // The rendering driver index is not set, which is equivalent to the SDL port -1.
    let pcs_renderer = pcs_window
        .into_canvas()
        .build()
        .expect("Failed to create SDL window");

    texture_creator.replace(pcs_renderer.texture_creator());

    let pcs_sdltexture = texture_creator
        .as_ref()
        .unwrap()
        .create_texture(
            PixelFormatEnum::ARGB8888,
            TextureAccess::Streaming,
            320,
            200,
        )
        .expect("Could not create video buffer");

    let mut pcs_updateRect = Rect::new(0, 0, 0, 0);

    // Handle 320x200 and 640x400 specially so they are unscaled.
    if pcs_mode.w == 320 && pcs_mode.h == 200 || pcs_mode.w == 640 && pcs_mode.h == 400 {
        pcs_updateRect.w = pcs_mode.w;
        pcs_updateRect.h = pcs_mode.h;
        pcs_updateRect.y = 0;
        pcs_updateRect.x = pcs_updateRect.y;
    } else {
        // Pillar box the 4:3 game
        pcs_updateRect.h = pcs_mode.h;
        pcs_updateRect.w = pcs_mode.h * 4 / 3;
        pcs_updateRect.x = (pcs_mode.w - pcs_updateRect.w) >> 1;
        pcs_updateRect.y = 0;
    }

    // Rust port: unnecessary in Rust
    // gs.screenseg.fill(0);

    //
    // set up game's library routines
    //
    // Rust port: This is just a null value initialization; it's overwritten immediately by _loadctrls()
    // let mut pcs_grmode = EGAgr;

    let mut pcs_joystick = [joyinfo_t {
        c2rust_unnamed: C2RustUnnamed_5 {
            controller: 0 as *const SDL_GameController as *mut SDL_GameController,
        },
        device: 0,
        isgamecontroller: false,
    }; 3];
    // Invalidate joysticks.
    pcs_joystick[2].device = -1;
    pcs_joystick[1].device = pcs_joystick[2].device;

    let mut pcs = PcrlibCState::new(
        pcs_renderer,
        pcs_sdltexture,
        pcs_updateRect,
        pcs_mode,
        pcs_joystick,
    );

    _loadctrls(pas, &mut pcs, sdl);

    if pcs.grmode == VGAgr && _vgaok {
        pcs.grmode = VGAgr;
    } else if matches!(pcs.grmode, EGAgr | VGAgr) && _egaok {
        pcs.grmode = EGAgr;
    } else {
        pcs.grmode = CGAgr;
    }

    let filename = format!("SOUNDS.{port_temp__extension}");
    let sound_data_buffer = bloadin(&filename).unwrap();

    pas.SoundData = SPKRtable::deserialize(sound_data_buffer.as_slice());

    StartupSound(pas);

    // Rust port: unnecessary (see method)
    // SetupKBD(&mut pcs);

    initrndt(true, pas);
    initrnd(true, pas);

    _loadhighscores(&mut pcs);

    loadgrfiles(gs, cps, &mut pcs);

    SetupEmulatedVBL(pas, &sdl);

    pcs
}

////////////////////
//
// _quit
//
////////////////////

// Rust port: Redesigning the exit is a pain. The original game was not designed to exit by interrupting
// the main loop, so even if exit points are added to the code on each level leading to the _quit() call,
// the game may still not exit in certain points. For this reason, the SDL port approach is left as is.
//
// Rust port: There are no occurrences (in the SDL port, at least) where an error is passed.
// In the original version, there are two cases - out of memory, and a certain EXE file not found.
pub fn _quit(error: Option<String>, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    if let Some(error) = &error {
        print!("{}", error);
        println!();
        println!();
        println!("For techinical assistance with running this software");
        println!("    call Softdisk Publishing at 1-318-221-8311");
        println!();
        std::process::exit(1);
    } else {
        _savehighscores(pcs);
        _savectrls(pas, pcs);
    }

    ShutdownSound(pas);
    ShutdownJoysticks(pcs);

    let renderer = pcs.renderer.take().unwrap();
    let window_context = renderer.window().context();

    drop(renderer);
    drop(window_context);

    // Rust port: Not necessary to nullify the pointers.
    // pcs.renderer = ptr::null();
    // pcs.window = ptr::null();

    std::process::exit(0);
}
