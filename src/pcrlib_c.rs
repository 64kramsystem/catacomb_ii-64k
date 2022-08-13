use ::libc;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn toupper(_: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn SDL_GetError() -> *const libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn US_CheckParm(_: *mut libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut _argc: libc::c_int;
    static mut _argv: *mut *mut libc::c_char;
    fn loadgrfiles();
    fn drawchartile(x: libc::c_int, y: libc::c_int, tile: libc::c_int);
    fn WaitVBL();
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
    fn initrnd(randomize: boolean);
    fn initrndt(randomize: boolean);
    fn PlaySound(sound: libc::c_int);
    fn ShutdownSound();
    fn StartupSound();
    static mut SoundData: *mut SPKRtable;
    static mut soundmode: soundtype;
    fn SDL_Quit();
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    fn SDL_Delay(ms: Uint32);
    fn SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer);
    fn SDL_RenderPresent(renderer_0: *mut SDL_Renderer);
    fn SDL_RenderCopy(
        renderer_0: *mut SDL_Renderer,
        texture: *mut SDL_Texture,
        srcrect: *const SDL_Rect,
        dstrect: *const SDL_Rect,
    ) -> libc::c_int;
    fn SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> libc::c_int;
    fn SDL_UpdateTexture(
        texture: *mut SDL_Texture,
        rect: *const SDL_Rect,
        pixels: *const libc::c_void,
        pitch: libc::c_int,
    ) -> libc::c_int;
    fn SDL_CreateTexture(
        renderer_0: *mut SDL_Renderer,
        format: Uint32,
        access: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> *mut SDL_Texture;
    fn SDL_CreateRenderer(
        window_0: *mut SDL_Window,
        index: libc::c_int,
        flags: Uint32,
    ) -> *mut SDL_Renderer;
    fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *mut libc::c_void);
    fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
    fn SDL_PumpEvents();
    fn SDL_GetDisplayBounds(displayIndex: libc::c_int, rect: *mut SDL_Rect) -> libc::c_int;
    fn SDL_GetCurrentDisplayMode(
        displayIndex: libc::c_int,
        mode_0: *mut SDL_DisplayMode,
    ) -> libc::c_int;
    fn SDL_CreateWindow(
        title: *const libc::c_char,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        flags: Uint32,
    ) -> *mut SDL_Window;
    fn SDL_DestroyWindow(window_0: *mut SDL_Window);
    fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
    fn SDL_GetMouseFocus() -> *mut SDL_Window;
    fn SDL_GetRelativeMouseState(x: *mut libc::c_int, y: *mut libc::c_int) -> Uint32;
    fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> libc::c_int;
    fn SDL_NumJoysticks() -> libc::c_int;
    fn SDL_JoystickOpen(device_index: libc::c_int) -> *mut SDL_Joystick;
    fn SDL_JoystickUpdate();
    fn SDL_JoystickGetAxis(joystick_0: *mut SDL_Joystick, axis: libc::c_int) -> Sint16;
    fn SDL_JoystickGetButton(joystick_0: *mut SDL_Joystick, button: libc::c_int) -> Uint8;
    fn SDL_JoystickClose(joystick_0: *mut SDL_Joystick);
    fn SDL_IsGameController(joystick_index: libc::c_int) -> SDL_bool;
    fn SDL_GameControllerOpen(joystick_index: libc::c_int) -> *mut SDL_GameController;
    fn SDL_GameControllerGetAxis(
        gamecontroller: *mut SDL_GameController,
        axis: SDL_GameControllerAxis,
    ) -> Sint16;
    fn SDL_GameControllerGetButton(
        gamecontroller: *mut SDL_GameController,
        button: SDL_GameControllerButton,
    ) -> Uint8;
    fn SDL_GameControllerClose(gamecontroller: *mut SDL_GameController);
    fn SetupEmulatedVBL();
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type word = uint16_t;
pub type sword = int16_t;
pub type dword = uint32_t;
pub type sdword = int32_t;
pub type dirtype = libc::c_uint;
pub const nodir: dirtype = 8;
pub const northwest: dirtype = 7;
pub const southwest: dirtype = 6;
pub const southeast: dirtype = 5;
pub const northeast: dirtype = 4;
pub const west: dirtype = 3;
pub const south: dirtype = 2;
pub const east: dirtype = 1;
pub const north: dirtype = 0;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Sint32 = int32_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
pub type C2RustUnnamed_0 = libc::c_uint;
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
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayMode {
    pub format: Uint32,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub refresh_rate: libc::c_int,
    pub driverdata: *mut libc::c_void,
}
pub type C2RustUnnamed_1 = libc::c_uint;
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
pub type C2RustUnnamed_2 = libc::c_uint;
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
pub type SDL_Scancode = libc::c_uint;
pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
pub const SDL_SCANCODE_END: SDL_Scancode = 77;
pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
pub const SDL_SCANCODE_0: SDL_Scancode = 39;
pub const SDL_SCANCODE_9: SDL_Scancode = 38;
pub const SDL_SCANCODE_8: SDL_Scancode = 37;
pub const SDL_SCANCODE_7: SDL_Scancode = 36;
pub const SDL_SCANCODE_6: SDL_Scancode = 35;
pub const SDL_SCANCODE_5: SDL_Scancode = 34;
pub const SDL_SCANCODE_4: SDL_Scancode = 33;
pub const SDL_SCANCODE_3: SDL_Scancode = 32;
pub const SDL_SCANCODE_2: SDL_Scancode = 31;
pub const SDL_SCANCODE_1: SDL_Scancode = 30;
pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
pub const SDL_SCANCODE_X: SDL_Scancode = 27;
pub const SDL_SCANCODE_W: SDL_Scancode = 26;
pub const SDL_SCANCODE_V: SDL_Scancode = 25;
pub const SDL_SCANCODE_U: SDL_Scancode = 24;
pub const SDL_SCANCODE_T: SDL_Scancode = 23;
pub const SDL_SCANCODE_S: SDL_Scancode = 22;
pub const SDL_SCANCODE_R: SDL_Scancode = 21;
pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
pub const SDL_SCANCODE_P: SDL_Scancode = 19;
pub const SDL_SCANCODE_O: SDL_Scancode = 18;
pub const SDL_SCANCODE_N: SDL_Scancode = 17;
pub const SDL_SCANCODE_M: SDL_Scancode = 16;
pub const SDL_SCANCODE_L: SDL_Scancode = 15;
pub const SDL_SCANCODE_K: SDL_Scancode = 14;
pub const SDL_SCANCODE_J: SDL_Scancode = 13;
pub const SDL_SCANCODE_I: SDL_Scancode = 12;
pub const SDL_SCANCODE_H: SDL_Scancode = 11;
pub const SDL_SCANCODE_G: SDL_Scancode = 10;
pub const SDL_SCANCODE_F: SDL_Scancode = 9;
pub const SDL_SCANCODE_E: SDL_Scancode = 8;
pub const SDL_SCANCODE_D: SDL_Scancode = 7;
pub const SDL_SCANCODE_C: SDL_Scancode = 6;
pub const SDL_SCANCODE_B: SDL_Scancode = 5;
pub const SDL_SCANCODE_A: SDL_Scancode = 4;
pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
pub type SDL_Keycode = Sint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Keysym {
    pub scancode: SDL_Scancode,
    pub sym: SDL_Keycode,
    pub mod_0: Uint16,
    pub unused: Uint32,
}
pub type SDL_Joystick = _SDL_Joystick;
pub type SDL_JoystickID = Sint32;
pub type SDL_GameController = _SDL_GameController;
pub type SDL_GameControllerAxis = libc::c_int;
pub const SDL_CONTROLLER_AXIS_MAX: SDL_GameControllerAxis = 6;
pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = 5;
pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = 4;
pub const SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = 3;
pub const SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = 2;
pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
pub const SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = -1;
pub type SDL_GameControllerButton = libc::c_int;
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
pub type SDL_TouchID = Sint64;
pub type SDL_FingerID = Sint64;
pub type SDL_GestureID = Sint64;
pub type C2RustUnnamed_3 = libc::c_uint;
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
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DisplayEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub display: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_WindowEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub event: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub data1: Sint32,
    pub data2: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub state: Uint8,
    pub repeat: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub keysym: SDL_Keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
    pub start: Sint32,
    pub length: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TextInputEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub text: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub state: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub xrel: Sint32,
    pub yrel: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub button: Uint8,
    pub state: Uint8,
    pub clicks: Uint8,
    pub padding1: Uint8,
    pub x: Sint32,
    pub y: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub which: Uint32,
    pub x: Sint32,
    pub y: Sint32,
    pub direction: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub ball: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub hat: Uint8,
    pub value: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub axis: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
    pub value: Sint16,
    pub padding4: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: SDL_JoystickID,
    pub button: Uint8,
    pub state: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Uint32,
    pub iscapture: Uint8,
    pub padding1: Uint8,
    pub padding2: Uint8,
    pub padding3: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
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
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub dTheta: libc::c_float,
    pub dDist: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub numFingers: Uint16,
    pub padding: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub touchId: SDL_TouchID,
    pub gestureId: SDL_GestureID,
    pub numFingers: Uint32,
    pub error: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_DropEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub file: *mut libc::c_char,
    pub windowID: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SensorEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub which: Sint32,
    pub data: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub windowID: Uint32,
    pub code: Sint32,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint32,
    pub timestamp: Uint32,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint32,
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
    pub padding: [Uint8; 56],
}
pub type SDL_EventFilter =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> libc::c_int>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SDL_TEXTUREACCESS_TARGET: C2RustUnnamed_4 = 2;
pub const SDL_TEXTUREACCESS_STREAMING: C2RustUnnamed_4 = 1;
pub const SDL_TEXTUREACCESS_STATIC: C2RustUnnamed_4 = 0;
pub type soundtype = libc::c_uint;
pub const sdlib: soundtype = 2;
pub const spkr: soundtype = 1;
pub const off: soundtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct spksndtype {
    pub start: word,
    pub priority: byte,
    pub samplerate: byte,
    pub name: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SPKRtable {
    pub id: [libc::c_char; 4],
    pub filelength: word,
    pub filler: [word; 5],
    pub sounds: [spksndtype; 63],
    pub freqdata: [word; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: boolean,
    pub button2: boolean,
}
pub type inputtype = libc::c_uint;
pub const demo: inputtype = 4;
pub const joystick2: inputtype = 3;
pub const joystick1: inputtype = 2;
pub const mouse: inputtype = 1;
pub const keyboard: inputtype = 0;
pub type demoenum = libc::c_uint;
pub const recording: demoenum = 2;
pub const demoplay: demoenum = 1;
pub const notdemo: demoenum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct joyinfo_t {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub device: libc::c_int,
    pub isgamecontroller: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub controller: *mut SDL_GameController,
    pub joy: *mut SDL_Joystick,
}
pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct scores {
    pub score: sdword,
    pub level: sword,
    pub initials: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ctlpaneltype {
    pub grmode: word,
    pub soundmode: word,
    pub playermode: [word; 3],
    pub JoyXlow: [sword; 3],
    pub JoyYlow: [sword; 3],
    pub JoyXhigh: [sword; 3],
    pub JoyYhigh: [sword; 3],
    pub MouseSensitivity: sword,
    pub key: [byte; 8],
    pub keyB1: byte,
    pub keyB2: byte,
}
#[inline]
unsafe extern "C" fn itoa(
    mut value: libc::c_int,
    mut str_0: *mut libc::c_char,
    mut base: libc::c_int,
) -> *mut libc::c_char {
    if base == 16 as libc::c_int {
        sprintf(str_0, b"%X\0" as *const u8 as *const libc::c_char, value);
    } else {
        sprintf(str_0, b"%d\0" as *const u8 as *const libc::c_char, value);
    }
    return str_0;
}
#[inline]
unsafe extern "C" fn ltoa(
    mut value: libc::c_int,
    mut str_0: *mut libc::c_char,
    mut base: libc::c_int,
) -> *mut libc::c_char {
    return itoa(value, str_0, base);
}
#[no_mangle]
pub static mut ch: libc::c_char = 0;
#[no_mangle]
pub static mut str: [libc::c_char; 80] = [0; 80];
#[no_mangle]
pub static mut playermode: [inputtype; 3] = [keyboard, keyboard, joystick1];
#[no_mangle]
pub static mut keydown: [boolean; 512] = [0; 512];
#[no_mangle]
pub static mut JoyXlow: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut JoyXhigh: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut JoyYlow: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut JoyYhigh: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut MouseSensitivity: libc::c_int = 0;
#[no_mangle]
pub static mut mouseEvent: boolean = 0;
#[no_mangle]
pub static mut key: [libc::c_int; 8] = [0; 8];
#[no_mangle]
pub static mut keyB1: libc::c_int = 0;
#[no_mangle]
pub static mut keyB2: libc::c_int = 0;
#[no_mangle]
pub static mut demobuffer: [libc::c_char; 5000] = [0; 5000];
#[no_mangle]
pub static mut demoptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut democount: libc::c_int = 0;
#[no_mangle]
pub static mut lastdemoval: libc::c_int = 0;
#[no_mangle]
pub static mut indemo: demoenum = notdemo;
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
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 128 as libc::c_int as libc::c_uint {
        keydown[i as usize] = false_0 as libc::c_int as boolean;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProcessEvents() {
    mouseEvent = false_0 as libc::c_int as boolean;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as libc::c_int as libc::c_uint {
            keydown[event.key.keysym.scancode as usize] = true_0 as libc::c_int as boolean;
            lastkey = event.key.keysym.scancode;
        } else if event.type_0 == SDL_KEYUP as libc::c_int as libc::c_uint {
            keydown[event.key.keysym.scancode as usize] = false_0 as libc::c_int as boolean;
        } else if event.type_0 == SDL_MOUSEMOTION as libc::c_int as libc::c_uint {
            mouseEvent = true_0 as libc::c_int as boolean;
        }
    }
}
static mut hasFocus: boolean = true_0 as libc::c_int as boolean;
unsafe extern "C" fn WatchUIEvents(
    mut _userdata: *mut libc::c_void,
    mut event: *mut SDL_Event,
) -> libc::c_int {
    if (*event).type_0 == SDL_QUIT as libc::c_int as libc::c_uint {
        _quit(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else if (*event).type_0 == SDL_WINDOWEVENT as libc::c_int as libc::c_uint {
        match (*event).window.event as libc::c_int {
            13 => {
                hasFocus = false_0 as libc::c_int as boolean;
                CheckMouseMode();
            }
            12 => {
                while SDL_GetMouseFocus() != window {
                    SDL_PumpEvents();
                    SDL_Delay(10 as libc::c_int as Uint32);
                }
                hasFocus = true_0 as libc::c_int as boolean;
                CheckMouseMode();
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ControlKBD() -> ControlStruct {
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    if keydown[key[north as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
    }
    if keydown[key[east as libc::c_int as usize] as usize] != 0 {
        xmove = 1 as libc::c_int;
    }
    if keydown[key[south as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
    }
    if keydown[key[west as libc::c_int as usize] as usize] != 0 {
        xmove = -(1 as libc::c_int);
    }
    if keydown[key[northeast as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
        xmove = 1 as libc::c_int;
    }
    if keydown[key[northwest as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
        xmove = -(1 as libc::c_int);
    }
    if keydown[key[southeast as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
        xmove = 1 as libc::c_int;
    }
    if keydown[key[southwest as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
        xmove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
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
    let mut newx: libc::c_int = 0;
    let mut newy: libc::c_int = 0;
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut buttons: libc::c_int = SDL_GetRelativeMouseState(&mut newx, &mut newy) as libc::c_int;
    action.button1 =
        (buttons & (1 as libc::c_int) << 1 as libc::c_int - 1 as libc::c_int) as boolean;
    action.button2 =
        (buttons & (1 as libc::c_int) << 3 as libc::c_int - 1 as libc::c_int) as boolean;
    if mouseEvent as libc::c_int == false_0 as libc::c_int {
        action.dir = nodir;
        return action;
    }
    if newx > MouseSensitivity {
        xmove = 1 as libc::c_int;
    } else if newx < -MouseSensitivity {
        xmove = -(1 as libc::c_int);
    }
    if newy > MouseSensitivity {
        ymove = 1 as libc::c_int;
    } else if newy < -MouseSensitivity {
        ymove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
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
    let mut j: libc::c_uint = 0;
    j = 1 as libc::c_int as libc::c_uint;
    while j < 3 as libc::c_int as libc::c_uint {
        if !(joystick[j as usize].device < 0 as libc::c_int) {
            if joystick[j as usize].isgamecontroller != 0 {
                SDL_GameControllerClose(joystick[j as usize].c2rust_unnamed.controller);
            } else {
                SDL_JoystickClose(joystick[j as usize].c2rust_unnamed.joy);
            }
            joystick[j as usize].device = -(1 as libc::c_int);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProbeJoysticks() {
    let mut j: libc::c_int = 0;
    if joystick[1 as libc::c_int as usize].device > 0 as libc::c_int
        || joystick[2 as libc::c_int as usize].device > 0 as libc::c_int
    {
        ShutdownJoysticks();
    }
    j = 1 as libc::c_int;
    while j < 3 as libc::c_int {
        if j - 1 as libc::c_int >= SDL_NumJoysticks() {
            joystick[j as usize].device = -(1 as libc::c_int);
        } else {
            joystick[j as usize].device = j - 1 as libc::c_int;
            joystick[j as usize].isgamecontroller =
                SDL_IsGameController(j - 1 as libc::c_int) as boolean;
            if SDL_IsGameController(j - 1 as libc::c_int) as u64 != 0 {
                joystick[j as usize].c2rust_unnamed.controller =
                    SDL_GameControllerOpen(j - 1 as libc::c_int);
            } else {
                joystick[j as usize].c2rust_unnamed.joy = SDL_JoystickOpen(j - 1 as libc::c_int);
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReadJoystick(
    mut joynum: libc::c_int,
    mut xcount: *mut libc::c_int,
    mut ycount: *mut libc::c_int,
) {
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    *xcount = 0 as libc::c_int;
    *ycount = 0 as libc::c_int;
    SDL_JoystickUpdate();
    if joystick[joynum as usize].isgamecontroller != 0 {
        a1 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as libc::c_int;
        a2 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as libc::c_int;
    } else {
        a1 = SDL_JoystickGetAxis(
            joystick[joynum as usize].c2rust_unnamed.joy,
            0 as libc::c_int,
        ) as libc::c_int;
        a2 = SDL_JoystickGetAxis(
            joystick[joynum as usize].c2rust_unnamed.joy,
            1 as libc::c_int,
        ) as libc::c_int;
    }
    *xcount = a1;
    *ycount = a2;
}
#[no_mangle]
pub unsafe extern "C" fn ControlJoystick(mut joynum: libc::c_int) -> ControlStruct {
    let mut joyx: libc::c_int = 0 as libc::c_int;
    let mut joyy: libc::c_int = 0 as libc::c_int;
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
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
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
        action.button2 = (SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_B,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
    } else {
        action.button1 = (SDL_JoystickGetButton(
            joystick[joynum as usize].c2rust_unnamed.joy,
            0 as libc::c_int,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
        action.button2 = (SDL_JoystickGetButton(
            joystick[joynum as usize].c2rust_unnamed.joy,
            1 as libc::c_int,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
    }
    if joyx == 0 as libc::c_int && joyy == 0 as libc::c_int {
        action.dir = nodir;
        return action;
    }
    if joyx > JoyXhigh[joynum as usize] {
        xmove = 1 as libc::c_int;
    } else if joyx < JoyXlow[joynum as usize] {
        xmove = -(1 as libc::c_int);
    }
    if joyy > JoyYhigh[joynum as usize] {
        ymove = 1 as libc::c_int;
    } else if joyy < JoyYlow[joynum as usize] {
        ymove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
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
pub unsafe extern "C" fn ControlPlayer(mut player: libc::c_int) -> ControlStruct {
    let mut ret: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut val: libc::c_int = 0;
    ProcessEvents();
    if indemo as libc::c_uint == notdemo as libc::c_int as libc::c_uint
        || indemo as libc::c_uint == recording as libc::c_int as libc::c_uint
    {
        match playermode[player as usize] as libc::c_uint {
            1 => {
                ret = ControlMouse();
            }
            2 => {
                ret = ControlJoystick(1 as libc::c_int);
            }
            3 => {
                ret = ControlJoystick(2 as libc::c_int);
            }
            0 | _ => {
                ret = ControlKBD();
            }
        }
        if indemo as libc::c_uint == recording as libc::c_int as libc::c_uint {
            val = ((ret.dir as libc::c_uint) << 2 as libc::c_int
                | ((ret.button2 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                | ret.button1 as libc::c_uint) as libc::c_int;
            let fresh0 = demoptr;
            demoptr = demoptr.offset(1);
            *fresh0 = val as libc::c_char;
        }
    } else {
        let fresh1 = demoptr;
        demoptr = demoptr.offset(1);
        val = *fresh1 as libc::c_int;
        ret.button1 = (val & 1 as libc::c_int) as boolean;
        ret.button2 = ((val & 2 as libc::c_int) >> 1 as libc::c_int) as boolean;
        ret.dir = ((val
            & 4 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 32 as libc::c_int)
            >> 2 as libc::c_int) as dirtype;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn RecordDemo() {
    demobuffer[0 as libc::c_int as usize] = level as libc::c_char;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char;
    indemo = recording;
}
#[no_mangle]
pub unsafe extern "C" fn LoadDemo(mut demonum: libc::c_int) {
    let mut st2: [libc::c_char; 5] = [0; 5];
    strcpy(
        str.as_mut_ptr(),
        b"DEMO\0" as *const u8 as *const libc::c_char,
    );
    itoa(demonum, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
    strcat(str.as_mut_ptr(), _extension);
    LoadFile(str.as_mut_ptr(), demobuffer.as_mut_ptr());
    level = demobuffer[0 as libc::c_int as usize] as sword;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char;
    indemo = demoplay;
}
#[no_mangle]
pub unsafe extern "C" fn SaveDemo(mut demonum: libc::c_int) {
    let mut st2: [libc::c_char; 5] = [0; 5];
    strcpy(
        str.as_mut_ptr(),
        b"DEMO\0" as *const u8 as *const libc::c_char,
    );
    itoa(demonum, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        demobuffer.as_mut_ptr(),
        demoptr.offset_from(
            &mut *demobuffer.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char,
        ) as libc::c_long,
    );
    indemo = notdemo;
}
#[no_mangle]
pub unsafe extern "C" fn clearkeys() {
    let mut i: libc::c_int = 0;
    while bioskey(1 as libc::c_int) != 0 {
        bioskey(0 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        keydown[i as usize] = 0 as libc::c_int as boolean;
        i += 1;
    }
}
unsafe extern "C" fn filelength(mut fd: libc::c_int) -> libc::c_long {
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
        return -(1 as libc::c_int) as libc::c_long;
    }
    return s.st_size;
}
#[no_mangle]
pub unsafe extern "C" fn LoadFile(
    mut filename: *mut libc::c_char,
    mut buffer: *mut libc::c_char,
) -> libc::c_ulong {
    let mut fd: libc::c_int = 0;
    fd = open(filename, 0o400 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_ulong;
    }
    let mut len: libc::c_long = filelength(fd);
    let mut bytesRead: ssize_t = read(fd, buffer as *mut libc::c_void, len as size_t);
    close(fd);
    return bytesRead as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn SaveFile(
    mut filename: *mut libc::c_char,
    mut buffer: *mut libc::c_char,
    mut size: libc::c_long,
) {
    let mut fd: libc::c_int = 0;
    fd = open(
        filename,
        0o1 as libc::c_int | 0 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return;
    }
    write(fd, buffer as *const libc::c_void, size as size_t);
    close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn bloadin(mut filename: *mut libc::c_char) -> *mut libc::c_void {
    let mut handle: libc::c_int = 0;
    let mut length: libc::c_long = 0;
    let mut location: *mut libc::c_char = 0 as *mut libc::c_char;
    handle = open(filename, 0 as libc::c_int);
    if handle != -(1 as libc::c_int) {
        length = filelength(handle);
        location = malloc(length as libc::c_ulong) as *mut libc::c_char;
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
pub static mut egaplaneofs: [dword; 4] = [0; 4];
#[no_mangle]
pub static mut sx: libc::c_int = 0;
#[no_mangle]
pub static mut sy: libc::c_int = 0;
#[no_mangle]
pub static mut leftedge: libc::c_int = 0;
#[no_mangle]
pub static mut win_xl: libc::c_int = 0;
#[no_mangle]
pub static mut win_yl: libc::c_int = 0;
#[no_mangle]
pub static mut win_xh: libc::c_int = 0;
#[no_mangle]
pub static mut win_yh: libc::c_int = 0;
#[no_mangle]
pub static mut screencenterx: libc::c_int = 19 as libc::c_int;
#[no_mangle]
pub static mut screencentery: libc::c_int = 11 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn drawwindow(
    mut xl: libc::c_int,
    mut yl: libc::c_int,
    mut xh: libc::c_int,
    mut yh: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    win_xl = xl;
    win_yl = yl;
    win_xh = xh;
    win_yh = yh;
    drawchar(xl, yl, 1 as libc::c_int);
    x = xl + 1 as libc::c_int;
    while x < xh {
        drawchar(x, yl, 2 as libc::c_int);
        x += 1;
    }
    drawchar(xh, yl, 3 as libc::c_int);
    y = yl + 1 as libc::c_int;
    while y < yh {
        drawchar(xl, y, 4 as libc::c_int);
        x = xl + 1 as libc::c_int;
        while x < xh {
            drawchar(x, y, ' ' as i32);
            x += 1;
        }
        drawchar(xh, y, 5 as libc::c_int);
        y += 1;
    }
    drawchar(xl, yh, 6 as libc::c_int);
    x = xl + 1 as libc::c_int;
    while x < xh {
        drawchar(x, yh, 7 as libc::c_int);
        x += 1;
    }
    drawchar(xh, yh, 8 as libc::c_int);
    leftedge = xl + 1 as libc::c_int;
    sx = leftedge;
    sy = yl + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bar(
    mut xl: libc::c_int,
    mut yl: libc::c_int,
    mut xh: libc::c_int,
    mut yh: libc::c_int,
    mut ch_0: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
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
#[no_mangle]
pub unsafe extern "C" fn centerwindow(mut width: libc::c_int, mut height: libc::c_int) {
    let mut xl: libc::c_int = screencenterx - width / 2 as libc::c_int;
    let mut yl: libc::c_int = screencentery - height / 2 as libc::c_int;
    drawwindow(
        xl,
        yl,
        xl + width + 1 as libc::c_int,
        yl + height + 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn expwin(mut width: libc::c_int, mut height: libc::c_int) {
    if width > 2 as libc::c_int {
        if height > 2 as libc::c_int {
            expwin(width - 2 as libc::c_int, height - 2 as libc::c_int);
        } else {
            expwinh(width - 2 as libc::c_int, height);
        }
    } else if height > 2 as libc::c_int {
        expwinv(width, height - 2 as libc::c_int);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
pub unsafe extern "C" fn expwinh(mut width: libc::c_int, mut height: libc::c_int) {
    if width > 2 as libc::c_int {
        expwinh(width - 2 as libc::c_int, height);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
pub unsafe extern "C" fn expwinv(mut width: libc::c_int, mut height: libc::c_int) {
    if height > 2 as libc::c_int {
        expwinv(width, height - 2 as libc::c_int);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
pub unsafe extern "C" fn bioskey(mut cmd: libc::c_int) -> libc::c_int {
    if lastkey as u64 != 0 {
        let mut oldkey: libc::c_int = lastkey as libc::c_int;
        if cmd != 1 as libc::c_int {
            lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as libc::c_int as libc::c_uint {
            if cmd == 1 as libc::c_int {
                lastkey = event.key.keysym.scancode;
                return lastkey as libc::c_int;
            }
            return event.key.keysym.scancode as libc::c_int;
        }
    }
    return lastkey as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UpdateScreen() {
    static mut EGAPalette: [Uint32; 16] = [
        0 as libc::c_int as Uint32,
        0xaa as libc::c_int as Uint32,
        0xaa00 as libc::c_int as Uint32,
        0xaaaa as libc::c_int as Uint32,
        0xaa0000 as libc::c_int as Uint32,
        0xaa00aa as libc::c_int as Uint32,
        0xaa5500 as libc::c_int as Uint32,
        0xaaaaaa as libc::c_int as Uint32,
        0x555555 as libc::c_int as Uint32,
        0x5555ff as libc::c_int as Uint32,
        0x55ff55 as libc::c_int as Uint32,
        0x55ffff as libc::c_int as Uint32,
        0xff5555 as libc::c_int as Uint32,
        0xff55ff as libc::c_int as Uint32,
        0xffff55 as libc::c_int as Uint32,
        0xffffff as libc::c_int as Uint32,
    ];
    static mut CGAPalette: [Uint32; 4] = [
        0 as libc::c_int as Uint32,
        0x55ffff as libc::c_int as Uint32,
        0xff55ff as libc::c_int as Uint32,
        0xffffff as libc::c_int as Uint32,
    ];
    static mut conv: [Uint32; 64000] = [0; 64000];
    let mut i: size_t = 0 as libc::c_int as size_t;
    if grmode as libc::c_uint == EGAgr as libc::c_int as libc::c_uint {
        while i < ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong {
            conv[i as usize] = EGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        while i < ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong {
            conv[i as usize] = CGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if false_0 as libc::c_int != 0
        && !(b"VGA Palette conversion not implemented.\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"false && \"VGA Palette conversion not implemented.\"\0" as *const u8
                as *const libc::c_char,
            b"/home/saverio/code/catacomb_ii_sdl-dev/source_project/pcrlib_c.c\0" as *const u8
                as *const libc::c_char,
            865 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"void UpdateScreen()\0"))
                .as_ptr(),
        );
    }
    SDL_UpdateTexture(
        sdltexture,
        0 as *const SDL_Rect,
        conv.as_mut_ptr() as *const libc::c_void,
        (320 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Uint32>() as libc::c_ulong) as libc::c_int,
    );
    SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, sdltexture, 0 as *const SDL_Rect, &mut updateRect);
    SDL_RenderPresent(renderer);
}
#[no_mangle]
pub unsafe extern "C" fn get() -> libc::c_int {
    let mut cycle: libc::c_int = 0;
    let mut key_0: libc::c_int = 0;
    loop {
        cycle = 9 as libc::c_int;
        loop {
            key_0 = bioskey(0 as libc::c_int);
            if !(key_0 == 0 && cycle < 13 as libc::c_int) {
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
        if !(key_0 == 0 as libc::c_int) {
            break;
        }
    }
    drawchar(sx, sy, ' ' as i32);
    UpdateScreen();
    return SDL_GetKeyFromScancode(key_0 as SDL_Scancode);
}
#[no_mangle]
pub unsafe extern "C" fn print(mut str_0: *const libc::c_char) {
    let mut ch_0: libc::c_char = 0;
    loop {
        let fresh3 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh3;
        if !(ch_0 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if ch_0 as libc::c_int == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as libc::c_int == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh4 = sx;
            sx = sx + 1;
            drawchar(fresh4, sy, ch_0 as byte as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn printchartile(mut str_0: *const libc::c_char) {
    let mut ch_0: libc::c_char = 0;
    loop {
        let fresh5 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh5;
        if !(ch_0 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if ch_0 as libc::c_int == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as libc::c_int == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh6 = sx;
            sx = sx + 1;
            drawchartile(fresh6, sy, ch_0 as byte as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn printint(mut val: libc::c_int) {
    itoa(val, str.as_mut_ptr(), 10 as libc::c_int);
    print(str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn printlong(mut val: libc::c_long) {
    ltoa(val as libc::c_int, str.as_mut_ptr(), 10 as libc::c_int);
    print(str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _Verify(mut filename: *mut libc::c_char) -> libc::c_long {
    let mut handle: libc::c_int = 0;
    let mut size: libc::c_long = 0;
    handle = open(filename, 0 as libc::c_int);
    if handle == -(1 as libc::c_int) {
        return 0 as libc::c_int as libc::c_long;
    }
    size = filelength(handle);
    close(handle);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn _printhexb(mut value: libc::c_uchar) {
    let mut loop_0: libc::c_int = 0;
    let mut hexstr: [libc::c_char; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [libc::c_char; 16]>(b"0123456789ABCDEF");
    let mut str_0: [libc::c_char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"\0\0");
    loop_0 = 0 as libc::c_int;
    while loop_0 < 2 as libc::c_int {
        str_0[0 as libc::c_int as usize] = hexstr[(value as libc::c_int
            >> (1 as libc::c_int - loop_0) * 4 as libc::c_int
            & 15 as libc::c_int) as usize];
        print(str_0.as_mut_ptr());
        loop_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _printhex(mut value: libc::c_uint) {
    print(b"$\0" as *const u8 as *const libc::c_char);
    _printhexb((value >> 8 as libc::c_int) as libc::c_uchar);
    _printhexb((value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn _printbin(mut value: libc::c_uint) {
    let mut loop_0: libc::c_int = 0;
    print(b"%\0" as *const u8 as *const libc::c_char);
    loop_0 = 0 as libc::c_int;
    while loop_0 < 16 as libc::c_int {
        if value >> 15 as libc::c_int - loop_0 & 1 as libc::c_int as libc::c_uint != 0 {
            print(b"1\0" as *const u8 as *const libc::c_char);
        } else {
            print(b"0\0" as *const u8 as *const libc::c_char);
        }
        loop_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _printc(mut string: *mut libc::c_char) {
    sx = 1 as libc::c_int + screencenterx
        - (strlen(string)).wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    print(string);
}
#[no_mangle]
pub unsafe extern "C" fn _inputint() -> libc::c_uint {
    let mut string: [libc::c_char; 18] = *::std::mem::transmute::<&[u8; 18], &mut [libc::c_char; 18]>(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut digit: libc::c_char = 0;
    let mut hexstr: [libc::c_char; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [libc::c_char; 16]>(b"0123456789ABCDEF");
    let mut value: libc::c_uint = 0;
    let mut loop_0: libc::c_uint = 0;
    let mut loop1: libc::c_uint = 0;
    _input(string.as_mut_ptr(), 17 as libc::c_int);
    if string[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        let mut digits: ssize_t = 0;
        digits = (strlen(string.as_mut_ptr())).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as ssize_t;
        if digits < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as libc::c_uint;
        }
        value = 0 as libc::c_int as libc::c_uint;
        loop1 = 0 as libc::c_int as libc::c_uint;
        while loop1 as libc::c_ulong <= digits as size_t {
            digit = toupper(
                string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int,
            ) as libc::c_char;
            loop_0 = 0 as libc::c_int as libc::c_uint;
            while loop_0 < 16 as libc::c_int as libc::c_uint {
                if digit as libc::c_int == hexstr[loop_0 as usize] as libc::c_int {
                    value |= loop_0
                        << (digits - loop1 as libc::c_long) * 4 as libc::c_int as libc::c_long;
                    break;
                } else {
                    loop_0 = loop_0.wrapping_add(1);
                }
            }
            loop1 = loop1.wrapping_add(1);
        }
    } else if string[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
        let mut digits_0: ssize_t = 0;
        digits_0 = (strlen(string.as_mut_ptr())).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as ssize_t;
        if digits_0 < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as libc::c_uint;
        }
        value = 0 as libc::c_int as libc::c_uint;
        loop1 = 0 as libc::c_int as libc::c_uint;
        while loop1 as libc::c_ulong <= digits_0 as size_t {
            if (string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int)
                < '0' as i32
                || string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int
                    > '1' as i32
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            value |= ((string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int
                - '0' as i32)
                << digits_0 - loop1 as libc::c_long) as libc::c_uint;
            loop1 = loop1.wrapping_add(1);
        }
    } else {
        value = atoi(string.as_mut_ptr()) as libc::c_uint;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn _input(
    mut string: *mut libc::c_char,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut key_0: libc::c_char = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut loop_0: libc::c_int = 0;
    loop {
        key_0 = toupper(get() & 0xff as libc::c_int) as libc::c_char;
        if (key_0 as libc::c_int == 127 as libc::c_int || key_0 as libc::c_int == 8 as libc::c_int)
            && count > 0 as libc::c_int
        {
            count -= 1;
            drawchar(sx, sy, ' ' as i32);
            sx -= 1;
        }
        if key_0 as libc::c_int >= ' ' as i32 && key_0 as libc::c_int <= 'z' as i32 && count < max {
            let fresh7 = count;
            count = count + 1;
            *string.offset(fresh7 as isize) = key_0;
            let fresh8 = sx;
            sx = sx + 1;
            drawchar(fresh8, sy, key_0 as libc::c_int);
        }
        if !(key_0 as libc::c_int != 27 as libc::c_int && key_0 as libc::c_int != 13 as libc::c_int)
        {
            break;
        }
    }
    loop_0 = count;
    while loop_0 < max {
        *string.offset(loop_0 as isize) = 0 as libc::c_int as libc::c_char;
        loop_0 += 1;
    }
    if key_0 as libc::c_int == 13 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
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
pub static mut score: sdword = 0;
#[no_mangle]
pub static mut level: sword = 0;
#[no_mangle]
pub static mut _numlevels: libc::c_int = 0;
#[no_mangle]
pub static mut _maxplayers: libc::c_int = 0;
#[no_mangle]
pub static mut _extension: *const libc::c_char = b"PCR\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut _cgaok: boolean = true_0 as libc::c_int as boolean;
#[no_mangle]
pub static mut _egaok: boolean = true_0 as libc::c_int as boolean;
#[no_mangle]
pub static mut _vgaok: boolean = false_0 as libc::c_int as boolean;
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
pub unsafe extern "C" fn ScancodeToDOS(mut sc: SDL_Scancode) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if DOSScanCodeMap[i as usize] as libc::c_uint == sc as libc::c_uint {
            return i;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CheckMouseMode() {
    SDL_SetRelativeMouseMode(
        (hasFocus as libc::c_int != 0
            && (playermode[1 as libc::c_int as usize] as libc::c_uint
                == mouse as libc::c_int as libc::c_uint
                || playermode[2 as libc::c_int as usize] as libc::c_uint
                    == mouse as libc::c_int as libc::c_uint)) as libc::c_int as SDL_bool,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _loadctrls() {
    let mut handle: libc::c_int = 0;
    strcpy(
        str.as_mut_ptr(),
        b"CTLPANEL.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    handle = open(
        str.as_mut_ptr(),
        0 as libc::c_int | 0 as libc::c_int,
        0o200 as libc::c_int | 0o400 as libc::c_int,
    );
    if handle == -(1 as libc::c_int) {
        grmode = VGAgr;
        soundmode = spkr;
        playermode[1 as libc::c_int as usize] = keyboard;
        playermode[2 as libc::c_int as usize] = joystick1;
        JoyXlow[2 as libc::c_int as usize] = 20 as libc::c_int;
        JoyXlow[1 as libc::c_int as usize] = JoyXlow[2 as libc::c_int as usize];
        JoyXhigh[2 as libc::c_int as usize] = 60 as libc::c_int;
        JoyXhigh[1 as libc::c_int as usize] = JoyXhigh[2 as libc::c_int as usize];
        JoyYlow[2 as libc::c_int as usize] = 20 as libc::c_int;
        JoyYlow[1 as libc::c_int as usize] = JoyYlow[2 as libc::c_int as usize];
        JoyYhigh[2 as libc::c_int as usize] = 60 as libc::c_int;
        JoyYhigh[1 as libc::c_int as usize] = JoyYhigh[2 as libc::c_int as usize];
        MouseSensitivity = 5 as libc::c_int;
        key[north as libc::c_int as usize] = SDL_SCANCODE_UP as libc::c_int;
        key[northeast as libc::c_int as usize] = SDL_SCANCODE_PAGEUP as libc::c_int;
        key[east as libc::c_int as usize] = SDL_SCANCODE_RIGHT as libc::c_int;
        key[southeast as libc::c_int as usize] = SDL_SCANCODE_PAGEDOWN as libc::c_int;
        key[south as libc::c_int as usize] = SDL_SCANCODE_DOWN as libc::c_int;
        key[southwest as libc::c_int as usize] = SDL_SCANCODE_END as libc::c_int;
        key[west as libc::c_int as usize] = SDL_SCANCODE_LEFT as libc::c_int;
        key[northwest as libc::c_int as usize] = SDL_SCANCODE_HOME as libc::c_int;
        keyB1 = SDL_SCANCODE_LCTRL as libc::c_int;
        keyB2 = SDL_SCANCODE_LALT as libc::c_int;
    } else {
        let mut ctlpanel: ctlpaneltype = ctlpaneltype {
            grmode: 0,
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
            ::std::mem::size_of::<ctlpaneltype>() as libc::c_ulong,
        );
        close(handle);
        grmode = ctlpanel.grmode as grtype;
        soundmode = ctlpanel.soundmode as soundtype;
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 3 as libc::c_int as libc::c_uint {
            playermode[i as usize] = ctlpanel.playermode[i as usize] as inputtype;
            JoyXlow[i as usize] = ctlpanel.JoyXlow[i as usize] as libc::c_int;
            JoyYlow[i as usize] = ctlpanel.JoyYlow[i as usize] as libc::c_int;
            JoyXhigh[i as usize] = ctlpanel.JoyXhigh[i as usize] as libc::c_int;
            JoyYhigh[i as usize] = ctlpanel.JoyYhigh[i as usize] as libc::c_int;
            if playermode[i as usize] as libc::c_uint == mouse as libc::c_int as libc::c_uint {
                CheckMouseMode();
            }
            if playermode[i as usize] as libc::c_uint == joystick1 as libc::c_int as libc::c_uint
                || playermode[i as usize] as libc::c_uint
                    == joystick2 as libc::c_int as libc::c_uint
            {
                ProbeJoysticks();
                if playermode[i as usize] as libc::c_uint
                    == joystick1 as libc::c_int as libc::c_uint
                    && joystick[1 as libc::c_int as usize].device < 0 as libc::c_int
                    || playermode[i as usize] as libc::c_uint
                        == joystick2 as libc::c_int as libc::c_uint
                        && joystick[2 as libc::c_int as usize].device < 0 as libc::c_int
                {
                    playermode[i as usize] = keyboard;
                }
            }
            i = i.wrapping_add(1);
        }
        MouseSensitivity = ctlpanel.MouseSensitivity as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 8 as libc::c_int as libc::c_uint {
            key[i as usize] = DOSScanCodeMap[ctlpanel.key[i as usize] as usize] as libc::c_int;
            i = i.wrapping_add(1);
        }
        keyB1 = DOSScanCodeMap[ctlpanel.keyB1 as usize] as libc::c_int;
        keyB2 = DOSScanCodeMap[ctlpanel.keyB2 as usize] as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _savectrls() {
    let mut handle: libc::c_int = 0;
    let mut ctlpanel: ctlpaneltype = ctlpaneltype {
        grmode: 0,
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
    strcpy(
        str.as_mut_ptr(),
        b"CTLPANEL.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    handle = open(
        str.as_mut_ptr(),
        0o1 as libc::c_int | 0 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if handle == -(1 as libc::c_int) {
        return;
    }
    ctlpanel.grmode = grmode as word;
    ctlpanel.soundmode = soundmode as word;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        ctlpanel.playermode[i as usize] = playermode[i as usize] as word;
        ctlpanel.JoyXlow[i as usize] = JoyXlow[i as usize] as sword;
        ctlpanel.JoyYlow[i as usize] = JoyYlow[i as usize] as sword;
        ctlpanel.JoyXhigh[i as usize] = JoyXhigh[i as usize] as sword;
        ctlpanel.JoyYhigh[i as usize] = JoyYhigh[i as usize] as sword;
        i = i.wrapping_add(1);
    }
    ctlpanel.MouseSensitivity = MouseSensitivity as sword;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        ctlpanel.key[i as usize] = ScancodeToDOS(key[i as usize] as SDL_Scancode) as byte;
        i = i.wrapping_add(1);
    }
    ctlpanel.keyB1 = ScancodeToDOS(keyB1 as SDL_Scancode) as byte;
    ctlpanel.keyB2 = ScancodeToDOS(keyB2 as SDL_Scancode) as byte;
    write(
        handle,
        &mut ctlpanel as *mut ctlpaneltype as *const libc::c_void,
        ::std::mem::size_of::<ctlpaneltype>() as libc::c_ulong,
    );
    close(handle);
}
#[no_mangle]
pub unsafe extern "C" fn _loadhighscores() {
    let mut i: libc::c_int = 0;
    strcpy(
        str.as_mut_ptr(),
        b"SCORES.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    if LoadFile(
        str.as_mut_ptr(),
        highscores.as_mut_ptr() as *mut libc::c_char,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            highscores[i as usize].score = 100 as libc::c_int;
            highscores[i as usize].level = 1 as libc::c_int as sword;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"PCR\0" as *const u8 as *const libc::c_char,
            );
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _savehighscores() {
    strcpy(
        str.as_mut_ptr(),
        b"SCORES.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        highscores.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[scores; 5]>() as libc::c_ulong as libc::c_long,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _showhighscores() {
    let mut i: libc::c_int = 0;
    let mut h: libc::c_long = 0;
    let mut st2: [libc::c_char; 10] = [0; 10];
    centerwindow(17 as libc::c_int, 17 as libc::c_int);
    print(b"\n   HIGH SCORES\n\n\0" as *const u8 as *const libc::c_char);
    print(b" #  SCORE LV  BY\n\0" as *const u8 as *const libc::c_char);
    print(b" - ------ -- ---\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        sx += 1;
        drawchar(sx, sy, '1' as i32 + i);
        sx += 2 as libc::c_int;
        h = highscores[i as usize].score as libc::c_long;
        if h < 100000 as libc::c_long {
            sx += 1;
        }
        if h < 10000 as libc::c_long {
            sx += 1;
        }
        if h < 1000 as libc::c_long {
            sx += 1;
        }
        if h < 100 as libc::c_long {
            sx += 1;
        }
        if h < 10 as libc::c_long {
            sx += 1;
        }
        ltoa(h as libc::c_int, str.as_mut_ptr(), 10 as libc::c_int);
        print(str.as_mut_ptr());
        sx += 1;
        if (highscores[i as usize].level as libc::c_int) < 10 as libc::c_int {
            sx += 1;
        }
        itoa(
            highscores[i as usize].level as libc::c_int,
            str.as_mut_ptr(),
            10 as libc::c_int,
        );
        print(str.as_mut_ptr());
        sx += 1;
        print((highscores[i as usize].initials).as_mut_ptr());
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    strcpy(
        str.as_mut_ptr(),
        b"SCORE:\0" as *const u8 as *const libc::c_char,
    );
    ltoa(score, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    _printc(str.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _checkhighscore() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if score > highscores[i as usize].score {
            j = 4 as libc::c_int;
            while i < j {
                k = j - 1 as libc::c_int;
                highscores[j as usize] = highscores[k as usize];
                j -= 1;
            }
            highscores[i as usize].score = score;
            highscores[i as usize].level = level;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"   \0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            i += 1;
        }
    }
    _showhighscores();
    UpdateScreen();
    if i < 5 as libc::c_int {
        PlaySound(16 as libc::c_int);
        clearkeys();
        sx = screencenterx - 17 as libc::c_int / 2 as libc::c_int + 14 as libc::c_int;
        sy = screencentery - 17 as libc::c_int / 2 as libc::c_int
            + 6 as libc::c_int
            + i * 2 as libc::c_int;
        j = 0 as libc::c_int;
        loop {
            k = get();
            ch = k as libc::c_char;
            if ch as libc::c_int >= ' ' as i32 && j < 3 as libc::c_int {
                drawchar(sx, sy, ch as libc::c_int);
                sx += 1;
                highscores[i as usize].initials[j as usize] = ch;
                j += 1;
            }
            if ch as libc::c_int == 8 as libc::c_int || k == 19200 as libc::c_int {
                if j > 0 as libc::c_int {
                    sx -= 1;
                    j -= 1;
                }
            }
            if !(ch as libc::c_int != 13 as libc::c_int) {
                break;
            }
        }
    }
}
#[no_mangle]
pub static mut screenseg: [byte; 64000] = [0; 64000];
static mut VideoParmStrings: [*const libc::c_char; 3] = [
    b"windowed\0" as *const u8 as *const libc::c_char,
    b"screen\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn _setupgame() {
    if SDL_Init(
        0x20 as libc::c_uint | 0x1 as libc::c_uint | 0x200 as libc::c_uint | 0x2000 as libc::c_uint,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Failed to initialize SDL: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    atexit(Some(SDL_Quit as unsafe extern "C" fn() -> ()));
    SDL_AddEventWatch(
        Some(
            WatchUIEvents as unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    let mut i: libc::c_int = 0;
    let mut windowed: boolean = false_0 as libc::c_int as boolean;
    let mut winWidth: libc::c_uint = 640 as libc::c_int as libc::c_uint;
    let mut winHeight: libc::c_uint = 480 as libc::c_int as libc::c_uint;
    let mut displayindex: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < _argc {
        match US_CheckParm(
            *_argv.offset(i as isize),
            VideoParmStrings.as_mut_ptr() as *mut *mut libc::c_char,
        ) {
            0 => {
                windowed = true_0 as libc::c_int as boolean;
                i += 1;
                if i < _argc {
                    winWidth = atoi(*_argv.offset(i as isize)) as libc::c_uint;
                }
                i += 1;
                if i < _argc {
                    winHeight = atoi(*_argv.offset(i as isize)) as libc::c_uint;
                }
            }
            1 => {
                i += 1;
                if i < _argc {
                    displayindex = atoi(*_argv.offset(i as isize));
                }
            }
            _ => {}
        }
        i += 1;
    }
    let mut bounds: SDL_Rect = SDL_Rect {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
    if SDL_GetCurrentDisplayMode(displayindex, &mut mode) < -(1 as libc::c_int)
        || SDL_GetDisplayBounds(displayindex, &mut bounds) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not get display mode: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    if windowed != 0 {
        bounds.x = (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int;
        bounds.y = (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int;
        mode.w = winWidth as libc::c_int;
        mode.h = winHeight as libc::c_int;
    }
    window = SDL_CreateWindow(
        b"The Catacomb\0" as *const u8 as *const libc::c_char,
        bounds.x,
        bounds.y,
        mode.w,
        mode.h,
        (if windowed as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            SDL_WINDOW_FULLSCREEN_DESKTOP as libc::c_int
        }) as Uint32,
    );
    if window.is_null() || {
        renderer = SDL_CreateRenderer(window, -(1 as libc::c_int), 0 as libc::c_int as Uint32);
        renderer.is_null()
    } {
        fprintf(
            stderr,
            b"Failed to create SDL window: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    sdltexture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888 as libc::c_int as Uint32,
        SDL_TEXTUREACCESS_STREAMING as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
    );
    if sdltexture.is_null() {
        fprintf(
            stderr,
            b"Could not create video buffer: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    if mode.w == 320 as libc::c_int && mode.h == 200 as libc::c_int
        || mode.w == 640 as libc::c_int && mode.h == 400 as libc::c_int
    {
        updateRect.w = mode.w;
        updateRect.h = mode.h;
        updateRect.y = 0 as libc::c_int;
        updateRect.x = updateRect.y;
    } else {
        updateRect.h = mode.h;
        updateRect.w = mode.h * 4 as libc::c_int / 3 as libc::c_int;
        updateRect.x = mode.w - updateRect.w >> 1 as libc::c_int;
        updateRect.y = 0 as libc::c_int;
    }
    memset(
        screenseg.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong,
    );
    grmode = EGAgr;
    joystick[2 as libc::c_int as usize].device = -(1 as libc::c_int);
    joystick[1 as libc::c_int as usize].device = joystick[2 as libc::c_int as usize].device;
    _loadctrls();
    if grmode as libc::c_uint == VGAgr as libc::c_int as libc::c_uint && _vgaok as libc::c_int != 0
    {
        grmode = VGAgr;
    } else if grmode as libc::c_uint >= EGAgr as libc::c_int as libc::c_uint
        && _egaok as libc::c_int != 0
    {
        grmode = EGAgr;
    } else {
        grmode = CGAgr;
    }
    strcpy(
        str.as_mut_ptr(),
        b"SOUNDS.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    SoundData = bloadin(str.as_mut_ptr()) as *mut SPKRtable;
    StartupSound();
    SetupKBD();
    initrndt(1 as libc::c_int as boolean);
    initrnd(1 as libc::c_int as boolean);
    _loadhighscores();
    loadgrfiles();
    SetupEmulatedVBL();
}
#[no_mangle]
pub unsafe extern "C" fn _quit(mut error: *mut libc::c_char) {
    if *error == 0 {
        _savehighscores();
        _savectrls();
    } else {
        puts(error);
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        puts(
            b"For techinical assistance with running this software\n\0" as *const u8
                as *const libc::c_char,
        );
        puts(
            b"    call Softdisk Publishing at 1-318-221-8311\n\0" as *const u8
                as *const libc::c_char,
        );
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    ShutdownSound();
    ShutdownJoysticks();
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    renderer = 0 as *mut SDL_Renderer;
    window = 0 as *mut SDL_Window;
    exit(0 as libc::c_int);
}
