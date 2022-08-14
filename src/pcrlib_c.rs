use std::convert::TryInto;

use ::libc;
use libc::O_RDONLY;

use crate::{
    catasm::drawchartile,
    demo_enum::demoenum,
    dir_type::dirtype::{self, *},
    extra_constants::{_extension, O_BINARY, SDL_BUTTON_LEFT, SDL_BUTTON_RIGHT},
    extra_macros::SDL_BUTTON,
    extra_types::boolean,
    gr_type::grtype::{self, *},
    indemo,
    sdl_scan_codes::*,
};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type SDL_Window;
    pub type _SDL_Joystick;
    pub type _SDL_GameController;
    pub type SDL_SysWMmsg;
    pub type SDL_Renderer;
    pub type SDL_Texture;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: u64) -> i64;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn toupper(_: i32) -> i32;
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
    fn SDL_GetError() -> *const i8;
    fn strlen(_: *const i8) -> u64;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn loadgrfiles();
    fn WaitVBL();
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn drawchar(x: i32, y: i32, charnum: i32);
    fn initrnd(randomize: boolean);
    fn initrndt(randomize: boolean);
    fn PlaySound(sound: i32);
    fn ShutdownSound();
    fn StartupSound();
    static mut SoundData: *mut SPKRtable;
    static mut soundmode: soundtype;
    fn SDL_Quit();
    fn SDL_Init(flags: u32) -> i32;
    fn SDL_Delay(ms: u32);
    fn SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer);
    fn SDL_RenderPresent(renderer_0: *mut SDL_Renderer);
    fn SDL_RenderCopy(
        renderer_0: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_Rect,
        dstrect: *const SDL_Rect,
    ) -> i32;
    fn SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> i32;
    fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const libc::c_void,
        pitch: i32,
    ) -> i32;
    fn SDL_CreateTexture(
        renderer_0: *mut SDL_Renderer,
        format: u32,
        access: i32,
        w: i32,
        h: i32,
    ) -> *mut SDL_Texture;
    fn SDL_CreateRenderer(window_0: *mut SDL_Window, index: i32, flags: u32) -> *mut SDL_Renderer;
    fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *mut libc::c_void);
    fn SDL_PollEvent(event: *mut SDL_Event) -> i32;
    fn SDL_PumpEvents();
    fn SDL_GetDisplayBounds(displayIndex: i32, rect: *mut SDL_Rect) -> i32;
    fn SDL_GetCurrentDisplayMode(displayIndex: i32, mode_0: *mut SDL_DisplayMode) -> i32;
    fn SDL_CreateWindow(
        title: *const i8,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32,
    ) -> *mut SDL_Window;
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
    fn SetupEmulatedVBL();
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
pub type C2RustUnnamed_0 = u32;
pub const SDL_PIXELFORMAT_EXTERNAL_OES: C2RustUnnamed_0 = 542328143;
pub const SDL_PIXELFORMAT_NV21: C2RustUnnamed_0 = 825382478;
pub const SDL_PIXELFORMAT_NV12: C2RustUnnamed_0 = 842094158;
pub const SDL_PIXELFORMAT_YVYU: C2RustUnnamed_0 = 1431918169;
pub const SDL_PIXELFORMAT_UYVY: C2RustUnnamed_0 = 1498831189;
pub const SDL_PIXELFORMAT_YUY2: C2RustUnnamed_0 = 844715353;
pub const SDL_PIXELFORMAT_IYUV: C2RustUnnamed_0 = 1448433993;
pub const SDL_PIXELFORMAT_YV12: C2RustUnnamed_0 = 842094169;
pub const SDL_PIXELFORMAT_ABGR32: C2RustUnnamed_0 = 373694468;
pub const SDL_PIXELFORMAT_BGRA32: C2RustUnnamed_0 = 372645892;
pub const SDL_PIXELFORMAT_ARGB32: C2RustUnnamed_0 = 377888772;
pub const SDL_PIXELFORMAT_RGBA32: C2RustUnnamed_0 = 376840196;
pub const SDL_PIXELFORMAT_ARGB2101010: C2RustUnnamed_0 = 372711428;
pub const SDL_PIXELFORMAT_BGRA8888: C2RustUnnamed_0 = 377888772;
pub const SDL_PIXELFORMAT_ABGR8888: C2RustUnnamed_0 = 376840196;
pub const SDL_PIXELFORMAT_RGBA8888: C2RustUnnamed_0 = 373694468;
pub const SDL_PIXELFORMAT_ARGB8888: C2RustUnnamed_0 = 372645892;
pub const SDL_PIXELFORMAT_BGRX8888: C2RustUnnamed_0 = 375789572;
pub const SDL_PIXELFORMAT_BGR888: C2RustUnnamed_0 = 374740996;
pub const SDL_PIXELFORMAT_RGBX8888: C2RustUnnamed_0 = 371595268;
pub const SDL_PIXELFORMAT_RGB888: C2RustUnnamed_0 = 370546692;
pub const SDL_PIXELFORMAT_BGR24: C2RustUnnamed_0 = 390076419;
pub const SDL_PIXELFORMAT_RGB24: C2RustUnnamed_0 = 386930691;
pub const SDL_PIXELFORMAT_BGR565: C2RustUnnamed_0 = 357896194;
pub const SDL_PIXELFORMAT_RGB565: C2RustUnnamed_0 = 353701890;
pub const SDL_PIXELFORMAT_BGRA5551: C2RustUnnamed_0 = 360976386;
pub const SDL_PIXELFORMAT_ABGR1555: C2RustUnnamed_0 = 359862274;
pub const SDL_PIXELFORMAT_RGBA5551: C2RustUnnamed_0 = 356782082;
pub const SDL_PIXELFORMAT_ARGB1555: C2RustUnnamed_0 = 355667970;
pub const SDL_PIXELFORMAT_BGRA4444: C2RustUnnamed_0 = 360845314;
pub const SDL_PIXELFORMAT_ABGR4444: C2RustUnnamed_0 = 359796738;
pub const SDL_PIXELFORMAT_RGBA4444: C2RustUnnamed_0 = 356651010;
pub const SDL_PIXELFORMAT_ARGB4444: C2RustUnnamed_0 = 355602434;
pub const SDL_PIXELFORMAT_BGR555: C2RustUnnamed_0 = 357764866;
pub const SDL_PIXELFORMAT_RGB555: C2RustUnnamed_0 = 353570562;
pub const SDL_PIXELFORMAT_RGB444: C2RustUnnamed_0 = 353504258;
pub const SDL_PIXELFORMAT_RGB332: C2RustUnnamed_0 = 336660481;
pub const SDL_PIXELFORMAT_INDEX8: C2RustUnnamed_0 = 318769153;
pub const SDL_PIXELFORMAT_INDEX4MSB: C2RustUnnamed_0 = 304088064;
pub const SDL_PIXELFORMAT_INDEX4LSB: C2RustUnnamed_0 = 303039488;
pub const SDL_PIXELFORMAT_INDEX1MSB: C2RustUnnamed_0 = 287310080;
pub const SDL_PIXELFORMAT_INDEX1LSB: C2RustUnnamed_0 = 286261504;
pub const SDL_PIXELFORMAT_UNKNOWN: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = u32;
pub const SDL_WINDOW_VULKAN: C2RustUnnamed_1 = 268435456;
pub const SDL_WINDOW_POPUP_MENU: C2RustUnnamed_1 = 524288;
pub const SDL_WINDOW_TOOLTIP: C2RustUnnamed_1 = 262144;
pub const SDL_WINDOW_UTILITY: C2RustUnnamed_1 = 131072;
pub const SDL_WINDOW_SKIP_TASKBAR: C2RustUnnamed_1 = 65536;
pub const SDL_WINDOW_ALWAYS_ON_TOP: C2RustUnnamed_1 = 32768;
pub const SDL_WINDOW_MOUSE_CAPTURE: C2RustUnnamed_1 = 16384;
pub const SDL_WINDOW_ALLOW_HIGHDPI: C2RustUnnamed_1 = 8192;
pub const SDL_WINDOW_FOREIGN: C2RustUnnamed_1 = 2048;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: C2RustUnnamed_1 = 4097;
pub const SDL_WINDOW_MOUSE_FOCUS: C2RustUnnamed_1 = 1024;
pub const SDL_WINDOW_INPUT_FOCUS: C2RustUnnamed_1 = 512;
pub const SDL_WINDOW_INPUT_GRABBED: C2RustUnnamed_1 = 256;
pub const SDL_WINDOW_MAXIMIZED: C2RustUnnamed_1 = 128;
pub const SDL_WINDOW_MINIMIZED: C2RustUnnamed_1 = 64;
pub const SDL_WINDOW_RESIZABLE: C2RustUnnamed_1 = 32;
pub const SDL_WINDOW_BORDERLESS: C2RustUnnamed_1 = 16;
pub const SDL_WINDOW_HIDDEN: C2RustUnnamed_1 = 8;
pub const SDL_WINDOW_SHOWN: C2RustUnnamed_1 = 4;
pub const SDL_WINDOW_OPENGL: C2RustUnnamed_1 = 2;
pub const SDL_WINDOW_FULLSCREEN: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = u32;
pub const SDL_WINDOWEVENT_HIT_TEST: C2RustUnnamed_2 = 16;
pub const SDL_WINDOWEVENT_TAKE_FOCUS: C2RustUnnamed_2 = 15;
pub const SDL_WINDOWEVENT_CLOSE: C2RustUnnamed_2 = 14;
pub const SDL_WINDOWEVENT_FOCUS_LOST: C2RustUnnamed_2 = 13;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: C2RustUnnamed_2 = 12;
pub const SDL_WINDOWEVENT_LEAVE: C2RustUnnamed_2 = 11;
pub const SDL_WINDOWEVENT_ENTER: C2RustUnnamed_2 = 10;
pub const SDL_WINDOWEVENT_RESTORED: C2RustUnnamed_2 = 9;
pub const SDL_WINDOWEVENT_MAXIMIZED: C2RustUnnamed_2 = 8;
pub const SDL_WINDOWEVENT_MINIMIZED: C2RustUnnamed_2 = 7;
pub const SDL_WINDOWEVENT_SIZE_CHANGED: C2RustUnnamed_2 = 6;
pub const SDL_WINDOWEVENT_RESIZED: C2RustUnnamed_2 = 5;
pub const SDL_WINDOWEVENT_MOVED: C2RustUnnamed_2 = 4;
pub const SDL_WINDOWEVENT_EXPOSED: C2RustUnnamed_2 = 3;
pub const SDL_WINDOWEVENT_HIDDEN: C2RustUnnamed_2 = 2;
pub const SDL_WINDOWEVENT_SHOWN: C2RustUnnamed_2 = 1;
pub const SDL_WINDOWEVENT_NONE: C2RustUnnamed_2 = 0;

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
pub const SDL_CONTROLLER_AXIS_MAX: SDL_GameControllerAxis = 6;
pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = 5;
pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = 4;
pub const SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = 3;
pub const SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = 2;
pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
pub const SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = -1;
pub type SDL_GameControllerButton = i32;
pub const SDL_CONTROLLER_BUTTON_MAX: SDL_GameControllerButton = 15;
pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: SDL_GameControllerButton = 14;
pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: SDL_GameControllerButton = 13;
pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: SDL_GameControllerButton = 12;
pub const SDL_CONTROLLER_BUTTON_DPAD_UP: SDL_GameControllerButton = 11;
pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: SDL_GameControllerButton = 10;
pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: SDL_GameControllerButton = 9;
pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: SDL_GameControllerButton = 8;
pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: SDL_GameControllerButton = 7;
pub const SDL_CONTROLLER_BUTTON_START: SDL_GameControllerButton = 6;
pub const SDL_CONTROLLER_BUTTON_GUIDE: SDL_GameControllerButton = 5;
pub const SDL_CONTROLLER_BUTTON_BACK: SDL_GameControllerButton = 4;
pub const SDL_CONTROLLER_BUTTON_Y: SDL_GameControllerButton = 3;
pub const SDL_CONTROLLER_BUTTON_X: SDL_GameControllerButton = 2;
pub const SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = 1;
pub const SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = 0;
pub const SDL_CONTROLLER_BUTTON_INVALID: SDL_GameControllerButton = -1;
pub type SDL_TouchID = i64;
pub type SDL_FingerID = i64;
pub type SDL_GestureID = i64;
pub type C2RustUnnamed_3 = u32;
pub const SDL_LASTEVENT: C2RustUnnamed_3 = 65535;
pub const SDL_USEREVENT: C2RustUnnamed_3 = 32768;
pub const SDL_RENDER_DEVICE_RESET: C2RustUnnamed_3 = 8193;
pub const SDL_RENDER_TARGETS_RESET: C2RustUnnamed_3 = 8192;
pub const SDL_SENSORUPDATE: C2RustUnnamed_3 = 4608;
pub const SDL_AUDIODEVICEREMOVED: C2RustUnnamed_3 = 4353;
pub const SDL_AUDIODEVICEADDED: C2RustUnnamed_3 = 4352;
pub const SDL_DROPCOMPLETE: C2RustUnnamed_3 = 4099;
pub const SDL_DROPBEGIN: C2RustUnnamed_3 = 4098;
pub const SDL_DROPTEXT: C2RustUnnamed_3 = 4097;
pub const SDL_DROPFILE: C2RustUnnamed_3 = 4096;
pub const SDL_CLIPBOARDUPDATE: C2RustUnnamed_3 = 2304;
pub const SDL_MULTIGESTURE: C2RustUnnamed_3 = 2050;
pub const SDL_DOLLARRECORD: C2RustUnnamed_3 = 2049;
pub const SDL_DOLLARGESTURE: C2RustUnnamed_3 = 2048;
pub const SDL_FINGERMOTION: C2RustUnnamed_3 = 1794;
pub const SDL_FINGERUP: C2RustUnnamed_3 = 1793;
pub const SDL_FINGERDOWN: C2RustUnnamed_3 = 1792;
pub const SDL_CONTROLLERDEVICEREMAPPED: C2RustUnnamed_3 = 1621;
pub const SDL_CONTROLLERDEVICEREMOVED: C2RustUnnamed_3 = 1620;
pub const SDL_CONTROLLERDEVICEADDED: C2RustUnnamed_3 = 1619;
pub const SDL_CONTROLLERBUTTONUP: C2RustUnnamed_3 = 1618;
pub const SDL_CONTROLLERBUTTONDOWN: C2RustUnnamed_3 = 1617;
pub const SDL_CONTROLLERAXISMOTION: C2RustUnnamed_3 = 1616;
pub const SDL_JOYDEVICEREMOVED: C2RustUnnamed_3 = 1542;
pub const SDL_JOYDEVICEADDED: C2RustUnnamed_3 = 1541;
pub const SDL_JOYBUTTONUP: C2RustUnnamed_3 = 1540;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed_3 = 1539;
pub const SDL_JOYHATMOTION: C2RustUnnamed_3 = 1538;
pub const SDL_JOYBALLMOTION: C2RustUnnamed_3 = 1537;
pub const SDL_JOYAXISMOTION: C2RustUnnamed_3 = 1536;
pub const SDL_MOUSEWHEEL: C2RustUnnamed_3 = 1027;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed_3 = 1026;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed_3 = 1025;
pub const SDL_MOUSEMOTION: C2RustUnnamed_3 = 1024;
pub const SDL_KEYMAPCHANGED: C2RustUnnamed_3 = 772;
pub const SDL_TEXTINPUT: C2RustUnnamed_3 = 771;
pub const SDL_TEXTEDITING: C2RustUnnamed_3 = 770;
pub const SDL_KEYUP: C2RustUnnamed_3 = 769;
pub const SDL_KEYDOWN: C2RustUnnamed_3 = 768;
pub const SDL_SYSWMEVENT: C2RustUnnamed_3 = 513;
pub const SDL_WINDOWEVENT: C2RustUnnamed_3 = 512;
pub const SDL_DISPLAYEVENT: C2RustUnnamed_3 = 336;
pub const SDL_APP_DIDENTERFOREGROUND: C2RustUnnamed_3 = 262;
pub const SDL_APP_WILLENTERFOREGROUND: C2RustUnnamed_3 = 261;
pub const SDL_APP_DIDENTERBACKGROUND: C2RustUnnamed_3 = 260;
pub const SDL_APP_WILLENTERBACKGROUND: C2RustUnnamed_3 = 259;
pub const SDL_APP_LOWMEMORY: C2RustUnnamed_3 = 258;
pub const SDL_APP_TERMINATING: C2RustUnnamed_3 = 257;
pub const SDL_QUIT: C2RustUnnamed_3 = 256;
pub const SDL_FIRSTEVENT: C2RustUnnamed_3 = 0;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct spksndtype {
    pub start: u16,
    pub priority: u8,
    pub samplerate: u8,
    pub name: [i8; 12],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SPKRtable {
    pub id: [i8; 4],
    pub filelength: u16,
    pub filler: [u16; 5],
    pub sounds: [spksndtype; 63],
    pub freqdata: [u16; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: boolean,
    pub button2: boolean,
}
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
pub struct scores {
    pub score: i32,
    pub level: i16,
    pub initials: [i8; 4],
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
#[inline]
unsafe extern "C" fn itoa(mut value: i32, mut str_0: *mut i8, mut base: i32) -> *mut i8 {
    if base == 16 {
        sprintf(str_0, b"%X\0" as *const u8 as *const i8, value);
    } else {
        sprintf(str_0, b"%d\0" as *const u8 as *const i8, value);
    }
    return str_0;
}
#[inline]
unsafe extern "C" fn ltoa(mut value: i32, mut str_0: *mut i8, mut base: i32) -> *mut i8 {
    return itoa(value, str_0, base);
}
#[no_mangle]
pub static mut ch: i8 = 0;
#[no_mangle]
pub static mut str: [i8; 80] = [0; 80];
#[no_mangle]
pub static mut playermode: [inputtype; 3] = [keyboard, keyboard, joystick1];
#[no_mangle]
pub static mut keydown: [boolean; 512] = [0; 512];
#[no_mangle]
pub static mut JoyXlow: [i32; 3] = [0; 3];
#[no_mangle]
pub static mut JoyXhigh: [i32; 3] = [0; 3];
#[no_mangle]
pub static mut JoyYlow: [i32; 3] = [0; 3];
#[no_mangle]
pub static mut JoyYhigh: [i32; 3] = [0; 3];
#[no_mangle]
pub static mut MouseSensitivity: i32 = 0;
#[no_mangle]
pub static mut mouseEvent: boolean = 0;
#[no_mangle]
pub static mut key: [i32; 8] = [0; 8];
#[no_mangle]
pub static mut keyB1: i32 = 0;
#[no_mangle]
pub static mut keyB2: i32 = 0;
#[no_mangle]
pub static mut demobuffer: [i8; 5000] = [0; 5000];
#[no_mangle]
pub static mut demoptr: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut democount: i32 = 0;
#[no_mangle]
pub static mut lastdemoval: i32 = 0;
static mut lastkey: SDL_Scancode = SDL_SCANCODE_UNKNOWN;
#[no_mangle]
pub static mut window: *mut SDL_Window = 0 as *const SDL_Window as *mut SDL_Window;
#[no_mangle]
pub static mut renderer: *mut SDL_Renderer = 0 as *const SDL_Renderer as *mut SDL_Renderer;
#[no_mangle]
pub static mut sdltexture: *mut SDL_Texture = 0 as *const SDL_Texture as *mut SDL_Texture;
#[no_mangle]
pub static mut updateRect: SDL_Rect = SDL_Rect {
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
#[no_mangle]
pub unsafe extern "C" fn SetupKBD() {
    let mut i: u32 = 0;
    i = 0;
    while i < 128 {
        keydown[i as usize] = false as boolean;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessEvents() {
    mouseEvent = false as boolean;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
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
                while SDL_GetMouseFocus() != window {
                    SDL_PumpEvents();
                    SDL_Delay(10);
                }
                hasFocus = true as boolean;
                CheckMouseMode();
            }
            _ => {}
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ControlKBD() -> ControlStruct {
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

#[no_mangle]
pub unsafe extern "C" fn ControlMouse() -> ControlStruct {
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut xmove: i32 = 0;
    let mut ymove: i32 = 0;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut buttons: i32 = SDL_GetRelativeMouseState(&mut newx, &mut newy) as i32;
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
unsafe extern "C" fn ShutdownJoysticks() {
    let mut j: u32 = 0;
    j = 1;
    while j < 3 {
        if !(joystick[j as usize].device < 0) {
            if joystick[j as usize].isgamecontroller != 0 {
                SDL_GameControllerClose(joystick[j as usize].c2rust_unnamed.controller);
            } else {
                SDL_JoystickClose(joystick[j as usize].c2rust_unnamed.joy);
            }
            joystick[j as usize].device = -(1);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProbeJoysticks() {
    let mut j: i32 = 0;
    if joystick[1].device > 0 || joystick[2].device > 0 {
        ShutdownJoysticks();
    }
    j = 1;
    while j < 3 {
        if j - 1 >= SDL_NumJoysticks() {
            joystick[j as usize].device = -(1);
        } else {
            joystick[j as usize].device = j - 1;
            joystick[j as usize].isgamecontroller = SDL_IsGameController(j - 1) as boolean;
            if SDL_IsGameController(j - 1) as u64 != 0 {
                joystick[j as usize].c2rust_unnamed.controller = SDL_GameControllerOpen(j - 1);
            } else {
                joystick[j as usize].c2rust_unnamed.joy = SDL_JoystickOpen(j - 1);
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReadJoystick(mut joynum: i32, mut xcount: *mut i32, mut ycount: *mut i32) {
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    *xcount = 0;
    *ycount = 0;
    SDL_JoystickUpdate();
    if joystick[joynum as usize].isgamecontroller != 0 {
        a1 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as i32;
        a2 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as i32;
    } else {
        a1 = SDL_JoystickGetAxis(joystick[joynum as usize].c2rust_unnamed.joy, 0) as i32;
        a2 = SDL_JoystickGetAxis(joystick[joynum as usize].c2rust_unnamed.joy, 1) as i32;
    }
    *xcount = a1;
    *ycount = a2;
}
#[no_mangle]
pub unsafe extern "C" fn ControlJoystick(mut joynum: i32) -> ControlStruct {
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
        action.button1 = (SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_A,
        ) as i32
            != 0) as i32 as boolean;
        action.button2 = (SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_B,
        ) as i32
            != 0) as i32 as boolean;
    } else {
        action.button1 = (SDL_JoystickGetButton(joystick[joynum as usize].c2rust_unnamed.joy, 0)
            as i32
            != 0) as i32 as boolean;
        action.button2 = (SDL_JoystickGetButton(joystick[joynum as usize].c2rust_unnamed.joy, 1)
            as i32
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
#[no_mangle]
pub unsafe extern "C" fn ControlPlayer(mut player: i32) -> ControlStruct {
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
        ret.dir = ((val & 4 + 8 + 16 + 32) >> 2).try_into().unwrap();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn RecordDemo() {
    demobuffer[0] = level as i8;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1) as *mut i8;
    indemo = demoenum::recording;
}
#[no_mangle]
pub unsafe extern "C" fn LoadDemo(mut demonum: i32) {
    let mut st2: [i8; 5] = [0; 5];
    strcpy(str.as_mut_ptr(), b"DEMO\0" as *const u8 as *const i8);
    itoa(demonum, st2.as_mut_ptr(), 10);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    LoadFile(str.as_mut_ptr(), demobuffer.as_mut_ptr());
    level = demobuffer[0] as i16;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1) as *mut i8;
    indemo = demoenum::demoplay;
}
#[no_mangle]
pub unsafe extern "C" fn SaveDemo(mut demonum: i32) {
    let mut st2: [i8; 5] = [0; 5];
    strcpy(str.as_mut_ptr(), b"DEMO\0" as *const u8 as *const i8);
    itoa(demonum, st2.as_mut_ptr(), 10);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        demobuffer.as_mut_ptr(),
        demoptr.offset_from(&mut *demobuffer.as_mut_ptr().offset(0) as *mut i8) as i64,
    );
    indemo = demoenum::notdemo;
}
#[no_mangle]
pub unsafe extern "C" fn clearkeys() {
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
unsafe extern "C" fn filelength(mut fd: i32) -> i64 {
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
#[no_mangle]
pub unsafe extern "C" fn LoadFile(mut filename: *mut i8, mut buffer: *mut i8) -> u64 {
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
#[no_mangle]
pub unsafe extern "C" fn SaveFile(mut filename: *mut i8, mut buffer: *mut i8, mut size: i64) {
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
#[no_mangle]
pub unsafe extern "C" fn bloadin(mut filename: *mut i8) -> *mut libc::c_void {
    let mut handle: i32 = 0;
    let mut length: i64 = 0;
    let mut location: *mut i8 = 0 as *mut i8;
    handle = open(filename, 0);
    if handle != -(1) {
        length = filelength(handle);
        location = malloc(length as u64) as *mut i8;
        close(handle);
        LoadFile(filename, location);
        return location as *mut libc::c_void;
    } else {
        return 0 as *mut libc::c_void;
    };
}
#[no_mangle]
pub static mut grmode: grtype = text;
#[no_mangle]
pub static mut charptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut tileptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut picptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut spriteptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut egaplaneofs: [u32; 4] = [0; 4];
#[no_mangle]
pub static mut sx: i32 = 0;
#[no_mangle]
pub static mut sy: i32 = 0;
#[no_mangle]
pub static mut leftedge: i32 = 0;
#[no_mangle]
pub static mut win_xl: i32 = 0;
#[no_mangle]
pub static mut win_yl: i32 = 0;
#[no_mangle]
pub static mut win_xh: i32 = 0;
#[no_mangle]
pub static mut win_yh: i32 = 0;

#[no_mangle]
pub unsafe extern "C" fn drawwindow(mut xl: i32, mut yl: i32, mut xh: i32, mut yh: i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    win_xl = xl;
    win_yl = yl;
    win_xh = xh;
    win_yh = yh;
    drawchar(xl, yl, 1);
    x = xl + 1;
    while x < xh {
        drawchar(x, yl, 2);
        x += 1;
    }
    drawchar(xh, yl, 3);
    y = yl + 1;
    while y < yh {
        drawchar(xl, y, 4);
        x = xl + 1;
        while x < xh {
            drawchar(x, y, ' ' as i32);
            x += 1;
        }
        drawchar(xh, y, 5);
        y += 1;
    }
    drawchar(xl, yh, 6);
    x = xl + 1;
    while x < xh {
        drawchar(x, yh, 7);
        x += 1;
    }
    drawchar(xh, yh, 8);
    leftedge = xl + 1;
    sx = leftedge;
    sy = yl + 1;
}
#[no_mangle]
pub unsafe extern "C" fn bar(mut xl: i32, mut yl: i32, mut xh: i32, mut yh: i32, mut ch_0: i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    y = yl;
    while y <= yh {
        x = xl;
        while x <= xh {
            drawchar(x, y, ch_0);
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn erasewindow() {
    bar(win_xl, win_yl, win_xh, win_yh, ' ' as i32);
}

pub unsafe fn centerwindow(
    mut width: i32,
    mut height: i32,
    screencenterx: &i32,
    screencentery: &i32,
) {
    let mut xl: i32 = screencenterx - width / 2;
    let mut yl: i32 = screencentery - height / 2;
    drawwindow(xl, yl, xl + width + 1, yl + height + 1);
}

pub unsafe fn expwin(mut width: i32, mut height: i32, screencenterx: &i32, screencentery: &i32) {
    if width > 2 {
        if height > 2 {
            expwin(width - 2, height - 2, screencenterx, screencentery);
        } else {
            expwinh(width - 2, height, screencenterx, screencentery);
        }
    } else if height > 2 {
        expwinv(width, height - 2, screencenterx, screencentery);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height, screencenterx, screencentery);
}

unsafe fn expwinh(mut width: i32, mut height: i32, screencenterx: &i32, screencentery: &i32) {
    if width > 2 {
        expwinh(width - 2, height, screencenterx, screencentery);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height, screencenterx, screencentery);
}

unsafe fn expwinv(mut width: i32, mut height: i32, screencenterx: &i32, screencentery: &i32) {
    if height > 2 {
        expwinv(width, height - 2, screencenterx, screencentery);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height, screencenterx, screencentery);
}
#[no_mangle]
pub unsafe extern "C" fn bioskey(mut cmd: i32) -> i32 {
    if lastkey as u64 != 0 {
        let mut oldkey: i32 = lastkey as i32;
        if cmd != 1 {
            lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
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
#[no_mangle]
pub unsafe extern "C" fn UpdateScreen() {
    static mut EGAPalette: [u32; 16] = [
        0, 0xaa, 0xaa00, 0xaaaa, 0xaa0000, 0xaa00aa, 0xaa5500, 0xaaaaaa, 0x555555, 0x5555ff,
        0x55ff55, 0x55ffff, 0xff5555, 0xff55ff, 0xffff55, 0xffffff,
    ];
    static mut CGAPalette: [u32; 4] = [0, 0x55ffff, 0xff55ff, 0xffffff];
    static mut conv: [u32; 64000] = [0; 64000];
    let mut i: u64 = 0;
    if grmode as u32 == EGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            conv[i as usize] = EGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if grmode as u32 == CGAgr as i32 as u32 {
        while i < ::std::mem::size_of::<[u8; 64000]>() as u64 {
            conv[i as usize] = CGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else {
        panic!("VGA Palette conversion not implemented.");
    }
    SDL_UpdateTexture(
        sdltexture,
        0 as *const SDL_Rect,
        conv.as_mut_ptr() as *const libc::c_void,
        (320 as i32 as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64) as i32,
    );
    SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, sdltexture, 0 as *const SDL_Rect, &mut updateRect);
    SDL_RenderPresent(renderer);
}
#[no_mangle]
pub unsafe extern "C" fn get() -> i32 {
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
            drawchar(sx, sy, fresh2);
            UpdateScreen();
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
    drawchar(sx, sy, ' ' as i32);
    UpdateScreen();
    return SDL_GetKeyFromScancode(key_0 as SDL_Scancode);
}
#[no_mangle]
pub unsafe extern "C" fn print(mut str_0: *const i8) {
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
            drawchar(fresh4, sy, ch_0 as u8 as i32);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn printchartile(mut str_0: *const i8) {
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
            drawchartile(fresh6, sy, ch_0 as u8 as i32);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn printint(mut val: i32) {
    itoa(val, str.as_mut_ptr(), 10);
    print(str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn printlong(mut val: i64) {
    ltoa(val as i32, str.as_mut_ptr(), 10);
    print(str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _Verify(mut filename: *mut i8) -> i64 {
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
#[no_mangle]
pub unsafe extern "C" fn _printhexb(mut value: libc::c_uchar) {
    let mut loop_0: i32 = 0;
    let mut hexstr: [i8; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [i8; 16]>(b"0123456789ABCDEF");
    let mut str_0: [i8; 2] = *::std::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"\0\0");
    loop_0 = 0;
    while loop_0 < 2 {
        str_0[0] = hexstr[(value as i32 >> (1 - loop_0) * 4 & 15) as usize];
        print(str_0.as_mut_ptr());
        loop_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _printhex(mut value: u32) {
    print(b"$\0" as *const u8 as *const i8);
    _printhexb((value >> 8) as libc::c_uchar);
    _printhexb((value & 0xff as i32 as u32) as libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn _printbin(mut value: u32) {
    let mut loop_0: i32 = 0;
    print(b"%\0" as *const u8 as *const i8);
    loop_0 = 0;
    while loop_0 < 16 {
        if value >> 15 - loop_0 & 1 != 0 {
            print(b"1\0" as *const u8 as *const i8);
        } else {
            print(b"0\0" as *const u8 as *const i8);
        }
        loop_0 += 1;
    }
}

unsafe fn _printc(mut string: *mut i8, screencenterx: &i32) {
    sx = 1 + screencenterx - (strlen(string)).wrapping_div(2) as i32;
    print(string);
}
#[no_mangle]
pub unsafe extern "C" fn _inputint() -> u32 {
    let mut string: [i8; 18] =
        *::std::mem::transmute::<&[u8; 18], &mut [i8; 18]>(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut digit: i8 = 0;
    let mut hexstr: [i8; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [i8; 16]>(b"0123456789ABCDEF");
    let mut value: u32 = 0;
    let mut loop_0: u32 = 0;
    let mut loop1: u32 = 0;
    _input(string.as_mut_ptr(), 17);
    if string[0] as i32 == '$' as i32 {
        let mut digits: i64 = 0;
        digits = (strlen(string.as_mut_ptr())).wrapping_sub(2) as i64;
        if digits < 0 {
            return 0;
        }
        value = 0;
        loop1 = 0;
        while loop1 as u64 <= digits as u64 {
            digit = toupper(string[loop1.wrapping_add(1) as usize] as i32) as i8;
            loop_0 = 0;
            while loop_0 < 16 {
                if digit as i32 == hexstr[loop_0 as usize] as i32 {
                    value |= loop_0 << (digits - loop1 as i64) * 4;
                    break;
                } else {
                    loop_0 = loop_0.wrapping_add(1);
                }
            }
            loop1 = loop1.wrapping_add(1);
        }
    } else if string[0] as i32 == '%' as i32 {
        let mut digits_0: i64 = 0;
        digits_0 = (strlen(string.as_mut_ptr())).wrapping_sub(2) as i64;
        if digits_0 < 0 {
            return 0;
        }
        value = 0;
        loop1 = 0;
        while loop1 as u64 <= digits_0 as u64 {
            if (string[loop1.wrapping_add(1) as usize] as i32) < '0' as i32
                || string[loop1.wrapping_add(1) as usize] as i32 > '1' as i32
            {
                return 0;
            }
            value |= ((string[loop1.wrapping_add(1) as usize] as i32 - '0' as i32)
                << digits_0 - loop1 as i64) as u32;
            loop1 = loop1.wrapping_add(1);
        }
    } else {
        value = atoi(string.as_mut_ptr()) as u32;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn _input(mut string: *mut i8, mut max: i32) -> i32 {
    let mut key_0: i8 = 0;
    let mut count: i32 = 0;
    let mut loop_0: i32 = 0;
    loop {
        key_0 = toupper(get() & 0xff as i32) as i8;
        if (key_0 as i32 == 127 || key_0 as i32 == 8) && count > 0 {
            count -= 1;
            drawchar(sx, sy, ' ' as i32);
            sx -= 1;
        }
        if key_0 as i32 >= ' ' as i32 && key_0 as i32 <= 'z' as i32 && count < max {
            let fresh7 = count;
            count = count + 1;
            *string.offset(fresh7 as isize) = key_0;
            let fresh8 = sx;
            sx = sx + 1;
            drawchar(fresh8, sy, key_0 as i32);
        }
        if !(key_0 as i32 != 27 && key_0 as i32 != 13) {
            break;
        }
    }
    loop_0 = count;
    while loop_0 < max {
        *string.offset(loop_0 as isize) = 0;
        loop_0 += 1;
    }
    if key_0 as i32 == 13 {
        return 1;
    }
    return 0;
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
#[no_mangle]
pub unsafe extern "C" fn ScancodeToDOS(mut sc: SDL_Scancode) -> i32 {
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
#[no_mangle]
pub unsafe extern "C" fn CheckMouseMode() {
    SDL_SetRelativeMouseMode(
        (hasFocus as i32 != 0
            && (playermode[1] as u32 == mouse as i32 as u32
                || playermode[2] as u32 == mouse as i32 as u32)) as i32 as SDL_bool,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _loadctrls() {
    let mut handle: i32 = 0;
    strcpy(str.as_mut_ptr(), b"CTLPANEL.\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
    // project.
    handle = open(
        str.as_mut_ptr(),
        O_RDONLY | O_BINARY,
        0o200 as i32 | 0o400 as i32,
    );
    if handle == -(1) {
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
#[no_mangle]
pub unsafe extern "C" fn _savectrls() {
    let mut handle: i32 = 0;
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
    strcpy(str.as_mut_ptr(), b"CTLPANEL.\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    handle = open(
        str.as_mut_ptr(),
        0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
        0o400 as i32 | 0o200 as i32,
    );
    if handle == -(1) {
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
#[no_mangle]
pub unsafe extern "C" fn _loadhighscores() {
    let mut i: i32 = 0;
    strcpy(str.as_mut_ptr(), b"SCORES.\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    if LoadFile(str.as_mut_ptr(), highscores.as_mut_ptr() as *mut i8) == 0 {
        i = 0;
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
#[no_mangle]
pub unsafe extern "C" fn _savehighscores() {
    strcpy(str.as_mut_ptr(), b"SCORES.\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        highscores.as_mut_ptr() as *mut i8,
        ::std::mem::size_of::<[scores; 5]>() as u64 as i64,
    );
}

pub unsafe fn _showhighscores(screencenterx: &i32, screencentery: &i32) {
    let mut i: i32 = 0;
    let mut h: i64 = 0;
    let mut st2: [i8; 10] = [0; 10];
    centerwindow(17, 17, screencenterx, screencentery);
    print(b"\n   HIGH SCORES\n\n\0" as *const u8 as *const i8);
    print(b" #  SCORE LV  BY\n\0" as *const u8 as *const i8);
    print(b" - ------ -- ---\n\0" as *const u8 as *const i8);
    i = 0;
    while i < 5 {
        sx += 1;
        drawchar(sx, sy, '1' as i32 + i);
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
        ltoa(h as i32, str.as_mut_ptr(), 10);
        print(str.as_mut_ptr());
        sx += 1;
        if (highscores[i as usize].level as i32) < 10 {
            sx += 1;
        }
        itoa(highscores[i as usize].level as i32, str.as_mut_ptr(), 10);
        print(str.as_mut_ptr());
        sx += 1;
        print((highscores[i as usize].initials).as_mut_ptr());
        print(b"\n\n\0" as *const u8 as *const i8);
        i += 1;
    }
    strcpy(str.as_mut_ptr(), b"SCORE:\0" as *const u8 as *const i8);
    ltoa(score, st2.as_mut_ptr(), 10);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    _printc(str.as_mut_ptr(), screencenterx);
}

pub unsafe fn _checkhighscore(screencenterx: &i32, screencentery: &i32) {
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
    _showhighscores(screencenterx, screencentery);
    UpdateScreen();
    if i < 5 {
        PlaySound(16);
        clearkeys();
        sx = screencenterx - 17 / 2 + 14;
        sy = screencentery - 17 / 2 + 6 + i * 2;
        j = 0;
        loop {
            k = get();
            ch = k as i8;
            if ch as i32 >= ' ' as i32 && j < 3 {
                drawchar(sx, sy, ch as i32);
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
#[no_mangle]
pub static mut screenseg: [u8; 64000] = [0; 64000];

const VIDEO_PARAM_WINDOWED: &str = "windowed";
const VIDEO_PARAM_FULLSCREEN: &str = "screen";

pub unsafe fn _setupgame() {
    if SDL_Init(0x20 as u32 | 0x1 as u32 | 0x200 as u32 | 0x2000 as u32) < 0 {
        fprintf(
            stderr,
            b"Failed to initialize SDL: %s\n\0" as *const u8 as *const i8,
            SDL_GetError(),
        );
        std::process::exit(1);
    }
    atexit(Some(SDL_Quit as unsafe extern "C" fn() -> ()));
    SDL_AddEventWatch(
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
    if SDL_GetCurrentDisplayMode(displayindex, &mut mode) < -(1)
        || SDL_GetDisplayBounds(displayindex, &mut bounds) < 0
    {
        fprintf(
            stderr,
            b"Could not get display mode: %s\n\0" as *const u8 as *const i8,
            SDL_GetError(),
        );
        std::process::exit(1);
    }
    if windowed {
        bounds.x = (0x1fff0000 as u32 | 0) as i32;
        bounds.y = (0x1fff0000 as u32 | 0) as i32;
        mode.w = winWidth as i32;
        mode.h = winHeight as i32;
    }
    window = SDL_CreateWindow(
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
        renderer = SDL_CreateRenderer(window, -(1), 0);
        renderer.is_null()
    } {
        fprintf(
            stderr,
            b"Failed to create SDL window: %s\n\0" as *const u8 as *const i8,
            SDL_GetError(),
        );
        std::process::exit(1);
    }
    sdltexture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888,
        SDL_TEXTUREACCESS_STREAMING as i32,
        320,
        200,
    );
    if sdltexture.is_null() {
        fprintf(
            stderr,
            b"Could not create video buffer: %s\n\0" as *const u8 as *const i8,
            SDL_GetError(),
        );
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
    memset(
        screenseg.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[u8; 64000]>() as u64,
    );
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
    strcpy(str.as_mut_ptr(), b"SOUNDS.\0" as *const u8 as *const i8);
    strcat(str.as_mut_ptr(), _extension);
    SoundData = bloadin(str.as_mut_ptr()) as *mut SPKRtable;
    StartupSound();
    SetupKBD();
    initrndt(1);
    initrnd(1);
    _loadhighscores();
    loadgrfiles();
    SetupEmulatedVBL();
}
#[no_mangle]
pub unsafe extern "C" fn _quit(mut error: *mut i8) {
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
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    renderer = 0 as *mut SDL_Renderer;
    window = 0 as *mut SDL_Window;
    std::process::exit(0);
}
