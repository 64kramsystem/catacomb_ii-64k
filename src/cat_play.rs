use ::libc;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:19"]
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
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:19"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:21"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint16_t, __uint8_t};
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:21"]
pub mod catdefs_h {
    #[c2rust::src_loc = "34:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "34:13"]
    pub const true_0: C2RustUnnamed = 1;
    #[c2rust::src_loc = "34:7"]
    pub const false_0: C2RustUnnamed = 0;
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
    #[c2rust::src_loc = "83:9"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "84:28"]
    pub const gunthinks: C2RustUnnamed_0 = 10;
    #[c2rust::src_loc = "84:18"]
    pub const gunthinke: C2RustUnnamed_0 = 9;
    #[c2rust::src_loc = "84:10"]
    pub const explode: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "84:5"]
    pub const fade: C2RustUnnamed_0 = 7;
    #[c2rust::src_loc = "83:72"]
    pub const idle: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "83:63"]
    pub const straight: C2RustUnnamed_0 = 5;
    #[c2rust::src_loc = "83:55"]
    pub const ramdiag: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "83:43"]
    pub const ramstraight: C2RustUnnamed_0 = 3;
    #[c2rust::src_loc = "83:33"]
    pub const dragoncmd: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "83:25"]
    pub const gargcmd: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "83:15"]
    pub const playercmd: C2RustUnnamed_0 = 0;
    #[c2rust::src_loc = "86:9"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "86:42"]
    pub const nukeshot: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "86:36"]
    pub const mshot: C2RustUnnamed_1 = 3;
    #[c2rust::src_loc = "86:30"]
    pub const pshot: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "86:22"]
    pub const monster: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "86:15"]
    pub const benign: C2RustUnnamed_1 = 0;
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
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_scancode.h:22"]
pub mod SDL_scancode_h {
    #[c2rust::src_loc = "43:9"]
    pub type C2RustUnnamed_2 = libc::c_uint;
    #[c2rust::src_loc = "407:5"]
    pub const SDL_NUM_SCANCODES: C2RustUnnamed_2 = 512;
    #[c2rust::src_loc = "401:5"]
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: C2RustUnnamed_2 = 286;
    #[c2rust::src_loc = "400:5"]
    pub const SDL_SCANCODE_AUDIOREWIND: C2RustUnnamed_2 = 285;
    #[c2rust::src_loc = "389:5"]
    pub const SDL_SCANCODE_APP2: C2RustUnnamed_2 = 284;
    #[c2rust::src_loc = "388:5"]
    pub const SDL_SCANCODE_APP1: C2RustUnnamed_2 = 283;
    #[c2rust::src_loc = "386:5"]
    pub const SDL_SCANCODE_SLEEP: C2RustUnnamed_2 = 282;
    #[c2rust::src_loc = "385:5"]
    pub const SDL_SCANCODE_EJECT: C2RustUnnamed_2 = 281;
    #[c2rust::src_loc = "384:5"]
    pub const SDL_SCANCODE_KBDILLUMUP: C2RustUnnamed_2 = 280;
    #[c2rust::src_loc = "383:5"]
    pub const SDL_SCANCODE_KBDILLUMDOWN: C2RustUnnamed_2 = 279;
    #[c2rust::src_loc = "382:5"]
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: C2RustUnnamed_2 = 278;
    #[c2rust::src_loc = "380:5"]
    pub const SDL_SCANCODE_DISPLAYSWITCH: C2RustUnnamed_2 = 277;
    #[c2rust::src_loc = "379:5"]
    pub const SDL_SCANCODE_BRIGHTNESSUP: C2RustUnnamed_2 = 276;
    #[c2rust::src_loc = "378:5"]
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: C2RustUnnamed_2 = 275;
    #[c2rust::src_loc = "367:5"]
    pub const SDL_SCANCODE_AC_BOOKMARKS: C2RustUnnamed_2 = 274;
    #[c2rust::src_loc = "366:5"]
    pub const SDL_SCANCODE_AC_REFRESH: C2RustUnnamed_2 = 273;
    #[c2rust::src_loc = "365:5"]
    pub const SDL_SCANCODE_AC_STOP: C2RustUnnamed_2 = 272;
    #[c2rust::src_loc = "364:5"]
    pub const SDL_SCANCODE_AC_FORWARD: C2RustUnnamed_2 = 271;
    #[c2rust::src_loc = "363:5"]
    pub const SDL_SCANCODE_AC_BACK: C2RustUnnamed_2 = 270;
    #[c2rust::src_loc = "362:5"]
    pub const SDL_SCANCODE_AC_HOME: C2RustUnnamed_2 = 269;
    #[c2rust::src_loc = "361:5"]
    pub const SDL_SCANCODE_AC_SEARCH: C2RustUnnamed_2 = 268;
    #[c2rust::src_loc = "360:5"]
    pub const SDL_SCANCODE_COMPUTER: C2RustUnnamed_2 = 267;
    #[c2rust::src_loc = "359:5"]
    pub const SDL_SCANCODE_CALCULATOR: C2RustUnnamed_2 = 266;
    #[c2rust::src_loc = "358:5"]
    pub const SDL_SCANCODE_MAIL: C2RustUnnamed_2 = 265;
    #[c2rust::src_loc = "357:5"]
    pub const SDL_SCANCODE_WWW: C2RustUnnamed_2 = 264;
    #[c2rust::src_loc = "356:5"]
    pub const SDL_SCANCODE_MEDIASELECT: C2RustUnnamed_2 = 263;
    #[c2rust::src_loc = "355:5"]
    pub const SDL_SCANCODE_AUDIOMUTE: C2RustUnnamed_2 = 262;
    #[c2rust::src_loc = "354:5"]
    pub const SDL_SCANCODE_AUDIOPLAY: C2RustUnnamed_2 = 261;
    #[c2rust::src_loc = "353:5"]
    pub const SDL_SCANCODE_AUDIOSTOP: C2RustUnnamed_2 = 260;
    #[c2rust::src_loc = "352:5"]
    pub const SDL_SCANCODE_AUDIOPREV: C2RustUnnamed_2 = 259;
    #[c2rust::src_loc = "351:5"]
    pub const SDL_SCANCODE_AUDIONEXT: C2RustUnnamed_2 = 258;
    #[c2rust::src_loc = "337:5"]
    pub const SDL_SCANCODE_MODE: C2RustUnnamed_2 = 257;
    #[c2rust::src_loc = "335:5"]
    pub const SDL_SCANCODE_RGUI: C2RustUnnamed_2 = 231;
    #[c2rust::src_loc = "334:5"]
    pub const SDL_SCANCODE_RALT: C2RustUnnamed_2 = 230;
    #[c2rust::src_loc = "333:5"]
    pub const SDL_SCANCODE_RSHIFT: C2RustUnnamed_2 = 229;
    #[c2rust::src_loc = "332:5"]
    pub const SDL_SCANCODE_RCTRL: C2RustUnnamed_2 = 228;
    #[c2rust::src_loc = "331:5"]
    pub const SDL_SCANCODE_LGUI: C2RustUnnamed_2 = 227;
    #[c2rust::src_loc = "330:5"]
    pub const SDL_SCANCODE_LALT: C2RustUnnamed_2 = 226;
    #[c2rust::src_loc = "329:5"]
    pub const SDL_SCANCODE_LSHIFT: C2RustUnnamed_2 = 225;
    #[c2rust::src_loc = "328:5"]
    pub const SDL_SCANCODE_LCTRL: C2RustUnnamed_2 = 224;
    #[c2rust::src_loc = "326:5"]
    pub const SDL_SCANCODE_KP_HEXADECIMAL: C2RustUnnamed_2 = 221;
    #[c2rust::src_loc = "325:5"]
    pub const SDL_SCANCODE_KP_DECIMAL: C2RustUnnamed_2 = 220;
    #[c2rust::src_loc = "324:5"]
    pub const SDL_SCANCODE_KP_OCTAL: C2RustUnnamed_2 = 219;
    #[c2rust::src_loc = "323:5"]
    pub const SDL_SCANCODE_KP_BINARY: C2RustUnnamed_2 = 218;
    #[c2rust::src_loc = "322:5"]
    pub const SDL_SCANCODE_KP_CLEARENTRY: C2RustUnnamed_2 = 217;
    #[c2rust::src_loc = "321:5"]
    pub const SDL_SCANCODE_KP_CLEAR: C2RustUnnamed_2 = 216;
    #[c2rust::src_loc = "320:5"]
    pub const SDL_SCANCODE_KP_PLUSMINUS: C2RustUnnamed_2 = 215;
    #[c2rust::src_loc = "319:5"]
    pub const SDL_SCANCODE_KP_MEMDIVIDE: C2RustUnnamed_2 = 214;
    #[c2rust::src_loc = "318:5"]
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: C2RustUnnamed_2 = 213;
    #[c2rust::src_loc = "317:5"]
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: C2RustUnnamed_2 = 212;
    #[c2rust::src_loc = "316:5"]
    pub const SDL_SCANCODE_KP_MEMADD: C2RustUnnamed_2 = 211;
    #[c2rust::src_loc = "315:5"]
    pub const SDL_SCANCODE_KP_MEMCLEAR: C2RustUnnamed_2 = 210;
    #[c2rust::src_loc = "314:5"]
    pub const SDL_SCANCODE_KP_MEMRECALL: C2RustUnnamed_2 = 209;
    #[c2rust::src_loc = "313:5"]
    pub const SDL_SCANCODE_KP_MEMSTORE: C2RustUnnamed_2 = 208;
    #[c2rust::src_loc = "312:5"]
    pub const SDL_SCANCODE_KP_EXCLAM: C2RustUnnamed_2 = 207;
    #[c2rust::src_loc = "311:5"]
    pub const SDL_SCANCODE_KP_AT: C2RustUnnamed_2 = 206;
    #[c2rust::src_loc = "310:5"]
    pub const SDL_SCANCODE_KP_SPACE: C2RustUnnamed_2 = 205;
    #[c2rust::src_loc = "309:5"]
    pub const SDL_SCANCODE_KP_HASH: C2RustUnnamed_2 = 204;
    #[c2rust::src_loc = "308:5"]
    pub const SDL_SCANCODE_KP_COLON: C2RustUnnamed_2 = 203;
    #[c2rust::src_loc = "307:5"]
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: C2RustUnnamed_2 = 202;
    #[c2rust::src_loc = "306:5"]
    pub const SDL_SCANCODE_KP_VERTICALBAR: C2RustUnnamed_2 = 201;
    #[c2rust::src_loc = "305:5"]
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: C2RustUnnamed_2 = 200;
    #[c2rust::src_loc = "304:5"]
    pub const SDL_SCANCODE_KP_AMPERSAND: C2RustUnnamed_2 = 199;
    #[c2rust::src_loc = "303:5"]
    pub const SDL_SCANCODE_KP_GREATER: C2RustUnnamed_2 = 198;
    #[c2rust::src_loc = "302:5"]
    pub const SDL_SCANCODE_KP_LESS: C2RustUnnamed_2 = 197;
    #[c2rust::src_loc = "301:5"]
    pub const SDL_SCANCODE_KP_PERCENT: C2RustUnnamed_2 = 196;
    #[c2rust::src_loc = "300:5"]
    pub const SDL_SCANCODE_KP_POWER: C2RustUnnamed_2 = 195;
    #[c2rust::src_loc = "299:5"]
    pub const SDL_SCANCODE_KP_XOR: C2RustUnnamed_2 = 194;
    #[c2rust::src_loc = "298:5"]
    pub const SDL_SCANCODE_KP_F: C2RustUnnamed_2 = 193;
    #[c2rust::src_loc = "297:5"]
    pub const SDL_SCANCODE_KP_E: C2RustUnnamed_2 = 192;
    #[c2rust::src_loc = "296:5"]
    pub const SDL_SCANCODE_KP_D: C2RustUnnamed_2 = 191;
    #[c2rust::src_loc = "295:5"]
    pub const SDL_SCANCODE_KP_C: C2RustUnnamed_2 = 190;
    #[c2rust::src_loc = "294:5"]
    pub const SDL_SCANCODE_KP_B: C2RustUnnamed_2 = 189;
    #[c2rust::src_loc = "293:5"]
    pub const SDL_SCANCODE_KP_A: C2RustUnnamed_2 = 188;
    #[c2rust::src_loc = "292:5"]
    pub const SDL_SCANCODE_KP_BACKSPACE: C2RustUnnamed_2 = 187;
    #[c2rust::src_loc = "291:5"]
    pub const SDL_SCANCODE_KP_TAB: C2RustUnnamed_2 = 186;
    #[c2rust::src_loc = "290:5"]
    pub const SDL_SCANCODE_KP_RIGHTBRACE: C2RustUnnamed_2 = 185;
    #[c2rust::src_loc = "289:5"]
    pub const SDL_SCANCODE_KP_LEFTBRACE: C2RustUnnamed_2 = 184;
    #[c2rust::src_loc = "288:5"]
    pub const SDL_SCANCODE_KP_RIGHTPAREN: C2RustUnnamed_2 = 183;
    #[c2rust::src_loc = "287:5"]
    pub const SDL_SCANCODE_KP_LEFTPAREN: C2RustUnnamed_2 = 182;
    #[c2rust::src_loc = "286:5"]
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: C2RustUnnamed_2 = 181;
    #[c2rust::src_loc = "285:5"]
    pub const SDL_SCANCODE_CURRENCYUNIT: C2RustUnnamed_2 = 180;
    #[c2rust::src_loc = "284:5"]
    pub const SDL_SCANCODE_DECIMALSEPARATOR: C2RustUnnamed_2 = 179;
    #[c2rust::src_loc = "283:5"]
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: C2RustUnnamed_2 = 178;
    #[c2rust::src_loc = "282:5"]
    pub const SDL_SCANCODE_KP_000: C2RustUnnamed_2 = 177;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_SCANCODE_KP_00: C2RustUnnamed_2 = 176;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_SCANCODE_EXSEL: C2RustUnnamed_2 = 164;
    #[c2rust::src_loc = "278:5"]
    pub const SDL_SCANCODE_CRSEL: C2RustUnnamed_2 = 163;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_SCANCODE_CLEARAGAIN: C2RustUnnamed_2 = 162;
    #[c2rust::src_loc = "276:5"]
    pub const SDL_SCANCODE_OPER: C2RustUnnamed_2 = 161;
    #[c2rust::src_loc = "275:5"]
    pub const SDL_SCANCODE_OUT: C2RustUnnamed_2 = 160;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_SCANCODE_SEPARATOR: C2RustUnnamed_2 = 159;
    #[c2rust::src_loc = "273:5"]
    pub const SDL_SCANCODE_RETURN2: C2RustUnnamed_2 = 158;
    #[c2rust::src_loc = "272:5"]
    pub const SDL_SCANCODE_PRIOR: C2RustUnnamed_2 = 157;
    #[c2rust::src_loc = "271:5"]
    pub const SDL_SCANCODE_CLEAR: C2RustUnnamed_2 = 156;
    #[c2rust::src_loc = "270:5"]
    pub const SDL_SCANCODE_CANCEL: C2RustUnnamed_2 = 155;
    #[c2rust::src_loc = "269:5"]
    pub const SDL_SCANCODE_SYSREQ: C2RustUnnamed_2 = 154;
    #[c2rust::src_loc = "268:5"]
    pub const SDL_SCANCODE_ALTERASE: C2RustUnnamed_2 = 153;
    #[c2rust::src_loc = "266:5"]
    pub const SDL_SCANCODE_LANG9: C2RustUnnamed_2 = 152;
    #[c2rust::src_loc = "265:5"]
    pub const SDL_SCANCODE_LANG8: C2RustUnnamed_2 = 151;
    #[c2rust::src_loc = "264:5"]
    pub const SDL_SCANCODE_LANG7: C2RustUnnamed_2 = 150;
    #[c2rust::src_loc = "263:5"]
    pub const SDL_SCANCODE_LANG6: C2RustUnnamed_2 = 149;
    #[c2rust::src_loc = "262:5"]
    pub const SDL_SCANCODE_LANG5: C2RustUnnamed_2 = 148;
    #[c2rust::src_loc = "261:5"]
    pub const SDL_SCANCODE_LANG4: C2RustUnnamed_2 = 147;
    #[c2rust::src_loc = "260:5"]
    pub const SDL_SCANCODE_LANG3: C2RustUnnamed_2 = 146;
    #[c2rust::src_loc = "259:5"]
    pub const SDL_SCANCODE_LANG2: C2RustUnnamed_2 = 145;
    #[c2rust::src_loc = "258:5"]
    pub const SDL_SCANCODE_LANG1: C2RustUnnamed_2 = 144;
    #[c2rust::src_loc = "257:5"]
    pub const SDL_SCANCODE_INTERNATIONAL9: C2RustUnnamed_2 = 143;
    #[c2rust::src_loc = "256:5"]
    pub const SDL_SCANCODE_INTERNATIONAL8: C2RustUnnamed_2 = 142;
    #[c2rust::src_loc = "255:5"]
    pub const SDL_SCANCODE_INTERNATIONAL7: C2RustUnnamed_2 = 141;
    #[c2rust::src_loc = "254:5"]
    pub const SDL_SCANCODE_INTERNATIONAL6: C2RustUnnamed_2 = 140;
    #[c2rust::src_loc = "253:5"]
    pub const SDL_SCANCODE_INTERNATIONAL5: C2RustUnnamed_2 = 139;
    #[c2rust::src_loc = "252:5"]
    pub const SDL_SCANCODE_INTERNATIONAL4: C2RustUnnamed_2 = 138;
    #[c2rust::src_loc = "251:5"]
    pub const SDL_SCANCODE_INTERNATIONAL3: C2RustUnnamed_2 = 137;
    #[c2rust::src_loc = "250:5"]
    pub const SDL_SCANCODE_INTERNATIONAL2: C2RustUnnamed_2 = 136;
    #[c2rust::src_loc = "248:5"]
    pub const SDL_SCANCODE_INTERNATIONAL1: C2RustUnnamed_2 = 135;
    #[c2rust::src_loc = "246:5"]
    pub const SDL_SCANCODE_KP_EQUALSAS400: C2RustUnnamed_2 = 134;
    #[c2rust::src_loc = "245:5"]
    pub const SDL_SCANCODE_KP_COMMA: C2RustUnnamed_2 = 133;
    #[c2rust::src_loc = "240:5"]
    pub const SDL_SCANCODE_VOLUMEDOWN: C2RustUnnamed_2 = 129;
    #[c2rust::src_loc = "239:5"]
    pub const SDL_SCANCODE_VOLUMEUP: C2RustUnnamed_2 = 128;
    #[c2rust::src_loc = "238:5"]
    pub const SDL_SCANCODE_MUTE: C2RustUnnamed_2 = 127;
    #[c2rust::src_loc = "237:5"]
    pub const SDL_SCANCODE_FIND: C2RustUnnamed_2 = 126;
    #[c2rust::src_loc = "236:5"]
    pub const SDL_SCANCODE_PASTE: C2RustUnnamed_2 = 125;
    #[c2rust::src_loc = "235:5"]
    pub const SDL_SCANCODE_COPY: C2RustUnnamed_2 = 124;
    #[c2rust::src_loc = "234:5"]
    pub const SDL_SCANCODE_CUT: C2RustUnnamed_2 = 123;
    #[c2rust::src_loc = "233:5"]
    pub const SDL_SCANCODE_UNDO: C2RustUnnamed_2 = 122;
    #[c2rust::src_loc = "232:5"]
    pub const SDL_SCANCODE_AGAIN: C2RustUnnamed_2 = 121;
    #[c2rust::src_loc = "231:5"]
    pub const SDL_SCANCODE_STOP: C2RustUnnamed_2 = 120;
    #[c2rust::src_loc = "230:5"]
    pub const SDL_SCANCODE_SELECT: C2RustUnnamed_2 = 119;
    #[c2rust::src_loc = "229:5"]
    pub const SDL_SCANCODE_MENU: C2RustUnnamed_2 = 118;
    #[c2rust::src_loc = "228:5"]
    pub const SDL_SCANCODE_HELP: C2RustUnnamed_2 = 117;
    #[c2rust::src_loc = "227:5"]
    pub const SDL_SCANCODE_EXECUTE: C2RustUnnamed_2 = 116;
    #[c2rust::src_loc = "226:5"]
    pub const SDL_SCANCODE_F24: C2RustUnnamed_2 = 115;
    #[c2rust::src_loc = "225:5"]
    pub const SDL_SCANCODE_F23: C2RustUnnamed_2 = 114;
    #[c2rust::src_loc = "224:5"]
    pub const SDL_SCANCODE_F22: C2RustUnnamed_2 = 113;
    #[c2rust::src_loc = "223:5"]
    pub const SDL_SCANCODE_F21: C2RustUnnamed_2 = 112;
    #[c2rust::src_loc = "222:5"]
    pub const SDL_SCANCODE_F20: C2RustUnnamed_2 = 111;
    #[c2rust::src_loc = "221:5"]
    pub const SDL_SCANCODE_F19: C2RustUnnamed_2 = 110;
    #[c2rust::src_loc = "220:5"]
    pub const SDL_SCANCODE_F18: C2RustUnnamed_2 = 109;
    #[c2rust::src_loc = "219:5"]
    pub const SDL_SCANCODE_F17: C2RustUnnamed_2 = 108;
    #[c2rust::src_loc = "218:5"]
    pub const SDL_SCANCODE_F16: C2RustUnnamed_2 = 107;
    #[c2rust::src_loc = "217:5"]
    pub const SDL_SCANCODE_F15: C2RustUnnamed_2 = 106;
    #[c2rust::src_loc = "216:5"]
    pub const SDL_SCANCODE_F14: C2RustUnnamed_2 = 105;
    #[c2rust::src_loc = "215:5"]
    pub const SDL_SCANCODE_F13: C2RustUnnamed_2 = 104;
    #[c2rust::src_loc = "214:5"]
    pub const SDL_SCANCODE_KP_EQUALS: C2RustUnnamed_2 = 103;
    #[c2rust::src_loc = "211:5"]
    pub const SDL_SCANCODE_POWER: C2RustUnnamed_2 = 102;
    #[c2rust::src_loc = "210:5"]
    pub const SDL_SCANCODE_APPLICATION: C2RustUnnamed_2 = 101;
    #[c2rust::src_loc = "200:5"]
    pub const SDL_SCANCODE_NONUSBACKSLASH: C2RustUnnamed_2 = 100;
    #[c2rust::src_loc = "198:5"]
    pub const SDL_SCANCODE_KP_PERIOD: C2RustUnnamed_2 = 99;
    #[c2rust::src_loc = "197:5"]
    pub const SDL_SCANCODE_KP_0: C2RustUnnamed_2 = 98;
    #[c2rust::src_loc = "196:5"]
    pub const SDL_SCANCODE_KP_9: C2RustUnnamed_2 = 97;
    #[c2rust::src_loc = "195:5"]
    pub const SDL_SCANCODE_KP_8: C2RustUnnamed_2 = 96;
    #[c2rust::src_loc = "194:5"]
    pub const SDL_SCANCODE_KP_7: C2RustUnnamed_2 = 95;
    #[c2rust::src_loc = "193:5"]
    pub const SDL_SCANCODE_KP_6: C2RustUnnamed_2 = 94;
    #[c2rust::src_loc = "192:5"]
    pub const SDL_SCANCODE_KP_5: C2RustUnnamed_2 = 93;
    #[c2rust::src_loc = "191:5"]
    pub const SDL_SCANCODE_KP_4: C2RustUnnamed_2 = 92;
    #[c2rust::src_loc = "190:5"]
    pub const SDL_SCANCODE_KP_3: C2RustUnnamed_2 = 91;
    #[c2rust::src_loc = "189:5"]
    pub const SDL_SCANCODE_KP_2: C2RustUnnamed_2 = 90;
    #[c2rust::src_loc = "188:5"]
    pub const SDL_SCANCODE_KP_1: C2RustUnnamed_2 = 89;
    #[c2rust::src_loc = "187:5"]
    pub const SDL_SCANCODE_KP_ENTER: C2RustUnnamed_2 = 88;
    #[c2rust::src_loc = "186:5"]
    pub const SDL_SCANCODE_KP_PLUS: C2RustUnnamed_2 = 87;
    #[c2rust::src_loc = "185:5"]
    pub const SDL_SCANCODE_KP_MINUS: C2RustUnnamed_2 = 86;
    #[c2rust::src_loc = "184:5"]
    pub const SDL_SCANCODE_KP_MULTIPLY: C2RustUnnamed_2 = 85;
    #[c2rust::src_loc = "183:5"]
    pub const SDL_SCANCODE_KP_DIVIDE: C2RustUnnamed_2 = 84;
    #[c2rust::src_loc = "181:5"]
    pub const SDL_SCANCODE_NUMLOCKCLEAR: C2RustUnnamed_2 = 83;
    #[c2rust::src_loc = "179:5"]
    pub const SDL_SCANCODE_UP: C2RustUnnamed_2 = 82;
    #[c2rust::src_loc = "178:5"]
    pub const SDL_SCANCODE_DOWN: C2RustUnnamed_2 = 81;
    #[c2rust::src_loc = "177:5"]
    pub const SDL_SCANCODE_LEFT: C2RustUnnamed_2 = 80;
    #[c2rust::src_loc = "176:5"]
    pub const SDL_SCANCODE_RIGHT: C2RustUnnamed_2 = 79;
    #[c2rust::src_loc = "175:5"]
    pub const SDL_SCANCODE_PAGEDOWN: C2RustUnnamed_2 = 78;
    #[c2rust::src_loc = "174:5"]
    pub const SDL_SCANCODE_END: C2RustUnnamed_2 = 77;
    #[c2rust::src_loc = "173:5"]
    pub const SDL_SCANCODE_DELETE: C2RustUnnamed_2 = 76;
    #[c2rust::src_loc = "172:5"]
    pub const SDL_SCANCODE_PAGEUP: C2RustUnnamed_2 = 75;
    #[c2rust::src_loc = "171:5"]
    pub const SDL_SCANCODE_HOME: C2RustUnnamed_2 = 74;
    #[c2rust::src_loc = "169:5"]
    pub const SDL_SCANCODE_INSERT: C2RustUnnamed_2 = 73;
    #[c2rust::src_loc = "168:5"]
    pub const SDL_SCANCODE_PAUSE: C2RustUnnamed_2 = 72;
    #[c2rust::src_loc = "167:5"]
    pub const SDL_SCANCODE_SCROLLLOCK: C2RustUnnamed_2 = 71;
    #[c2rust::src_loc = "166:5"]
    pub const SDL_SCANCODE_PRINTSCREEN: C2RustUnnamed_2 = 70;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_SCANCODE_F12: C2RustUnnamed_2 = 69;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_SCANCODE_F11: C2RustUnnamed_2 = 68;
    #[c2rust::src_loc = "162:5"]
    pub const SDL_SCANCODE_F10: C2RustUnnamed_2 = 67;
    #[c2rust::src_loc = "161:5"]
    pub const SDL_SCANCODE_F9: C2RustUnnamed_2 = 66;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_SCANCODE_F8: C2RustUnnamed_2 = 65;
    #[c2rust::src_loc = "159:5"]
    pub const SDL_SCANCODE_F7: C2RustUnnamed_2 = 64;
    #[c2rust::src_loc = "158:5"]
    pub const SDL_SCANCODE_F6: C2RustUnnamed_2 = 63;
    #[c2rust::src_loc = "157:5"]
    pub const SDL_SCANCODE_F5: C2RustUnnamed_2 = 62;
    #[c2rust::src_loc = "156:5"]
    pub const SDL_SCANCODE_F4: C2RustUnnamed_2 = 61;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_SCANCODE_F3: C2RustUnnamed_2 = 60;
    #[c2rust::src_loc = "154:5"]
    pub const SDL_SCANCODE_F2: C2RustUnnamed_2 = 59;
    #[c2rust::src_loc = "153:5"]
    pub const SDL_SCANCODE_F1: C2RustUnnamed_2 = 58;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_SCANCODE_CAPSLOCK: C2RustUnnamed_2 = 57;
    #[c2rust::src_loc = "149:5"]
    pub const SDL_SCANCODE_SLASH: C2RustUnnamed_2 = 56;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_SCANCODE_PERIOD: C2RustUnnamed_2 = 55;
    #[c2rust::src_loc = "147:5"]
    pub const SDL_SCANCODE_COMMA: C2RustUnnamed_2 = 54;
    #[c2rust::src_loc = "130:5"]
    pub const SDL_SCANCODE_GRAVE: C2RustUnnamed_2 = 53;
    #[c2rust::src_loc = "129:5"]
    pub const SDL_SCANCODE_APOSTROPHE: C2RustUnnamed_2 = 52;
    #[c2rust::src_loc = "128:5"]
    pub const SDL_SCANCODE_SEMICOLON: C2RustUnnamed_2 = 51;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_SCANCODE_NONUSHASH: C2RustUnnamed_2 = 50;
    #[c2rust::src_loc = "102:5"]
    pub const SDL_SCANCODE_BACKSLASH: C2RustUnnamed_2 = 49;
    #[c2rust::src_loc = "101:5"]
    pub const SDL_SCANCODE_RIGHTBRACKET: C2RustUnnamed_2 = 48;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_SCANCODE_LEFTBRACKET: C2RustUnnamed_2 = 47;
    #[c2rust::src_loc = "99:5"]
    pub const SDL_SCANCODE_EQUALS: C2RustUnnamed_2 = 46;
    #[c2rust::src_loc = "98:5"]
    pub const SDL_SCANCODE_MINUS: C2RustUnnamed_2 = 45;
    #[c2rust::src_loc = "96:5"]
    pub const SDL_SCANCODE_SPACE: C2RustUnnamed_2 = 44;
    #[c2rust::src_loc = "95:5"]
    pub const SDL_SCANCODE_TAB: C2RustUnnamed_2 = 43;
    #[c2rust::src_loc = "94:5"]
    pub const SDL_SCANCODE_BACKSPACE: C2RustUnnamed_2 = 42;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_SCANCODE_ESCAPE: C2RustUnnamed_2 = 41;
    #[c2rust::src_loc = "92:5"]
    pub const SDL_SCANCODE_RETURN: C2RustUnnamed_2 = 40;
    #[c2rust::src_loc = "90:5"]
    pub const SDL_SCANCODE_0: C2RustUnnamed_2 = 39;
    #[c2rust::src_loc = "89:5"]
    pub const SDL_SCANCODE_9: C2RustUnnamed_2 = 38;
    #[c2rust::src_loc = "88:5"]
    pub const SDL_SCANCODE_8: C2RustUnnamed_2 = 37;
    #[c2rust::src_loc = "87:5"]
    pub const SDL_SCANCODE_7: C2RustUnnamed_2 = 36;
    #[c2rust::src_loc = "86:5"]
    pub const SDL_SCANCODE_6: C2RustUnnamed_2 = 35;
    #[c2rust::src_loc = "85:5"]
    pub const SDL_SCANCODE_5: C2RustUnnamed_2 = 34;
    #[c2rust::src_loc = "84:5"]
    pub const SDL_SCANCODE_4: C2RustUnnamed_2 = 33;
    #[c2rust::src_loc = "83:5"]
    pub const SDL_SCANCODE_3: C2RustUnnamed_2 = 32;
    #[c2rust::src_loc = "82:5"]
    pub const SDL_SCANCODE_2: C2RustUnnamed_2 = 31;
    #[c2rust::src_loc = "81:5"]
    pub const SDL_SCANCODE_1: C2RustUnnamed_2 = 30;
    #[c2rust::src_loc = "79:5"]
    pub const SDL_SCANCODE_Z: C2RustUnnamed_2 = 29;
    #[c2rust::src_loc = "78:5"]
    pub const SDL_SCANCODE_Y: C2RustUnnamed_2 = 28;
    #[c2rust::src_loc = "77:5"]
    pub const SDL_SCANCODE_X: C2RustUnnamed_2 = 27;
    #[c2rust::src_loc = "76:5"]
    pub const SDL_SCANCODE_W: C2RustUnnamed_2 = 26;
    #[c2rust::src_loc = "75:5"]
    pub const SDL_SCANCODE_V: C2RustUnnamed_2 = 25;
    #[c2rust::src_loc = "74:5"]
    pub const SDL_SCANCODE_U: C2RustUnnamed_2 = 24;
    #[c2rust::src_loc = "73:5"]
    pub const SDL_SCANCODE_T: C2RustUnnamed_2 = 23;
    #[c2rust::src_loc = "72:5"]
    pub const SDL_SCANCODE_S: C2RustUnnamed_2 = 22;
    #[c2rust::src_loc = "71:5"]
    pub const SDL_SCANCODE_R: C2RustUnnamed_2 = 21;
    #[c2rust::src_loc = "70:5"]
    pub const SDL_SCANCODE_Q: C2RustUnnamed_2 = 20;
    #[c2rust::src_loc = "69:5"]
    pub const SDL_SCANCODE_P: C2RustUnnamed_2 = 19;
    #[c2rust::src_loc = "68:5"]
    pub const SDL_SCANCODE_O: C2RustUnnamed_2 = 18;
    #[c2rust::src_loc = "67:5"]
    pub const SDL_SCANCODE_N: C2RustUnnamed_2 = 17;
    #[c2rust::src_loc = "66:5"]
    pub const SDL_SCANCODE_M: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "65:5"]
    pub const SDL_SCANCODE_L: C2RustUnnamed_2 = 15;
    #[c2rust::src_loc = "64:5"]
    pub const SDL_SCANCODE_K: C2RustUnnamed_2 = 14;
    #[c2rust::src_loc = "63:5"]
    pub const SDL_SCANCODE_J: C2RustUnnamed_2 = 13;
    #[c2rust::src_loc = "62:5"]
    pub const SDL_SCANCODE_I: C2RustUnnamed_2 = 12;
    #[c2rust::src_loc = "61:5"]
    pub const SDL_SCANCODE_H: C2RustUnnamed_2 = 11;
    #[c2rust::src_loc = "60:5"]
    pub const SDL_SCANCODE_G: C2RustUnnamed_2 = 10;
    #[c2rust::src_loc = "59:5"]
    pub const SDL_SCANCODE_F: C2RustUnnamed_2 = 9;
    #[c2rust::src_loc = "58:5"]
    pub const SDL_SCANCODE_E: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "57:5"]
    pub const SDL_SCANCODE_D: C2RustUnnamed_2 = 7;
    #[c2rust::src_loc = "56:5"]
    pub const SDL_SCANCODE_C: C2RustUnnamed_2 = 6;
    #[c2rust::src_loc = "55:5"]
    pub const SDL_SCANCODE_B: C2RustUnnamed_2 = 5;
    #[c2rust::src_loc = "54:5"]
    pub const SDL_SCANCODE_A: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "45:5"]
    pub const SDL_SCANCODE_UNKNOWN: C2RustUnnamed_2 = 0;
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/pcrlib.h:22"]
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
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "290:8"]
    pub struct scores {
        pub score: sdword,
        pub level: sword,
        pub initials: [libc::c_char; 4],
    }
    use super::catdefs_h::{boolean, dirtype, sdword, sword};
    extern "C" {
        #[c2rust::src_loc = "324:16"]
        pub static mut GODMODE: boolean;
        #[c2rust::src_loc = "310:1"]
        pub fn bioskey(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "297:33"]
        pub static mut highscores: [scores; 5];
        #[c2rust::src_loc = "273:14"]
        pub static mut level: sword;
        #[c2rust::src_loc = "272:15"]
        pub static mut score: sdword;
        #[c2rust::src_loc = "252:1"]
        pub fn centerwindow(width: libc::c_int, height: libc::c_int);
        #[c2rust::src_loc = "248:1"]
        pub fn printlong(val: libc::c_long);
        #[c2rust::src_loc = "247:1"]
        pub fn printint(val: libc::c_int);
        #[c2rust::src_loc = "242:1"]
        pub fn print(str: *const libc::c_char);
        #[c2rust::src_loc = "241:1"]
        pub fn _inputint() -> libc::c_uint;
        #[c2rust::src_loc = "239:1"]
        pub fn get() -> libc::c_int;
        #[c2rust::src_loc = "236:12"]
        pub static mut screencenterx: libc::c_int;
        #[c2rust::src_loc = "226:1"]
        pub fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
        #[c2rust::src_loc = "179:1"]
        pub fn WaitVBL();
        #[c2rust::src_loc = "178:1"]
        pub fn UpdateScreen();
        #[c2rust::src_loc = "175:15"]
        pub static mut sy: libc::c_int;
        #[c2rust::src_loc = "175:12"]
        pub static mut sx: libc::c_int;
        #[c2rust::src_loc = "147:1"]
        pub fn clearkeys();
        #[c2rust::src_loc = "144:1"]
        pub fn rndt() -> libc::c_int;
        #[c2rust::src_loc = "142:1"]
        pub fn initrndt(randomize: boolean);
        #[c2rust::src_loc = "128:1"]
        pub fn RecordDemo();
        #[c2rust::src_loc = "126:1"]
        pub fn SaveDemo(demonum: libc::c_int);
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
        #[c2rust::src_loc = "29:13"]
        pub static mut ch: libc::c_char;
    }
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catacomb.h:22"]
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
    use super::catdefs_h::{
        activeobj, boolean, byte, dirtype, north, objdeftype, objtype, sbyte, sword, word,
    };
    use super::pcrlib_h::ControlStruct;
    extern "C" {
        #[c2rust::src_loc = "91:27"]
        pub static mut resetgame: boolean;
        #[c2rust::src_loc = "94:24"]
        pub static mut ctrl: ControlStruct;
        #[c2rust::src_loc = "91:18"]
        pub static mut exitdemo: boolean;
        #[c2rust::src_loc = "68:14"]
        pub static mut side: libc::c_int;
        #[c2rust::src_loc = "58:18"]
        pub static mut opposite: [dirtype; 9];
        #[c2rust::src_loc = "63:14"]
        pub static mut originx: libc::c_int;
        #[c2rust::src_loc = "63:23"]
        pub static mut originy: libc::c_int;
        #[c2rust::src_loc = "76:21"]
        pub static mut objdef: [objdeftype; 23];
        #[c2rust::src_loc = "52:19"]
        pub static mut gamexit: exittype;
        #[c2rust::src_loc = "72:24"]
        pub static mut altobj: objtype;
        #[c2rust::src_loc = "73:14"]
        pub static mut altnum: libc::c_int;
        #[c2rust::src_loc = "66:16"]
        pub static mut items: [sword; 6];
        #[c2rust::src_loc = "62:14"]
        pub static mut view: [[libc::c_int; 86]; 87];
        #[c2rust::src_loc = "83:24"]
        pub static mut chkspot: libc::c_int;
        #[c2rust::src_loc = "83:14"]
        pub static mut chkx: libc::c_int;
        #[c2rust::src_loc = "83:19"]
        pub static mut chky: libc::c_int;
        #[c2rust::src_loc = "61:14"]
        pub static mut background: [[libc::c_int; 86]; 87];
        #[c2rust::src_loc = "74:14"]
        pub static mut numobj: libc::c_int;
        #[c2rust::src_loc = "47:1"]
        pub fn eraseobj();
        #[c2rust::src_loc = "48:1"]
        pub fn drawobj();
        #[c2rust::src_loc = "72:18"]
        pub static mut obj: objtype;
        #[c2rust::src_loc = "71:20"]
        pub static mut o: [activeobj; 201];
        #[c2rust::src_loc = "74:21"]
        pub static mut objecton: libc::c_int;
        #[c2rust::src_loc = "33:1"]
        pub fn restore();
        #[c2rust::src_loc = "30:1"]
        pub fn dofkeys();
        #[c2rust::src_loc = "31:1"]
        pub fn loadlevel();
        #[c2rust::src_loc = "78:28"]
        pub static mut leveldone: boolean;
        #[c2rust::src_loc = "85:15"]
        pub static mut frameon: word;
        #[c2rust::src_loc = "69:14"]
        pub static mut boltsleft: libc::c_int;
        #[c2rust::src_loc = "54:15"]
        pub static mut altmeters: [[libc::c_char; 14]; 14];
        #[c2rust::src_loc = "56:15"]
        pub static mut meters: [[libc::c_char; 14]; 14];
        #[c2rust::src_loc = "67:14"]
        pub static mut shotpower: libc::c_int;
        #[c2rust::src_loc = "49:1"]
        pub fn doall();
        #[c2rust::src_loc = "32:1"]
        pub fn clearold();
        #[c2rust::src_loc = "78:18"]
        pub static mut playdone: boolean;
        #[c2rust::src_loc = "27:1"]
        pub fn refresh();
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "840:12"]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:22"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::catacomb_h::{
    altmeters, altnum, altobj, background, boltsleft, chkspot, chkx, chky, clearold, ctrl, doall,
    dofkeys, drawobj, eraseobj, exitdemo, exittype, frameon, gamexit, items, killed, leveldone,
    loadlevel, meters, numobj, o, obj, objdef, objecton, opposite, originx, originy, playdone,
    quited, refresh, reseted, resetgame, restore, shotpower, side, victorious, view,
};
pub use self::catdefs_h::{
    activeobj, benign, bigshot, boolean, byte, classtype, dead1, dead2, dead3, dead4, dead5, dead6,
    dirtype, dragon, dragoncmd, east, explode, fade, false_0, gargcmd, gargoyle, goblin, gune,
    guns, gunthinke, gunthinks, idle, lastclass, monster, mshot, nodir, north, northeast,
    northwest, nothing, nukeshot, objdeftype, objtype, ogre, player, playercmd, pshot, ramdiag,
    ramstraight, rock, sbyte, sdword, secretgate, shot, skeleton, south, southeast, southwest,
    straight, sword, teleporter, torch, true_0, turbogre, wallhit, west, word, C2RustUnnamed,
    C2RustUnnamed_0, C2RustUnnamed_1,
};
pub use self::pcrlib_h::{
    _inputint, bioskey, centerwindow, ch, clearkeys, demoenum, demoplay, drawchar, get, highscores,
    indemo, initrndt, keydown, level, notdemo, print, printint, printlong, recording, rndt, score,
    scores, screencenterx, sx, sy, ControlPlayer, ControlStruct, PlaySound, RecordDemo, SaveDemo,
    UpdateScreen, WaitEndSound, WaitVBL, GODMODE,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint8_t};
