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
use ::c2rust_out::*;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/unistd.h:26"]
pub mod unistd_h {
    #[c2rust::src_loc = "220:1"]
    pub type ssize_t = __ssize_t;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.6/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/ctype.h:30"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "125:12"]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint16_t, __uint8_t};
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:33"]
pub mod catdefs_h {
    #[c2rust::src_loc = "34:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "34:13"]
    pub const true_0: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "34:7"]
    pub const false_0: C2RustUnnamed_0 = 0;
    #[c2rust::src_loc = "35:1"]
    pub type boolean = uint16_t;
    #[c2rust::src_loc = "37:1"]
    pub type byte = uint8_t;
    #[c2rust::src_loc = "38:1"]
    pub type sbyte = int8_t;
    #[c2rust::src_loc = "39:1"]
    pub type word = uint16_t;
    #[c2rust::src_loc = "40:1"]
    pub type sword = int16_t;
    #[c2rust::src_loc = "42:1"]
    pub type sdword = int32_t;
    #[c2rust::src_loc = "80:9"]
    pub type dirtype = libc::c_uint;
    #[c2rust::src_loc = "81:18"]
    pub const nodir: dirtype = 8;
    #[c2rust::src_loc = "81:8"]
    pub const northwest: dirtype = 7;
    #[c2rust::src_loc = "80:57"]
    pub const southwest: dirtype = 6;
    #[c2rust::src_loc = "80:47"]
    pub const southeast: dirtype = 5;
    #[c2rust::src_loc = "80:37"]
    pub const northeast: dirtype = 4;
    #[c2rust::src_loc = "80:32"]
    pub const west: dirtype = 3;
    #[c2rust::src_loc = "80:26"]
    pub const south: dirtype = 2;
    #[c2rust::src_loc = "80:21"]
    pub const east: dirtype = 1;
    #[c2rust::src_loc = "80:15"]
    pub const north: dirtype = 0;
    #[c2rust::src_loc = "88:9"]
    pub type classtype = libc::c_uint;
    #[c2rust::src_loc = "90:32"]
    pub const lastclass: classtype = 23;
    #[c2rust::src_loc = "90:27"]
    pub const guns: classtype = 22;
    #[c2rust::src_loc = "90:22"]
    pub const gune: classtype = 21;
    #[c2rust::src_loc = "90:11"]
    pub const secretgate: classtype = 20;
    #[c2rust::src_loc = "90:5"]
    pub const torch: classtype = 19;
    #[c2rust::src_loc = "89:67"]
    pub const teleporter: classtype = 18;
    #[c2rust::src_loc = "89:61"]
    pub const dead6: classtype = 17;
    #[c2rust::src_loc = "89:55"]
    pub const dead5: classtype = 16;
    #[c2rust::src_loc = "89:49"]
    pub const dead4: classtype = 15;
    #[c2rust::src_loc = "89:43"]
    pub const dead3: classtype = 14;
    #[c2rust::src_loc = "89:37"]
    pub const dead2: classtype = 13;
    #[c2rust::src_loc = "89:31"]
    pub const dead1: classtype = 12;
    #[c2rust::src_loc = "89:26"]
    pub const rock: classtype = 11;
    #[c2rust::src_loc = "89:18"]
    pub const bigshot: classtype = 10;
    #[c2rust::src_loc = "89:13"]
    pub const shot: classtype = 9;
    #[c2rust::src_loc = "89:5"]
    pub const wallhit: classtype = 8;
    #[c2rust::src_loc = "88:67"]
    pub const turbogre: classtype = 7;
    #[c2rust::src_loc = "88:60"]
    pub const dragon: classtype = 6;
    #[c2rust::src_loc = "88:51"]
    pub const gargoyle: classtype = 5;
    #[c2rust::src_loc = "88:46"]
    pub const ogre: classtype = 4;
    #[c2rust::src_loc = "88:37"]
    pub const skeleton: classtype = 3;
    #[c2rust::src_loc = "88:30"]
    pub const goblin: classtype = 2;
    #[c2rust::src_loc = "88:23"]
    pub const player: classtype = 1;
    #[c2rust::src_loc = "88:15"]
    pub const nothing: classtype = 0;
    #[c2rust::src_loc = "92:9"]
    pub type statetype = libc::c_uint;
    #[c2rust::src_loc = "92:30"]
    pub const inscores: statetype = 2;
    #[c2rust::src_loc = "92:22"]
    pub const intitle: statetype = 1;
    #[c2rust::src_loc = "92:15"]
    pub const ingame: statetype = 0;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "96:9"]
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
    #[c2rust::src_loc = "109:9"]
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
    #[c2rust::src_loc = "124:9"]
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
    #[inline]
    #[c2rust::src_loc = "53:1"]
    pub unsafe extern "C" fn itoa(
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
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    use super::stdio_h::sprintf;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_scancode.h:34"]
pub mod SDL_scancode_h {
    #[c2rust::src_loc = "43:9"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "407:5"]
    pub const SDL_NUM_SCANCODES: C2RustUnnamed_1 = 512;
    #[c2rust::src_loc = "401:5"]
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: C2RustUnnamed_1 = 286;
    #[c2rust::src_loc = "400:5"]
    pub const SDL_SCANCODE_AUDIOREWIND: C2RustUnnamed_1 = 285;
    #[c2rust::src_loc = "389:5"]
    pub const SDL_SCANCODE_APP2: C2RustUnnamed_1 = 284;
    #[c2rust::src_loc = "388:5"]
    pub const SDL_SCANCODE_APP1: C2RustUnnamed_1 = 283;
    #[c2rust::src_loc = "386:5"]
    pub const SDL_SCANCODE_SLEEP: C2RustUnnamed_1 = 282;
    #[c2rust::src_loc = "385:5"]
    pub const SDL_SCANCODE_EJECT: C2RustUnnamed_1 = 281;
    #[c2rust::src_loc = "384:5"]
    pub const SDL_SCANCODE_KBDILLUMUP: C2RustUnnamed_1 = 280;
    #[c2rust::src_loc = "383:5"]
    pub const SDL_SCANCODE_KBDILLUMDOWN: C2RustUnnamed_1 = 279;
    #[c2rust::src_loc = "382:5"]
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: C2RustUnnamed_1 = 278;
    #[c2rust::src_loc = "380:5"]
    pub const SDL_SCANCODE_DISPLAYSWITCH: C2RustUnnamed_1 = 277;
    #[c2rust::src_loc = "379:5"]
    pub const SDL_SCANCODE_BRIGHTNESSUP: C2RustUnnamed_1 = 276;
    #[c2rust::src_loc = "378:5"]
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: C2RustUnnamed_1 = 275;
    #[c2rust::src_loc = "367:5"]
    pub const SDL_SCANCODE_AC_BOOKMARKS: C2RustUnnamed_1 = 274;
    #[c2rust::src_loc = "366:5"]
    pub const SDL_SCANCODE_AC_REFRESH: C2RustUnnamed_1 = 273;
    #[c2rust::src_loc = "365:5"]
    pub const SDL_SCANCODE_AC_STOP: C2RustUnnamed_1 = 272;
    #[c2rust::src_loc = "364:5"]
    pub const SDL_SCANCODE_AC_FORWARD: C2RustUnnamed_1 = 271;
    #[c2rust::src_loc = "363:5"]
    pub const SDL_SCANCODE_AC_BACK: C2RustUnnamed_1 = 270;
    #[c2rust::src_loc = "362:5"]
    pub const SDL_SCANCODE_AC_HOME: C2RustUnnamed_1 = 269;
    #[c2rust::src_loc = "361:5"]
    pub const SDL_SCANCODE_AC_SEARCH: C2RustUnnamed_1 = 268;
    #[c2rust::src_loc = "360:5"]
    pub const SDL_SCANCODE_COMPUTER: C2RustUnnamed_1 = 267;
    #[c2rust::src_loc = "359:5"]
    pub const SDL_SCANCODE_CALCULATOR: C2RustUnnamed_1 = 266;
    #[c2rust::src_loc = "358:5"]
    pub const SDL_SCANCODE_MAIL: C2RustUnnamed_1 = 265;
    #[c2rust::src_loc = "357:5"]
    pub const SDL_SCANCODE_WWW: C2RustUnnamed_1 = 264;
    #[c2rust::src_loc = "356:5"]
    pub const SDL_SCANCODE_MEDIASELECT: C2RustUnnamed_1 = 263;
    #[c2rust::src_loc = "355:5"]
    pub const SDL_SCANCODE_AUDIOMUTE: C2RustUnnamed_1 = 262;
    #[c2rust::src_loc = "354:5"]
    pub const SDL_SCANCODE_AUDIOPLAY: C2RustUnnamed_1 = 261;
    #[c2rust::src_loc = "353:5"]
    pub const SDL_SCANCODE_AUDIOSTOP: C2RustUnnamed_1 = 260;
    #[c2rust::src_loc = "352:5"]
    pub const SDL_SCANCODE_AUDIOPREV: C2RustUnnamed_1 = 259;
    #[c2rust::src_loc = "351:5"]
    pub const SDL_SCANCODE_AUDIONEXT: C2RustUnnamed_1 = 258;
    #[c2rust::src_loc = "337:5"]
    pub const SDL_SCANCODE_MODE: C2RustUnnamed_1 = 257;
    #[c2rust::src_loc = "335:5"]
    pub const SDL_SCANCODE_RGUI: C2RustUnnamed_1 = 231;
    #[c2rust::src_loc = "334:5"]
    pub const SDL_SCANCODE_RALT: C2RustUnnamed_1 = 230;
    #[c2rust::src_loc = "333:5"]
    pub const SDL_SCANCODE_RSHIFT: C2RustUnnamed_1 = 229;
    #[c2rust::src_loc = "332:5"]
    pub const SDL_SCANCODE_RCTRL: C2RustUnnamed_1 = 228;
    #[c2rust::src_loc = "331:5"]
    pub const SDL_SCANCODE_LGUI: C2RustUnnamed_1 = 227;
    #[c2rust::src_loc = "330:5"]
    pub const SDL_SCANCODE_LALT: C2RustUnnamed_1 = 226;
    #[c2rust::src_loc = "329:5"]
    pub const SDL_SCANCODE_LSHIFT: C2RustUnnamed_1 = 225;
    #[c2rust::src_loc = "328:5"]
    pub const SDL_SCANCODE_LCTRL: C2RustUnnamed_1 = 224;
    #[c2rust::src_loc = "326:5"]
    pub const SDL_SCANCODE_KP_HEXADECIMAL: C2RustUnnamed_1 = 221;
    #[c2rust::src_loc = "325:5"]
    pub const SDL_SCANCODE_KP_DECIMAL: C2RustUnnamed_1 = 220;
    #[c2rust::src_loc = "324:5"]
    pub const SDL_SCANCODE_KP_OCTAL: C2RustUnnamed_1 = 219;
    #[c2rust::src_loc = "323:5"]
    pub const SDL_SCANCODE_KP_BINARY: C2RustUnnamed_1 = 218;
    #[c2rust::src_loc = "322:5"]
    pub const SDL_SCANCODE_KP_CLEARENTRY: C2RustUnnamed_1 = 217;
    #[c2rust::src_loc = "321:5"]
    pub const SDL_SCANCODE_KP_CLEAR: C2RustUnnamed_1 = 216;
    #[c2rust::src_loc = "320:5"]
    pub const SDL_SCANCODE_KP_PLUSMINUS: C2RustUnnamed_1 = 215;
    #[c2rust::src_loc = "319:5"]
    pub const SDL_SCANCODE_KP_MEMDIVIDE: C2RustUnnamed_1 = 214;
    #[c2rust::src_loc = "318:5"]
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: C2RustUnnamed_1 = 213;
    #[c2rust::src_loc = "317:5"]
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: C2RustUnnamed_1 = 212;
    #[c2rust::src_loc = "316:5"]
    pub const SDL_SCANCODE_KP_MEMADD: C2RustUnnamed_1 = 211;
    #[c2rust::src_loc = "315:5"]
    pub const SDL_SCANCODE_KP_MEMCLEAR: C2RustUnnamed_1 = 210;
    #[c2rust::src_loc = "314:5"]
    pub const SDL_SCANCODE_KP_MEMRECALL: C2RustUnnamed_1 = 209;
    #[c2rust::src_loc = "313:5"]
    pub const SDL_SCANCODE_KP_MEMSTORE: C2RustUnnamed_1 = 208;
    #[c2rust::src_loc = "312:5"]
    pub const SDL_SCANCODE_KP_EXCLAM: C2RustUnnamed_1 = 207;
    #[c2rust::src_loc = "311:5"]
    pub const SDL_SCANCODE_KP_AT: C2RustUnnamed_1 = 206;
    #[c2rust::src_loc = "310:5"]
    pub const SDL_SCANCODE_KP_SPACE: C2RustUnnamed_1 = 205;
    #[c2rust::src_loc = "309:5"]
    pub const SDL_SCANCODE_KP_HASH: C2RustUnnamed_1 = 204;
    #[c2rust::src_loc = "308:5"]
    pub const SDL_SCANCODE_KP_COLON: C2RustUnnamed_1 = 203;
    #[c2rust::src_loc = "307:5"]
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: C2RustUnnamed_1 = 202;
    #[c2rust::src_loc = "306:5"]
    pub const SDL_SCANCODE_KP_VERTICALBAR: C2RustUnnamed_1 = 201;
    #[c2rust::src_loc = "305:5"]
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: C2RustUnnamed_1 = 200;
    #[c2rust::src_loc = "304:5"]
    pub const SDL_SCANCODE_KP_AMPERSAND: C2RustUnnamed_1 = 199;
    #[c2rust::src_loc = "303:5"]
    pub const SDL_SCANCODE_KP_GREATER: C2RustUnnamed_1 = 198;
    #[c2rust::src_loc = "302:5"]
    pub const SDL_SCANCODE_KP_LESS: C2RustUnnamed_1 = 197;
    #[c2rust::src_loc = "301:5"]
    pub const SDL_SCANCODE_KP_PERCENT: C2RustUnnamed_1 = 196;
    #[c2rust::src_loc = "300:5"]
    pub const SDL_SCANCODE_KP_POWER: C2RustUnnamed_1 = 195;
    #[c2rust::src_loc = "299:5"]
    pub const SDL_SCANCODE_KP_XOR: C2RustUnnamed_1 = 194;
    #[c2rust::src_loc = "298:5"]
    pub const SDL_SCANCODE_KP_F: C2RustUnnamed_1 = 193;
    #[c2rust::src_loc = "297:5"]
    pub const SDL_SCANCODE_KP_E: C2RustUnnamed_1 = 192;
    #[c2rust::src_loc = "296:5"]
    pub const SDL_SCANCODE_KP_D: C2RustUnnamed_1 = 191;
    #[c2rust::src_loc = "295:5"]
    pub const SDL_SCANCODE_KP_C: C2RustUnnamed_1 = 190;
    #[c2rust::src_loc = "294:5"]
    pub const SDL_SCANCODE_KP_B: C2RustUnnamed_1 = 189;
    #[c2rust::src_loc = "293:5"]
    pub const SDL_SCANCODE_KP_A: C2RustUnnamed_1 = 188;
    #[c2rust::src_loc = "292:5"]
    pub const SDL_SCANCODE_KP_BACKSPACE: C2RustUnnamed_1 = 187;
    #[c2rust::src_loc = "291:5"]
    pub const SDL_SCANCODE_KP_TAB: C2RustUnnamed_1 = 186;
    #[c2rust::src_loc = "290:5"]
    pub const SDL_SCANCODE_KP_RIGHTBRACE: C2RustUnnamed_1 = 185;
    #[c2rust::src_loc = "289:5"]
    pub const SDL_SCANCODE_KP_LEFTBRACE: C2RustUnnamed_1 = 184;
    #[c2rust::src_loc = "288:5"]
    pub const SDL_SCANCODE_KP_RIGHTPAREN: C2RustUnnamed_1 = 183;
    #[c2rust::src_loc = "287:5"]
    pub const SDL_SCANCODE_KP_LEFTPAREN: C2RustUnnamed_1 = 182;
    #[c2rust::src_loc = "286:5"]
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: C2RustUnnamed_1 = 181;
    #[c2rust::src_loc = "285:5"]
    pub const SDL_SCANCODE_CURRENCYUNIT: C2RustUnnamed_1 = 180;
    #[c2rust::src_loc = "284:5"]
    pub const SDL_SCANCODE_DECIMALSEPARATOR: C2RustUnnamed_1 = 179;
    #[c2rust::src_loc = "283:5"]
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: C2RustUnnamed_1 = 178;
    #[c2rust::src_loc = "282:5"]
    pub const SDL_SCANCODE_KP_000: C2RustUnnamed_1 = 177;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_SCANCODE_KP_00: C2RustUnnamed_1 = 176;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_SCANCODE_EXSEL: C2RustUnnamed_1 = 164;
    #[c2rust::src_loc = "278:5"]
    pub const SDL_SCANCODE_CRSEL: C2RustUnnamed_1 = 163;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_SCANCODE_CLEARAGAIN: C2RustUnnamed_1 = 162;
    #[c2rust::src_loc = "276:5"]
    pub const SDL_SCANCODE_OPER: C2RustUnnamed_1 = 161;
    #[c2rust::src_loc = "275:5"]
    pub const SDL_SCANCODE_OUT: C2RustUnnamed_1 = 160;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_SCANCODE_SEPARATOR: C2RustUnnamed_1 = 159;
    #[c2rust::src_loc = "273:5"]
    pub const SDL_SCANCODE_RETURN2: C2RustUnnamed_1 = 158;
    #[c2rust::src_loc = "272:5"]
    pub const SDL_SCANCODE_PRIOR: C2RustUnnamed_1 = 157;
    #[c2rust::src_loc = "271:5"]
    pub const SDL_SCANCODE_CLEAR: C2RustUnnamed_1 = 156;
    #[c2rust::src_loc = "270:5"]
    pub const SDL_SCANCODE_CANCEL: C2RustUnnamed_1 = 155;
    #[c2rust::src_loc = "269:5"]
    pub const SDL_SCANCODE_SYSREQ: C2RustUnnamed_1 = 154;
    #[c2rust::src_loc = "268:5"]
    pub const SDL_SCANCODE_ALTERASE: C2RustUnnamed_1 = 153;
    #[c2rust::src_loc = "266:5"]
    pub const SDL_SCANCODE_LANG9: C2RustUnnamed_1 = 152;
    #[c2rust::src_loc = "265:5"]
    pub const SDL_SCANCODE_LANG8: C2RustUnnamed_1 = 151;
    #[c2rust::src_loc = "264:5"]
    pub const SDL_SCANCODE_LANG7: C2RustUnnamed_1 = 150;
    #[c2rust::src_loc = "263:5"]
    pub const SDL_SCANCODE_LANG6: C2RustUnnamed_1 = 149;
    #[c2rust::src_loc = "262:5"]
    pub const SDL_SCANCODE_LANG5: C2RustUnnamed_1 = 148;
    #[c2rust::src_loc = "261:5"]
    pub const SDL_SCANCODE_LANG4: C2RustUnnamed_1 = 147;
    #[c2rust::src_loc = "260:5"]
    pub const SDL_SCANCODE_LANG3: C2RustUnnamed_1 = 146;
    #[c2rust::src_loc = "259:5"]
    pub const SDL_SCANCODE_LANG2: C2RustUnnamed_1 = 145;
    #[c2rust::src_loc = "258:5"]
    pub const SDL_SCANCODE_LANG1: C2RustUnnamed_1 = 144;
    #[c2rust::src_loc = "257:5"]
    pub const SDL_SCANCODE_INTERNATIONAL9: C2RustUnnamed_1 = 143;
    #[c2rust::src_loc = "256:5"]
    pub const SDL_SCANCODE_INTERNATIONAL8: C2RustUnnamed_1 = 142;
    #[c2rust::src_loc = "255:5"]
    pub const SDL_SCANCODE_INTERNATIONAL7: C2RustUnnamed_1 = 141;
    #[c2rust::src_loc = "254:5"]
    pub const SDL_SCANCODE_INTERNATIONAL6: C2RustUnnamed_1 = 140;
    #[c2rust::src_loc = "253:5"]
    pub const SDL_SCANCODE_INTERNATIONAL5: C2RustUnnamed_1 = 139;
    #[c2rust::src_loc = "252:5"]
    pub const SDL_SCANCODE_INTERNATIONAL4: C2RustUnnamed_1 = 138;
    #[c2rust::src_loc = "251:5"]
    pub const SDL_SCANCODE_INTERNATIONAL3: C2RustUnnamed_1 = 137;
    #[c2rust::src_loc = "250:5"]
    pub const SDL_SCANCODE_INTERNATIONAL2: C2RustUnnamed_1 = 136;
    #[c2rust::src_loc = "248:5"]
    pub const SDL_SCANCODE_INTERNATIONAL1: C2RustUnnamed_1 = 135;
    #[c2rust::src_loc = "246:5"]
    pub const SDL_SCANCODE_KP_EQUALSAS400: C2RustUnnamed_1 = 134;
    #[c2rust::src_loc = "245:5"]
    pub const SDL_SCANCODE_KP_COMMA: C2RustUnnamed_1 = 133;
    #[c2rust::src_loc = "240:5"]
    pub const SDL_SCANCODE_VOLUMEDOWN: C2RustUnnamed_1 = 129;
    #[c2rust::src_loc = "239:5"]
    pub const SDL_SCANCODE_VOLUMEUP: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "238:5"]
    pub const SDL_SCANCODE_MUTE: C2RustUnnamed_1 = 127;
    #[c2rust::src_loc = "237:5"]
    pub const SDL_SCANCODE_FIND: C2RustUnnamed_1 = 126;
    #[c2rust::src_loc = "236:5"]
    pub const SDL_SCANCODE_PASTE: C2RustUnnamed_1 = 125;
    #[c2rust::src_loc = "235:5"]
    pub const SDL_SCANCODE_COPY: C2RustUnnamed_1 = 124;
    #[c2rust::src_loc = "234:5"]
    pub const SDL_SCANCODE_CUT: C2RustUnnamed_1 = 123;
    #[c2rust::src_loc = "233:5"]
    pub const SDL_SCANCODE_UNDO: C2RustUnnamed_1 = 122;
    #[c2rust::src_loc = "232:5"]
    pub const SDL_SCANCODE_AGAIN: C2RustUnnamed_1 = 121;
    #[c2rust::src_loc = "231:5"]
    pub const SDL_SCANCODE_STOP: C2RustUnnamed_1 = 120;
    #[c2rust::src_loc = "230:5"]
    pub const SDL_SCANCODE_SELECT: C2RustUnnamed_1 = 119;
    #[c2rust::src_loc = "229:5"]
    pub const SDL_SCANCODE_MENU: C2RustUnnamed_1 = 118;
    #[c2rust::src_loc = "228:5"]
    pub const SDL_SCANCODE_HELP: C2RustUnnamed_1 = 117;
    #[c2rust::src_loc = "227:5"]
    pub const SDL_SCANCODE_EXECUTE: C2RustUnnamed_1 = 116;
    #[c2rust::src_loc = "226:5"]
    pub const SDL_SCANCODE_F24: C2RustUnnamed_1 = 115;
    #[c2rust::src_loc = "225:5"]
    pub const SDL_SCANCODE_F23: C2RustUnnamed_1 = 114;
    #[c2rust::src_loc = "224:5"]
    pub const SDL_SCANCODE_F22: C2RustUnnamed_1 = 113;
    #[c2rust::src_loc = "223:5"]
    pub const SDL_SCANCODE_F21: C2RustUnnamed_1 = 112;
    #[c2rust::src_loc = "222:5"]
    pub const SDL_SCANCODE_F20: C2RustUnnamed_1 = 111;
    #[c2rust::src_loc = "221:5"]
    pub const SDL_SCANCODE_F19: C2RustUnnamed_1 = 110;
    #[c2rust::src_loc = "220:5"]
    pub const SDL_SCANCODE_F18: C2RustUnnamed_1 = 109;
    #[c2rust::src_loc = "219:5"]
    pub const SDL_SCANCODE_F17: C2RustUnnamed_1 = 108;
    #[c2rust::src_loc = "218:5"]
    pub const SDL_SCANCODE_F16: C2RustUnnamed_1 = 107;
    #[c2rust::src_loc = "217:5"]
    pub const SDL_SCANCODE_F15: C2RustUnnamed_1 = 106;
    #[c2rust::src_loc = "216:5"]
    pub const SDL_SCANCODE_F14: C2RustUnnamed_1 = 105;
    #[c2rust::src_loc = "215:5"]
    pub const SDL_SCANCODE_F13: C2RustUnnamed_1 = 104;
    #[c2rust::src_loc = "214:5"]
    pub const SDL_SCANCODE_KP_EQUALS: C2RustUnnamed_1 = 103;
    #[c2rust::src_loc = "211:5"]
    pub const SDL_SCANCODE_POWER: C2RustUnnamed_1 = 102;
    #[c2rust::src_loc = "210:5"]
    pub const SDL_SCANCODE_APPLICATION: C2RustUnnamed_1 = 101;
    #[c2rust::src_loc = "200:5"]
    pub const SDL_SCANCODE_NONUSBACKSLASH: C2RustUnnamed_1 = 100;
    #[c2rust::src_loc = "198:5"]
    pub const SDL_SCANCODE_KP_PERIOD: C2RustUnnamed_1 = 99;
    #[c2rust::src_loc = "197:5"]
    pub const SDL_SCANCODE_KP_0: C2RustUnnamed_1 = 98;
    #[c2rust::src_loc = "196:5"]
    pub const SDL_SCANCODE_KP_9: C2RustUnnamed_1 = 97;
    #[c2rust::src_loc = "195:5"]
    pub const SDL_SCANCODE_KP_8: C2RustUnnamed_1 = 96;
    #[c2rust::src_loc = "194:5"]
    pub const SDL_SCANCODE_KP_7: C2RustUnnamed_1 = 95;
    #[c2rust::src_loc = "193:5"]
    pub const SDL_SCANCODE_KP_6: C2RustUnnamed_1 = 94;
    #[c2rust::src_loc = "192:5"]
    pub const SDL_SCANCODE_KP_5: C2RustUnnamed_1 = 93;
    #[c2rust::src_loc = "191:5"]
    pub const SDL_SCANCODE_KP_4: C2RustUnnamed_1 = 92;
    #[c2rust::src_loc = "190:5"]
    pub const SDL_SCANCODE_KP_3: C2RustUnnamed_1 = 91;
    #[c2rust::src_loc = "189:5"]
    pub const SDL_SCANCODE_KP_2: C2RustUnnamed_1 = 90;
    #[c2rust::src_loc = "188:5"]
    pub const SDL_SCANCODE_KP_1: C2RustUnnamed_1 = 89;
    #[c2rust::src_loc = "187:5"]
    pub const SDL_SCANCODE_KP_ENTER: C2RustUnnamed_1 = 88;
    #[c2rust::src_loc = "186:5"]
    pub const SDL_SCANCODE_KP_PLUS: C2RustUnnamed_1 = 87;
    #[c2rust::src_loc = "185:5"]
    pub const SDL_SCANCODE_KP_MINUS: C2RustUnnamed_1 = 86;
    #[c2rust::src_loc = "184:5"]
    pub const SDL_SCANCODE_KP_MULTIPLY: C2RustUnnamed_1 = 85;
    #[c2rust::src_loc = "183:5"]
    pub const SDL_SCANCODE_KP_DIVIDE: C2RustUnnamed_1 = 84;
    #[c2rust::src_loc = "181:5"]
    pub const SDL_SCANCODE_NUMLOCKCLEAR: C2RustUnnamed_1 = 83;
    #[c2rust::src_loc = "179:5"]
    pub const SDL_SCANCODE_UP: C2RustUnnamed_1 = 82;
    #[c2rust::src_loc = "178:5"]
    pub const SDL_SCANCODE_DOWN: C2RustUnnamed_1 = 81;
    #[c2rust::src_loc = "177:5"]
    pub const SDL_SCANCODE_LEFT: C2RustUnnamed_1 = 80;
    #[c2rust::src_loc = "176:5"]
    pub const SDL_SCANCODE_RIGHT: C2RustUnnamed_1 = 79;
    #[c2rust::src_loc = "175:5"]
    pub const SDL_SCANCODE_PAGEDOWN: C2RustUnnamed_1 = 78;
    #[c2rust::src_loc = "174:5"]
    pub const SDL_SCANCODE_END: C2RustUnnamed_1 = 77;
    #[c2rust::src_loc = "173:5"]
    pub const SDL_SCANCODE_DELETE: C2RustUnnamed_1 = 76;
    #[c2rust::src_loc = "172:5"]
    pub const SDL_SCANCODE_PAGEUP: C2RustUnnamed_1 = 75;
    #[c2rust::src_loc = "171:5"]
    pub const SDL_SCANCODE_HOME: C2RustUnnamed_1 = 74;
    #[c2rust::src_loc = "169:5"]
    pub const SDL_SCANCODE_INSERT: C2RustUnnamed_1 = 73;
    #[c2rust::src_loc = "168:5"]
    pub const SDL_SCANCODE_PAUSE: C2RustUnnamed_1 = 72;
    #[c2rust::src_loc = "167:5"]
    pub const SDL_SCANCODE_SCROLLLOCK: C2RustUnnamed_1 = 71;
    #[c2rust::src_loc = "166:5"]
    pub const SDL_SCANCODE_PRINTSCREEN: C2RustUnnamed_1 = 70;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_SCANCODE_F12: C2RustUnnamed_1 = 69;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_SCANCODE_F11: C2RustUnnamed_1 = 68;
    #[c2rust::src_loc = "162:5"]
    pub const SDL_SCANCODE_F10: C2RustUnnamed_1 = 67;
    #[c2rust::src_loc = "161:5"]
    pub const SDL_SCANCODE_F9: C2RustUnnamed_1 = 66;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_SCANCODE_F8: C2RustUnnamed_1 = 65;
    #[c2rust::src_loc = "159:5"]
    pub const SDL_SCANCODE_F7: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "158:5"]
    pub const SDL_SCANCODE_F6: C2RustUnnamed_1 = 63;
    #[c2rust::src_loc = "157:5"]
    pub const SDL_SCANCODE_F5: C2RustUnnamed_1 = 62;
    #[c2rust::src_loc = "156:5"]
    pub const SDL_SCANCODE_F4: C2RustUnnamed_1 = 61;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_SCANCODE_F3: C2RustUnnamed_1 = 60;
    #[c2rust::src_loc = "154:5"]
    pub const SDL_SCANCODE_F2: C2RustUnnamed_1 = 59;
    #[c2rust::src_loc = "153:5"]
    pub const SDL_SCANCODE_F1: C2RustUnnamed_1 = 58;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_SCANCODE_CAPSLOCK: C2RustUnnamed_1 = 57;
    #[c2rust::src_loc = "149:5"]
    pub const SDL_SCANCODE_SLASH: C2RustUnnamed_1 = 56;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_SCANCODE_PERIOD: C2RustUnnamed_1 = 55;
    #[c2rust::src_loc = "147:5"]
    pub const SDL_SCANCODE_COMMA: C2RustUnnamed_1 = 54;
    #[c2rust::src_loc = "130:5"]
    pub const SDL_SCANCODE_GRAVE: C2RustUnnamed_1 = 53;
    #[c2rust::src_loc = "129:5"]
    pub const SDL_SCANCODE_APOSTROPHE: C2RustUnnamed_1 = 52;
    #[c2rust::src_loc = "128:5"]
    pub const SDL_SCANCODE_SEMICOLON: C2RustUnnamed_1 = 51;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_SCANCODE_NONUSHASH: C2RustUnnamed_1 = 50;
    #[c2rust::src_loc = "102:5"]
    pub const SDL_SCANCODE_BACKSLASH: C2RustUnnamed_1 = 49;
    #[c2rust::src_loc = "101:5"]
    pub const SDL_SCANCODE_RIGHTBRACKET: C2RustUnnamed_1 = 48;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_SCANCODE_LEFTBRACKET: C2RustUnnamed_1 = 47;
    #[c2rust::src_loc = "99:5"]
    pub const SDL_SCANCODE_EQUALS: C2RustUnnamed_1 = 46;
    #[c2rust::src_loc = "98:5"]
    pub const SDL_SCANCODE_MINUS: C2RustUnnamed_1 = 45;
    #[c2rust::src_loc = "96:5"]
    pub const SDL_SCANCODE_SPACE: C2RustUnnamed_1 = 44;
    #[c2rust::src_loc = "95:5"]
    pub const SDL_SCANCODE_TAB: C2RustUnnamed_1 = 43;
    #[c2rust::src_loc = "94:5"]
    pub const SDL_SCANCODE_BACKSPACE: C2RustUnnamed_1 = 42;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_SCANCODE_ESCAPE: C2RustUnnamed_1 = 41;
    #[c2rust::src_loc = "92:5"]
    pub const SDL_SCANCODE_RETURN: C2RustUnnamed_1 = 40;
    #[c2rust::src_loc = "90:5"]
    pub const SDL_SCANCODE_0: C2RustUnnamed_1 = 39;
    #[c2rust::src_loc = "89:5"]
    pub const SDL_SCANCODE_9: C2RustUnnamed_1 = 38;
    #[c2rust::src_loc = "88:5"]
    pub const SDL_SCANCODE_8: C2RustUnnamed_1 = 37;
    #[c2rust::src_loc = "87:5"]
    pub const SDL_SCANCODE_7: C2RustUnnamed_1 = 36;
    #[c2rust::src_loc = "86:5"]
    pub const SDL_SCANCODE_6: C2RustUnnamed_1 = 35;
    #[c2rust::src_loc = "85:5"]
    pub const SDL_SCANCODE_5: C2RustUnnamed_1 = 34;
    #[c2rust::src_loc = "84:5"]
    pub const SDL_SCANCODE_4: C2RustUnnamed_1 = 33;
    #[c2rust::src_loc = "83:5"]
    pub const SDL_SCANCODE_3: C2RustUnnamed_1 = 32;
    #[c2rust::src_loc = "82:5"]
    pub const SDL_SCANCODE_2: C2RustUnnamed_1 = 31;
    #[c2rust::src_loc = "81:5"]
    pub const SDL_SCANCODE_1: C2RustUnnamed_1 = 30;
    #[c2rust::src_loc = "79:5"]
    pub const SDL_SCANCODE_Z: C2RustUnnamed_1 = 29;
    #[c2rust::src_loc = "78:5"]
    pub const SDL_SCANCODE_Y: C2RustUnnamed_1 = 28;
    #[c2rust::src_loc = "77:5"]
    pub const SDL_SCANCODE_X: C2RustUnnamed_1 = 27;
    #[c2rust::src_loc = "76:5"]
    pub const SDL_SCANCODE_W: C2RustUnnamed_1 = 26;
    #[c2rust::src_loc = "75:5"]
    pub const SDL_SCANCODE_V: C2RustUnnamed_1 = 25;
    #[c2rust::src_loc = "74:5"]
    pub const SDL_SCANCODE_U: C2RustUnnamed_1 = 24;
    #[c2rust::src_loc = "73:5"]
    pub const SDL_SCANCODE_T: C2RustUnnamed_1 = 23;
    #[c2rust::src_loc = "72:5"]
    pub const SDL_SCANCODE_S: C2RustUnnamed_1 = 22;
    #[c2rust::src_loc = "71:5"]
    pub const SDL_SCANCODE_R: C2RustUnnamed_1 = 21;
    #[c2rust::src_loc = "70:5"]
    pub const SDL_SCANCODE_Q: C2RustUnnamed_1 = 20;
    #[c2rust::src_loc = "69:5"]
    pub const SDL_SCANCODE_P: C2RustUnnamed_1 = 19;
    #[c2rust::src_loc = "68:5"]
    pub const SDL_SCANCODE_O: C2RustUnnamed_1 = 18;
    #[c2rust::src_loc = "67:5"]
    pub const SDL_SCANCODE_N: C2RustUnnamed_1 = 17;
    #[c2rust::src_loc = "66:5"]
    pub const SDL_SCANCODE_M: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "65:5"]
    pub const SDL_SCANCODE_L: C2RustUnnamed_1 = 15;
    #[c2rust::src_loc = "64:5"]
    pub const SDL_SCANCODE_K: C2RustUnnamed_1 = 14;
    #[c2rust::src_loc = "63:5"]
    pub const SDL_SCANCODE_J: C2RustUnnamed_1 = 13;
    #[c2rust::src_loc = "62:5"]
    pub const SDL_SCANCODE_I: C2RustUnnamed_1 = 12;
    #[c2rust::src_loc = "61:5"]
    pub const SDL_SCANCODE_H: C2RustUnnamed_1 = 11;
    #[c2rust::src_loc = "60:5"]
    pub const SDL_SCANCODE_G: C2RustUnnamed_1 = 10;
    #[c2rust::src_loc = "59:5"]
    pub const SDL_SCANCODE_F: C2RustUnnamed_1 = 9;
    #[c2rust::src_loc = "58:5"]
    pub const SDL_SCANCODE_E: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "57:5"]
    pub const SDL_SCANCODE_D: C2RustUnnamed_1 = 7;
    #[c2rust::src_loc = "56:5"]
    pub const SDL_SCANCODE_C: C2RustUnnamed_1 = 6;
    #[c2rust::src_loc = "55:5"]
    pub const SDL_SCANCODE_B: C2RustUnnamed_1 = 5;
    #[c2rust::src_loc = "54:5"]
    pub const SDL_SCANCODE_A: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "45:5"]
    pub const SDL_SCANCODE_UNKNOWN: C2RustUnnamed_1 = 0;
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/pcrlib.h:34"]
pub mod pcrlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct ControlStruct {
        pub dir: dirtype,
        pub button1: boolean,
        pub button2: boolean,
    }
    #[c2rust::src_loc = "95:1"]
    pub type demoenum = libc::c_uint;
    #[c2rust::src_loc = "95:33"]
    pub const recording: demoenum = 2;
    #[c2rust::src_loc = "95:24"]
    pub const demoplay: demoenum = 1;
    #[c2rust::src_loc = "95:16"]
    pub const notdemo: demoenum = 0;
    #[c2rust::src_loc = "171:9"]
    pub type grtype = libc::c_uint;
    #[c2rust::src_loc = "171:32"]
    pub const VGAgr: grtype = 3;
    #[c2rust::src_loc = "171:26"]
    pub const EGAgr: grtype = 2;
    #[c2rust::src_loc = "171:20"]
    pub const CGAgr: grtype = 1;
    #[c2rust::src_loc = "171:15"]
    pub const text: grtype = 0;
    use super::catdefs_h::{boolean, dirtype, sdword, sword, word};
    extern "C" {
        #[c2rust::src_loc = "318:1"]
        pub fn RLEExpand(source: *mut libc::c_char, dest: *mut libc::c_char, origlen: libc::c_long);
        #[c2rust::src_loc = "310:1"]
        pub fn bioskey(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "308:1"]
        pub fn _quit(_: *mut libc::c_char);
        #[c2rust::src_loc = "307:1"]
        pub fn _setupgame();
        #[c2rust::src_loc = "305:1"]
        pub fn _checkhighscore();
        #[c2rust::src_loc = "304:1"]
        pub fn _showhighscores();
        #[c2rust::src_loc = "287:20"]
        pub static mut _extension: *const libc::c_char;
        #[c2rust::src_loc = "285:24"]
        pub static mut _maxplayers: libc::c_int;
        #[c2rust::src_loc = "285:12"]
        pub static mut _numlevels: libc::c_int;
        #[c2rust::src_loc = "272:15"]
        pub static mut score: sdword;
        #[c2rust::src_loc = "273:14"]
        pub static mut level: sword;
        #[c2rust::src_loc = "253:1"]
        pub fn expwin(width: libc::c_int, height: libc::c_int);
        #[c2rust::src_loc = "252:1"]
        pub fn centerwindow(width: libc::c_int, height: libc::c_int);
        #[c2rust::src_loc = "251:1"]
        pub fn bar(
            xl: libc::c_int,
            yl: libc::c_int,
            xh: libc::c_int,
            yh: libc::c_int,
            ch_0: libc::c_int,
        );
        #[c2rust::src_loc = "249:1"]
        pub fn drawwindow(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int);
        #[c2rust::src_loc = "247:1"]
        pub fn printint(val: libc::c_int);
        #[c2rust::src_loc = "243:1"]
        pub fn printchartile(str_0: *const libc::c_char);
        #[c2rust::src_loc = "242:1"]
        pub fn print(str_0: *const libc::c_char);
        #[c2rust::src_loc = "239:1"]
        pub fn get() -> libc::c_int;
        #[c2rust::src_loc = "238:1"]
        pub fn controlpanel();
        #[c2rust::src_loc = "236:27"]
        pub static mut screencentery: libc::c_int;
        #[c2rust::src_loc = "236:12"]
        pub static mut screencenterx: libc::c_int;
        #[c2rust::src_loc = "228:1"]
        pub fn drawchartile(x: libc::c_int, y: libc::c_int, tile: libc::c_int);
        #[c2rust::src_loc = "227:1"]
        pub fn drawpic(x: libc::c_int, y: libc::c_int, picnum: libc::c_int);
        #[c2rust::src_loc = "226:1"]
        pub fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
        #[c2rust::src_loc = "224:1"]
        pub fn installgrfile(
            filename: *mut libc::c_char,
            unpack: libc::c_int,
            inmem: *mut libc::c_void,
        );
        #[c2rust::src_loc = "179:1"]
        pub fn WaitVBL();
        #[c2rust::src_loc = "178:1"]
        pub fn UpdateScreen();
        #[c2rust::src_loc = "175:18"]
        pub static mut leftedge: libc::c_int;
        #[c2rust::src_loc = "175:15"]
        pub static mut sy: libc::c_int;
        #[c2rust::src_loc = "175:12"]
        pub static mut sx: libc::c_int;
        #[c2rust::src_loc = "173:15"]
        pub static mut grmode: grtype;
        #[c2rust::src_loc = "149:1"]
        pub fn _Verify(filename: *mut libc::c_char) -> libc::c_long;
        #[c2rust::src_loc = "147:1"]
        pub fn clearkeys();
        #[c2rust::src_loc = "145:1"]
        pub fn rnd(_: word) -> libc::c_int;
        #[c2rust::src_loc = "144:1"]
        pub fn rndt() -> libc::c_int;
        #[c2rust::src_loc = "138:1"]
        pub fn bloadin(filename: *mut libc::c_char) -> *mut libc::c_void;
        #[c2rust::src_loc = "136:1"]
        pub fn LoadFile(filename: *mut libc::c_char, buffer: *mut libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "125:1"]
        pub fn LoadDemo(demonum: libc::c_int);
        #[c2rust::src_loc = "123:1"]
        pub fn ControlPlayer(player_0: libc::c_int) -> ControlStruct;
        #[c2rust::src_loc = "96:22"]
        pub static mut indemo: demoenum;
        #[c2rust::src_loc = "90:16"]
        pub static mut keydown: [boolean; 512];
        #[c2rust::src_loc = "74:1"]
        pub fn WaitEndSound();
        #[c2rust::src_loc = "70:1"]
        pub fn PlaySound(sound: libc::c_int);
        #[c2rust::src_loc = "29:16"]
        pub static mut str: [libc::c_char; 80];
        #[c2rust::src_loc = "29:13"]
        pub static mut ch: libc::c_char;
    }
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catacomb.h:34"]
pub mod catacomb_h {
    #[c2rust::src_loc = "51:11"]
    pub type exittype = libc::c_uint;
    #[c2rust::src_loc = "51:39"]
    pub const victorious: exittype = 3;
    #[c2rust::src_loc = "51:31"]
    pub const reseted: exittype = 2;
    #[c2rust::src_loc = "51:24"]
    pub const killed: exittype = 1;
    #[c2rust::src_loc = "51:17"]
    pub const quited: exittype = 0;
    extern "C" {
        #[c2rust::src_loc = "45:1"]
        pub fn givenuke();
        #[c2rust::src_loc = "44:1"]
        pub fn givebolt();
        #[c2rust::src_loc = "43:1"]
        pub fn givepotion();
        #[c2rust::src_loc = "28:1"]
        pub fn playloop();
        #[c2rust::src_loc = "25:1"]
        pub fn initobjects();
        #[c2rust::src_loc = "39:1"]
        pub fn printhighscore();
        #[c2rust::src_loc = "41:1"]
        pub fn printbody();
        #[c2rust::src_loc = "40:1"]
        pub fn printshotpower();
        #[c2rust::src_loc = "38:1"]
        pub fn printscore();
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "565:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:29"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:34"]
pub mod strings_h {
    extern "C" {
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "130:14"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "122:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:34"]
pub mod fcntl_h {
    extern "C" {
        #[c2rust::src_loc = "168:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    }
}
pub use self::catacomb_h::{
    exittype, givebolt, givenuke, givepotion, initobjects, killed, playloop, printbody,
    printhighscore, printscore, printshotpower, quited, reseted, victorious,
};
pub use self::catdefs_h::{
    activeobj, bigshot, boolean, byte, classtype, dead1, dead2, dead3, dead4, dead5, dead6,
    dirtype, dragon, east, false_0, gargoyle, goblin, gune, guns, ingame, inscores, intitle, itoa,
    lastclass, nodir, north, northeast, northwest, nothing, objdeftype, objtype, ogre, player,
    rock, sbyte, sdword, secretgate, shot, skeleton, south, southeast, southwest, statetype, sword,
    teleporter, torch, true_0, turbogre, wallhit, west, word, C2RustUnnamed_0,
};
pub use self::ctype_h::{
    C2RustUnnamed, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint,
    _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, tolower, toupper,
};
use self::fcntl_h::open;
pub use self::pcrlib_h::{
    _extension, _maxplayers, _numlevels, bar, bloadin, centerwindow, ch, controlpanel, demoenum,
    demoplay, drawchar, drawchartile, drawpic, drawwindow, expwin, get, grtype, indemo,
    installgrfile, keydown, level, notdemo, print, printchartile, printint, recording, score,
    screencenterx, screencentery, str, text, CGAgr, ControlPlayer, ControlStruct, EGAgr, LoadDemo,
    LoadFile, PlaySound, RLEExpand, UpdateScreen, VGAgr, WaitEndSound, WaitVBL, _Verify,
    _checkhighscore, _quit, _setupgame, _showhighscores, bioskey, clearkeys, grmode, leftedge, rnd,
    rndt, sx, sy,
};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint8_t};
use self::stdio_h::{printf, sprintf};
use self::stdlib_h::{exit, free};
use self::string_h::{memset, strcat, strcpy};
use self::strings_h::strcasecmp;
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __ssize_t, __uint16_t, __uint8_t};
pub use self::unistd_h::{close, read, ssize_t, write};
pub use self::SDL_scancode_h::{
    C2RustUnnamed_1, SDL_NUM_SCANCODES, SDL_SCANCODE_0, SDL_SCANCODE_1, SDL_SCANCODE_2,
    SDL_SCANCODE_3, SDL_SCANCODE_4, SDL_SCANCODE_5, SDL_SCANCODE_6, SDL_SCANCODE_7, SDL_SCANCODE_8,
    SDL_SCANCODE_9, SDL_SCANCODE_A, SDL_SCANCODE_AC_BACK, SDL_SCANCODE_AC_BOOKMARKS,
    SDL_SCANCODE_AC_FORWARD, SDL_SCANCODE_AC_HOME, SDL_SCANCODE_AC_REFRESH, SDL_SCANCODE_AC_SEARCH,
    SDL_SCANCODE_AC_STOP, SDL_SCANCODE_AGAIN, SDL_SCANCODE_ALTERASE, SDL_SCANCODE_APOSTROPHE,
    SDL_SCANCODE_APP1, SDL_SCANCODE_APP2, SDL_SCANCODE_APPLICATION, SDL_SCANCODE_AUDIOFASTFORWARD,
    SDL_SCANCODE_AUDIOMUTE, SDL_SCANCODE_AUDIONEXT, SDL_SCANCODE_AUDIOPLAY, SDL_SCANCODE_AUDIOPREV,
    SDL_SCANCODE_AUDIOREWIND, SDL_SCANCODE_AUDIOSTOP, SDL_SCANCODE_B, SDL_SCANCODE_BACKSLASH,
    SDL_SCANCODE_BACKSPACE, SDL_SCANCODE_BRIGHTNESSDOWN, SDL_SCANCODE_BRIGHTNESSUP, SDL_SCANCODE_C,
    SDL_SCANCODE_CALCULATOR, SDL_SCANCODE_CANCEL, SDL_SCANCODE_CAPSLOCK, SDL_SCANCODE_CLEAR,
    SDL_SCANCODE_CLEARAGAIN, SDL_SCANCODE_COMMA, SDL_SCANCODE_COMPUTER, SDL_SCANCODE_COPY,
    SDL_SCANCODE_CRSEL, SDL_SCANCODE_CURRENCYSUBUNIT, SDL_SCANCODE_CURRENCYUNIT, SDL_SCANCODE_CUT,
    SDL_SCANCODE_D, SDL_SCANCODE_DECIMALSEPARATOR, SDL_SCANCODE_DELETE, SDL_SCANCODE_DISPLAYSWITCH,
    SDL_SCANCODE_DOWN, SDL_SCANCODE_E, SDL_SCANCODE_EJECT, SDL_SCANCODE_END, SDL_SCANCODE_EQUALS,
    SDL_SCANCODE_ESCAPE, SDL_SCANCODE_EXECUTE, SDL_SCANCODE_EXSEL, SDL_SCANCODE_F, SDL_SCANCODE_F1,
    SDL_SCANCODE_F10, SDL_SCANCODE_F11, SDL_SCANCODE_F12, SDL_SCANCODE_F13, SDL_SCANCODE_F14,
    SDL_SCANCODE_F15, SDL_SCANCODE_F16, SDL_SCANCODE_F17, SDL_SCANCODE_F18, SDL_SCANCODE_F19,
    SDL_SCANCODE_F2, SDL_SCANCODE_F20, SDL_SCANCODE_F21, SDL_SCANCODE_F22, SDL_SCANCODE_F23,
    SDL_SCANCODE_F24, SDL_SCANCODE_F3, SDL_SCANCODE_F4, SDL_SCANCODE_F5, SDL_SCANCODE_F6,
    SDL_SCANCODE_F7, SDL_SCANCODE_F8, SDL_SCANCODE_F9, SDL_SCANCODE_FIND, SDL_SCANCODE_G,
    SDL_SCANCODE_GRAVE, SDL_SCANCODE_H, SDL_SCANCODE_HELP, SDL_SCANCODE_HOME, SDL_SCANCODE_I,
    SDL_SCANCODE_INSERT, SDL_SCANCODE_INTERNATIONAL1, SDL_SCANCODE_INTERNATIONAL2,
    SDL_SCANCODE_INTERNATIONAL3, SDL_SCANCODE_INTERNATIONAL4, SDL_SCANCODE_INTERNATIONAL5,
    SDL_SCANCODE_INTERNATIONAL6, SDL_SCANCODE_INTERNATIONAL7, SDL_SCANCODE_INTERNATIONAL8,
    SDL_SCANCODE_INTERNATIONAL9, SDL_SCANCODE_J, SDL_SCANCODE_K, SDL_SCANCODE_KBDILLUMDOWN,
    SDL_SCANCODE_KBDILLUMTOGGLE, SDL_SCANCODE_KBDILLUMUP, SDL_SCANCODE_KP_0, SDL_SCANCODE_KP_00,
    SDL_SCANCODE_KP_000, SDL_SCANCODE_KP_1, SDL_SCANCODE_KP_2, SDL_SCANCODE_KP_3,
    SDL_SCANCODE_KP_4, SDL_SCANCODE_KP_5, SDL_SCANCODE_KP_6, SDL_SCANCODE_KP_7, SDL_SCANCODE_KP_8,
    SDL_SCANCODE_KP_9, SDL_SCANCODE_KP_A, SDL_SCANCODE_KP_AMPERSAND, SDL_SCANCODE_KP_AT,
    SDL_SCANCODE_KP_B, SDL_SCANCODE_KP_BACKSPACE, SDL_SCANCODE_KP_BINARY, SDL_SCANCODE_KP_C,
    SDL_SCANCODE_KP_CLEAR, SDL_SCANCODE_KP_CLEARENTRY, SDL_SCANCODE_KP_COLON,
    SDL_SCANCODE_KP_COMMA, SDL_SCANCODE_KP_D, SDL_SCANCODE_KP_DBLAMPERSAND,
    SDL_SCANCODE_KP_DBLVERTICALBAR, SDL_SCANCODE_KP_DECIMAL, SDL_SCANCODE_KP_DIVIDE,
    SDL_SCANCODE_KP_E, SDL_SCANCODE_KP_ENTER, SDL_SCANCODE_KP_EQUALS, SDL_SCANCODE_KP_EQUALSAS400,
    SDL_SCANCODE_KP_EXCLAM, SDL_SCANCODE_KP_F, SDL_SCANCODE_KP_GREATER, SDL_SCANCODE_KP_HASH,
    SDL_SCANCODE_KP_HEXADECIMAL, SDL_SCANCODE_KP_LEFTBRACE, SDL_SCANCODE_KP_LEFTPAREN,
    SDL_SCANCODE_KP_LESS, SDL_SCANCODE_KP_MEMADD, SDL_SCANCODE_KP_MEMCLEAR,
    SDL_SCANCODE_KP_MEMDIVIDE, SDL_SCANCODE_KP_MEMMULTIPLY, SDL_SCANCODE_KP_MEMRECALL,
    SDL_SCANCODE_KP_MEMSTORE, SDL_SCANCODE_KP_MEMSUBTRACT, SDL_SCANCODE_KP_MINUS,
    SDL_SCANCODE_KP_MULTIPLY, SDL_SCANCODE_KP_OCTAL, SDL_SCANCODE_KP_PERCENT,
    SDL_SCANCODE_KP_PERIOD, SDL_SCANCODE_KP_PLUS, SDL_SCANCODE_KP_PLUSMINUS, SDL_SCANCODE_KP_POWER,
    SDL_SCANCODE_KP_RIGHTBRACE, SDL_SCANCODE_KP_RIGHTPAREN, SDL_SCANCODE_KP_SPACE,
    SDL_SCANCODE_KP_TAB, SDL_SCANCODE_KP_VERTICALBAR, SDL_SCANCODE_KP_XOR, SDL_SCANCODE_L,
    SDL_SCANCODE_LALT, SDL_SCANCODE_LANG1, SDL_SCANCODE_LANG2, SDL_SCANCODE_LANG3,
    SDL_SCANCODE_LANG4, SDL_SCANCODE_LANG5, SDL_SCANCODE_LANG6, SDL_SCANCODE_LANG7,
    SDL_SCANCODE_LANG8, SDL_SCANCODE_LANG9, SDL_SCANCODE_LCTRL, SDL_SCANCODE_LEFT,
    SDL_SCANCODE_LEFTBRACKET, SDL_SCANCODE_LGUI, SDL_SCANCODE_LSHIFT, SDL_SCANCODE_M,
    SDL_SCANCODE_MAIL, SDL_SCANCODE_MEDIASELECT, SDL_SCANCODE_MENU, SDL_SCANCODE_MINUS,
    SDL_SCANCODE_MODE, SDL_SCANCODE_MUTE, SDL_SCANCODE_N, SDL_SCANCODE_NONUSBACKSLASH,
    SDL_SCANCODE_NONUSHASH, SDL_SCANCODE_NUMLOCKCLEAR, SDL_SCANCODE_O, SDL_SCANCODE_OPER,
    SDL_SCANCODE_OUT, SDL_SCANCODE_P, SDL_SCANCODE_PAGEDOWN, SDL_SCANCODE_PAGEUP,
    SDL_SCANCODE_PASTE, SDL_SCANCODE_PAUSE, SDL_SCANCODE_PERIOD, SDL_SCANCODE_POWER,
    SDL_SCANCODE_PRINTSCREEN, SDL_SCANCODE_PRIOR, SDL_SCANCODE_Q, SDL_SCANCODE_R,
    SDL_SCANCODE_RALT, SDL_SCANCODE_RCTRL, SDL_SCANCODE_RETURN, SDL_SCANCODE_RETURN2,
    SDL_SCANCODE_RGUI, SDL_SCANCODE_RIGHT, SDL_SCANCODE_RIGHTBRACKET, SDL_SCANCODE_RSHIFT,
    SDL_SCANCODE_S, SDL_SCANCODE_SCROLLLOCK, SDL_SCANCODE_SELECT, SDL_SCANCODE_SEMICOLON,
    SDL_SCANCODE_SEPARATOR, SDL_SCANCODE_SLASH, SDL_SCANCODE_SLEEP, SDL_SCANCODE_SPACE,
    SDL_SCANCODE_STOP, SDL_SCANCODE_SYSREQ, SDL_SCANCODE_T, SDL_SCANCODE_TAB,
    SDL_SCANCODE_THOUSANDSSEPARATOR, SDL_SCANCODE_U, SDL_SCANCODE_UNDO, SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UP, SDL_SCANCODE_V, SDL_SCANCODE_VOLUMEDOWN, SDL_SCANCODE_VOLUMEUP,
    SDL_SCANCODE_W, SDL_SCANCODE_WWW, SDL_SCANCODE_X, SDL_SCANCODE_Y, SDL_SCANCODE_Z,
};
extern "C" {
    #[c2rust::src_loc = "144:1"]
    pub fn cgarefresh();
    #[c2rust::src_loc = "145:1"]
    pub fn egarefresh();
}
#[no_mangle]
#[c2rust::src_loc = "44:8"]
pub static mut altmeters: [[libc::c_char; 14]; 14] = [
    [
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        23 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        0,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "60:8"]
pub static mut meters: [[libc::c_char; 14]; 14] = [
    [
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        127 as libc::c_int as libc::c_char,
        0,
    ],
    [
        26 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        28 as libc::c_int as libc::c_char,
        0,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "76:10"]
pub static mut opposite: [dirtype; 9] = [
    south, west, north, east, southwest, northwest, northeast, southeast, nodir,
];
#[no_mangle]
#[c2rust::src_loc = "85:12"]
pub static mut gamexit: exittype = quited;
#[no_mangle]
#[c2rust::src_loc = "87:7"]
pub static mut oldtiles: [libc::c_int; 576] = [0; 576];
#[no_mangle]
#[c2rust::src_loc = "88:7"]
pub static mut background: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
#[c2rust::src_loc = "89:7"]
pub static mut view: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
#[c2rust::src_loc = "90:7"]
pub static mut originx: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "90:16"]
pub static mut originy: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "91:8"]
pub static mut priority: [byte; 2048] = [0; 2048];
#[no_mangle]
#[c2rust::src_loc = "93:9"]
pub static mut items: [sword; 6] = [0; 6];
#[no_mangle]
#[c2rust::src_loc = "93:18"]
pub static mut saveitems: [sword; 6] = [0; 6];
#[no_mangle]
#[c2rust::src_loc = "94:7"]
pub static mut shotpower: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "95:7"]
pub static mut side: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "96:7"]
pub static mut boltsleft: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "98:13"]
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
#[c2rust::src_loc = "98:25"]
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
#[c2rust::src_loc = "99:11"]
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
#[c2rust::src_loc = "99:17"]
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
#[c2rust::src_loc = "100:7"]
pub static mut altnum: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "101:7"]
pub static mut numobj: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "101:14"]
pub static mut objecton: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "103:14"]
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
#[c2rust::src_loc = "105:11"]
pub static mut playdone: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "105:21"]
pub static mut leveldone: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "107:11"]
pub static mut tempb: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "108:9"]
pub static mut tempp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "110:7"]
pub static mut chkx: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "110:12"]
pub static mut chky: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "110:17"]
pub static mut chkspot: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "112:8"]
pub static mut frameon: word = 0;
#[no_mangle]
#[c2rust::src_loc = "113:9"]
pub static mut grmem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "114:13"]
pub static mut clvar: classtype = nothing;
#[no_mangle]
#[c2rust::src_loc = "116:7"]
pub static mut VGAPAL: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "118:11"]
pub static mut exitdemo: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "118:20"]
pub static mut resetgame: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "119:13"]
pub static mut gamestate: statetype = ingame;
#[no_mangle]
#[c2rust::src_loc = "121:17"]
pub static mut ctrl: ControlStruct = ControlStruct {
    dir: north,
    button1: 0,
    button2: 0,
};
#[no_mangle]
#[c2rust::src_loc = "123:9"]
pub static mut pics: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "123:16"]
pub static mut picsexact: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "125:12"]
pub static mut EGADATASTART: libc::c_uint = 0;
#[no_mangle]
#[c2rust::src_loc = "127:10"]
pub static mut savescore: sdword = 0;
#[no_mangle]
#[c2rust::src_loc = "130:10"]
pub static mut GODMODE: boolean = false_0 as libc::c_int as boolean;
#[no_mangle]
#[c2rust::src_loc = "159:6"]
pub static mut demowin: [[libc::c_char; 16]; 5] = [
    [
        14 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        16 as libc::c_int as libc::c_char,
    ],
    [
        17 as libc::c_int as libc::c_char,
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
        18 as libc::c_int as libc::c_char,
    ],
    [
        17 as libc::c_int as libc::c_char,
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
        18 as libc::c_int as libc::c_char,
    ],
    [
        17 as libc::c_int as libc::c_char,
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
        18 as libc::c_int as libc::c_char,
    ],
    [
        19 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn refresh() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut basex: libc::c_int = 0;
    let mut basey: libc::c_int = 0;
    let mut underwin: [[word; 16]; 5] = [[0; 16]; 5];
    basex = originx + 4 as libc::c_int;
    basey = originy + 17 as libc::c_int;
    if indemo as u64 != 0 {
        y = 0 as libc::c_int;
        while y <= 4 as libc::c_int {
            x = 0 as libc::c_int;
            while x <= 15 as libc::c_int {
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
        y = 0 as libc::c_int;
        while y <= 4 as libc::c_int {
            x = 0 as libc::c_int;
            while x <= 15 as libc::c_int {
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
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn simplerefresh() {
    WaitVBL();
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        cgarefresh();
    } else {
        egarefresh();
    };
}
#[no_mangle]
#[c2rust::src_loc = "220:1"]
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
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
    } else {
        pics = bloadin(b"EGACHARS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
        picsexact = pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn clearold() {
    memset(
        &mut oldtiles as *mut [libc::c_int; 576] as *mut libc::c_void,
        0xff as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 576]>() as libc::c_ulong,
    );
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn restore() {
    clearold();
    simplerefresh();
}
#[no_mangle]
#[c2rust::src_loc = "263:1"]
pub unsafe extern "C" fn wantmore() -> boolean {
    sx = 2 as libc::c_int;
    sy = 20 as libc::c_int;
    print(b"(space for more/esc)\0" as *const u8 as *const libc::c_char);
    sx = 12 as libc::c_int;
    sy = 21 as libc::c_int;
    ch = get() as libc::c_char;
    if ch as libc::c_int == 27 as libc::c_int {
        return false_0 as libc::c_int as boolean;
    }
    return true_0 as libc::c_int as boolean;
}
#[c2rust::src_loc = "281:1"]
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
    while yy <= y + size - 1 as libc::c_int {
        xx = x;
        while xx <= x + size - 1 as libc::c_int {
            let fresh0 = tilenum;
            tilenum = tilenum + 1;
            drawchartile(xx, yy, fresh0);
            xx += 1;
        }
        yy += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn help() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
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
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
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
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
    print(b"Button 1 / ctrl key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Builds shot power.  \n\0" as *const u8 as *const libc::c_char);
    print(b"If the shot power   \n\0" as *const u8 as *const libc::c_char);
    print(b"meter is full when  \n\0" as *const u8 as *const libc::c_char);
    print(b"the button is       \n\0" as *const u8 as *const libc::c_char);
    print(b"released, a super   \n\0" as *const u8 as *const libc::c_char);
    print(b"shot will be        \n\0" as *const u8 as *const libc::c_char);
    print(b"launched.           \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    y = 11 as libc::c_int;
    while y <= 18 as libc::c_int {
        x = 3 as libc::c_int;
        while x <= 20 as libc::c_int {
            drawchartile(x, y, 128 as libc::c_int);
            x += 1;
        }
        y += 1;
    }
    charpic(
        4 as libc::c_int,
        14 as libc::c_int,
        player,
        east,
        2 as libc::c_int,
    );
    charpic(
        19 as libc::c_int,
        15 as libc::c_int,
        shot,
        east,
        1 as libc::c_int,
    );
    charpic(
        17 as libc::c_int,
        14 as libc::c_int,
        shot,
        east,
        0 as libc::c_int,
    );
    charpic(
        15 as libc::c_int,
        15 as libc::c_int,
        shot,
        east,
        1 as libc::c_int,
    );
    charpic(
        8 as libc::c_int,
        14 as libc::c_int,
        bigshot,
        east,
        0 as libc::c_int,
    );
    if wantmore() == 0 {
        return;
    }
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
    print(b"Button 2 / alt key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Allows you to move  \n\0" as *const u8 as *const libc::c_char);
    print(b"without changing the\n\0" as *const u8 as *const libc::c_char);
    print(b"direction you are   \n\0" as *const u8 as *const libc::c_char);
    print(b"facing.  Good for   \n\0" as *const u8 as *const libc::c_char);
    print(b"searching walls and \n\0" as *const u8 as *const libc::c_char);
    print(b"fighting retreats.  \n\0" as *const u8 as *const libc::c_char);
    y = 11 as libc::c_int;
    while y <= 18 as libc::c_int {
        x = 3 as libc::c_int;
        while x <= 20 as libc::c_int {
            if y == 15 as libc::c_int {
                drawchartile(x, y, 129 as libc::c_int);
            } else if y == 16 as libc::c_int {
                drawchartile(x, y, 131 as libc::c_int);
            } else {
                drawchartile(x, y, 128 as libc::c_int);
            }
            x += 1;
        }
        y += 1;
    }
    charpic(
        6 as libc::c_int,
        13 as libc::c_int,
        player,
        south,
        2 as libc::c_int,
    );
    sx = 6 as libc::c_int;
    sy = 15 as libc::c_int;
    print(b"\x1D\x1D\x1E\x1E\x1F\x1F\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
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
    centerwindow(20 as libc::c_int, 20 as libc::c_int);
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
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn reset() {
    centerwindow(18 as libc::c_int, 1 as libc::c_int);
    print(b"reset game (y/n)?\0" as *const u8 as *const libc::c_char);
    ch = get() as libc::c_char;
    if ch as libc::c_int == 'y' as i32 {
        gamexit = killed;
        playdone = true_0 as libc::c_int as boolean;
    }
}
#[no_mangle]
#[c2rust::src_loc = "450:1"]
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
    itoa(level as libc::c_int, st.as_mut_ptr(), 10 as libc::c_int);
    strcat(filename.as_mut_ptr(), st.as_mut_ptr());
    strcat(
        filename.as_mut_ptr(),
        b".CA2\0" as *const u8 as *const libc::c_char,
    );
    LoadFile(filename.as_mut_ptr(), rle.as_mut_ptr());
    RLEExpand(
        &mut *rle.as_mut_ptr().offset(4 as libc::c_int as isize),
        sm.as_mut_ptr(),
        4096 as libc::c_int as libc::c_long,
    );
    numobj = 0 as libc::c_int;
    o[0 as libc::c_int as usize].x = 13 as libc::c_int as byte;
    o[0 as libc::c_int as usize].y = 13 as libc::c_int as byte;
    o[0 as libc::c_int as usize].stage = 0 as libc::c_int as byte;
    o[0 as libc::c_int as usize].delay = 0 as libc::c_int as byte;
    o[0 as libc::c_int as usize].dir = east as libc::c_int as word;
    o[0 as libc::c_int as usize].oldx = 0 as libc::c_int as byte;
    o[0 as libc::c_int as usize].oldy = 0 as libc::c_int as byte;
    o[0 as libc::c_int as usize].oldtile = -(1 as libc::c_int) as sword;
    yy = 0 as libc::c_int;
    while yy < 64 as libc::c_int {
        xx = 0 as libc::c_int;
        while xx < 64 as libc::c_int {
            btile = sm[(yy * 64 as libc::c_int + xx) as usize] as byte;
            if (btile as libc::c_int) < 230 as libc::c_int {
                background[(yy + 11 as libc::c_int) as usize][(xx + 11 as libc::c_int) as usize] =
                    btile as libc::c_int;
            } else {
                background[(yy + 11 as libc::c_int) as usize][(xx + 11 as libc::c_int) as usize] =
                    128 as libc::c_int;
                if tokens[(btile as libc::c_int - 230 as libc::c_int) as usize] as libc::c_uint
                    == player as libc::c_int as libc::c_uint
                {
                    o[0 as libc::c_int as usize].x = (xx + 11 as libc::c_int) as byte;
                    o[0 as libc::c_int as usize].y = (yy + 11 as libc::c_int) as byte;
                } else {
                    numobj += 1;
                    o[numobj as usize].active = false_0 as libc::c_int as boolean;
                    o[numobj as usize].class =
                        tokens[(btile as libc::c_int - 230 as libc::c_int) as usize] as word;
                    o[numobj as usize].x = (xx + 11 as libc::c_int) as byte;
                    o[numobj as usize].y = (yy + 11 as libc::c_int) as byte;
                    o[numobj as usize].stage = 0 as libc::c_int as byte;
                    o[numobj as usize].delay = 0 as libc::c_int as byte;
                    o[numobj as usize].dir = (rndt() / 64 as libc::c_int) as dirtype as word;
                    o[numobj as usize].hp =
                        objdef[o[numobj as usize].class as usize].hitpoints as sbyte;
                    o[numobj as usize].oldx = o[numobj as usize].x;
                    o[numobj as usize].oldy = o[numobj as usize].y;
                    o[numobj as usize].oldtile = -(1 as libc::c_int) as sword;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    originx = o[0 as libc::c_int as usize].x as libc::c_int - 11 as libc::c_int;
    originy = o[0 as libc::c_int as usize].y as libc::c_int - 11 as libc::c_int;
    shotpower = 0 as libc::c_int;
    y = 11 as libc::c_int - 1 as libc::c_int;
    while y < 65 as libc::c_int + 11 as libc::c_int {
        x = 11 as libc::c_int - 1 as libc::c_int;
        while x < 64 as libc::c_int + 11 as libc::c_int {
            view[y as usize][x as usize] = background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    sx = 33 as libc::c_int;
    sy = 1 as libc::c_int;
    printint(level as libc::c_int);
    print(b" \0" as *const u8 as *const libc::c_char);
    restore();
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        saveitems[i as usize] = items[i as usize];
        i += 1;
    }
    savescore = score;
    saveo[0 as libc::c_int as usize] = o[0 as libc::c_int as usize];
}
#[no_mangle]
#[c2rust::src_loc = "562:1"]
pub unsafe extern "C" fn drawside() {
    let mut i: libc::c_int = 0;
    sx = 0 as libc::c_int;
    while sx < 40 as libc::c_int {
        drawchar(sx, 24 as libc::c_int, 0 as libc::c_int);
        sx += 1;
    }
    sy = 0 as libc::c_int;
    while sy < 24 as libc::c_int {
        drawchar(39 as libc::c_int, sy, 0 as libc::c_int);
        sy += 1;
    }
    drawwindow(
        24 as libc::c_int,
        0 as libc::c_int,
        38 as libc::c_int,
        23 as libc::c_int,
    );
    print(
        b"  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n\0" as *const u8 as *const libc::c_char,
    );
    print(b" shot power\n\n\n    body\n\n\n\0" as *const u8 as *const libc::c_char);
    printhighscore();
    printbody();
    printshotpower();
    printscore();
    sx = 33 as libc::c_int;
    sy = 1 as libc::c_int;
    printint(level as libc::c_int);
    drawpic(
        25 as libc::c_int * 8 as libc::c_int,
        17 as libc::c_int * 8 as libc::c_int,
        13 as libc::c_int,
    );
    i = 1 as libc::c_int;
    while i <= items[1 as libc::c_int as usize] as libc::c_int && i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 7 as libc::c_int, 31 as libc::c_int);
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= items[2 as libc::c_int as usize] as libc::c_int && i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 8 as libc::c_int, 29 as libc::c_int);
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= items[3 as libc::c_int as usize] as libc::c_int && i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 9 as libc::c_int, 30 as libc::c_int);
        i += 1;
    }
    i = 1 as libc::c_int;
    while i <= items[5 as libc::c_int as usize] as libc::c_int && i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 10 as libc::c_int, 30 as libc::c_int);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "607:1"]
pub unsafe extern "C" fn playsetup() {
    let mut i: libc::c_int = 0;
    shotpower = 0 as libc::c_int;
    bar(
        0 as libc::c_int,
        0 as libc::c_int,
        23 as libc::c_int,
        23 as libc::c_int,
        0 as libc::c_int,
    );
    if level as libc::c_int == 0 as libc::c_int {
        i = 1 as libc::c_int;
        while i < 6 as libc::c_int {
            items[i as usize] = 0 as libc::c_int as sword;
            i += 1;
        }
        score = 0 as libc::c_int;
        level = 1 as libc::c_int as sword;
        o[0 as libc::c_int as usize].active = true_0 as libc::c_int as boolean;
        o[0 as libc::c_int as usize].class = player as libc::c_int as word;
        o[0 as libc::c_int as usize].hp = 13 as libc::c_int as sbyte;
        o[0 as libc::c_int as usize].dir = west as libc::c_int as word;
        o[0 as libc::c_int as usize].stage = 0 as libc::c_int as byte;
        o[0 as libc::c_int as usize].delay = 0 as libc::c_int as byte;
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
#[c2rust::src_loc = "654:1"]
pub unsafe extern "C" fn repaintscreen() {
    match gamestate as libc::c_uint {
        1 => {
            drawpic(0 as libc::c_int, 0 as libc::c_int, 14 as libc::c_int);
        }
        0 => {
            restore();
            drawside();
            printscore();
            sx = 33 as libc::c_int;
            sy = 1 as libc::c_int;
            printint(level as libc::c_int);
        }
        2 => {
            restore();
            drawside();
            printscore();
            sx = 33 as libc::c_int;
            sy = 1 as libc::c_int;
            printint(level as libc::c_int);
            indemo = demoplay;
        }
        _ => {
            sy = 10 as libc::c_int;
            sx = sy;
            print(b"Bad gamestate!\0" as *const u8 as *const libc::c_char);
            clearkeys();
            get();
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "698:1"]
pub unsafe extern "C" fn dofkeys() {
    let mut handle: libc::c_int = 0;
    let mut key: libc::c_int = bioskey(1 as libc::c_int);
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
            expwin(18 as libc::c_int, 1 as libc::c_int);
            print(b"RESET GAME (Y/N)?\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            if ch as libc::c_int == 'Y' as i32 {
                resetgame = true_0 as libc::c_int as boolean;
            }
        }
        61 => {
            clearkeys();
            expwin(22 as libc::c_int, 4 as libc::c_int);
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
                    str[4 as libc::c_int as usize] = ch;
                    if _Verify(str.as_mut_ptr()) != 0 {
                        print(
                            b"\nGame exists,\noverwrite (Y/N)?\0" as *const u8
                                as *const libc::c_char,
                        );
                        ch = get() as libc::c_char;
                        if ch as libc::c_int != 'Y' as i32 && ch as libc::c_int != 'y' as i32 {
                            current_block_72 = 13303144130133872306;
                        } else {
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
                            sy -= 1;
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
                            sx = leftedge;
                            sy -= 1;
                            current_block_72 = 16924917904204750491;
                        }
                    } else {
                        current_block_72 = 16924917904204750491;
                    }
                    match current_block_72 {
                        13303144130133872306 => {}
                        _ => {
                            handle = open(
                                str.as_mut_ptr(),
                                0o1 as libc::c_int
                                    | 0 as libc::c_int
                                    | 0o100 as libc::c_int
                                    | 0o1000 as libc::c_int,
                                0o400 as libc::c_int | 0o200 as libc::c_int,
                            );
                            if handle == -(1 as libc::c_int) {
                                return;
                            }
                            write(
                                handle,
                                &mut saveitems as *mut [sword; 6] as *const libc::c_void,
                                ::core::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut savescore as *mut sdword as *const libc::c_void,
                                ::core::mem::size_of::<sdword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut level as *mut sword as *const libc::c_void,
                                ::core::mem::size_of::<sword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut *saveo.as_mut_ptr().offset(0 as libc::c_int as isize)
                                    as *mut activeobj
                                    as *const libc::c_void,
                                ::core::mem::size_of::<activeobj>() as libc::c_ulong,
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
            expwin(22 as libc::c_int, 4 as libc::c_int);
            print(b"Load game #(1-9):\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            drawchar(sx, sy, ch as libc::c_int);
            if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
                strcpy(
                    str.as_mut_ptr(),
                    b"GAME0.CA2\0" as *const u8 as *const libc::c_char,
                );
                str[4 as libc::c_int as usize] = ch;
                handle = open(
                    str.as_mut_ptr(),
                    0 as libc::c_int | 0 as libc::c_int,
                    0o200 as libc::c_int | 0o400 as libc::c_int,
                );
                if handle == -(1 as libc::c_int) {
                    print(b"\nGame not found.\0" as *const u8 as *const libc::c_char);
                    get();
                } else {
                    read(
                        handle,
                        &mut items as *mut [sword; 6] as *mut libc::c_void,
                        ::core::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut score as *mut sdword as *mut libc::c_void,
                        ::core::mem::size_of::<sdword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut level as *mut sword as *mut libc::c_void,
                        ::core::mem::size_of::<sword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut *o.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut activeobj
                            as *mut libc::c_void,
                        ::core::mem::size_of::<activeobj>() as libc::c_ulong,
                    );
                    close(handle);
                    exitdemo = true_0 as libc::c_int as boolean;
                    if indemo as libc::c_uint != notdemo as libc::c_int as libc::c_uint {
                        playdone = true_0 as libc::c_int as boolean;
                    }
                    drawside();
                    leveldone = true_0 as libc::c_int as boolean;
                }
            }
        }
        66 => {
            clearkeys();
            expwin(7 as libc::c_int, 1 as libc::c_int);
            print(b"PAUSED\0" as *const u8 as *const libc::c_char);
            get();
        }
        67 => {
            clearkeys();
            expwin(12 as libc::c_int, 1 as libc::c_int);
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
#[c2rust::src_loc = "840:1"]
pub unsafe extern "C" fn dotitlepage() {
    let mut i: libc::c_int = 0;
    drawpic(0 as libc::c_int, 0 as libc::c_int, 14 as libc::c_int);
    UpdateScreen();
    gamestate = intitle;
    i = 0 as libc::c_int;
    while i < 300 as libc::c_int {
        WaitVBL();
        indemo = notdemo;
        ctrl = ControlPlayer(1 as libc::c_int);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
        {
            level = 0 as libc::c_int as sword;
            exitdemo = true_0 as libc::c_int as boolean;
            break;
        } else {
            indemo = demoplay;
            if bioskey(1 as libc::c_int) != 0 {
                dofkeys();
                UpdateScreen();
            }
            if exitdemo != 0 {
                break;
            }
            i += 1;
        }
    }
    gamestate = ingame;
}
#[no_mangle]
#[c2rust::src_loc = "881:1"]
pub unsafe extern "C" fn doendpage() {
    WaitEndSound();
    drawpic(0 as libc::c_int, 0 as libc::c_int, 15 as libc::c_int);
    PlaySound(3 as libc::c_int);
    WaitEndSound();
    PlaySound(3 as libc::c_int);
    WaitEndSound();
    PlaySound(3 as libc::c_int);
    WaitEndSound();
    PlaySound(3 as libc::c_int);
    WaitEndSound();
    drawwindow(
        0 as libc::c_int,
        0 as libc::c_int,
        17 as libc::c_int,
        9 as libc::c_int,
    );
    print(b"Congratulation! \n\0" as *const u8 as *const libc::c_char);
    print(b"One as skilled  \n\0" as *const u8 as *const libc::c_char);
    print(b"as yourself     \n\0" as *const u8 as *const libc::c_char);
    print(b"deserves the    \n\0" as *const u8 as *const libc::c_char);
    print(b"10,000,000 gold \n\0" as *const u8 as *const libc::c_char);
    print(b"you pulled out  \n\0" as *const u8 as *const libc::c_char);
    print(b"of the palace! \0" as *const u8 as *const libc::c_char);
    clearkeys();
    get();
    drawwindow(
        0 as libc::c_int,
        0 as libc::c_int,
        17 as libc::c_int,
        9 as libc::c_int,
    );
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
#[c2rust::src_loc = "930:1"]
pub unsafe extern "C" fn dodemo() {
    let mut i: libc::c_int = 0;
    while exitdemo == 0 {
        dotitlepage();
        if exitdemo != 0 {
            break;
        }
        i = rnd((1 as libc::c_int - 1 as libc::c_int) as word) + 1 as libc::c_int;
        LoadDemo(i);
        level = 0 as libc::c_int as sword;
        playsetup();
        playloop();
        if exitdemo != 0 {
            break;
        }
        level = 0 as libc::c_int as sword;
        gamestate = inscores;
        indemo = demoplay;
        _showhighscores();
        UpdateScreen();
        i = 0 as libc::c_int;
        while i < 500 as libc::c_int {
            WaitVBL();
            indemo = notdemo;
            ctrl = ControlPlayer(1 as libc::c_int);
            if ctrl.button1 as libc::c_int != 0
                || ctrl.button2 as libc::c_int != 0
                || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
            {
                exitdemo = true_0 as libc::c_int as boolean;
                break;
            } else {
                if bioskey(1 as libc::c_int) != 0 {
                    dofkeys();
                }
                if exitdemo != 0 {
                    break;
                }
                i += 1;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "985:1"]
pub unsafe extern "C" fn gameover() {
    let mut i: libc::c_int = 0;
    expwin(11 as libc::c_int, 4 as libc::c_int);
    print(b"\n GAME OVER\n     \0" as *const u8 as *const libc::c_char);
    UpdateScreen();
    WaitEndSound();
    i = 0 as libc::c_int;
    while i < 120 as libc::c_int {
        WaitVBL();
        i += 1;
    }
    gamestate = inscores;
    _checkhighscore();
    level = 0 as libc::c_int as sword;
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        WaitVBL();
        ctrl = ControlPlayer(1 as libc::c_int);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
        {
            break;
        }
        if bioskey(1 as libc::c_int) != 0 {
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
#[c2rust::src_loc = "1022:1"]
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
    i = 0 as libc::c_int;
    while !(*strings).is_null() && **strings as libc::c_int != 0 {
        let fresh1 = strings;
        strings = strings.offset(1);
        s = *fresh1;
        p = parm;
        cp = 0 as libc::c_int as libc::c_char;
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
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1059:5"]
pub static mut _argc: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "1060:8"]
pub static mut _argv: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[c2rust::src_loc = "1061:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    _argc = argc;
    _argv = argv;
    if _argc > 1 as libc::c_int
        && strcasecmp(
            *_argv.offset(1 as libc::c_int as isize),
            b"/VER\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(
            b"CatacombSDL\nVersion 1.03\n\nUsage: catacomb [windowed <width> <height>] [screen <num>]\n\nPorted by Braden \"Blzut3\" Obrzut and Rene \"Havoc\" Nicolaus\nIncludes PC Speaker emulator by K1n9_Duk3\nBased on The Catacomb source code:\nCopyright 1990-1993 Softdisk Publishing\nCopyright 1993-2014 Flat Rock Software\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    initobjects();
    memset(
        &mut priority as *mut [byte; 2048] as *mut libc::c_void,
        99 as libc::c_int,
        ::core::mem::size_of::<[byte; 2048]>() as libc::c_ulong,
    );
    priority[128 as libc::c_int as usize] = 0 as libc::c_int as byte;
    i = objdef[teleporter as libc::c_int as usize].firstchar as libc::c_int;
    while i
        <= objdef[teleporter as libc::c_int as usize].firstchar as libc::c_int + 20 as libc::c_int
    {
        priority[i as usize] = 0 as libc::c_int as byte;
        i += 1;
    }
    clvar = dead2;
    while clvar as libc::c_uint <= dead5 as libc::c_int as libc::c_uint {
        i = objdef[clvar as usize].firstchar as libc::c_int;
        while i
            <= objdef[clvar as usize].firstchar as libc::c_int
                + objdef[clvar as usize].size as libc::c_int
                    * objdef[clvar as usize].size as libc::c_int
        {
            priority[i as usize] = 0 as libc::c_int as byte;
            i += 1;
        }
        clvar += 1;
    }
    i = 152 as libc::c_int;
    while i <= 161 as libc::c_int {
        priority[i as usize] = 2 as libc::c_int as byte;
        i += 1;
    }
    i = objdef[bigshot as libc::c_int as usize].firstchar as libc::c_int;
    while i <= objdef[bigshot as libc::c_int as usize].firstchar as libc::c_int + 31 as libc::c_int
    {
        priority[i as usize] = 2 as libc::c_int as byte;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i <= 256 as libc::c_int - 1 as libc::c_int {
        if priority[i as usize] as libc::c_int == 99 as libc::c_int {
            priority[i as usize] = 3 as libc::c_int as byte;
        }
        i += 1;
    }
    priority[167 as libc::c_int as usize] = 1 as libc::c_int as byte;
    i = 256 as libc::c_int;
    while i <= 2047 as libc::c_int {
        if priority[i as usize] as libc::c_int == 99 as libc::c_int {
            priority[i as usize] = 4 as libc::c_int as byte;
        }
        i += 1;
    }
    i = objdef[player as libc::c_int as usize].firstchar as libc::c_int;
    while i <= objdef[player as libc::c_int as usize].firstchar as libc::c_int + 63 as libc::c_int {
        priority[i as usize] = 5 as libc::c_int as byte;
        i += 1;
    }
    side = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x <= 85 as libc::c_int {
        y = 0 as libc::c_int;
        while y <= 11 as libc::c_int - 1 as libc::c_int {
            view[x as usize][y as usize] = 129 as libc::c_int;
            view[x as usize][(85 as libc::c_int - y) as usize] = 129 as libc::c_int;
            background[x as usize][y as usize] = 129 as libc::c_int;
            background[x as usize][(85 as libc::c_int - y) as usize] = 129 as libc::c_int;
            y += 1;
        }
        view[86 as libc::c_int as usize][x as usize] = 129 as libc::c_int;
        x += 1;
    }
    y = 11 as libc::c_int;
    while y <= 74 as libc::c_int {
        x = 0 as libc::c_int;
        while x <= 11 as libc::c_int - 1 as libc::c_int {
            view[x as usize][y as usize] = 129 as libc::c_int;
            view[(85 as libc::c_int - x) as usize][y as usize] = 129 as libc::c_int;
            background[x as usize][y as usize] = 129 as libc::c_int;
            background[(85 as libc::c_int - x) as usize][y as usize] = 129 as libc::c_int;
            x += 1;
        }
        y += 1;
    }
    _numlevels = 30 as libc::c_int;
    _maxplayers = 1 as libc::c_int;
    _extension = b"CA2\0" as *const u8 as *const libc::c_char;
    _setupgame();
    expwin(33 as libc::c_int, 13 as libc::c_int);
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
    screencentery = 11 as libc::c_int;
    screencenterx = 11 as libc::c_int;
    exitdemo = false_0 as libc::c_int as boolean;
    level = 0 as libc::c_int as sword;
    loop {
        dodemo();
        playsetup();
        indemo = notdemo;
        gamestate = ingame;
        playloop();
        if indemo as u64 == 0 {
            exitdemo = false_0 as libc::c_int as boolean;
            if level as libc::c_int > 30 as libc::c_int {
                doendpage();
            }
            gameover();
        }
    }
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
