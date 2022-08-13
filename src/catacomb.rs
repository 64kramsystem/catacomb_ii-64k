#![allow(clippy::all)]
#![deny(clippy::correctness)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[allow(unused_imports)] // the import is actually used!
use ::catacomb_lib::*;
use catacomb_lib::{extra_constants::*, pcrlib_c::_setupgame};
use libc::O_RDONLY;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn givenuke();
    fn givebolt();
    fn givepotion();
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn playloop();
    fn initobjects();
    fn RLEExpand(source: *mut libc::c_char, dest: *mut libc::c_char, origlen: libc::c_long);
    fn bioskey(_: libc::c_int) -> libc::c_int;
    fn _quit(_: *mut libc::c_char);
    fn _checkhighscore();
    fn _showhighscores();
    static mut _extension: *const libc::c_char;
    static mut _maxplayers: libc::c_int;
    static mut _numlevels: libc::c_int;
    static mut score: sdword;
    fn printhighscore();
    fn printbody();
    fn printshotpower();
    fn printscore();
    static mut level: sword;
    fn expwin(width: libc::c_int, height: libc::c_int);
    fn centerwindow(width: libc::c_int, height: libc::c_int);
    fn bar(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int, ch_0: libc::c_int);
    fn drawwindow(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int);
    fn printint(val: libc::c_int);
    fn printchartile(str_0: *const libc::c_char);
    fn print(str_0: *const libc::c_char);
    fn get() -> libc::c_int;
    fn controlpanel();
    static mut screencentery: libc::c_int;
    static mut screencenterx: libc::c_int;
    fn drawchartile(x: libc::c_int, y: libc::c_int, tile: libc::c_int);
    fn drawpic(x: libc::c_int, y: libc::c_int, picnum: libc::c_int);
    fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
    fn installgrfile(filename: *mut libc::c_char, inmem: *mut libc::c_void);
    fn WaitVBL();
    fn UpdateScreen();
    static mut leftedge: libc::c_int;
    static mut sy: libc::c_int;
    static mut sx: libc::c_int;
    static mut grmode: grtype;
    fn _Verify(filename: *mut libc::c_char) -> libc::c_long;
    fn clearkeys();
    fn rnd(_: word) -> libc::c_int;
    fn rndt() -> libc::c_int;
    fn bloadin(filename: *mut libc::c_char) -> *mut libc::c_void;
    fn LoadFile(filename: *mut libc::c_char, buffer: *mut libc::c_char) -> libc::c_ulong;
    fn LoadDemo(demonum: libc::c_int);
    fn ControlPlayer(player_0: libc::c_int) -> ControlStruct;
    static mut indemo: demoenum;
    static mut keydown: [boolean; 512];
    fn WaitEndSound();
    fn PlaySound(sound: libc::c_int);
    static mut str: [libc::c_char; 80];
    static mut ch: libc::c_char;
    fn cgarefresh();
    fn egarefresh();
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type sbyte = int8_t;
pub type word = uint16_t;
pub type sword = int16_t;
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
pub type classtype = libc::c_uint;
pub const lastclass: classtype = 23;
pub const guns: classtype = 22;
pub const gune: classtype = 21;
pub const secretgate: classtype = 20;
pub const torch: classtype = 19;
pub const teleporter: classtype = 18;
pub const dead6: classtype = 17;
pub const dead5: classtype = 16;
pub const dead4: classtype = 15;
pub const dead3: classtype = 14;
pub const dead2: classtype = 13;
pub const dead1: classtype = 12;
pub const rock: classtype = 11;
pub const bigshot: classtype = 10;
pub const shot: classtype = 9;
pub const wallhit: classtype = 8;
pub const turbogre: classtype = 7;
pub const dragon: classtype = 6;
pub const gargoyle: classtype = 5;
pub const ogre: classtype = 4;
pub const skeleton: classtype = 3;
pub const goblin: classtype = 2;
pub const player: classtype = 1;
pub const nothing: classtype = 0;
pub type statetype = libc::c_uint;
pub const inscores: statetype = 2;
pub const intitle: statetype = 1;
pub const ingame: statetype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct activeobj {
    pub active: boolean,
    pub class: word,
    pub x: byte,
    pub y: byte,
    pub stage: byte,
    pub delay: byte,
    pub dir: word,
    pub hp: sbyte,
    pub oldx: byte,
    pub oldy: byte,
    pub oldtile: sword,
    pub filler: [byte; 1],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objdeftype {
    pub think: byte,
    pub contact: byte,
    pub solid: byte,
    pub firstchar: word,
    pub size: byte,
    pub stages: byte,
    pub dirmask: byte,
    pub speed: word,
    pub hitpoints: byte,
    pub damage: byte,
    pub points: word,
    pub filler: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objtype {
    pub active: boolean,
    pub class: word,
    pub x: byte,
    pub y: byte,
    pub stage: byte,
    pub delay: byte,
    pub dir: word,
    pub hp: sbyte,
    pub oldx: byte,
    pub oldy: byte,
    pub oldtile: sword,
    pub filler: [byte; 1],
    pub think: byte,
    pub contact: byte,
    pub solid: byte,
    pub firstchar: word,
    pub size: byte,
    pub stages: byte,
    pub dirmask: byte,
    pub speed: word,
    pub hitpoints: byte,
    pub damage: byte,
    pub points: word,
    pub filler2: [byte; 2],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SDL_NUM_SCANCODES: C2RustUnnamed_1 = 512;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: C2RustUnnamed_1 = 286;
pub const SDL_SCANCODE_AUDIOREWIND: C2RustUnnamed_1 = 285;
pub const SDL_SCANCODE_APP2: C2RustUnnamed_1 = 284;
pub const SDL_SCANCODE_APP1: C2RustUnnamed_1 = 283;
pub const SDL_SCANCODE_SLEEP: C2RustUnnamed_1 = 282;
pub const SDL_SCANCODE_EJECT: C2RustUnnamed_1 = 281;
pub const SDL_SCANCODE_KBDILLUMUP: C2RustUnnamed_1 = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: C2RustUnnamed_1 = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: C2RustUnnamed_1 = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: C2RustUnnamed_1 = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: C2RustUnnamed_1 = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: C2RustUnnamed_1 = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: C2RustUnnamed_1 = 274;
pub const SDL_SCANCODE_AC_REFRESH: C2RustUnnamed_1 = 273;
pub const SDL_SCANCODE_AC_STOP: C2RustUnnamed_1 = 272;
pub const SDL_SCANCODE_AC_FORWARD: C2RustUnnamed_1 = 271;
pub const SDL_SCANCODE_AC_BACK: C2RustUnnamed_1 = 270;
pub const SDL_SCANCODE_AC_HOME: C2RustUnnamed_1 = 269;
pub const SDL_SCANCODE_AC_SEARCH: C2RustUnnamed_1 = 268;
pub const SDL_SCANCODE_COMPUTER: C2RustUnnamed_1 = 267;
pub const SDL_SCANCODE_CALCULATOR: C2RustUnnamed_1 = 266;
pub const SDL_SCANCODE_MAIL: C2RustUnnamed_1 = 265;
pub const SDL_SCANCODE_WWW: C2RustUnnamed_1 = 264;
pub const SDL_SCANCODE_MEDIASELECT: C2RustUnnamed_1 = 263;
pub const SDL_SCANCODE_AUDIOMUTE: C2RustUnnamed_1 = 262;
pub const SDL_SCANCODE_AUDIOPLAY: C2RustUnnamed_1 = 261;
pub const SDL_SCANCODE_AUDIOSTOP: C2RustUnnamed_1 = 260;
pub const SDL_SCANCODE_AUDIOPREV: C2RustUnnamed_1 = 259;
pub const SDL_SCANCODE_AUDIONEXT: C2RustUnnamed_1 = 258;
pub const SDL_SCANCODE_MODE: C2RustUnnamed_1 = 257;
pub const SDL_SCANCODE_RGUI: C2RustUnnamed_1 = 231;
pub const SDL_SCANCODE_RALT: C2RustUnnamed_1 = 230;
pub const SDL_SCANCODE_RSHIFT: C2RustUnnamed_1 = 229;
pub const SDL_SCANCODE_RCTRL: C2RustUnnamed_1 = 228;
pub const SDL_SCANCODE_LGUI: C2RustUnnamed_1 = 227;
pub const SDL_SCANCODE_LALT: C2RustUnnamed_1 = 226;
pub const SDL_SCANCODE_LSHIFT: C2RustUnnamed_1 = 225;
pub const SDL_SCANCODE_LCTRL: C2RustUnnamed_1 = 224;
pub const SDL_SCANCODE_KP_HEXADECIMAL: C2RustUnnamed_1 = 221;
pub const SDL_SCANCODE_KP_DECIMAL: C2RustUnnamed_1 = 220;
pub const SDL_SCANCODE_KP_OCTAL: C2RustUnnamed_1 = 219;
pub const SDL_SCANCODE_KP_BINARY: C2RustUnnamed_1 = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: C2RustUnnamed_1 = 217;
pub const SDL_SCANCODE_KP_CLEAR: C2RustUnnamed_1 = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: C2RustUnnamed_1 = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: C2RustUnnamed_1 = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: C2RustUnnamed_1 = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: C2RustUnnamed_1 = 212;
pub const SDL_SCANCODE_KP_MEMADD: C2RustUnnamed_1 = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: C2RustUnnamed_1 = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: C2RustUnnamed_1 = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: C2RustUnnamed_1 = 208;
pub const SDL_SCANCODE_KP_EXCLAM: C2RustUnnamed_1 = 207;
pub const SDL_SCANCODE_KP_AT: C2RustUnnamed_1 = 206;
pub const SDL_SCANCODE_KP_SPACE: C2RustUnnamed_1 = 205;
pub const SDL_SCANCODE_KP_HASH: C2RustUnnamed_1 = 204;
pub const SDL_SCANCODE_KP_COLON: C2RustUnnamed_1 = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: C2RustUnnamed_1 = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: C2RustUnnamed_1 = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: C2RustUnnamed_1 = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: C2RustUnnamed_1 = 199;
pub const SDL_SCANCODE_KP_GREATER: C2RustUnnamed_1 = 198;
pub const SDL_SCANCODE_KP_LESS: C2RustUnnamed_1 = 197;
pub const SDL_SCANCODE_KP_PERCENT: C2RustUnnamed_1 = 196;
pub const SDL_SCANCODE_KP_POWER: C2RustUnnamed_1 = 195;
pub const SDL_SCANCODE_KP_XOR: C2RustUnnamed_1 = 194;
pub const SDL_SCANCODE_KP_F: C2RustUnnamed_1 = 193;
pub const SDL_SCANCODE_KP_E: C2RustUnnamed_1 = 192;
pub const SDL_SCANCODE_KP_D: C2RustUnnamed_1 = 191;
pub const SDL_SCANCODE_KP_C: C2RustUnnamed_1 = 190;
pub const SDL_SCANCODE_KP_B: C2RustUnnamed_1 = 189;
pub const SDL_SCANCODE_KP_A: C2RustUnnamed_1 = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: C2RustUnnamed_1 = 187;
pub const SDL_SCANCODE_KP_TAB: C2RustUnnamed_1 = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: C2RustUnnamed_1 = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: C2RustUnnamed_1 = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: C2RustUnnamed_1 = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: C2RustUnnamed_1 = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: C2RustUnnamed_1 = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: C2RustUnnamed_1 = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: C2RustUnnamed_1 = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: C2RustUnnamed_1 = 178;
pub const SDL_SCANCODE_KP_000: C2RustUnnamed_1 = 177;
pub const SDL_SCANCODE_KP_00: C2RustUnnamed_1 = 176;
pub const SDL_SCANCODE_EXSEL: C2RustUnnamed_1 = 164;
pub const SDL_SCANCODE_CRSEL: C2RustUnnamed_1 = 163;
pub const SDL_SCANCODE_CLEARAGAIN: C2RustUnnamed_1 = 162;
pub const SDL_SCANCODE_OPER: C2RustUnnamed_1 = 161;
pub const SDL_SCANCODE_OUT: C2RustUnnamed_1 = 160;
pub const SDL_SCANCODE_SEPARATOR: C2RustUnnamed_1 = 159;
pub const SDL_SCANCODE_RETURN2: C2RustUnnamed_1 = 158;
pub const SDL_SCANCODE_PRIOR: C2RustUnnamed_1 = 157;
pub const SDL_SCANCODE_CLEAR: C2RustUnnamed_1 = 156;
pub const SDL_SCANCODE_CANCEL: C2RustUnnamed_1 = 155;
pub const SDL_SCANCODE_SYSREQ: C2RustUnnamed_1 = 154;
pub const SDL_SCANCODE_ALTERASE: C2RustUnnamed_1 = 153;
pub const SDL_SCANCODE_LANG9: C2RustUnnamed_1 = 152;
pub const SDL_SCANCODE_LANG8: C2RustUnnamed_1 = 151;
pub const SDL_SCANCODE_LANG7: C2RustUnnamed_1 = 150;
pub const SDL_SCANCODE_LANG6: C2RustUnnamed_1 = 149;
pub const SDL_SCANCODE_LANG5: C2RustUnnamed_1 = 148;
pub const SDL_SCANCODE_LANG4: C2RustUnnamed_1 = 147;
pub const SDL_SCANCODE_LANG3: C2RustUnnamed_1 = 146;
pub const SDL_SCANCODE_LANG2: C2RustUnnamed_1 = 145;
pub const SDL_SCANCODE_LANG1: C2RustUnnamed_1 = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: C2RustUnnamed_1 = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: C2RustUnnamed_1 = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: C2RustUnnamed_1 = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: C2RustUnnamed_1 = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: C2RustUnnamed_1 = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: C2RustUnnamed_1 = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: C2RustUnnamed_1 = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: C2RustUnnamed_1 = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: C2RustUnnamed_1 = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: C2RustUnnamed_1 = 134;
pub const SDL_SCANCODE_KP_COMMA: C2RustUnnamed_1 = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: C2RustUnnamed_1 = 129;
pub const SDL_SCANCODE_VOLUMEUP: C2RustUnnamed_1 = 128;
pub const SDL_SCANCODE_MUTE: C2RustUnnamed_1 = 127;
pub const SDL_SCANCODE_FIND: C2RustUnnamed_1 = 126;
pub const SDL_SCANCODE_PASTE: C2RustUnnamed_1 = 125;
pub const SDL_SCANCODE_COPY: C2RustUnnamed_1 = 124;
pub const SDL_SCANCODE_CUT: C2RustUnnamed_1 = 123;
pub const SDL_SCANCODE_UNDO: C2RustUnnamed_1 = 122;
pub const SDL_SCANCODE_AGAIN: C2RustUnnamed_1 = 121;
pub const SDL_SCANCODE_STOP: C2RustUnnamed_1 = 120;
pub const SDL_SCANCODE_SELECT: C2RustUnnamed_1 = 119;
pub const SDL_SCANCODE_MENU: C2RustUnnamed_1 = 118;
pub const SDL_SCANCODE_HELP: C2RustUnnamed_1 = 117;
pub const SDL_SCANCODE_EXECUTE: C2RustUnnamed_1 = 116;
pub const SDL_SCANCODE_F24: C2RustUnnamed_1 = 115;
pub const SDL_SCANCODE_F23: C2RustUnnamed_1 = 114;
pub const SDL_SCANCODE_F22: C2RustUnnamed_1 = 113;
pub const SDL_SCANCODE_F21: C2RustUnnamed_1 = 112;
pub const SDL_SCANCODE_F20: C2RustUnnamed_1 = 111;
pub const SDL_SCANCODE_F19: C2RustUnnamed_1 = 110;
pub const SDL_SCANCODE_F18: C2RustUnnamed_1 = 109;
pub const SDL_SCANCODE_F17: C2RustUnnamed_1 = 108;
pub const SDL_SCANCODE_F16: C2RustUnnamed_1 = 107;
pub const SDL_SCANCODE_F15: C2RustUnnamed_1 = 106;
pub const SDL_SCANCODE_F14: C2RustUnnamed_1 = 105;
pub const SDL_SCANCODE_F13: C2RustUnnamed_1 = 104;
pub const SDL_SCANCODE_KP_EQUALS: C2RustUnnamed_1 = 103;
pub const SDL_SCANCODE_POWER: C2RustUnnamed_1 = 102;
pub const SDL_SCANCODE_APPLICATION: C2RustUnnamed_1 = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: C2RustUnnamed_1 = 100;
pub const SDL_SCANCODE_KP_PERIOD: C2RustUnnamed_1 = 99;
pub const SDL_SCANCODE_KP_0: C2RustUnnamed_1 = 98;
pub const SDL_SCANCODE_KP_9: C2RustUnnamed_1 = 97;
pub const SDL_SCANCODE_KP_8: C2RustUnnamed_1 = 96;
pub const SDL_SCANCODE_KP_7: C2RustUnnamed_1 = 95;
pub const SDL_SCANCODE_KP_6: C2RustUnnamed_1 = 94;
pub const SDL_SCANCODE_KP_5: C2RustUnnamed_1 = 93;
pub const SDL_SCANCODE_KP_4: C2RustUnnamed_1 = 92;
pub const SDL_SCANCODE_KP_3: C2RustUnnamed_1 = 91;
pub const SDL_SCANCODE_KP_2: C2RustUnnamed_1 = 90;
pub const SDL_SCANCODE_KP_1: C2RustUnnamed_1 = 89;
pub const SDL_SCANCODE_KP_ENTER: C2RustUnnamed_1 = 88;
pub const SDL_SCANCODE_KP_PLUS: C2RustUnnamed_1 = 87;
pub const SDL_SCANCODE_KP_MINUS: C2RustUnnamed_1 = 86;
pub const SDL_SCANCODE_KP_MULTIPLY: C2RustUnnamed_1 = 85;
pub const SDL_SCANCODE_KP_DIVIDE: C2RustUnnamed_1 = 84;
pub const SDL_SCANCODE_NUMLOCKCLEAR: C2RustUnnamed_1 = 83;
pub const SDL_SCANCODE_UP: C2RustUnnamed_1 = 82;
pub const SDL_SCANCODE_DOWN: C2RustUnnamed_1 = 81;
pub const SDL_SCANCODE_LEFT: C2RustUnnamed_1 = 80;
pub const SDL_SCANCODE_RIGHT: C2RustUnnamed_1 = 79;
pub const SDL_SCANCODE_PAGEDOWN: C2RustUnnamed_1 = 78;
pub const SDL_SCANCODE_END: C2RustUnnamed_1 = 77;
pub const SDL_SCANCODE_DELETE: C2RustUnnamed_1 = 76;
pub const SDL_SCANCODE_PAGEUP: C2RustUnnamed_1 = 75;
pub const SDL_SCANCODE_HOME: C2RustUnnamed_1 = 74;
pub const SDL_SCANCODE_INSERT: C2RustUnnamed_1 = 73;
pub const SDL_SCANCODE_PAUSE: C2RustUnnamed_1 = 72;
pub const SDL_SCANCODE_SCROLLLOCK: C2RustUnnamed_1 = 71;
pub const SDL_SCANCODE_PRINTSCREEN: C2RustUnnamed_1 = 70;
pub const SDL_SCANCODE_F12: C2RustUnnamed_1 = 69;
pub const SDL_SCANCODE_F11: C2RustUnnamed_1 = 68;
pub const SDL_SCANCODE_F10: C2RustUnnamed_1 = 67;
pub const SDL_SCANCODE_F9: C2RustUnnamed_1 = 66;
pub const SDL_SCANCODE_F8: C2RustUnnamed_1 = 65;
pub const SDL_SCANCODE_F7: C2RustUnnamed_1 = 64;
pub const SDL_SCANCODE_F6: C2RustUnnamed_1 = 63;
pub const SDL_SCANCODE_F5: C2RustUnnamed_1 = 62;
pub const SDL_SCANCODE_F4: C2RustUnnamed_1 = 61;
pub const SDL_SCANCODE_F3: C2RustUnnamed_1 = 60;
pub const SDL_SCANCODE_F2: C2RustUnnamed_1 = 59;
pub const SDL_SCANCODE_F1: C2RustUnnamed_1 = 58;
pub const SDL_SCANCODE_CAPSLOCK: C2RustUnnamed_1 = 57;
pub const SDL_SCANCODE_SLASH: C2RustUnnamed_1 = 56;
pub const SDL_SCANCODE_PERIOD: C2RustUnnamed_1 = 55;
pub const SDL_SCANCODE_COMMA: C2RustUnnamed_1 = 54;
pub const SDL_SCANCODE_GRAVE: C2RustUnnamed_1 = 53;
pub const SDL_SCANCODE_APOSTROPHE: C2RustUnnamed_1 = 52;
pub const SDL_SCANCODE_SEMICOLON: C2RustUnnamed_1 = 51;
pub const SDL_SCANCODE_NONUSHASH: C2RustUnnamed_1 = 50;
pub const SDL_SCANCODE_BACKSLASH: C2RustUnnamed_1 = 49;
pub const SDL_SCANCODE_RIGHTBRACKET: C2RustUnnamed_1 = 48;
pub const SDL_SCANCODE_LEFTBRACKET: C2RustUnnamed_1 = 47;
pub const SDL_SCANCODE_EQUALS: C2RustUnnamed_1 = 46;
pub const SDL_SCANCODE_MINUS: C2RustUnnamed_1 = 45;
pub const SDL_SCANCODE_SPACE: C2RustUnnamed_1 = 44;
pub const SDL_SCANCODE_TAB: C2RustUnnamed_1 = 43;
pub const SDL_SCANCODE_BACKSPACE: C2RustUnnamed_1 = 42;
pub const SDL_SCANCODE_ESCAPE: C2RustUnnamed_1 = 41;
pub const SDL_SCANCODE_RETURN: C2RustUnnamed_1 = 40;
pub const SDL_SCANCODE_0: C2RustUnnamed_1 = 39;
pub const SDL_SCANCODE_9: C2RustUnnamed_1 = 38;
pub const SDL_SCANCODE_8: C2RustUnnamed_1 = 37;
pub const SDL_SCANCODE_7: C2RustUnnamed_1 = 36;
pub const SDL_SCANCODE_6: C2RustUnnamed_1 = 35;
pub const SDL_SCANCODE_5: C2RustUnnamed_1 = 34;
pub const SDL_SCANCODE_4: C2RustUnnamed_1 = 33;
pub const SDL_SCANCODE_3: C2RustUnnamed_1 = 32;
pub const SDL_SCANCODE_2: C2RustUnnamed_1 = 31;
pub const SDL_SCANCODE_1: C2RustUnnamed_1 = 30;
pub const SDL_SCANCODE_Z: C2RustUnnamed_1 = 29;
pub const SDL_SCANCODE_Y: C2RustUnnamed_1 = 28;
pub const SDL_SCANCODE_X: C2RustUnnamed_1 = 27;
pub const SDL_SCANCODE_W: C2RustUnnamed_1 = 26;
pub const SDL_SCANCODE_V: C2RustUnnamed_1 = 25;
pub const SDL_SCANCODE_U: C2RustUnnamed_1 = 24;
pub const SDL_SCANCODE_T: C2RustUnnamed_1 = 23;
pub const SDL_SCANCODE_S: C2RustUnnamed_1 = 22;
pub const SDL_SCANCODE_R: C2RustUnnamed_1 = 21;
pub const SDL_SCANCODE_Q: C2RustUnnamed_1 = 20;
pub const SDL_SCANCODE_P: C2RustUnnamed_1 = 19;
pub const SDL_SCANCODE_O: C2RustUnnamed_1 = 18;
pub const SDL_SCANCODE_N: C2RustUnnamed_1 = 17;
pub const SDL_SCANCODE_M: C2RustUnnamed_1 = 16;
pub const SDL_SCANCODE_L: C2RustUnnamed_1 = 15;
pub const SDL_SCANCODE_K: C2RustUnnamed_1 = 14;
pub const SDL_SCANCODE_J: C2RustUnnamed_1 = 13;
pub const SDL_SCANCODE_I: C2RustUnnamed_1 = 12;
pub const SDL_SCANCODE_H: C2RustUnnamed_1 = 11;
pub const SDL_SCANCODE_G: C2RustUnnamed_1 = 10;
pub const SDL_SCANCODE_F: C2RustUnnamed_1 = 9;
pub const SDL_SCANCODE_E: C2RustUnnamed_1 = 8;
pub const SDL_SCANCODE_D: C2RustUnnamed_1 = 7;
pub const SDL_SCANCODE_C: C2RustUnnamed_1 = 6;
pub const SDL_SCANCODE_B: C2RustUnnamed_1 = 5;
pub const SDL_SCANCODE_A: C2RustUnnamed_1 = 4;
pub const SDL_SCANCODE_UNKNOWN: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: boolean,
    pub button2: boolean,
}
pub type demoenum = libc::c_uint;
pub const recording: demoenum = 2;
pub const demoplay: demoenum = 1;
pub const notdemo: demoenum = 0;
pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
pub type exittype = libc::c_uint;
pub const victorious: exittype = 3;
pub const reseted: exittype = 2;
pub const killed: exittype = 1;
pub const quited: exittype = 0;
#[inline]
unsafe extern "C" fn itoa(
    mut value: libc::c_int,
    mut str_0: *mut libc::c_char,
    mut base: libc::c_int,
) -> *mut libc::c_char {
    if base == 16 {
        sprintf(str_0, b"%X\0" as *const u8 as *const libc::c_char, value);
    } else {
        sprintf(str_0, b"%d\0" as *const u8 as *const libc::c_char, value);
    }
    return str_0;
}
#[no_mangle]
pub static mut altmeters: [[libc::c_char; 14]; 14] = [
    [
        127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [23, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 0],
];
#[no_mangle]
pub static mut meters: [[libc::c_char; 14]; 14] = [
    [
        127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [26, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 0],
];
#[no_mangle]
pub static mut opposite: [dirtype; 9] = [
    south, west, north, east, southwest, northwest, northeast, southeast, nodir,
];
#[no_mangle]
pub static mut gamexit: exittype = quited;
#[no_mangle]
pub static mut oldtiles: [libc::c_int; 576] = [0; 576];
#[no_mangle]
pub static mut background: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
pub static mut view: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
pub static mut originx: libc::c_int = 0;
#[no_mangle]
pub static mut originy: libc::c_int = 0;
#[no_mangle]
pub static mut priority: [byte; 2048] = [0; 2048];
#[no_mangle]
pub static mut items: [sword; 6] = [0; 6];
#[no_mangle]
pub static mut saveitems: [sword; 6] = [0; 6];
#[no_mangle]
pub static mut shotpower: libc::c_int = 0;
#[no_mangle]
pub static mut side: libc::c_int = 0;
#[no_mangle]
pub static mut boltsleft: libc::c_int = 0;
#[no_mangle]
pub static mut o: [activeobj; 201] = [activeobj {
    active: 0,
    class: 0,
    x: 0,
    y: 0,
    stage: 0,
    delay: 0,
    dir: 0,
    hp: 0,
    oldx: 0,
    oldy: 0,
    oldtile: 0,
    filler: [0; 1],
}; 201];
#[no_mangle]
pub static mut saveo: [activeobj; 1] = [activeobj {
    active: 0,
    class: 0,
    x: 0,
    y: 0,
    stage: 0,
    delay: 0,
    dir: 0,
    hp: 0,
    oldx: 0,
    oldy: 0,
    oldtile: 0,
    filler: [0; 1],
}; 1];
#[no_mangle]
pub static mut obj: objtype = objtype {
    active: 0,
    class: 0,
    x: 0,
    y: 0,
    stage: 0,
    delay: 0,
    dir: 0,
    hp: 0,
    oldx: 0,
    oldy: 0,
    oldtile: 0,
    filler: [0; 1],
    think: 0,
    contact: 0,
    solid: 0,
    firstchar: 0,
    size: 0,
    stages: 0,
    dirmask: 0,
    speed: 0,
    hitpoints: 0,
    damage: 0,
    points: 0,
    filler2: [0; 2],
};
#[no_mangle]
pub static mut altobj: objtype = objtype {
    active: 0,
    class: 0,
    x: 0,
    y: 0,
    stage: 0,
    delay: 0,
    dir: 0,
    hp: 0,
    oldx: 0,
    oldy: 0,
    oldtile: 0,
    filler: [0; 1],
    think: 0,
    contact: 0,
    solid: 0,
    firstchar: 0,
    size: 0,
    stages: 0,
    dirmask: 0,
    speed: 0,
    hitpoints: 0,
    damage: 0,
    points: 0,
    filler2: [0; 2],
};
#[no_mangle]
pub static mut altnum: libc::c_int = 0;
#[no_mangle]
pub static mut numobj: libc::c_int = 0;
#[no_mangle]
pub static mut objecton: libc::c_int = 0;
#[no_mangle]
pub static mut objdef: [objdeftype; 23] = [objdeftype {
    think: 0,
    contact: 0,
    solid: 0,
    firstchar: 0,
    size: 0,
    stages: 0,
    dirmask: 0,
    speed: 0,
    hitpoints: 0,
    damage: 0,
    points: 0,
    filler: [0; 2],
}; 23];
#[no_mangle]
pub static mut playdone: boolean = 0;
#[no_mangle]
pub static mut leveldone: boolean = 0;
#[no_mangle]
pub static mut tempb: boolean = 0;
#[no_mangle]
pub static mut tempp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut chkx: libc::c_int = 0;
#[no_mangle]
pub static mut chky: libc::c_int = 0;
#[no_mangle]
pub static mut chkspot: libc::c_int = 0;
#[no_mangle]
pub static mut frameon: word = 0;
#[no_mangle]
pub static mut grmem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut VGAPAL: libc::c_int = 0;
#[no_mangle]
pub static mut exitdemo: bool = false;
#[no_mangle]
pub static mut resetgame: boolean = 0;
#[no_mangle]
pub static mut gamestate: statetype = ingame;
#[no_mangle]
pub static mut ctrl: ControlStruct = ControlStruct {
    dir: north,
    button1: 0,
    button2: 0,
};
#[no_mangle]
pub static mut pics: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut picsexact: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut EGADATASTART: libc::c_uint = 0;
#[no_mangle]
pub static mut savescore: sdword = 0;
#[no_mangle]
pub static mut GODMODE: boolean = false as boolean;
#[no_mangle]
pub static mut demowin: [[libc::c_char; 16]; 5] = [
    [
        14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16,
    ],
    [
        17,
        ' ' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        18,
    ],
    [
        17,
        'S' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        18,
    ],
    [
        17,
        'F' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        18,
    ],
    [
        19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn refresh() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut basex: libc::c_int = 0;
    let mut basey: libc::c_int = 0;
    let mut underwin: [[word; 16]; 5] = [[0; 16]; 5];
    basex = originx + 4;
    basey = originy + 17;
    if indemo as u64 != 0 {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                underwin[y as usize][x as usize] =
                    view[(y + basey) as usize][(x + basex) as usize] as word;
                view[(y + basey) as usize][(x + basex) as usize] =
                    demowin[y as usize][x as usize] as libc::c_int;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        cgarefresh();
    } else {
        egarefresh();
    }
    if indemo as u64 != 0 {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                view[(y + basey) as usize][(x + basex) as usize] =
                    underwin[y as usize][x as usize] as libc::c_int;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
}
#[no_mangle]
pub unsafe extern "C" fn simplerefresh() {
    WaitVBL();
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        cgarefresh();
    } else {
        egarefresh();
    };
}
#[no_mangle]
pub unsafe extern "C" fn loadgrfiles() {
    if !picsexact.is_null() {
        free(picsexact as *mut libc::c_void);
    }
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        pics = bloadin(b"CGACHARS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
        picsexact = pics;
        installgrfile(
            b"CGAPICS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    } else {
        pics = bloadin(b"EGACHARS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
        picsexact = pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn clearold() {
    memset(
        &mut oldtiles as *mut [libc::c_int; 576] as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 576]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn restore() {
    clearold();
    simplerefresh();
}
#[no_mangle]
pub unsafe extern "C" fn wantmore() -> boolean {
    sx = 2;
    sy = 20;
    print(b"(space for more/esc)\0" as *const u8 as *const libc::c_char);
    sx = 12;
    sy = 21;
    ch = get() as libc::c_char;
    if ch as libc::c_int == 27 {
        return false as boolean;
    }
    return true as boolean;
}
unsafe extern "C" fn charpic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut c: classtype,
    mut dir: dirtype,
    mut stage: libc::c_int,
) {
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tilenum: libc::c_int = 0;
    size = objdef[c as usize].size as libc::c_int;
    tilenum = (objdef[c as usize].firstchar as libc::c_uint).wrapping_add(
        ((size * size) as libc::c_uint).wrapping_mul(
            (dir as libc::c_uint & objdef[c as usize].dirmask as libc::c_uint)
                .wrapping_mul(objdef[c as usize].stages as libc::c_uint)
                .wrapping_add(stage as libc::c_uint),
        ),
    ) as libc::c_int;
    yy = y;
    while yy <= y + size - 1 {
        xx = x;
        while xx <= x + size - 1 {
            let fresh0 = tilenum;
            tilenum = tilenum + 1;
            drawchartile(xx, yy, fresh0);
            xx += 1;
        }
        yy += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn help() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    centerwindow(20, 20);
    print(b"  C A T A C O M B   \n\0" as *const u8 as *const libc::c_char);
    print(b"   - - - - - - -    \n\0" as *const u8 as *const libc::c_char);
    print(b" by John Carmack    \n\0" as *const u8 as *const libc::c_char);
    print(b"                    \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    print(b"f1 = help           \n\0" as *const u8 as *const libc::c_char);
    print(b"f2 = control panel  \n\0" as *const u8 as *const libc::c_char);
    print(b"f3 = game reset     \n\0" as *const u8 as *const libc::c_char);
    print(b"f4 = save game      \n\0" as *const u8 as *const libc::c_char);
    print(b"f5 = load saved game\n\0" as *const u8 as *const libc::c_char);
    print(b"f9 = pause          \n\0" as *const u8 as *const libc::c_char);
    print(b"f10 / ESC = quit    \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    print(b"hit fire at the demo\n\0" as *const u8 as *const libc::c_char);
    print(b"to begin playing.   \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\nKeyboard controls:  \n\n\0" as *const u8 as *const libc::c_char);
    print(b"move    : arrows    \n\0" as *const u8 as *const libc::c_char);
    print(b"button1 : ctrl      \n\0" as *const u8 as *const libc::c_char);
    print(b"button2 : alt       \n\0" as *const u8 as *const libc::c_char);
    print(b"\nTo switch to mouse \n\0" as *const u8 as *const libc::c_char);
    print(b"or joystick control,\n\0" as *const u8 as *const libc::c_char);
    print(b"hit f2             \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"Button 1 / ctrl key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Builds shot power.  \n\0" as *const u8 as *const libc::c_char);
    print(b"If the shot power   \n\0" as *const u8 as *const libc::c_char);
    print(b"meter is full when  \n\0" as *const u8 as *const libc::c_char);
    print(b"the button is       \n\0" as *const u8 as *const libc::c_char);
    print(b"released, a super   \n\0" as *const u8 as *const libc::c_char);
    print(b"shot will be        \n\0" as *const u8 as *const libc::c_char);
    print(b"launched.           \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2);
    charpic(19, 15, shot, east, 1);
    charpic(17, 14, shot, east, 0);
    charpic(15, 15, shot, east, 1);
    charpic(8, 14, bigshot, east, 0);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"Button 2 / alt key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Allows you to move  \n\0" as *const u8 as *const libc::c_char);
    print(b"without changing the\n\0" as *const u8 as *const libc::c_char);
    print(b"direction you are   \n\0" as *const u8 as *const libc::c_char);
    print(b"facing.  Good for   \n\0" as *const u8 as *const libc::c_char);
    print(b"searching walls and \n\0" as *const u8 as *const libc::c_char);
    print(b"fighting retreats.  \n\0" as *const u8 as *const libc::c_char);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            if y == 15 {
                drawchartile(x, y, 129);
            } else if y == 16 {
                drawchartile(x, y, 131);
            } else {
                drawchartile(x, y, 128);
            }
            x += 1;
        }
        y += 1;
    }
    charpic(6, 13, player, south, 2);
    sx = 6;
    sy = 15;
    print(b"\x1D\x1D\x1E\x1E\x1F\x1F\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\"P\" or \"space\" will \n\0" as *const u8 as *const libc::c_char);
    print(b"take a healing      \n\0" as *const u8 as *const libc::c_char);
    print(b"potion if you have  \n\0" as *const u8 as *const libc::c_char);
    print(b"one.  This restores \n\0" as *const u8 as *const libc::c_char);
    print(b"the body meter to   \n\0" as *const u8 as *const libc::c_char);
    print(b"full strength.  Keep\n\0" as *const u8 as *const libc::c_char);
    print(b"a sharp eye on the  \n\0" as *const u8 as *const libc::c_char);
    print(b"meter, because when \n\0" as *const u8 as *const libc::c_char);
    print(b"it runs out, you are\n\0" as *const u8 as *const libc::c_char);
    print(b"dead!               \n\n\0" as *const u8 as *const libc::c_char);
    print(b"\"B\" will cast a bolt\n\0" as *const u8 as *const libc::c_char);
    print(b"spell if you have   \n\0" as *const u8 as *const libc::c_char);
    print(b"any.  You can mow   \n\0" as *const u8 as *const libc::c_char);
    print(b"down a lot of       \n\0" as *const u8 as *const libc::c_char);
    print(b"monsters with a bit \n\0" as *const u8 as *const libc::c_char);
    print(b"of skill.           \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\"N\" or \"enter\" will \n\0" as *const u8 as *const libc::c_char);
    print(b"cast a nuke spell.  \n\0" as *const u8 as *const libc::c_char);
    print(b"This usually wipes  \n\0" as *const u8 as *const libc::c_char);
    print(b"out all the monsters\n\0" as *const u8 as *const libc::c_char);
    print(b"near you.  Consider \n\0" as *const u8 as *const libc::c_char);
    print(b"it a panic button   \n\0" as *const u8 as *const libc::c_char);
    print(b"when you are being  \n\0" as *const u8 as *const libc::c_char);
    print(b"mobbed by monsters! \n\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"POTIONS:       \x80\xA2\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"SCROLLS:       \x80\xA3\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b" (BOLTS/NUKES) \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"TREASURE:      \x80\xA7\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b" (POINTS)      \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    wantmore();
}
#[no_mangle]
pub unsafe extern "C" fn reset() {
    centerwindow(18, 1);
    print(b"reset game (y/n)?\0" as *const u8 as *const libc::c_char);
    ch = get() as libc::c_char;
    if ch as libc::c_int == 'y' as i32 {
        gamexit = killed;
        playdone = true as boolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn loadlevel() {
    let mut i: libc::c_int = 0;
    let mut tokens: [classtype; 26] = [
        player, teleporter, goblin, skeleton, ogre, gargoyle, dragon, turbogre, guns, gune,
        secretgate, nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing,
        nothing, nothing, nothing, nothing, nothing, nothing, nothing,
    ];
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut st: [libc::c_char; 64] = [0; 64];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut btile: byte = 0;
    let mut sm: [libc::c_char; 4096] = [0; 4096];
    let mut rle: [libc::c_char; 4096] = [0; 4096];
    strcpy(
        filename.as_mut_ptr(),
        b"LEVEL\0" as *const u8 as *const libc::c_char,
    );
    itoa(level as libc::c_int, st.as_mut_ptr(), 10);
    strcat(filename.as_mut_ptr(), st.as_mut_ptr());
    strcat(
        filename.as_mut_ptr(),
        b".CA2\0" as *const u8 as *const libc::c_char,
    );
    LoadFile(filename.as_mut_ptr(), rle.as_mut_ptr());
    RLEExpand(&mut *rle.as_mut_ptr().offset(4), sm.as_mut_ptr(), 4096);
    numobj = 0;
    o[0].x = 13;
    o[0].y = 13;
    o[0].stage = 0;
    o[0].delay = 0;
    o[0].dir = east as libc::c_int as word;
    o[0].oldx = 0;
    o[0].oldy = 0;
    o[0].oldtile = -(1) as sword;
    yy = 0;
    while yy < 64 {
        xx = 0;
        while xx < 64 {
            btile = sm[(yy * 64 + xx) as usize] as byte;
            if (btile as libc::c_int) < 230 {
                background[(yy + 11) as usize][(xx + 11) as usize] = btile as libc::c_int;
            } else {
                background[(yy + 11) as usize][(xx + 11) as usize] = 128;
                if tokens[(btile as libc::c_int - 230) as usize] as libc::c_uint
                    == player as libc::c_int as libc::c_uint
                {
                    o[0].x = (xx + 11) as byte;
                    o[0].y = (yy + 11) as byte;
                } else {
                    numobj += 1;
                    o[numobj as usize].active = false as boolean;
                    o[numobj as usize].class =
                        tokens[(btile as libc::c_int - 230) as usize] as word;
                    o[numobj as usize].x = (xx + 11) as byte;
                    o[numobj as usize].y = (yy + 11) as byte;
                    o[numobj as usize].stage = 0;
                    o[numobj as usize].delay = 0;
                    o[numobj as usize].dir = (rndt() / 64) as dirtype as word;
                    o[numobj as usize].hp =
                        objdef[o[numobj as usize].class as usize].hitpoints as sbyte;
                    o[numobj as usize].oldx = o[numobj as usize].x;
                    o[numobj as usize].oldy = o[numobj as usize].y;
                    o[numobj as usize].oldtile = -(1) as sword;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    originx = o[0].x as libc::c_int - 11;
    originy = o[0].y as libc::c_int - 11;
    shotpower = 0;
    y = 11 - 1;
    while y < 65 + 11 {
        x = 11 - 1;
        while x < 64 + 11 {
            view[y as usize][x as usize] = background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    sx = 33;
    sy = 1;
    printint(level as libc::c_int);
    print(b" \0" as *const u8 as *const libc::c_char);
    restore();
    i = 0;
    while i < 6 {
        saveitems[i as usize] = items[i as usize];
        i += 1;
    }
    savescore = score;
    saveo[0] = o[0];
}
#[no_mangle]
pub unsafe extern "C" fn drawside() {
    let mut i: libc::c_int = 0;
    sx = 0;
    while sx < 40 {
        drawchar(sx, 24, 0);
        sx += 1;
    }
    sy = 0;
    while sy < 24 {
        drawchar(39, sy, 0);
        sy += 1;
    }
    drawwindow(24, 0, 38, 23);
    print(
        b"  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n\0" as *const u8 as *const libc::c_char,
    );
    print(b" shot power\n\n\n    body\n\n\n\0" as *const u8 as *const libc::c_char);
    printhighscore();
    printbody();
    printshotpower();
    printscore();
    sx = 33;
    sy = 1;
    printint(level as libc::c_int);
    drawpic(25 * 8, 17 * 8, 13);
    i = 1;
    while i <= items[1] as libc::c_int && i < 11 {
        drawchar(26 + i, 7, 31);
        i += 1;
    }
    i = 1;
    while i <= items[2] as libc::c_int && i < 11 {
        drawchar(26 + i, 8, 29);
        i += 1;
    }
    i = 1;
    while i <= items[3] as libc::c_int && i < 11 {
        drawchar(26 + i, 9, 30);
        i += 1;
    }
    i = 1;
    while i <= items[5] as libc::c_int && i < 11 {
        drawchar(26 + i, 10, 30);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn playsetup() {
    let mut i: libc::c_int = 0;
    shotpower = 0;
    bar(0, 0, 23, 23, 0);
    if level as libc::c_int == 0 {
        i = 1;
        while i < 6 {
            items[i as usize] = 0;
            i += 1;
        }
        score = 0;
        level = 1;
        o[0].active = true as boolean;
        o[0].class = player as libc::c_int as word;
        o[0].hp = 13;
        o[0].dir = west as libc::c_int as word;
        o[0].stage = 0;
        o[0].delay = 0;
        drawside();
        givenuke();
        givenuke();
        givebolt();
        givebolt();
        givebolt();
        givepotion();
        givepotion();
        givepotion();
    } else {
        drawside();
    };
}
#[no_mangle]
pub unsafe extern "C" fn repaintscreen() {
    match gamestate as libc::c_uint {
        1 => {
            drawpic(0, 0, 14);
        }
        0 => {
            restore();
            drawside();
            printscore();
            sx = 33;
            sy = 1;
            printint(level as libc::c_int);
        }
        2 => {
            restore();
            drawside();
            printscore();
            sx = 33;
            sy = 1;
            printint(level as libc::c_int);
            indemo = demoplay;
        }
        _ => {
            sy = 10;
            sx = sy;
            print(b"Bad gamestate!\0" as *const u8 as *const libc::c_char);
            clearkeys();
            get();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dofkeys() {
    let mut handle: libc::c_int = 0;
    let mut key: libc::c_int = bioskey(1);
    if key == SDL_SCANCODE_ESCAPE as libc::c_int {
        key = SDL_SCANCODE_F10 as libc::c_int;
    }
    if key < SDL_SCANCODE_F1 as libc::c_int || key > SDL_SCANCODE_F10 as libc::c_int {
        return;
    }
    let mut current_block_72: u64;
    match key {
        58 => {
            clearkeys();
            help();
        }
        59 => {
            clearkeys();
            controlpanel();
        }
        60 => {
            clearkeys();
            expwin(18, 1);
            print(b"RESET GAME (Y/N)?\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            if ch as libc::c_int == 'Y' as i32 {
                resetgame = true as boolean;
            }
        }
        61 => {
            clearkeys();
            expwin(22, 4);
            if indemo as libc::c_uint != notdemo as libc::c_int as libc::c_uint {
                print(b"Can't save game here!\0" as *const u8 as *const libc::c_char);
                get();
            } else {
                print(b"Save as game #(1-9):\0" as *const u8 as *const libc::c_char);
                ch = toupper(get()) as libc::c_char;
                drawchar(sx, sy, ch as libc::c_int);
                if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
                    strcpy(
                        str.as_mut_ptr(),
                        b"GAME0.CA2\0" as *const u8 as *const libc::c_char,
                    );
                    str[4] = ch;
                    if _Verify(str.as_mut_ptr()) != 0 {
                        print(
                            b"\nGame exists,\noverwrite (Y/N)?\0" as *const u8
                                as *const libc::c_char,
                        );
                        ch = get() as libc::c_char;
                        if ch as libc::c_int != 'Y' as i32 && ch as libc::c_int != 'y' as i32 {
                            current_block_72 = 919954187481050311;
                        } else {
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
                            sy -= 1;
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
                            sx = leftedge;
                            sy -= 1;
                            current_block_72 = 1836292691772056875;
                        }
                    } else {
                        current_block_72 = 1836292691772056875;
                    }
                    match current_block_72 {
                        919954187481050311 => {}
                        _ => {
                            handle = open(
                                str.as_mut_ptr(),
                                0o1 as libc::c_int
                                    | 0
                                    | 0o100 as libc::c_int
                                    | 0o1000 as libc::c_int,
                                0o400 as libc::c_int | 0o200 as libc::c_int,
                            );
                            if handle == -(1) {
                                return;
                            }
                            write(
                                handle,
                                &mut saveitems as *mut [sword; 6] as *const libc::c_void,
                                ::std::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut savescore as *mut sdword as *const libc::c_void,
                                ::std::mem::size_of::<sdword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut level as *mut sword as *const libc::c_void,
                                ::std::mem::size_of::<sword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut *saveo.as_mut_ptr().offset(0) as *mut activeobj
                                    as *const libc::c_void,
                                ::std::mem::size_of::<activeobj>() as libc::c_ulong,
                            );
                            close(handle);
                            print(b"\nGame saved.  Hit F5\n\0" as *const u8 as *const libc::c_char);
                            print(b"when you wish to\n\0" as *const u8 as *const libc::c_char);
                            print(b"restart the game.\0" as *const u8 as *const libc::c_char);
                            get();
                        }
                    }
                }
            }
        }
        62 => {
            clearkeys();
            expwin(22, 4);
            print(b"Load game #(1-9):\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            drawchar(sx, sy, ch as libc::c_int);
            if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
                strcpy(
                    str.as_mut_ptr(),
                    b"GAME0.CA2\0" as *const u8 as *const libc::c_char,
                );
                str[4] = ch;
                // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
                // project.
                handle = open(
                    str.as_mut_ptr(),
                    O_RDONLY | O_BINARY,
                    0o200 as libc::c_int | 0o400 as libc::c_int,
                );
                if handle == -(1) {
                    print(b"\nGame not found.\0" as *const u8 as *const libc::c_char);
                    get();
                } else {
                    read(
                        handle,
                        &mut items as *mut [sword; 6] as *mut libc::c_void,
                        ::std::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut score as *mut sdword as *mut libc::c_void,
                        ::std::mem::size_of::<sdword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut level as *mut sword as *mut libc::c_void,
                        ::std::mem::size_of::<sword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut *o.as_mut_ptr().offset(0) as *mut activeobj as *mut libc::c_void,
                        ::std::mem::size_of::<activeobj>() as libc::c_ulong,
                    );
                    close(handle);
                    exitdemo = true;
                    if indemo as libc::c_uint != notdemo as libc::c_int as libc::c_uint {
                        playdone = true as boolean;
                    }
                    drawside();
                    leveldone = true as boolean;
                }
            }
        }
        66 => {
            clearkeys();
            expwin(7, 1);
            print(b"PAUSED\0" as *const u8 as *const libc::c_char);
            get();
        }
        67 => {
            clearkeys();
            expwin(12, 1);
            print(b"QUIT (Y/N)?\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            if ch as libc::c_int == 'Y' as i32 {
                _quit(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            }
        }
        _ => return,
    }
    clearold();
    clearkeys();
    repaintscreen();
}
#[no_mangle]
pub unsafe extern "C" fn dotitlepage() {
    let mut i: libc::c_int = 0;
    drawpic(0, 0, 14);
    UpdateScreen();
    gamestate = intitle;
    i = 0;
    while i < 300 {
        WaitVBL();
        indemo = notdemo;
        ctrl = ControlPlayer(1);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
        {
            level = 0;
            exitdemo = true;
            break;
        } else {
            indemo = demoplay;
            if bioskey(1) != 0 {
                dofkeys();
                UpdateScreen();
            }
            if exitdemo {
                break;
            }
            i += 1;
        }
    }
    gamestate = ingame;
}
#[no_mangle]
pub unsafe extern "C" fn doendpage() {
    WaitEndSound();
    drawpic(0, 0, 15);
    PlaySound(3);
    WaitEndSound();
    PlaySound(3);
    WaitEndSound();
    PlaySound(3);
    WaitEndSound();
    PlaySound(3);
    WaitEndSound();
    drawwindow(0, 0, 17, 9);
    print(b"Congratulation! \n\0" as *const u8 as *const libc::c_char);
    print(b"One as skilled  \n\0" as *const u8 as *const libc::c_char);
    print(b"as yourself     \n\0" as *const u8 as *const libc::c_char);
    print(b"deserves the    \n\0" as *const u8 as *const libc::c_char);
    print(b"10,000,000 gold \n\0" as *const u8 as *const libc::c_char);
    print(b"you pulled out  \n\0" as *const u8 as *const libc::c_char);
    print(b"of the palace! \0" as *const u8 as *const libc::c_char);
    clearkeys();
    get();
    drawwindow(0, 0, 17, 9);
    print(b"Let us know what\n\0" as *const u8 as *const libc::c_char);
    print(b"you enjoyed     \n\0" as *const u8 as *const libc::c_char);
    print(b"about this game,\n\0" as *const u8 as *const libc::c_char);
    print(b"so we can give  \n\0" as *const u8 as *const libc::c_char);
    print(b"you more of it. \n\0" as *const u8 as *const libc::c_char);
    print(b"Thank you for   \n\0" as *const u8 as *const libc::c_char);
    print(b"playing!\0" as *const u8 as *const libc::c_char);
    get();
}
#[no_mangle]
pub unsafe extern "C" fn dodemo() {
    let mut i: libc::c_int = 0;
    while !exitdemo {
        dotitlepage();
        if exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1) + 1;
        LoadDemo(i);
        level = 0;
        playsetup();
        playloop();
        if exitdemo {
            break;
        }
        level = 0;
        gamestate = inscores;
        indemo = demoplay;
        _showhighscores();
        UpdateScreen();
        i = 0;
        while i < 500 {
            WaitVBL();
            indemo = notdemo;
            ctrl = ControlPlayer(1);
            if ctrl.button1 as libc::c_int != 0
                || ctrl.button2 as libc::c_int != 0
                || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
            {
                exitdemo = true;
                break;
            } else {
                if bioskey(1) != 0 {
                    dofkeys();
                }
                if exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gameover() {
    let mut i: libc::c_int = 0;
    expwin(11, 4);
    print(b"\n GAME OVER\n     \0" as *const u8 as *const libc::c_char);
    UpdateScreen();
    WaitEndSound();
    i = 0;
    while i < 120 {
        WaitVBL();
        i += 1;
    }
    gamestate = inscores;
    _checkhighscore();
    level = 0;
    i = 0;
    while i < 500 {
        WaitVBL();
        ctrl = ControlPlayer(1);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
        {
            break;
        }
        if bioskey(1) != 0 {
            dofkeys();
        }
        if exitdemo as libc::c_int != 0
            || indemo as libc::c_uint == demoplay as libc::c_int as libc::c_uint
        {
            break;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn US_CheckParm(
    mut parm: *mut libc::c_char,
    mut strings: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: libc::c_char = 0;
    let mut cs: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    while *(*__ctype_b_loc()).offset(*parm as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        parm = parm.offset(1);
    }
    i = 0;
    while !(*strings).is_null() && **strings as libc::c_int != 0 {
        let fresh1 = strings;
        strings = strings.offset(1);
        s = *fresh1;
        p = parm;
        cp = 0;
        cs = cp;
        while cs as libc::c_int == cp as libc::c_int {
            let fresh2 = s;
            s = s.offset(1);
            cs = *fresh2;
            if cs == 0 {
                return i;
            }
            let fresh3 = p;
            p = p.offset(1);
            cp = *fresh3;
            if *(*__ctype_b_loc()).offset(cs as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cs = tolower(cs as libc::c_int) as libc::c_char;
            }
            if *(*__ctype_b_loc()).offset(cp as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cp = tolower(cp as libc::c_int) as libc::c_char;
            }
        }
        i += 1;
    }
    return -(1);
}

/***************************************************************************/
/***************************************************************************/

/*=========================*/
/*			   */
/* m a i n   p r o g r a m */
/*			   */
/*=========================*/

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();

    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }

    unsafe {
        if args.len() > 1 && strcasecmp(args[1], b"/VER\0" as *const u8 as *const libc::c_char) == 0
        {
            print!(
                "\
                    CatacombSDL\n\
                    Version 1.03\n
\
                    Usage: catacomb [windowed <width> <height>] [screen <num>]\n
\
                    Ported by Braden \"Blzut3\" Obrzut and Rene \"Havoc\" Nicolaus\n\
                    Includes PC Speaker emulator by K1n9_Duk3\n\
                    Based on The Catacomb source code:\n\
                    Copyright 1990-1993 Softdisk Publishing\n\
                    Copyright 1993-2014 Flat Rock Software\n\
                "
            );
            exit(0);
        }

        initobjects();

        priority.fill(99);

        priority[blankfloor] = 0;
        for i in objdef[teleporter as usize].firstchar..=objdef[teleporter as usize].firstchar + 20
        {
            priority[i as usize] = 0;
        }
        for clvar in dead2..=dead5 {
            for i in objdef[clvar as usize].firstchar
                ..=(objdef[clvar as usize].firstchar
                    + objdef[clvar as usize].size as u16 * objdef[clvar as usize].size as u16)
            {
                priority[i as usize] = 0; /*deadthing*/
            }
        }
        for i in 152..=161 {
            priority[i] = 2; /*shots*/
        }
        for i in objdef[bigshot as usize].firstchar..=(objdef[bigshot as usize].firstchar + 31) {
            priority[i as usize] = 2; /*bigshot*/
        }
        for i in 0..=(tile2s - 1) {
            if priority[i] == 99 {
                priority[i] = 3; /*most 1*1 tiles are walls, etc*/
            }
        }
        priority[167] = 1; // chest
        for i in tile2s..=maxpics {
            if priority[i] as libc::c_int == 99 {
                priority[i] = 4; /*most bigger tiles are monsters*/
            }
        }
        for i in objdef[player as usize].firstchar..=(objdef[player as usize].firstchar + 63) {
            priority[i as usize] = 5; /*player*/
        }

        side = 0;

        for x in 0..=85 {
            for y in 0..=(topoff - 1) {
                view[x][y] = solidwall;
                view[x][(85 - y)] = solidwall;
                background[x][y] = solidwall;
                background[x][(85 - y)] = solidwall;
            }
            view[86][x] = solidwall;
        }
        for y in 11..=74 {
            for x in 0..=(leftoff - 1) {
                view[x][y] = solidwall;
                view[(85 - x)][y] = solidwall;
                background[x][y] = solidwall;
                background[(85 - x)][y] = solidwall;
            }
        }

        //   puts ("CATACOMB II is executing");

        //  _dontplay = 1;	// no sounds for debugging and profiling

        _numlevels = 30;
        _maxplayers = 1;

        _extension = b"CA2\0" as *const u8 as *const libc::c_char;

        _setupgame(args);

        expwin(33, 13);
        print(b"  Softdisk Publishing presents\n\n\0" as *const u8 as *const libc::c_char);
        print(b"          The Catacomb\n\n\0" as *const u8 as *const libc::c_char);
        print(b"        By John Carmack\n\n\0" as *const u8 as *const libc::c_char);
        print(b"       Copyright 1990-93\n\0" as *const u8 as *const libc::c_char);
        print(b"      Softdisk Publishing\0" as *const u8 as *const libc::c_char);
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        print(b"         Press a key:\0" as *const u8 as *const libc::c_char);
        get();

        clearkeys();

        screencentery = 11;
        screencenterx = 11;

        exitdemo = false;
        level = 0;

        // go until quit () is called
        loop {
            dodemo();
            playsetup();
            indemo = notdemo;
            gamestate = ingame;
            playloop();
            if indemo == 0 {
                exitdemo = false;
                if level > 30 {
                    doendpage(); // finished all levels
                }
                gameover();
            }
        }
    }
}