use self::stdlib_h::{abs, atoi};
use self::string_h::memcpy;
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint16_t, __uint8_t};
pub use self::SDL_scancode_h::{
    C2RustUnnamed_2, SDL_NUM_SCANCODES, SDL_SCANCODE_0, SDL_SCANCODE_1, SDL_SCANCODE_2,
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
#[no_mangle]
#[c2rust::src_loc = "32:1"]
pub unsafe extern "C" fn newobject() -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    loop {
        if !(i <= numobj) {
            current_block = 12675440807659640239;
            break;
        }
        if o[i as usize].class as libc::c_int == nothing as libc::c_int {
            current_block = 2118292289288785552;
            break;
        }
        i += 1;
    }
    match current_block {
        12675440807659640239 => {
            if numobj < 200 as libc::c_int {
                numobj += 1;
            }
            i = numobj;
        }
        _ => {}
    }
    o[i as usize].oldtile = -(1 as libc::c_int) as sword;
    o[i as usize].oldx = 0 as libc::c_int as byte;
    o[i as usize].oldy = 0 as libc::c_int as byte;
    return i;
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn printscore() {
    sx = 31 as libc::c_int;
    sy = 3 as libc::c_int;
    printlong(score as libc::c_long);
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn printhighscore() {
    sx = 31 as libc::c_int;
    sy = 5 as libc::c_int;
    printlong(highscores[1 as libc::c_int as usize].score as libc::c_long);
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn printshotpower() {
    sx = 25 as libc::c_int;
    sy = 13 as libc::c_int;
    if shotpower == 13 as libc::c_int {
        print((altmeters[13 as libc::c_int as usize]).as_mut_ptr());
    } else {
        print((meters[shotpower as usize]).as_mut_ptr());
    };
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn printbody() {
    sx = 25 as libc::c_int;
    sy = 16 as libc::c_int;
    if o[0 as libc::c_int as usize].hp as libc::c_int > 6 as libc::c_int {
        print((meters[o[0 as libc::c_int as usize].hp as usize]).as_mut_ptr());
    } else {
        print((altmeters[o[0 as libc::c_int as usize].hp as usize]).as_mut_ptr());
    };
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn levelcleared() {
    let mut warp: [libc::c_char; 3] = [0; 3];
    let mut value: libc::c_int = 0;
    leveldone = true_0 as libc::c_int as boolean;
    warp[0 as libc::c_int as usize] = (background
        [(altobj.y as libc::c_int + 2 as libc::c_int) as usize][altobj.x as usize]
        as libc::c_char as libc::c_int
        - 161 as libc::c_int) as libc::c_char;
    if (warp[0 as libc::c_int as usize] as libc::c_int) < '0' as i32
        || warp[0 as libc::c_int as usize] as libc::c_int > '9' as i32
    {
        warp[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    }
    warp[1 as libc::c_int as usize] = (background
        [(altobj.y as libc::c_int + 2 as libc::c_int) as usize]
        [(altobj.x as libc::c_int + 1 as libc::c_int) as usize]
        as libc::c_char as libc::c_int
        - 161 as libc::c_int) as libc::c_char;
    if (warp[1 as libc::c_int as usize] as libc::c_int) < '0' as i32
        || warp[1 as libc::c_int as usize] as libc::c_int > '9' as i32
    {
        warp[2 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    }
    value = atoi(warp.as_mut_ptr());
    if value > 0 as libc::c_int {
        level = value as sword;
    } else {
        level += 1;
    }
    if level as libc::c_int > 30 as libc::c_int {
        playdone = true_0 as libc::c_int as boolean;
        gamexit = victorious;
    }
}
#[no_mangle]
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn givekey() {
    let mut i: libc::c_int = 0;
    i = items[1 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int;
    items[1 as libc::c_int as usize] = i as sword;
    if i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 7 as libc::c_int, 31 as libc::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "163:1"]
pub unsafe extern "C" fn givepotion() {
    let mut i: libc::c_int = 0;
    i = items[2 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int;
    items[2 as libc::c_int as usize] = i as sword;
    if i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 8 as libc::c_int, 29 as libc::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn givebolt() {
    let mut i: libc::c_int = 0;
    i = items[3 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int;
    items[3 as libc::c_int as usize] = i as sword;
    if i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 9 as libc::c_int, 30 as libc::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn givenuke() {
    let mut i: libc::c_int = 0;
    i = items[5 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int;
    items[5 as libc::c_int as usize] = i as sword;
    if i < 11 as libc::c_int {
        drawchar(26 as libc::c_int + i, 10 as libc::c_int, 30 as libc::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn takekey() -> boolean {
    let mut i: libc::c_int = 0;
    if items[1 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        i = items[1 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        items[1 as libc::c_int as usize] = i as sword;
        if i < 10 as libc::c_int {
            drawchar(27 as libc::c_int + i, 7 as libc::c_int, 32 as libc::c_int);
        }
        PlaySound(11 as libc::c_int);
        return true_0 as libc::c_int as boolean;
    } else {
        PlaySound(14 as libc::c_int);
        return false_0 as libc::c_int as boolean;
    };
}
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn takepotion() {
    let mut i: libc::c_int = 0;
    if items[2 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        i = items[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        items[2 as libc::c_int as usize] = i as sword;
        if i < 11 as libc::c_int {
            drawchar(27 as libc::c_int + i, 8 as libc::c_int, 32 as libc::c_int);
        }
        PlaySound(12 as libc::c_int);
        o[0 as libc::c_int as usize].hp = 13 as libc::c_int as sbyte;
        obj.hp = 13 as libc::c_int as sbyte;
        printbody();
    } else {
        PlaySound(14 as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "247:1"]
pub unsafe extern "C" fn castbolt() {
    let mut i: libc::c_int = 0;
    if items[3 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        i = items[3 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        items[3 as libc::c_int as usize] = i as sword;
        if i < 11 as libc::c_int {
            drawchar(27 as libc::c_int + i, 9 as libc::c_int, 32 as libc::c_int);
        }
        boltsleft = 8 as libc::c_int;
        PlaySound(13 as libc::c_int);
    } else {
        PlaySound(14 as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "270:1"]
pub unsafe extern "C" fn castnuke() {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut base: activeobj = activeobj {
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
    };
    if items[5 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
        PlaySound(14 as libc::c_int);
        return;
    }
    i = items[5 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
    items[5 as libc::c_int as usize] = i as sword;
    if i < 11 as libc::c_int {
        drawchar(27 as libc::c_int + i, 10 as libc::c_int, 32 as libc::c_int);
    }
    base.delay = 0 as libc::c_int as byte;
    base.stage = 0 as libc::c_int as byte;
    base.active = true_0 as libc::c_int as boolean;
    base.x = obj.x;
    base.y = obj.y;
    base.oldx = base.x;
    base.oldy = base.y;
    base.oldtile = -(1 as libc::c_int) as sword;
    base.class = bigshot as libc::c_int as word;
    x = -(1 as libc::c_int);
    while x <= 1 as libc::c_int {
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as libc::c_int + x * 2 as libc::c_int) as byte;
        o[n as usize].dir = north as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as libc::c_int + x * 2 as libc::c_int) as byte;
        o[n as usize].dir = south as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as libc::c_int + x * 2 as libc::c_int) as byte;
        o[n as usize].dir = east as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as libc::c_int + x * 2 as libc::c_int) as byte;
        o[n as usize].dir = west as libc::c_int as word;
        x += 1;
    }
    PlaySound(13 as libc::c_int);
    obj.stage = 2 as libc::c_int as byte;
    obj.delay = 4 as libc::c_int as byte;
}
#[no_mangle]
#[c2rust::src_loc = "332:1"]
pub unsafe extern "C" fn playshoot() {
    let mut new: libc::c_int = 0;
    obj.stage = 2 as libc::c_int as byte;
    obj.delay = 4 as libc::c_int as byte;
    PlaySound(5 as libc::c_int);
    new = newobject();
    o[new as usize].class = shot as libc::c_int as word;
    side ^= 1 as libc::c_int;
    o[new as usize].delay = 0 as libc::c_int as byte;
    o[new as usize].stage = 0 as libc::c_int as byte;
    o[new as usize].active = true_0 as libc::c_int as boolean;
    o[new as usize].dir = obj.dir;
    match o[new as usize].dir as libc::c_int {
        0 => {
            o[new as usize].x = (obj.x as libc::c_int + side) as byte;
            o[new as usize].y = obj.y;
        }
        1 => {
            o[new as usize].x = (obj.x as libc::c_int + 1 as libc::c_int) as byte;
            o[new as usize].y = (obj.y as libc::c_int + side) as byte;
        }
        2 => {
            o[new as usize].x = (obj.x as libc::c_int + side) as byte;
            o[new as usize].y = (obj.y as libc::c_int + 1 as libc::c_int) as byte;
        }
        3 => {
            o[new as usize].x = obj.x;
            o[new as usize].y = (obj.y as libc::c_int + side) as byte;
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "371:1"]
pub unsafe extern "C" fn playbigshoot() {
    let mut new: libc::c_int = 0;
    obj.stage = 2 as libc::c_int as byte;
    if boltsleft == 0 as libc::c_int {
        obj.delay = 4 as libc::c_int as byte;
    }
    PlaySound(4 as libc::c_int);
    new = newobject();
    o[new as usize].delay = 0 as libc::c_int as byte;
    o[new as usize].stage = 0 as libc::c_int as byte;
    o[new as usize].active = true_0 as libc::c_int as boolean;
    o[new as usize].dir = obj.dir;
    o[new as usize].x = obj.x;
    o[new as usize].y = obj.y;
    o[new as usize].class = bigshot as libc::c_int as word;
}
#[no_mangle]
#[c2rust::src_loc = "397:1"]
pub unsafe extern "C" fn givescroll() {
    if rndt() < 128 as libc::c_int {
        givebolt();
    } else {
        givenuke();
    };
}
#[no_mangle]
#[c2rust::src_loc = "413:1"]
pub unsafe extern "C" fn opendoor() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    PlaySound(11 as libc::c_int);
    x = chkx;
    y = chky;
    if chkspot == 165 as libc::c_int {
        loop {
            view[y as usize][x as usize] = 128 as libc::c_int;
            background[y as usize][x as usize] = 128 as libc::c_int;
            y -= 1;
            if !(view[y as usize][x as usize] == 165 as libc::c_int) {
                break;
            }
        }
        y = chky + 1 as libc::c_int;
        while view[y as usize][x as usize] == 165 as libc::c_int {
            view[y as usize][x as usize] = 128 as libc::c_int;
            background[y as usize][x as usize] = 128 as libc::c_int;
            y += 1;
        }
    } else {
        loop {
            view[y as usize][x as usize] = 128 as libc::c_int;
            background[y as usize][x as usize] = 128 as libc::c_int;
            x -= 1;
            if !(view[y as usize][x as usize] == 166 as libc::c_int) {
                break;
            }
        }
        x = chkx + 1 as libc::c_int;
        while view[y as usize][x as usize] == 166 as libc::c_int {
            view[y as usize][x as usize] = 128 as libc::c_int;
            background[y as usize][x as usize] = 128 as libc::c_int;
            x += 1;
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "468:1"]
pub unsafe extern "C" fn tagobject() {
    let mut i: libc::c_int = altobj.hp as libc::c_int;
    if GODMODE as libc::c_int != 0 && altobj.class as libc::c_int == player as libc::c_int {
        return;
    }
    altobj.hp = (altobj.hp as libc::c_int - obj.damage as libc::c_int) as sbyte;
    if i <= obj.damage as libc::c_int {
        if altobj.class as libc::c_int == player as libc::c_int {
            o[0 as libc::c_int as usize].hp = 0 as libc::c_int as sbyte;
            altobj.hp = o[0 as libc::c_int as usize].hp;
            printbody();
            PlaySound(10 as libc::c_int);
            playdone = true_0 as libc::c_int as boolean;
            gamexit = killed;
        } else {
            score = score + altobj.points as libc::c_int;
            printscore();
            PlaySound(9 as libc::c_int);
        }
        o[altnum as usize].class = (dead1 as libc::c_int - 1 as libc::c_int
            + altobj.size as libc::c_int) as classtype as word;
        o[altnum as usize].delay = 2 as libc::c_int as byte;
        o[altnum as usize].stage = 0 as libc::c_int as byte;
    } else {
        if o[altnum as usize].class as libc::c_int == guns as libc::c_int
            || o[altnum as usize].class as libc::c_int == gune as libc::c_int
        {
            return;
        }
        o[altnum as usize].hp = altobj.hp;
        o[altnum as usize].stage = 3 as libc::c_int as byte;
        if altnum == 0 as libc::c_int {
            o[0 as libc::c_int as usize].delay = 2 as libc::c_int as byte;
            printbody();
            PlaySound(8 as libc::c_int);
        } else {
            o[altnum as usize].delay = 4 as libc::c_int as byte;
            PlaySound(7 as libc::c_int);
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "537:1"]
pub unsafe extern "C" fn intomonster() -> boolean {
    let mut gotit: boolean = 0;
    altnum = 0 as libc::c_int;
    gotit = false_0 as libc::c_int as boolean;
    loop {
        memcpy(
            &mut altobj.active as *mut boolean as *mut libc::c_void,
            &mut *o.as_mut_ptr().offset(altnum as isize) as *mut activeobj as *const libc::c_void,
            ::core::mem::size_of::<activeobj>() as libc::c_ulong,
        );
        if altobj.class as libc::c_int > nothing as libc::c_int && altnum != objecton {
            memcpy(
                &mut altobj.think as *mut byte as *mut libc::c_void,
                &mut *objdef.as_mut_ptr().offset(altobj.class as isize) as *mut objdeftype
                    as *const libc::c_void,
                ::core::mem::size_of::<objdeftype>() as libc::c_ulong,
            );
            if chkx >= altobj.x as libc::c_int
                && (chkx - altobj.x as libc::c_int) < altobj.size as libc::c_int
                && chky >= altobj.y as libc::c_int
                && (chky - altobj.y as libc::c_int) < altobj.size as libc::c_int
            {
                if altobj.solid != 0 {
                    gotit = true_0 as libc::c_int as boolean;
                } else if objecton == 0 as libc::c_int
                    && (altobj.class as libc::c_int == teleporter as libc::c_int
                        || altobj.class as libc::c_int == secretgate as libc::c_int)
                {
                    levelcleared();
                }
            }
        }
        if gotit == 0 {
            altnum += 1;
        }
        if gotit as libc::c_int != 0 || altnum > numobj {
            break;
        }
    }
    if gotit == 0 {
        return true_0 as libc::c_int as boolean;
    }
    match obj.contact as libc::c_int {
        0 => return false_0 as libc::c_int as boolean,
        1 | 3 => {
            if altnum == 0 as libc::c_int {
                tagobject();
                obj.stage = 2 as libc::c_int as byte;
                obj.delay = 20 as libc::c_int as byte;
            } else if altobj.class as libc::c_int == shot as libc::c_int {
                return true_0 as libc::c_int as boolean;
            }
            return false_0 as libc::c_int as boolean;
        }
        2 => {
            if altnum > 0 as libc::c_int {
                tagobject();
            }
            return false_0 as libc::c_int as boolean;
        }
        4 => {
            tagobject();
            return true_0 as libc::c_int as boolean;
        }
        _ => {}
    }
    return false_0 as libc::c_int as boolean;
}
#[no_mangle]
#[c2rust::src_loc = "616:1"]
pub unsafe extern "C" fn walkthrough() -> boolean {
    let mut new: libc::c_int = 0;
    if chkspot == 128 as libc::c_int {
        return true_0 as libc::c_int as boolean;
    }
    if chkspot >= 256 as libc::c_int
        && chkspot
            <= 256 as libc::c_int
                + 67 as libc::c_int * 4 as libc::c_int
                + 35 as libc::c_int * 9 as libc::c_int
                + 19 as libc::c_int * 16 as libc::c_int
                + 19 as libc::c_int * 25 as libc::c_int
    {
        return intomonster();
    }
    if chkspot >= 129 as libc::c_int && chkspot <= 135 as libc::c_int {
        if obj.contact as libc::c_int == pshot as libc::c_int
            || obj.contact as libc::c_int == nukeshot as libc::c_int
            || obj.contact as libc::c_int == mshot as libc::c_int
        {
            new = newobject();
            o[new as usize].active = true_0 as libc::c_int as boolean;
            o[new as usize].x = chkx as byte;
            o[new as usize].y = chky as byte;
            o[new as usize].stage = 0 as libc::c_int as byte;
            o[new as usize].delay = 2 as libc::c_int as byte;
            o[new as usize].class = wallhit as libc::c_int as word;
            PlaySound(6 as libc::c_int);
        }
        return false_0 as libc::c_int as boolean;
    }
    if chkspot >= 136 as libc::c_int && chkspot <= 145 as libc::c_int {
        if obj.contact as libc::c_int == pshot as libc::c_int
            || obj.contact as libc::c_int == nukeshot as libc::c_int
        {
            PlaySound(6 as libc::c_int);
            if chkspot < 143 as libc::c_int {
                background[chky as usize][chkx as usize] = 128 as libc::c_int;
            } else {
                background[chky as usize][chkx as usize] = chkspot + 19 as libc::c_int;
            }
            new = newobject();
            o[new as usize].active = true_0 as libc::c_int as boolean;
            o[new as usize].x = chkx as byte;
            o[new as usize].y = chky as byte;
            o[new as usize].stage = 0 as libc::c_int as byte;
            o[new as usize].delay = 2 as libc::c_int as byte;
            o[new as usize].class = dead1 as libc::c_int as word;
            if obj.contact as libc::c_int == pshot as libc::c_int {
                return false_0 as libc::c_int as boolean;
            } else {
                return true_0 as libc::c_int as boolean;
            }
        } else {
            return false_0 as libc::c_int as boolean;
        }
    }
    if chkspot == 162 as libc::c_int {
        if obj.class as libc::c_int == player as libc::c_int {
            givepotion();
            view[chky as usize][chkx as usize] = 128 as libc::c_int;
            background[chky as usize][chkx as usize] = 128 as libc::c_int;
            PlaySound(2 as libc::c_int);
        }
        return true_0 as libc::c_int as boolean;
    }
    if chkspot == 163 as libc::c_int {
        if obj.class as libc::c_int == player as libc::c_int {
            givescroll();
            view[chky as usize][chkx as usize] = 128 as libc::c_int;
            background[chky as usize][chkx as usize] = 128 as libc::c_int;
            PlaySound(2 as libc::c_int);
        }
        return true_0 as libc::c_int as boolean;
    }
    if chkspot == 164 as libc::c_int {
        if obj.class as libc::c_int == player as libc::c_int {
            givekey();
            view[chky as usize][chkx as usize] = 128 as libc::c_int;
            background[chky as usize][chkx as usize] = 128 as libc::c_int;
            PlaySound(2 as libc::c_int);
        }
        return true_0 as libc::c_int as boolean;
    }
    if chkspot == 165 as libc::c_int || chkspot == 166 as libc::c_int {
        if obj.class as libc::c_int == player as libc::c_int {
            if takekey() != 0 {
                opendoor();
                return true_0 as libc::c_int as boolean;
            }
        }
        return false_0 as libc::c_int as boolean;
    }
    if chkspot == 167 as libc::c_int {
        if obj.class as libc::c_int == player as libc::c_int {
            score += 500 as libc::c_int;
            printscore();
            background[chky as usize][chkx as usize] = 128 as libc::c_int;
            view[chky as usize][chkx as usize] = 128 as libc::c_int;
            PlaySound(3 as libc::c_int);
        }
        return true_0 as libc::c_int as boolean;
    }
    if chkspot >= 29 as libc::c_int && chkspot <= 31 as libc::c_int {
        return true_0 as libc::c_int as boolean;
    }
    return false_0 as libc::c_int as boolean;
}
#[no_mangle]
#[c2rust::src_loc = "766:1"]
pub unsafe extern "C" fn walk() -> boolean {
    let mut i: libc::c_int = 0;
    let mut newx: libc::c_int = 0;
    let mut newy: libc::c_int = 0;
    let mut deltay: libc::c_int = 0;
    let mut deltax: libc::c_int = 0;
    let mut try_0: boolean = 0;
    match obj.dir as libc::c_int {
        0 => {
            newx = obj.x as libc::c_int;
            newy = obj.y as libc::c_int - 1 as libc::c_int;
            chkx = newx;
            chky = newy;
            deltax = 1 as libc::c_int;
            deltay = 0 as libc::c_int;
        }
        1 => {
            newx = obj.x as libc::c_int + 1 as libc::c_int;
            newy = obj.y as libc::c_int;
            chkx = obj.x as libc::c_int + obj.size as libc::c_int;
            chky = newy;
            deltax = 0 as libc::c_int;
            deltay = 1 as libc::c_int;
        }
        2 => {
            newx = obj.x as libc::c_int;
            newy = obj.y as libc::c_int + 1 as libc::c_int;
            chkx = newx;
            chky = obj.y as libc::c_int + obj.size as libc::c_int;
            deltax = 1 as libc::c_int;
            deltay = 0 as libc::c_int;
        }
        3 => {
            newx = obj.x as libc::c_int - 1 as libc::c_int;
            newy = obj.y as libc::c_int;
            chkx = newx;
            chky = newy;
            deltax = 0 as libc::c_int;
            deltay = 1 as libc::c_int;
        }
        _ => return false_0 as libc::c_int as boolean,
    }
    i = 1 as libc::c_int;
    while i <= obj.size as libc::c_int {
        chkspot = view[chky as usize][chkx as usize];
        if chkspot != 128 as libc::c_int {
            try_0 = walkthrough();
            if leveldone != 0 {
                return true_0 as libc::c_int as boolean;
            }
            if obj.stage as libc::c_int == 2 as libc::c_int {
                return true_0 as libc::c_int as boolean;
            }
            if try_0 == 0 {
                return false_0 as libc::c_int as boolean;
            }
        }
        chkx = chkx + deltax;
        chky = chky + deltay;
        i += 1;
    }
    obj.x = newx as byte;
    obj.y = newy as byte;
    obj.stage = (obj.stage as libc::c_int ^ 1 as libc::c_int) as byte;
    return true_0 as libc::c_int as boolean;
}
#[no_mangle]
#[c2rust::src_loc = "846:1"]
pub unsafe extern "C" fn playercmdthink() {
    let mut olddir: dirtype = north;
    let mut c: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    c = ControlPlayer(1 as libc::c_int);
    obj.stage = (obj.stage as libc::c_int & 1 as libc::c_int) as byte;
    if c.button1 as libc::c_int != 0
        && c.button2 as libc::c_int != 0
        && keydown[SDL_SCANCODE_Q as libc::c_int as usize] as libc::c_int != 0
    {
        givepotion();
        givescroll();
        givekey();
    }
    if (c.dir as libc::c_uint) < nodir as libc::c_int as libc::c_uint
        && frameon as libc::c_int % 2 as libc::c_int != 0
    {
        if c.button2 != 0 {
            olddir = obj.dir as dirtype;
        }
        if c.dir as libc::c_uint > west as libc::c_int as libc::c_uint {
            if frameon as libc::c_int / 2 as libc::c_int % 2 as libc::c_int != 0 {
                match c.dir as libc::c_uint {
                    4 => {
                        obj.dir = east as libc::c_int as word;
                        walk();
                        c.dir = north;
                    }
                    5 => {
                        obj.dir = south as libc::c_int as word;
                        walk();
                        c.dir = east;
                    }
                    6 => {
                        obj.dir = west as libc::c_int as word;
                        walk();
                        c.dir = south;
                    }
                    7 => {
                        obj.dir = north as libc::c_int as word;
                        walk();
                        c.dir = west;
                    }
                    _ => {}
                }
            } else {
                match c.dir as libc::c_uint {
                    4 => {
                        obj.dir = north as libc::c_int as word;
                        walk();
                        c.dir = east;
                    }
                    5 => {
                        obj.dir = east as libc::c_int as word;
                        walk();
                        c.dir = south;
                    }
                    6 => {
                        obj.dir = south as libc::c_int as word;
                        walk();
                        c.dir = west;
                    }
                    7 => {
                        obj.dir = west as libc::c_int as word;
                        walk();
                        c.dir = north;
                    }
                    _ => {}
                }
            }
        }
        obj.dir = c.dir as word;
        if walk() == 0 {
            PlaySound(1 as libc::c_int);
        }
        if c.button2 != 0 {
            obj.dir = olddir as word;
        }
    } else if c.button2 == 0 {
        match c.dir as libc::c_uint {
            7 | 0 => {
                obj.dir = north as libc::c_int as word;
            }
            4 | 1 => {
                obj.dir = east as libc::c_int as word;
            }
            5 | 2 => {
                obj.dir = south as libc::c_int as word;
            }
            6 | 3 => {
                obj.dir = west as libc::c_int as word;
            }
            8 | _ => {}
        }
    }
    originx = obj.x as libc::c_int - 11 as libc::c_int;
    originy = obj.y as libc::c_int - 11 as libc::c_int;
    if boltsleft > 0 as libc::c_int {
        if frameon as libc::c_int % 3 as libc::c_int == 0 as libc::c_int {
            playbigshoot();
            boltsleft -= 1;
        }
    } else if c.button1 != 0 {
        if shotpower == 0 as libc::c_int {
            shotpower = 1 as libc::c_int;
        } else if shotpower < 13 as libc::c_int && frameon as libc::c_int % 2 as libc::c_int != 0 {
            shotpower += 1;
        }
        printshotpower();
    } else if shotpower > 0 as libc::c_int {
        if shotpower == 13 as libc::c_int {
            playbigshoot();
        } else {
            playshoot();
        }
        shotpower = 0 as libc::c_int;
        printshotpower();
    }
    if indemo as u64 == 0 {
        if keydown[SDL_SCANCODE_P as libc::c_int as usize] as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
        {
            if (obj.hp as libc::c_int) < 13 as libc::c_int {
                takepotion();
                keydown[SDL_SCANCODE_Q as libc::c_int as usize] = false_0 as libc::c_int as boolean;
                keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] =
                    false_0 as libc::c_int as boolean;
            }
        } else if keydown[SDL_SCANCODE_B as libc::c_int as usize] != 0 {
            castbolt();
            keydown[SDL_SCANCODE_B as libc::c_int as usize] = false_0 as libc::c_int as boolean;
        } else if keydown[SDL_SCANCODE_N as libc::c_int as usize] as libc::c_int != 0
            || keydown[SDL_SCANCODE_RETURN as libc::c_int as usize] as libc::c_int != 0
        {
            castnuke();
            keydown[SDL_SCANCODE_N as libc::c_int as usize] = false_0 as libc::c_int as boolean;
            keydown[SDL_SCANCODE_RETURN as libc::c_int as usize] =
                false_0 as libc::c_int as boolean;
        }
    }
    dofkeys();
    if resetgame != 0 {
        resetgame = false_0 as libc::c_int as boolean;
        playdone = true_0 as libc::c_int as boolean;
        return;
    }
    match indemo as libc::c_uint {
        0 => {
            if keydown[SDL_SCANCODE_C as libc::c_int as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_T as libc::c_int as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
            {
                centerwindow(16 as libc::c_int, 2 as libc::c_int);
                print(b"warp to which\nlevel (1-99)?\0" as *const u8 as *const libc::c_char);
                clearkeys();
                level = _inputint() as sword;
                if (level as libc::c_int) < 1 as libc::c_int {
                    level = 1 as libc::c_int as sword;
                }
                if level as libc::c_int > 30 as libc::c_int {
                    level = 30 as libc::c_int as sword;
                }
                restore();
                leveldone = true_0 as libc::c_int as boolean;
            }
            if keydown[SDL_SCANCODE_C as libc::c_int as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_T as libc::c_int as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_TAB as libc::c_int as usize] as libc::c_int != 0
            {
                if GODMODE != 0 {
                    centerwindow(13 as libc::c_int, 1 as libc::c_int);
                    print(b"God Mode Off\0" as *const u8 as *const libc::c_char);
                    GODMODE = false_0 as libc::c_int as boolean;
                } else {
                    centerwindow(12 as libc::c_int, 1 as libc::c_int);
                    print(b"God Mode On\0" as *const u8 as *const libc::c_char);
                    GODMODE = true_0 as libc::c_int as boolean;
                }
                UpdateScreen();
                clearkeys();
                while bioskey(0 as libc::c_int) == 0 as libc::c_int {
                    WaitVBL();
                }
                restore();
                clearkeys();
            }
        }
        1 => {
            indemo = notdemo;
            ctrl = ControlPlayer(1 as libc::c_int);
            if ctrl.button1 as libc::c_int != 0
                || ctrl.button2 as libc::c_int != 0
                || keydown[SDL_SCANCODE_SPACE as libc::c_int as usize] as libc::c_int != 0
            {
                indemo = demoplay;
                exitdemo = true_0 as libc::c_int as boolean;
                leveldone = true_0 as libc::c_int as boolean;
                level = 0 as libc::c_int as sword;
                return;
            }
            indemo = demoplay;
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "1114:1"]
pub unsafe extern "C" fn chasethink(mut diagonal: boolean) {
    let mut deltax: libc::c_int = 0;
    let mut deltay: libc::c_int = 0;
    let mut d: [dirtype; 3] = [north; 3];
    let mut tdir: libc::c_int = 0;
    let mut olddir: libc::c_int = 0;
    let mut turnaround: libc::c_int = 0;
    obj.stage = (obj.stage as libc::c_int & 1 as libc::c_int) as byte;
    olddir = obj.dir as libc::c_int;
    turnaround = opposite[olddir as usize] as libc::c_int;
    deltax = o[0 as libc::c_int as usize].x as libc::c_int - obj.x as libc::c_int;
    deltay = o[0 as libc::c_int as usize].y as libc::c_int - obj.y as libc::c_int;
    d[1 as libc::c_int as usize] = nodir;
    d[2 as libc::c_int as usize] = nodir;
    if deltax > 0 as libc::c_int {
        d[1 as libc::c_int as usize] = east;
    }
    if deltax < 0 as libc::c_int {
        d[1 as libc::c_int as usize] = west;
    }
    if deltay > 0 as libc::c_int {
        d[2 as libc::c_int as usize] = south;
    }
    if deltay < 0 as libc::c_int {
        d[2 as libc::c_int as usize] = north;
    }
    if abs(deltay) > abs(deltax) {
        tdir = d[1 as libc::c_int as usize] as libc::c_int;
        d[1 as libc::c_int as usize] = d[2 as libc::c_int as usize];
        d[2 as libc::c_int as usize] = tdir as dirtype;
    }
    if d[1 as libc::c_int as usize] as libc::c_uint == turnaround as libc::c_uint {
        d[1 as libc::c_int as usize] = nodir;
    }
    if d[2 as libc::c_int as usize] as libc::c_uint == turnaround as libc::c_uint {
        d[2 as libc::c_int as usize] = nodir;
    }
    if diagonal != 0 {
        if d[1 as libc::c_int as usize] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[1 as libc::c_int as usize] as word;
            if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                return;
            }
        }
        if d[2 as libc::c_int as usize] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[2 as libc::c_int as usize] as word;
            if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                return;
            }
        }
    } else {
        if d[2 as libc::c_int as usize] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[2 as libc::c_int as usize] as word;
            if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                return;
            }
        }
        if d[1 as libc::c_int as usize] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[1 as libc::c_int as usize] as word;
            if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                return;
            }
        }
    }
    obj.dir = olddir as word;
    if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
        return;
    }
    if rndt() > 128 as libc::c_int {
        tdir = north as libc::c_int;
        while tdir <= west as libc::c_int {
            if tdir != turnaround {
                obj.dir = tdir as word;
                if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = west as libc::c_int;
        while tdir >= north as libc::c_int {
            if tdir != turnaround {
                obj.dir = tdir as word;
                if walk() as libc::c_int != 0 || obj.stage as libc::c_int == 3 as libc::c_int {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    obj.dir = turnaround as word;
    walk();
}
#[no_mangle]
#[c2rust::src_loc = "1229:1"]
pub unsafe extern "C" fn gargthink() {
    let mut n: libc::c_int = 0;
    if rndt() > 220 as libc::c_int {
        obj.stage = 2 as libc::c_int as byte;
        obj.delay = 6 as libc::c_int as byte;
        PlaySound(5 as libc::c_int);
        n = newobject();
        o[n as usize].class = rock as libc::c_int as word;
        o[n as usize].delay = 0 as libc::c_int as byte;
        o[n as usize].stage = 0 as libc::c_int as byte;
        o[n as usize].active = true_0 as libc::c_int as boolean;
        o[n as usize].dir = obj.dir;
        match obj.dir as libc::c_int {
            0 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 as libc::c_int + side) as byte;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as libc::c_int + 3 as libc::c_int) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 1 as libc::c_int + side) as byte;
            }
            2 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 as libc::c_int + side) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 3 as libc::c_int) as byte;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as libc::c_int + 1 as libc::c_int + side) as byte;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false_0 as libc::c_int as boolean);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1288:1"]
pub unsafe extern "C" fn dragonthink() {
    let mut n: libc::c_int = 0;
    if rndt() > 220 as libc::c_int {
        obj.stage = 2 as libc::c_int as byte;
        obj.delay = 6 as libc::c_int as byte;
        PlaySound(5 as libc::c_int);
        n = newobject();
        o[n as usize].class = bigshot as libc::c_int as word;
        o[n as usize].delay = 0 as libc::c_int as byte;
        o[n as usize].stage = 0 as libc::c_int as byte;
        o[n as usize].active = true_0 as libc::c_int as boolean;
        o[n as usize].dir = obj.dir;
        match o[n as usize].dir as libc::c_int {
            0 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 as libc::c_int + side) as byte;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as libc::c_int + 3 as libc::c_int) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 1 as libc::c_int + side) as byte;
            }
            2 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 as libc::c_int + side) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 3 as libc::c_int) as byte;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as libc::c_int + 1 as libc::c_int + side) as byte;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false_0 as libc::c_int as boolean);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1347:1"]
pub unsafe extern "C" fn gunthink(mut dir: libc::c_int) {
    let mut n: libc::c_int = 0;
    PlaySound(5 as libc::c_int);
    obj.stage = 0 as libc::c_int as byte;
    n = newobject();
    o[n as usize].class = bigshot as libc::c_int as word;
    o[n as usize].delay = 0 as libc::c_int as byte;
    o[n as usize].stage = 0 as libc::c_int as byte;
    o[n as usize].active = true_0 as libc::c_int as boolean;
    o[n as usize].dir = dir as word;
    o[n as usize].x = obj.x;
    o[n as usize].y = obj.y;
}
#[no_mangle]
#[c2rust::src_loc = "1375:1"]
pub unsafe extern "C" fn shooterthink() {
    if (obj.x as libc::c_int) < originx - 1 as libc::c_int
        || (obj.y as libc::c_int) < originy - 1 as libc::c_int
        || obj.x as libc::c_int > originx + 22 as libc::c_int
        || obj.y as libc::c_int > originy + 22 as libc::c_int
        || walk() == 0
        || obj.stage as libc::c_int == 2 as libc::c_int
    {
        obj.class = nothing as libc::c_int as word;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1393:1"]
pub unsafe extern "C" fn idlethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2 as libc::c_int as byte;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.stage = 0 as libc::c_int as byte;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1409:1"]
pub unsafe extern "C" fn fadethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2 as libc::c_int as byte;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.class = nothing as libc::c_int as word;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1427:1"]
pub unsafe extern "C" fn killnear(mut chkx_0: libc::c_int, mut chky_0: libc::c_int) {
    let mut spot: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    spot = background[chky_0 as usize][chkx_0 as usize];
    if spot < 136 as libc::c_int || spot > 145 as libc::c_int {
        return;
    }
    PlaySound(6 as libc::c_int);
    if spot < 143 as libc::c_int {
        background[chky_0 as usize][chkx_0 as usize] = 128 as libc::c_int;
    } else {
        background[chky_0 as usize][chkx_0 as usize] = spot + 19 as libc::c_int;
    }
    new = newobject();
    o[new as usize].active = true_0 as libc::c_int as boolean;
    o[new as usize].x = chkx_0 as byte;
    o[new as usize].y = chky_0 as byte;
    o[new as usize].stage = 0 as libc::c_int as byte;
    o[new as usize].delay = 2 as libc::c_int as byte;
    o[new as usize].class = dead1 as libc::c_int as word;
}
#[no_mangle]
#[c2rust::src_loc = "1458:1"]
pub unsafe extern "C" fn explodethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    if obj.stage as libc::c_int == 1 as libc::c_int {
        killnear(
            obj.x as libc::c_int - 1 as libc::c_int,
            obj.y as libc::c_int,
        );
        killnear(
            obj.x as libc::c_int,
            obj.y as libc::c_int - 1 as libc::c_int,
        );
        killnear(
            obj.x as libc::c_int + 1 as libc::c_int,
            obj.y as libc::c_int,
        );
        killnear(
            obj.x as libc::c_int,
            obj.y as libc::c_int + 1 as libc::c_int,
        );
    }
    obj.delay = 2 as libc::c_int as byte;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.class = nothing as libc::c_int as word;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1486:1"]
pub unsafe extern "C" fn think() {
    if obj.delay as libc::c_int > 0 as libc::c_int {
        obj.delay = (obj.delay).wrapping_sub(1);
    } else if rndt() < obj.speed as libc::c_int {
        match obj.think as libc::c_int {
            0 => {
                playercmdthink();
            }
            3 => {
                chasethink(false_0 as libc::c_int as boolean);
            }
            4 => {
                chasethink(true_0 as libc::c_int as boolean);
            }
            1 => {
                gargthink();
            }
            2 => {
                dragonthink();
            }
            5 => {
                shooterthink();
            }
            6 => {
                idlethink();
            }
            7 => {
                fadethink();
            }
            8 => {
                explodethink();
            }
            9 => {
                gunthink(west as libc::c_int);
            }
            10 => {
                gunthink(north as libc::c_int);
            }
            _ => {}
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1520:1"]
pub unsafe extern "C" fn doactive() {
    if obj.class as libc::c_int != dead1 as libc::c_int
        && ((obj.x as libc::c_int) < originx - 10 as libc::c_int
            || obj.x as libc::c_int > originx + 34 as libc::c_int
            || (obj.y as libc::c_int) < originy - 10 as libc::c_int
            || obj.y as libc::c_int > originy + 34 as libc::c_int)
    {
        o[objecton as usize].active = false_0 as libc::c_int as boolean;
    } else {
        think();
        eraseobj();
        if playdone != 0 {
            return;
        }
        if obj.class as libc::c_int > nothing as libc::c_int {
            drawobj();
        }
        memcpy(
            &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj as *mut libc::c_void,
            &mut obj as *mut objtype as *const libc::c_void,
            ::core::mem::size_of::<activeobj>() as libc::c_ulong,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "1554:1"]
pub unsafe extern "C" fn doinactive() {
    if obj.x as libc::c_int + obj.size as libc::c_int >= originx
        && (obj.x as libc::c_int) < originx + 24 as libc::c_int
        && obj.y as libc::c_int + obj.size as libc::c_int >= originy
        && (obj.y as libc::c_int) < originy + 24 as libc::c_int
    {
        obj.active = true_0 as libc::c_int as boolean;
        obj.dir = north as libc::c_int as word;
        memcpy(
            &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj as *mut libc::c_void,
            &mut obj.active as *mut boolean as *const libc::c_void,
            ::core::mem::size_of::<activeobj>() as libc::c_ulong,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1578:1"]
pub unsafe extern "C" fn playloop() {
    screencenterx = 11 as libc::c_int;
    loop {
        if indemo as u64 == 0 {
            centerwindow(11 as libc::c_int, 2 as libc::c_int);
            print(b" Entering\nlevel \0" as *const u8 as *const libc::c_char);
            printint(level as libc::c_int);
            print(b"...\0" as *const u8 as *const libc::c_char);
            PlaySound(17 as libc::c_int);
            WaitEndSound();
        }
        clearold();
        loadlevel();
        leveldone = false_0 as libc::c_int as boolean;
        if keydown[SDL_SCANCODE_F7 as libc::c_int as usize] as libc::c_int != 0
            && keydown[SDL_SCANCODE_D as libc::c_int as usize] as libc::c_int != 0
        {
            clearold();
            refresh();
            refresh();
            clearkeys();
            centerwindow(12 as libc::c_int, 1 as libc::c_int);
            print(b"RECORD DEMO\0" as *const u8 as *const libc::c_char);
            loop {
                ch = get() as libc::c_char;
                if !(ch as libc::c_int != 13 as libc::c_int) {
                    break;
                }
            }
            RecordDemo();
            clearold();
            clearkeys();
        }
        playdone = false_0 as libc::c_int as boolean;
        frameon = 0 as libc::c_int as word;
        boltsleft = 0 as libc::c_int;
        shotpower = 0 as libc::c_int;
        initrndt(false_0 as libc::c_int as boolean);
        printshotpower();
        doall();
        if indemo as libc::c_uint == recording as libc::c_int as libc::c_uint {
            clearkeys();
            centerwindow(15 as libc::c_int, 1 as libc::c_int);
            print(b"SAVE AS DEMO#:\0" as *const u8 as *const libc::c_char);
            loop {
                ch = get() as libc::c_char;
                if !((ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32) {
                    break;
                }
            }
            SaveDemo(ch as libc::c_int - '0' as i32);
            clearold();
            refresh();
            refresh();
        }
        if indemo as u64 != 0 {
            playdone = true_0 as libc::c_int as boolean;
        }
        if !(playdone == 0) {
            break;
        }
    }
}
