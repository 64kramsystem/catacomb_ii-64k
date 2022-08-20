use ::libc;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:54"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    use super::types_h::__int16_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:55"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_scancode.h:55"]
pub mod SDL_scancode_h {
    #[c2rust::src_loc = "43:9"]
    pub type SDL_Scancode = libc::c_uint;
    #[c2rust::src_loc = "407:5"]
    pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
    #[c2rust::src_loc = "401:5"]
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
    #[c2rust::src_loc = "400:5"]
    pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
    #[c2rust::src_loc = "389:5"]
    pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
    #[c2rust::src_loc = "388:5"]
    pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
    #[c2rust::src_loc = "386:5"]
    pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
    #[c2rust::src_loc = "385:5"]
    pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
    #[c2rust::src_loc = "384:5"]
    pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
    #[c2rust::src_loc = "383:5"]
    pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
    #[c2rust::src_loc = "382:5"]
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
    #[c2rust::src_loc = "380:5"]
    pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
    #[c2rust::src_loc = "379:5"]
    pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
    #[c2rust::src_loc = "378:5"]
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
    #[c2rust::src_loc = "367:5"]
    pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
    #[c2rust::src_loc = "366:5"]
    pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
    #[c2rust::src_loc = "365:5"]
    pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
    #[c2rust::src_loc = "364:5"]
    pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
    #[c2rust::src_loc = "363:5"]
    pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
    #[c2rust::src_loc = "362:5"]
    pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
    #[c2rust::src_loc = "361:5"]
    pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
    #[c2rust::src_loc = "360:5"]
    pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
    #[c2rust::src_loc = "359:5"]
    pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
    #[c2rust::src_loc = "358:5"]
    pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
    #[c2rust::src_loc = "357:5"]
    pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
    #[c2rust::src_loc = "356:5"]
    pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
    #[c2rust::src_loc = "355:5"]
    pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
    #[c2rust::src_loc = "354:5"]
    pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
    #[c2rust::src_loc = "353:5"]
    pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
    #[c2rust::src_loc = "352:5"]
    pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
    #[c2rust::src_loc = "351:5"]
    pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
    #[c2rust::src_loc = "337:5"]
    pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
    #[c2rust::src_loc = "335:5"]
    pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
    #[c2rust::src_loc = "334:5"]
    pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
    #[c2rust::src_loc = "333:5"]
    pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
    #[c2rust::src_loc = "332:5"]
    pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
    #[c2rust::src_loc = "331:5"]
    pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
    #[c2rust::src_loc = "330:5"]
    pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
    #[c2rust::src_loc = "329:5"]
    pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
    #[c2rust::src_loc = "328:5"]
    pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
    #[c2rust::src_loc = "326:5"]
    pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
    #[c2rust::src_loc = "325:5"]
    pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
    #[c2rust::src_loc = "324:5"]
    pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
    #[c2rust::src_loc = "323:5"]
    pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
    #[c2rust::src_loc = "322:5"]
    pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
    #[c2rust::src_loc = "321:5"]
    pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
    #[c2rust::src_loc = "320:5"]
    pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
    #[c2rust::src_loc = "319:5"]
    pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
    #[c2rust::src_loc = "318:5"]
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
    #[c2rust::src_loc = "317:5"]
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
    #[c2rust::src_loc = "316:5"]
    pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
    #[c2rust::src_loc = "315:5"]
    pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
    #[c2rust::src_loc = "314:5"]
    pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
    #[c2rust::src_loc = "313:5"]
    pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
    #[c2rust::src_loc = "312:5"]
    pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
    #[c2rust::src_loc = "311:5"]
    pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
    #[c2rust::src_loc = "310:5"]
    pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
    #[c2rust::src_loc = "309:5"]
    pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
    #[c2rust::src_loc = "308:5"]
    pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
    #[c2rust::src_loc = "307:5"]
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
    #[c2rust::src_loc = "306:5"]
    pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
    #[c2rust::src_loc = "305:5"]
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
    #[c2rust::src_loc = "304:5"]
    pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
    #[c2rust::src_loc = "303:5"]
    pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
    #[c2rust::src_loc = "302:5"]
    pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
    #[c2rust::src_loc = "301:5"]
    pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
    #[c2rust::src_loc = "300:5"]
    pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
    #[c2rust::src_loc = "299:5"]
    pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
    #[c2rust::src_loc = "298:5"]
    pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
    #[c2rust::src_loc = "297:5"]
    pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
    #[c2rust::src_loc = "296:5"]
    pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
    #[c2rust::src_loc = "295:5"]
    pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
    #[c2rust::src_loc = "294:5"]
    pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
    #[c2rust::src_loc = "293:5"]
    pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
    #[c2rust::src_loc = "292:5"]
    pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
    #[c2rust::src_loc = "291:5"]
    pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
    #[c2rust::src_loc = "290:5"]
    pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
    #[c2rust::src_loc = "289:5"]
    pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
    #[c2rust::src_loc = "288:5"]
    pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
    #[c2rust::src_loc = "287:5"]
    pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
    #[c2rust::src_loc = "286:5"]
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
    #[c2rust::src_loc = "285:5"]
    pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
    #[c2rust::src_loc = "284:5"]
    pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
    #[c2rust::src_loc = "283:5"]
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
    #[c2rust::src_loc = "282:5"]
    pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
    #[c2rust::src_loc = "278:5"]
    pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
    #[c2rust::src_loc = "276:5"]
    pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
    #[c2rust::src_loc = "275:5"]
    pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
    #[c2rust::src_loc = "273:5"]
    pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
    #[c2rust::src_loc = "272:5"]
    pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
    #[c2rust::src_loc = "271:5"]
    pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
    #[c2rust::src_loc = "270:5"]
    pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
    #[c2rust::src_loc = "269:5"]
    pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
    #[c2rust::src_loc = "268:5"]
    pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
    #[c2rust::src_loc = "266:5"]
    pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
    #[c2rust::src_loc = "265:5"]
    pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
    #[c2rust::src_loc = "264:5"]
    pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
    #[c2rust::src_loc = "263:5"]
    pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
    #[c2rust::src_loc = "262:5"]
    pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
    #[c2rust::src_loc = "261:5"]
    pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
    #[c2rust::src_loc = "260:5"]
    pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
    #[c2rust::src_loc = "259:5"]
    pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
    #[c2rust::src_loc = "258:5"]
    pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
    #[c2rust::src_loc = "257:5"]
    pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
    #[c2rust::src_loc = "256:5"]
    pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
    #[c2rust::src_loc = "255:5"]
    pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
    #[c2rust::src_loc = "254:5"]
    pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
    #[c2rust::src_loc = "253:5"]
    pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
    #[c2rust::src_loc = "252:5"]
    pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
    #[c2rust::src_loc = "251:5"]
    pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
    #[c2rust::src_loc = "250:5"]
    pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
    #[c2rust::src_loc = "248:5"]
    pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
    #[c2rust::src_loc = "246:5"]
    pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
    #[c2rust::src_loc = "245:5"]
    pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
    #[c2rust::src_loc = "240:5"]
    pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
    #[c2rust::src_loc = "239:5"]
    pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
    #[c2rust::src_loc = "238:5"]
    pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
    #[c2rust::src_loc = "237:5"]
    pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
    #[c2rust::src_loc = "236:5"]
    pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
    #[c2rust::src_loc = "235:5"]
    pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
    #[c2rust::src_loc = "234:5"]
    pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
    #[c2rust::src_loc = "233:5"]
    pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
    #[c2rust::src_loc = "232:5"]
    pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
    #[c2rust::src_loc = "231:5"]
    pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
    #[c2rust::src_loc = "230:5"]
    pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
    #[c2rust::src_loc = "229:5"]
    pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
    #[c2rust::src_loc = "228:5"]
    pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
    #[c2rust::src_loc = "227:5"]
    pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
    #[c2rust::src_loc = "226:5"]
    pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
    #[c2rust::src_loc = "225:5"]
    pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
    #[c2rust::src_loc = "224:5"]
    pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
    #[c2rust::src_loc = "223:5"]
    pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
    #[c2rust::src_loc = "222:5"]
    pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
    #[c2rust::src_loc = "221:5"]
    pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
    #[c2rust::src_loc = "220:5"]
    pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
    #[c2rust::src_loc = "219:5"]
    pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
    #[c2rust::src_loc = "218:5"]
    pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
    #[c2rust::src_loc = "217:5"]
    pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
    #[c2rust::src_loc = "216:5"]
    pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
    #[c2rust::src_loc = "215:5"]
    pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
    #[c2rust::src_loc = "214:5"]
    pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
    #[c2rust::src_loc = "211:5"]
    pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
    #[c2rust::src_loc = "210:5"]
    pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
    #[c2rust::src_loc = "200:5"]
    pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
    #[c2rust::src_loc = "198:5"]
    pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
    #[c2rust::src_loc = "197:5"]
    pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
    #[c2rust::src_loc = "196:5"]
    pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
    #[c2rust::src_loc = "195:5"]
    pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
    #[c2rust::src_loc = "194:5"]
    pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
    #[c2rust::src_loc = "193:5"]
    pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
    #[c2rust::src_loc = "192:5"]
    pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
    #[c2rust::src_loc = "191:5"]
    pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
    #[c2rust::src_loc = "190:5"]
    pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
    #[c2rust::src_loc = "189:5"]
    pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
    #[c2rust::src_loc = "188:5"]
    pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
    #[c2rust::src_loc = "187:5"]
    pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
    #[c2rust::src_loc = "186:5"]
    pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
    #[c2rust::src_loc = "185:5"]
    pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
    #[c2rust::src_loc = "184:5"]
    pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
    #[c2rust::src_loc = "183:5"]
    pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
    #[c2rust::src_loc = "181:5"]
    pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
    #[c2rust::src_loc = "179:5"]
    pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
    #[c2rust::src_loc = "178:5"]
    pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
    #[c2rust::src_loc = "177:5"]
    pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
    #[c2rust::src_loc = "176:5"]
    pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
    #[c2rust::src_loc = "175:5"]
    pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
    #[c2rust::src_loc = "174:5"]
    pub const SDL_SCANCODE_END: SDL_Scancode = 77;
    #[c2rust::src_loc = "173:5"]
    pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
    #[c2rust::src_loc = "172:5"]
    pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
    #[c2rust::src_loc = "171:5"]
    pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
    #[c2rust::src_loc = "169:5"]
    pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
    #[c2rust::src_loc = "168:5"]
    pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
    #[c2rust::src_loc = "167:5"]
    pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
    #[c2rust::src_loc = "166:5"]
    pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
    #[c2rust::src_loc = "162:5"]
    pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
    #[c2rust::src_loc = "161:5"]
    pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
    #[c2rust::src_loc = "159:5"]
    pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
    #[c2rust::src_loc = "158:5"]
    pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
    #[c2rust::src_loc = "157:5"]
    pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
    #[c2rust::src_loc = "156:5"]
    pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
    #[c2rust::src_loc = "154:5"]
    pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
    #[c2rust::src_loc = "153:5"]
    pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
    #[c2rust::src_loc = "149:5"]
    pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
    #[c2rust::src_loc = "147:5"]
    pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
    #[c2rust::src_loc = "130:5"]
    pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
    #[c2rust::src_loc = "129:5"]
    pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
    #[c2rust::src_loc = "128:5"]
    pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
    #[c2rust::src_loc = "102:5"]
    pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
    #[c2rust::src_loc = "101:5"]
    pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
    #[c2rust::src_loc = "99:5"]
    pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
    #[c2rust::src_loc = "98:5"]
    pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
    #[c2rust::src_loc = "96:5"]
    pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
    #[c2rust::src_loc = "95:5"]
    pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
    #[c2rust::src_loc = "94:5"]
    pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
    #[c2rust::src_loc = "92:5"]
    pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
    #[c2rust::src_loc = "90:5"]
    pub const SDL_SCANCODE_0: SDL_Scancode = 39;
    #[c2rust::src_loc = "89:5"]
    pub const SDL_SCANCODE_9: SDL_Scancode = 38;
    #[c2rust::src_loc = "88:5"]
    pub const SDL_SCANCODE_8: SDL_Scancode = 37;
    #[c2rust::src_loc = "87:5"]
    pub const SDL_SCANCODE_7: SDL_Scancode = 36;
    #[c2rust::src_loc = "86:5"]
    pub const SDL_SCANCODE_6: SDL_Scancode = 35;
    #[c2rust::src_loc = "85:5"]
    pub const SDL_SCANCODE_5: SDL_Scancode = 34;
    #[c2rust::src_loc = "84:5"]
    pub const SDL_SCANCODE_4: SDL_Scancode = 33;
    #[c2rust::src_loc = "83:5"]
    pub const SDL_SCANCODE_3: SDL_Scancode = 32;
    #[c2rust::src_loc = "82:5"]
    pub const SDL_SCANCODE_2: SDL_Scancode = 31;
    #[c2rust::src_loc = "81:5"]
    pub const SDL_SCANCODE_1: SDL_Scancode = 30;
    #[c2rust::src_loc = "79:5"]
    pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
    #[c2rust::src_loc = "78:5"]
    pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
    #[c2rust::src_loc = "77:5"]
    pub const SDL_SCANCODE_X: SDL_Scancode = 27;
    #[c2rust::src_loc = "76:5"]
    pub const SDL_SCANCODE_W: SDL_Scancode = 26;
    #[c2rust::src_loc = "75:5"]
    pub const SDL_SCANCODE_V: SDL_Scancode = 25;
    #[c2rust::src_loc = "74:5"]
    pub const SDL_SCANCODE_U: SDL_Scancode = 24;
    #[c2rust::src_loc = "73:5"]
    pub const SDL_SCANCODE_T: SDL_Scancode = 23;
    #[c2rust::src_loc = "72:5"]
    pub const SDL_SCANCODE_S: SDL_Scancode = 22;
    #[c2rust::src_loc = "71:5"]
    pub const SDL_SCANCODE_R: SDL_Scancode = 21;
    #[c2rust::src_loc = "70:5"]
    pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
    #[c2rust::src_loc = "69:5"]
    pub const SDL_SCANCODE_P: SDL_Scancode = 19;
    #[c2rust::src_loc = "68:5"]
    pub const SDL_SCANCODE_O: SDL_Scancode = 18;
    #[c2rust::src_loc = "67:5"]
    pub const SDL_SCANCODE_N: SDL_Scancode = 17;
    #[c2rust::src_loc = "66:5"]
    pub const SDL_SCANCODE_M: SDL_Scancode = 16;
    #[c2rust::src_loc = "65:5"]
    pub const SDL_SCANCODE_L: SDL_Scancode = 15;
    #[c2rust::src_loc = "64:5"]
    pub const SDL_SCANCODE_K: SDL_Scancode = 14;
    #[c2rust::src_loc = "63:5"]
    pub const SDL_SCANCODE_J: SDL_Scancode = 13;
    #[c2rust::src_loc = "62:5"]
    pub const SDL_SCANCODE_I: SDL_Scancode = 12;
    #[c2rust::src_loc = "61:5"]
    pub const SDL_SCANCODE_H: SDL_Scancode = 11;
    #[c2rust::src_loc = "60:5"]
    pub const SDL_SCANCODE_G: SDL_Scancode = 10;
    #[c2rust::src_loc = "59:5"]
    pub const SDL_SCANCODE_F: SDL_Scancode = 9;
    #[c2rust::src_loc = "58:5"]
    pub const SDL_SCANCODE_E: SDL_Scancode = 8;
    #[c2rust::src_loc = "57:5"]
    pub const SDL_SCANCODE_D: SDL_Scancode = 7;
    #[c2rust::src_loc = "56:5"]
    pub const SDL_SCANCODE_C: SDL_Scancode = 6;
    #[c2rust::src_loc = "55:5"]
    pub const SDL_SCANCODE_B: SDL_Scancode = 5;
    #[c2rust::src_loc = "54:5"]
    pub const SDL_SCANCODE_A: SDL_Scancode = 4;
    #[c2rust::src_loc = "45:5"]
    pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_keycode.h:55"]
pub mod SDL_keycode_h {
    #[c2rust::src_loc = "50:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "319:5"]
    pub const SDLK_AUDIOFASTFORWARD: C2RustUnnamed = 1073742110;
    #[c2rust::src_loc = "318:5"]
    pub const SDLK_AUDIOREWIND: C2RustUnnamed = 1073742109;
    #[c2rust::src_loc = "316:5"]
    pub const SDLK_APP2: C2RustUnnamed = 1073742108;
    #[c2rust::src_loc = "315:5"]
    pub const SDLK_APP1: C2RustUnnamed = 1073742107;
    #[c2rust::src_loc = "314:5"]
    pub const SDLK_SLEEP: C2RustUnnamed = 1073742106;
    #[c2rust::src_loc = "313:5"]
    pub const SDLK_EJECT: C2RustUnnamed = 1073742105;
    #[c2rust::src_loc = "312:5"]
    pub const SDLK_KBDILLUMUP: C2RustUnnamed = 1073742104;
    #[c2rust::src_loc = "311:5"]
    pub const SDLK_KBDILLUMDOWN: C2RustUnnamed = 1073742103;
    #[c2rust::src_loc = "309:5"]
    pub const SDLK_KBDILLUMTOGGLE: C2RustUnnamed = 1073742102;
    #[c2rust::src_loc = "308:5"]
    pub const SDLK_DISPLAYSWITCH: C2RustUnnamed = 1073742101;
    #[c2rust::src_loc = "307:5"]
    pub const SDLK_BRIGHTNESSUP: C2RustUnnamed = 1073742100;
    #[c2rust::src_loc = "305:5"]
    pub const SDLK_BRIGHTNESSDOWN: C2RustUnnamed = 1073742099;
    #[c2rust::src_loc = "303:5"]
    pub const SDLK_AC_BOOKMARKS: C2RustUnnamed = 1073742098;
    #[c2rust::src_loc = "302:5"]
    pub const SDLK_AC_REFRESH: C2RustUnnamed = 1073742097;
    #[c2rust::src_loc = "301:5"]
    pub const SDLK_AC_STOP: C2RustUnnamed = 1073742096;
    #[c2rust::src_loc = "300:5"]
    pub const SDLK_AC_FORWARD: C2RustUnnamed = 1073742095;
    #[c2rust::src_loc = "299:5"]
    pub const SDLK_AC_BACK: C2RustUnnamed = 1073742094;
    #[c2rust::src_loc = "298:5"]
    pub const SDLK_AC_HOME: C2RustUnnamed = 1073742093;
    #[c2rust::src_loc = "297:5"]
    pub const SDLK_AC_SEARCH: C2RustUnnamed = 1073742092;
    #[c2rust::src_loc = "296:5"]
    pub const SDLK_COMPUTER: C2RustUnnamed = 1073742091;
    #[c2rust::src_loc = "295:5"]
    pub const SDLK_CALCULATOR: C2RustUnnamed = 1073742090;
    #[c2rust::src_loc = "294:5"]
    pub const SDLK_MAIL: C2RustUnnamed = 1073742089;
    #[c2rust::src_loc = "293:5"]
    pub const SDLK_WWW: C2RustUnnamed = 1073742088;
    #[c2rust::src_loc = "292:5"]
    pub const SDLK_MEDIASELECT: C2RustUnnamed = 1073742087;
    #[c2rust::src_loc = "291:5"]
    pub const SDLK_AUDIOMUTE: C2RustUnnamed = 1073742086;
    #[c2rust::src_loc = "290:5"]
    pub const SDLK_AUDIOPLAY: C2RustUnnamed = 1073742085;
    #[c2rust::src_loc = "289:5"]
    pub const SDLK_AUDIOSTOP: C2RustUnnamed = 1073742084;
    #[c2rust::src_loc = "288:5"]
    pub const SDLK_AUDIOPREV: C2RustUnnamed = 1073742083;
    #[c2rust::src_loc = "287:5"]
    pub const SDLK_AUDIONEXT: C2RustUnnamed = 1073742082;
    #[c2rust::src_loc = "285:5"]
    pub const SDLK_MODE: C2RustUnnamed = 1073742081;
    #[c2rust::src_loc = "283:5"]
    pub const SDLK_RGUI: C2RustUnnamed = 1073742055;
    #[c2rust::src_loc = "282:5"]
    pub const SDLK_RALT: C2RustUnnamed = 1073742054;
    #[c2rust::src_loc = "281:5"]
    pub const SDLK_RSHIFT: C2RustUnnamed = 1073742053;
    #[c2rust::src_loc = "280:5"]
    pub const SDLK_RCTRL: C2RustUnnamed = 1073742052;
    #[c2rust::src_loc = "279:5"]
    pub const SDLK_LGUI: C2RustUnnamed = 1073742051;
    #[c2rust::src_loc = "278:5"]
    pub const SDLK_LALT: C2RustUnnamed = 1073742050;
    #[c2rust::src_loc = "277:5"]
    pub const SDLK_LSHIFT: C2RustUnnamed = 1073742049;
    #[c2rust::src_loc = "276:5"]
    pub const SDLK_LCTRL: C2RustUnnamed = 1073742048;
    #[c2rust::src_loc = "273:5"]
    pub const SDLK_KP_HEXADECIMAL: C2RustUnnamed = 1073742045;
    #[c2rust::src_loc = "272:5"]
    pub const SDLK_KP_DECIMAL: C2RustUnnamed = 1073742044;
    #[c2rust::src_loc = "271:5"]
    pub const SDLK_KP_OCTAL: C2RustUnnamed = 1073742043;
    #[c2rust::src_loc = "270:5"]
    pub const SDLK_KP_BINARY: C2RustUnnamed = 1073742042;
    #[c2rust::src_loc = "269:5"]
    pub const SDLK_KP_CLEARENTRY: C2RustUnnamed = 1073742041;
    #[c2rust::src_loc = "268:5"]
    pub const SDLK_KP_CLEAR: C2RustUnnamed = 1073742040;
    #[c2rust::src_loc = "267:5"]
    pub const SDLK_KP_PLUSMINUS: C2RustUnnamed = 1073742039;
    #[c2rust::src_loc = "266:5"]
    pub const SDLK_KP_MEMDIVIDE: C2RustUnnamed = 1073742038;
    #[c2rust::src_loc = "264:5"]
    pub const SDLK_KP_MEMMULTIPLY: C2RustUnnamed = 1073742037;
    #[c2rust::src_loc = "262:5"]
    pub const SDLK_KP_MEMSUBTRACT: C2RustUnnamed = 1073742036;
    #[c2rust::src_loc = "261:5"]
    pub const SDLK_KP_MEMADD: C2RustUnnamed = 1073742035;
    #[c2rust::src_loc = "260:5"]
    pub const SDLK_KP_MEMCLEAR: C2RustUnnamed = 1073742034;
    #[c2rust::src_loc = "259:5"]
    pub const SDLK_KP_MEMRECALL: C2RustUnnamed = 1073742033;
    #[c2rust::src_loc = "258:5"]
    pub const SDLK_KP_MEMSTORE: C2RustUnnamed = 1073742032;
    #[c2rust::src_loc = "257:5"]
    pub const SDLK_KP_EXCLAM: C2RustUnnamed = 1073742031;
    #[c2rust::src_loc = "256:5"]
    pub const SDLK_KP_AT: C2RustUnnamed = 1073742030;
    #[c2rust::src_loc = "255:5"]
    pub const SDLK_KP_SPACE: C2RustUnnamed = 1073742029;
    #[c2rust::src_loc = "254:5"]
    pub const SDLK_KP_HASH: C2RustUnnamed = 1073742028;
    #[c2rust::src_loc = "253:5"]
    pub const SDLK_KP_COLON: C2RustUnnamed = 1073742027;
    #[c2rust::src_loc = "251:5"]
    pub const SDLK_KP_DBLVERTICALBAR: C2RustUnnamed = 1073742026;
    #[c2rust::src_loc = "249:5"]
    pub const SDLK_KP_VERTICALBAR: C2RustUnnamed = 1073742025;
    #[c2rust::src_loc = "247:5"]
    pub const SDLK_KP_DBLAMPERSAND: C2RustUnnamed = 1073742024;
    #[c2rust::src_loc = "246:5"]
    pub const SDLK_KP_AMPERSAND: C2RustUnnamed = 1073742023;
    #[c2rust::src_loc = "245:5"]
    pub const SDLK_KP_GREATER: C2RustUnnamed = 1073742022;
    #[c2rust::src_loc = "244:5"]
    pub const SDLK_KP_LESS: C2RustUnnamed = 1073742021;
    #[c2rust::src_loc = "243:5"]
    pub const SDLK_KP_PERCENT: C2RustUnnamed = 1073742020;
    #[c2rust::src_loc = "242:5"]
    pub const SDLK_KP_POWER: C2RustUnnamed = 1073742019;
    #[c2rust::src_loc = "241:5"]
    pub const SDLK_KP_XOR: C2RustUnnamed = 1073742018;
    #[c2rust::src_loc = "240:5"]
    pub const SDLK_KP_F: C2RustUnnamed = 1073742017;
    #[c2rust::src_loc = "239:5"]
    pub const SDLK_KP_E: C2RustUnnamed = 1073742016;
    #[c2rust::src_loc = "238:5"]
    pub const SDLK_KP_D: C2RustUnnamed = 1073742015;
    #[c2rust::src_loc = "237:5"]
    pub const SDLK_KP_C: C2RustUnnamed = 1073742014;
    #[c2rust::src_loc = "236:5"]
    pub const SDLK_KP_B: C2RustUnnamed = 1073742013;
    #[c2rust::src_loc = "235:5"]
    pub const SDLK_KP_A: C2RustUnnamed = 1073742012;
    #[c2rust::src_loc = "234:5"]
    pub const SDLK_KP_BACKSPACE: C2RustUnnamed = 1073742011;
    #[c2rust::src_loc = "233:5"]
    pub const SDLK_KP_TAB: C2RustUnnamed = 1073742010;
    #[c2rust::src_loc = "232:5"]
    pub const SDLK_KP_RIGHTBRACE: C2RustUnnamed = 1073742009;
    #[c2rust::src_loc = "231:5"]
    pub const SDLK_KP_LEFTBRACE: C2RustUnnamed = 1073742008;
    #[c2rust::src_loc = "230:5"]
    pub const SDLK_KP_RIGHTPAREN: C2RustUnnamed = 1073742007;
    #[c2rust::src_loc = "229:5"]
    pub const SDLK_KP_LEFTPAREN: C2RustUnnamed = 1073742006;
    #[c2rust::src_loc = "227:5"]
    pub const SDLK_CURRENCYSUBUNIT: C2RustUnnamed = 1073742005;
    #[c2rust::src_loc = "226:5"]
    pub const SDLK_CURRENCYUNIT: C2RustUnnamed = 1073742004;
    #[c2rust::src_loc = "224:5"]
    pub const SDLK_DECIMALSEPARATOR: C2RustUnnamed = 1073742003;
    #[c2rust::src_loc = "222:5"]
    pub const SDLK_THOUSANDSSEPARATOR: C2RustUnnamed = 1073742002;
    #[c2rust::src_loc = "221:5"]
    pub const SDLK_KP_000: C2RustUnnamed = 1073742001;
    #[c2rust::src_loc = "220:5"]
    pub const SDLK_KP_00: C2RustUnnamed = 1073742000;
    #[c2rust::src_loc = "218:5"]
    pub const SDLK_EXSEL: C2RustUnnamed = 1073741988;
    #[c2rust::src_loc = "217:5"]
    pub const SDLK_CRSEL: C2RustUnnamed = 1073741987;
    #[c2rust::src_loc = "216:5"]
    pub const SDLK_CLEARAGAIN: C2RustUnnamed = 1073741986;
    #[c2rust::src_loc = "215:5"]
    pub const SDLK_OPER: C2RustUnnamed = 1073741985;
    #[c2rust::src_loc = "214:5"]
    pub const SDLK_OUT: C2RustUnnamed = 1073741984;
    #[c2rust::src_loc = "213:5"]
    pub const SDLK_SEPARATOR: C2RustUnnamed = 1073741983;
    #[c2rust::src_loc = "212:5"]
    pub const SDLK_RETURN2: C2RustUnnamed = 1073741982;
    #[c2rust::src_loc = "211:5"]
    pub const SDLK_PRIOR: C2RustUnnamed = 1073741981;
    #[c2rust::src_loc = "210:5"]
    pub const SDLK_CLEAR: C2RustUnnamed = 1073741980;
    #[c2rust::src_loc = "209:5"]
    pub const SDLK_CANCEL: C2RustUnnamed = 1073741979;
    #[c2rust::src_loc = "208:5"]
    pub const SDLK_SYSREQ: C2RustUnnamed = 1073741978;
    #[c2rust::src_loc = "207:5"]
    pub const SDLK_ALTERASE: C2RustUnnamed = 1073741977;
    #[c2rust::src_loc = "204:5"]
    pub const SDLK_KP_EQUALSAS400: C2RustUnnamed = 1073741958;
    #[c2rust::src_loc = "203:5"]
    pub const SDLK_KP_COMMA: C2RustUnnamed = 1073741957;
    #[c2rust::src_loc = "202:5"]
    pub const SDLK_VOLUMEDOWN: C2RustUnnamed = 1073741953;
    #[c2rust::src_loc = "201:5"]
    pub const SDLK_VOLUMEUP: C2RustUnnamed = 1073741952;
    #[c2rust::src_loc = "200:5"]
    pub const SDLK_MUTE: C2RustUnnamed = 1073741951;
    #[c2rust::src_loc = "199:5"]
    pub const SDLK_FIND: C2RustUnnamed = 1073741950;
    #[c2rust::src_loc = "198:5"]
    pub const SDLK_PASTE: C2RustUnnamed = 1073741949;
    #[c2rust::src_loc = "197:5"]
    pub const SDLK_COPY: C2RustUnnamed = 1073741948;
    #[c2rust::src_loc = "196:5"]
    pub const SDLK_CUT: C2RustUnnamed = 1073741947;
    #[c2rust::src_loc = "195:5"]
    pub const SDLK_UNDO: C2RustUnnamed = 1073741946;
    #[c2rust::src_loc = "194:5"]
    pub const SDLK_AGAIN: C2RustUnnamed = 1073741945;
    #[c2rust::src_loc = "193:5"]
    pub const SDLK_STOP: C2RustUnnamed = 1073741944;
    #[c2rust::src_loc = "192:5"]
    pub const SDLK_SELECT: C2RustUnnamed = 1073741943;
    #[c2rust::src_loc = "191:5"]
    pub const SDLK_MENU: C2RustUnnamed = 1073741942;
    #[c2rust::src_loc = "190:5"]
    pub const SDLK_HELP: C2RustUnnamed = 1073741941;
    #[c2rust::src_loc = "189:5"]
    pub const SDLK_EXECUTE: C2RustUnnamed = 1073741940;
    #[c2rust::src_loc = "188:5"]
    pub const SDLK_F24: C2RustUnnamed = 1073741939;
    #[c2rust::src_loc = "187:5"]
    pub const SDLK_F23: C2RustUnnamed = 1073741938;
    #[c2rust::src_loc = "186:5"]
    pub const SDLK_F22: C2RustUnnamed = 1073741937;
    #[c2rust::src_loc = "185:5"]
    pub const SDLK_F21: C2RustUnnamed = 1073741936;
    #[c2rust::src_loc = "184:5"]
    pub const SDLK_F20: C2RustUnnamed = 1073741935;
    #[c2rust::src_loc = "183:5"]
    pub const SDLK_F19: C2RustUnnamed = 1073741934;
    #[c2rust::src_loc = "182:5"]
    pub const SDLK_F18: C2RustUnnamed = 1073741933;
    #[c2rust::src_loc = "181:5"]
    pub const SDLK_F17: C2RustUnnamed = 1073741932;
    #[c2rust::src_loc = "180:5"]
    pub const SDLK_F16: C2RustUnnamed = 1073741931;
    #[c2rust::src_loc = "179:5"]
    pub const SDLK_F15: C2RustUnnamed = 1073741930;
    #[c2rust::src_loc = "178:5"]
    pub const SDLK_F14: C2RustUnnamed = 1073741929;
    #[c2rust::src_loc = "177:5"]
    pub const SDLK_F13: C2RustUnnamed = 1073741928;
    #[c2rust::src_loc = "176:5"]
    pub const SDLK_KP_EQUALS: C2RustUnnamed = 1073741927;
    #[c2rust::src_loc = "175:5"]
    pub const SDLK_POWER: C2RustUnnamed = 1073741926;
    #[c2rust::src_loc = "174:5"]
    pub const SDLK_APPLICATION: C2RustUnnamed = 1073741925;
    #[c2rust::src_loc = "172:5"]
    pub const SDLK_KP_PERIOD: C2RustUnnamed = 1073741923;
    #[c2rust::src_loc = "171:5"]
    pub const SDLK_KP_0: C2RustUnnamed = 1073741922;
    #[c2rust::src_loc = "170:5"]
    pub const SDLK_KP_9: C2RustUnnamed = 1073741921;
    #[c2rust::src_loc = "169:5"]
    pub const SDLK_KP_8: C2RustUnnamed = 1073741920;
    #[c2rust::src_loc = "168:5"]
    pub const SDLK_KP_7: C2RustUnnamed = 1073741919;
    #[c2rust::src_loc = "167:5"]
    pub const SDLK_KP_6: C2RustUnnamed = 1073741918;
    #[c2rust::src_loc = "166:5"]
    pub const SDLK_KP_5: C2RustUnnamed = 1073741917;
    #[c2rust::src_loc = "165:5"]
    pub const SDLK_KP_4: C2RustUnnamed = 1073741916;
    #[c2rust::src_loc = "164:5"]
    pub const SDLK_KP_3: C2RustUnnamed = 1073741915;
    #[c2rust::src_loc = "163:5"]
    pub const SDLK_KP_2: C2RustUnnamed = 1073741914;
    #[c2rust::src_loc = "162:5"]
    pub const SDLK_KP_1: C2RustUnnamed = 1073741913;
    #[c2rust::src_loc = "161:5"]
    pub const SDLK_KP_ENTER: C2RustUnnamed = 1073741912;
    #[c2rust::src_loc = "160:5"]
    pub const SDLK_KP_PLUS: C2RustUnnamed = 1073741911;
    #[c2rust::src_loc = "159:5"]
    pub const SDLK_KP_MINUS: C2RustUnnamed = 1073741910;
    #[c2rust::src_loc = "158:5"]
    pub const SDLK_KP_MULTIPLY: C2RustUnnamed = 1073741909;
    #[c2rust::src_loc = "157:5"]
    pub const SDLK_KP_DIVIDE: C2RustUnnamed = 1073741908;
    #[c2rust::src_loc = "156:5"]
    pub const SDLK_NUMLOCKCLEAR: C2RustUnnamed = 1073741907;
    #[c2rust::src_loc = "154:5"]
    pub const SDLK_UP: C2RustUnnamed = 1073741906;
    #[c2rust::src_loc = "153:5"]
    pub const SDLK_DOWN: C2RustUnnamed = 1073741905;
    #[c2rust::src_loc = "152:5"]
    pub const SDLK_LEFT: C2RustUnnamed = 1073741904;
    #[c2rust::src_loc = "151:5"]
    pub const SDLK_RIGHT: C2RustUnnamed = 1073741903;
    #[c2rust::src_loc = "150:5"]
    pub const SDLK_PAGEDOWN: C2RustUnnamed = 1073741902;
    #[c2rust::src_loc = "149:5"]
    pub const SDLK_END: C2RustUnnamed = 1073741901;
    #[c2rust::src_loc = "148:5"]
    pub const SDLK_DELETE: C2RustUnnamed = 127;
    #[c2rust::src_loc = "147:5"]
    pub const SDLK_PAGEUP: C2RustUnnamed = 1073741899;
    #[c2rust::src_loc = "146:5"]
    pub const SDLK_HOME: C2RustUnnamed = 1073741898;
    #[c2rust::src_loc = "145:5"]
    pub const SDLK_INSERT: C2RustUnnamed = 1073741897;
    #[c2rust::src_loc = "144:5"]
    pub const SDLK_PAUSE: C2RustUnnamed = 1073741896;
    #[c2rust::src_loc = "143:5"]
    pub const SDLK_SCROLLLOCK: C2RustUnnamed = 1073741895;
    #[c2rust::src_loc = "142:5"]
    pub const SDLK_PRINTSCREEN: C2RustUnnamed = 1073741894;
    #[c2rust::src_loc = "140:5"]
    pub const SDLK_F12: C2RustUnnamed = 1073741893;
    #[c2rust::src_loc = "139:5"]
    pub const SDLK_F11: C2RustUnnamed = 1073741892;
    #[c2rust::src_loc = "138:5"]
    pub const SDLK_F10: C2RustUnnamed = 1073741891;
    #[c2rust::src_loc = "137:5"]
    pub const SDLK_F9: C2RustUnnamed = 1073741890;
    #[c2rust::src_loc = "136:5"]
    pub const SDLK_F8: C2RustUnnamed = 1073741889;
    #[c2rust::src_loc = "135:5"]
    pub const SDLK_F7: C2RustUnnamed = 1073741888;
    #[c2rust::src_loc = "134:5"]
    pub const SDLK_F6: C2RustUnnamed = 1073741887;
    #[c2rust::src_loc = "133:5"]
    pub const SDLK_F5: C2RustUnnamed = 1073741886;
    #[c2rust::src_loc = "132:5"]
    pub const SDLK_F4: C2RustUnnamed = 1073741885;
    #[c2rust::src_loc = "131:5"]
    pub const SDLK_F3: C2RustUnnamed = 1073741884;
    #[c2rust::src_loc = "130:5"]
    pub const SDLK_F2: C2RustUnnamed = 1073741883;
    #[c2rust::src_loc = "129:5"]
    pub const SDLK_F1: C2RustUnnamed = 1073741882;
    #[c2rust::src_loc = "127:5"]
    pub const SDLK_CAPSLOCK: C2RustUnnamed = 1073741881;
    #[c2rust::src_loc = "125:5"]
    pub const SDLK_z: C2RustUnnamed = 122;
    #[c2rust::src_loc = "124:5"]
    pub const SDLK_y: C2RustUnnamed = 121;
    #[c2rust::src_loc = "123:5"]
    pub const SDLK_x: C2RustUnnamed = 120;
    #[c2rust::src_loc = "122:5"]
    pub const SDLK_w: C2RustUnnamed = 119;
    #[c2rust::src_loc = "121:5"]
    pub const SDLK_v: C2RustUnnamed = 118;
    #[c2rust::src_loc = "120:5"]
    pub const SDLK_u: C2RustUnnamed = 117;
    #[c2rust::src_loc = "119:5"]
    pub const SDLK_t: C2RustUnnamed = 116;
    #[c2rust::src_loc = "118:5"]
    pub const SDLK_s: C2RustUnnamed = 115;
    #[c2rust::src_loc = "117:5"]
    pub const SDLK_r: C2RustUnnamed = 114;
    #[c2rust::src_loc = "116:5"]
    pub const SDLK_q: C2RustUnnamed = 113;
    #[c2rust::src_loc = "115:5"]
    pub const SDLK_p: C2RustUnnamed = 112;
    #[c2rust::src_loc = "114:5"]
    pub const SDLK_o: C2RustUnnamed = 111;
    #[c2rust::src_loc = "113:5"]
    pub const SDLK_n: C2RustUnnamed = 110;
    #[c2rust::src_loc = "112:5"]
    pub const SDLK_m: C2RustUnnamed = 109;
    #[c2rust::src_loc = "111:5"]
    pub const SDLK_l: C2RustUnnamed = 108;
    #[c2rust::src_loc = "110:5"]
    pub const SDLK_k: C2RustUnnamed = 107;
    #[c2rust::src_loc = "109:5"]
    pub const SDLK_j: C2RustUnnamed = 106;
    #[c2rust::src_loc = "108:5"]
    pub const SDLK_i: C2RustUnnamed = 105;
    #[c2rust::src_loc = "107:5"]
    pub const SDLK_h: C2RustUnnamed = 104;
    #[c2rust::src_loc = "106:5"]
    pub const SDLK_g: C2RustUnnamed = 103;
    #[c2rust::src_loc = "105:5"]
    pub const SDLK_f: C2RustUnnamed = 102;
    #[c2rust::src_loc = "104:5"]
    pub const SDLK_e: C2RustUnnamed = 101;
    #[c2rust::src_loc = "103:5"]
    pub const SDLK_d: C2RustUnnamed = 100;
    #[c2rust::src_loc = "102:5"]
    pub const SDLK_c: C2RustUnnamed = 99;
    #[c2rust::src_loc = "101:5"]
    pub const SDLK_b: C2RustUnnamed = 98;
    #[c2rust::src_loc = "100:5"]
    pub const SDLK_a: C2RustUnnamed = 97;
    #[c2rust::src_loc = "99:5"]
    pub const SDLK_BACKQUOTE: C2RustUnnamed = 96;
    #[c2rust::src_loc = "98:5"]
    pub const SDLK_UNDERSCORE: C2RustUnnamed = 95;
    #[c2rust::src_loc = "97:5"]
    pub const SDLK_CARET: C2RustUnnamed = 94;
    #[c2rust::src_loc = "96:5"]
    pub const SDLK_RIGHTBRACKET: C2RustUnnamed = 93;
    #[c2rust::src_loc = "95:5"]
    pub const SDLK_BACKSLASH: C2RustUnnamed = 92;
    #[c2rust::src_loc = "94:5"]
    pub const SDLK_LEFTBRACKET: C2RustUnnamed = 91;
    #[c2rust::src_loc = "90:5"]
    pub const SDLK_AT: C2RustUnnamed = 64;
    #[c2rust::src_loc = "89:5"]
    pub const SDLK_QUESTION: C2RustUnnamed = 63;
    #[c2rust::src_loc = "88:5"]
    pub const SDLK_GREATER: C2RustUnnamed = 62;
    #[c2rust::src_loc = "87:5"]
    pub const SDLK_EQUALS: C2RustUnnamed = 61;
    #[c2rust::src_loc = "86:5"]
    pub const SDLK_LESS: C2RustUnnamed = 60;
    #[c2rust::src_loc = "85:5"]
    pub const SDLK_SEMICOLON: C2RustUnnamed = 59;
    #[c2rust::src_loc = "84:5"]
    pub const SDLK_COLON: C2RustUnnamed = 58;
    #[c2rust::src_loc = "83:5"]
    pub const SDLK_9: C2RustUnnamed = 57;
    #[c2rust::src_loc = "82:5"]
    pub const SDLK_8: C2RustUnnamed = 56;
    #[c2rust::src_loc = "81:5"]
    pub const SDLK_7: C2RustUnnamed = 55;
    #[c2rust::src_loc = "80:5"]
    pub const SDLK_6: C2RustUnnamed = 54;
    #[c2rust::src_loc = "79:5"]
    pub const SDLK_5: C2RustUnnamed = 53;
    #[c2rust::src_loc = "78:5"]
    pub const SDLK_4: C2RustUnnamed = 52;
    #[c2rust::src_loc = "77:5"]
    pub const SDLK_3: C2RustUnnamed = 51;
    #[c2rust::src_loc = "76:5"]
    pub const SDLK_2: C2RustUnnamed = 50;
    #[c2rust::src_loc = "75:5"]
    pub const SDLK_1: C2RustUnnamed = 49;
    #[c2rust::src_loc = "74:5"]
    pub const SDLK_0: C2RustUnnamed = 48;
    #[c2rust::src_loc = "73:5"]
    pub const SDLK_SLASH: C2RustUnnamed = 47;
    #[c2rust::src_loc = "72:5"]
    pub const SDLK_PERIOD: C2RustUnnamed = 46;
    #[c2rust::src_loc = "71:5"]
    pub const SDLK_MINUS: C2RustUnnamed = 45;
    #[c2rust::src_loc = "70:5"]
    pub const SDLK_COMMA: C2RustUnnamed = 44;
    #[c2rust::src_loc = "69:5"]
    pub const SDLK_PLUS: C2RustUnnamed = 43;
    #[c2rust::src_loc = "68:5"]
    pub const SDLK_ASTERISK: C2RustUnnamed = 42;
    #[c2rust::src_loc = "67:5"]
    pub const SDLK_RIGHTPAREN: C2RustUnnamed = 41;
    #[c2rust::src_loc = "66:5"]
    pub const SDLK_LEFTPAREN: C2RustUnnamed = 40;
    #[c2rust::src_loc = "65:5"]
    pub const SDLK_QUOTE: C2RustUnnamed = 39;
    #[c2rust::src_loc = "64:5"]
    pub const SDLK_AMPERSAND: C2RustUnnamed = 38;
    #[c2rust::src_loc = "63:5"]
    pub const SDLK_DOLLAR: C2RustUnnamed = 36;
    #[c2rust::src_loc = "62:5"]
    pub const SDLK_PERCENT: C2RustUnnamed = 37;
    #[c2rust::src_loc = "61:5"]
    pub const SDLK_HASH: C2RustUnnamed = 35;
    #[c2rust::src_loc = "60:5"]
    pub const SDLK_QUOTEDBL: C2RustUnnamed = 34;
    #[c2rust::src_loc = "59:5"]
    pub const SDLK_EXCLAIM: C2RustUnnamed = 33;
    #[c2rust::src_loc = "58:5"]
    pub const SDLK_SPACE: C2RustUnnamed = 32;
    #[c2rust::src_loc = "57:5"]
    pub const SDLK_TAB: C2RustUnnamed = 9;
    #[c2rust::src_loc = "56:5"]
    pub const SDLK_BACKSPACE: C2RustUnnamed = 8;
    #[c2rust::src_loc = "55:5"]
    pub const SDLK_ESCAPE: C2RustUnnamed = 27;
    #[c2rust::src_loc = "54:5"]
    pub const SDLK_RETURN: C2RustUnnamed = 13;
    #[c2rust::src_loc = "52:5"]
    pub const SDLK_UNKNOWN: C2RustUnnamed = 0;
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:57"]
pub mod catdefs_h {
    #[c2rust::src_loc = "35:1"]
    pub type boolean = uint16_t;
    #[c2rust::src_loc = "37:1"]
    pub type byte = uint8_t;
    #[c2rust::src_loc = "39:1"]
    pub type word = uint16_t;
    #[c2rust::src_loc = "40:1"]
    pub type sword = int16_t;
    #[c2rust::src_loc = "41:1"]
    pub type dword = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct farptr {
        pub ofs: word,
        pub seg: word,
    }
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
    #[inline]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn flatptr(mut ptr: farptr) -> dword {
        return (((ptr.seg as libc::c_int) << 4 as libc::c_int) + ptr.ofs as libc::c_int) as dword;
    }
    #[inline]
    #[c2rust::src_loc = "53:1"]
    pub unsafe extern "C" fn itoa(
        mut value: libc::c_int,
        mut str: *mut libc::c_char,
        mut base: libc::c_int,
    ) -> *mut libc::c_char {
        if base == 16 as libc::c_int {
            sprintf(str, b"%X\0" as *const u8 as *const libc::c_char, value);
        } else {
            sprintf(str, b"%d\0" as *const u8 as *const libc::c_char, value);
        }
        return str;
    }
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
    use super::stdio_h::sprintf;
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/pcrlib.h:58"]
pub mod pcrlib_h {
    #[c2rust::src_loc = "47:9"]
    pub type soundtype = libc::c_uint;
    #[c2rust::src_loc = "47:24"]
    pub const sdlib: soundtype = 2;
    #[c2rust::src_loc = "47:19"]
    pub const spkr: soundtype = 1;
    #[c2rust::src_loc = "47:15"]
    pub const off: soundtype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct ControlStruct {
        pub dir: dirtype,
        pub button1: boolean,
        pub button2: boolean,
    }
    #[c2rust::src_loc = "87:9"]
    pub type inputtype = libc::c_uint;
    #[c2rust::src_loc = "87:50"]
    pub const demo: inputtype = 4;
    #[c2rust::src_loc = "87:40"]
    pub const joystick2: inputtype = 3;
    #[c2rust::src_loc = "87:30"]
    pub const joystick1: inputtype = 2;
    #[c2rust::src_loc = "87:24"]
    pub const mouse: inputtype = 1;
    #[c2rust::src_loc = "87:15"]
    pub const keyboard: inputtype = 0;
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
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "191:9"]
    pub struct spritetype {
        pub width: sword,
        pub height: sword,
        pub shapeptr: dword,
        pub maskptr: dword,
        pub xl: sword,
        pub yl: sword,
        pub xh: sword,
        pub yh: sword,
        pub name: [libc::c_char; 12],
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "200:9"]
    pub struct pictype {
        pub width: sword,
        pub height: sword,
        pub shapeptr: dword,
        pub name: [libc::c_char; 8],
    }
    use super::catdefs_h::{boolean, dirtype, dword, sword};
    use super::SDL_scancode_h::{SDL_Scancode, SDL_SCANCODE_UNKNOWN};
    extern "C" {
        #[c2rust::src_loc = "263:1"]
        pub fn loadgrfiles();
        #[c2rust::src_loc = "249:1"]
        pub fn drawwindow(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int);
        #[c2rust::src_loc = "310:1"]
        pub fn bioskey(_: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "239:1"]
        pub fn get() -> libc::c_int;
        #[c2rust::src_loc = "253:1"]
        pub fn expwin(width: libc::c_int, height: libc::c_int);
        #[c2rust::src_loc = "250:1"]
        pub fn erasewindow();
        #[c2rust::src_loc = "286:32"]
        pub static mut _vgaok: boolean;
        #[c2rust::src_loc = "286:24"]
        pub static mut _egaok: boolean;
        #[c2rust::src_loc = "242:1"]
        pub fn print(str: *const libc::c_char);
        #[c2rust::src_loc = "264:1"]
        pub fn repaintscreen();
        #[c2rust::src_loc = "236:27"]
        pub static mut screencentery: libc::c_int;
        #[c2rust::src_loc = "236:12"]
        pub static mut screencenterx: libc::c_int;
        #[c2rust::src_loc = "227:1"]
        pub fn drawpic(x: libc::c_int, y: libc::c_int, picnum: libc::c_int);
        #[c2rust::src_loc = "226:1"]
        pub fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
        #[c2rust::src_loc = "217:14"]
        pub static mut egaplaneofs: [dword; 4];
        #[c2rust::src_loc = "216:14"]
        pub static mut spriteptr: *mut libc::c_void;
        #[c2rust::src_loc = "215:14"]
        pub static mut picptr: *mut libc::c_void;
        #[c2rust::src_loc = "214:14"]
        pub static mut tileptr: *mut libc::c_void;
        #[c2rust::src_loc = "213:14"]
        pub static mut charptr: *mut libc::c_void;
        #[c2rust::src_loc = "179:1"]
        pub fn WaitVBL();
        #[c2rust::src_loc = "178:1"]
        pub fn UpdateScreen();
        #[c2rust::src_loc = "175:27"]
        pub static mut xormask: libc::c_int;
        #[c2rust::src_loc = "175:18"]
        pub static mut leftedge: libc::c_int;
        #[c2rust::src_loc = "175:15"]
        pub static mut sy: libc::c_int;
        #[c2rust::src_loc = "175:12"]
        pub static mut sx: libc::c_int;
        #[c2rust::src_loc = "173:15"]
        pub static mut grmode: grtype;
        #[c2rust::src_loc = "147:1"]
        pub fn clearkeys();
        #[c2rust::src_loc = "138:1"]
        pub fn bloadin(filename: *mut libc::c_char) -> *mut libc::c_void;
        #[c2rust::src_loc = "122:1"]
        pub fn ControlJoystick(joynum: libc::c_int) -> ControlStruct;
        #[c2rust::src_loc = "118:1"]
        pub fn ReadJoystick(
            joynum: libc::c_int,
            xcount: *mut libc::c_int,
            ycount: *mut libc::c_int,
        );
        #[c2rust::src_loc = "117:1"]
        pub fn ProbeJoysticks();
        #[c2rust::src_loc = "115:1"]
        pub fn CheckMouseMode();
        #[c2rust::src_loc = "114:1"]
        pub fn ScancodeToDOS(sc: SDL_Scancode) -> libc::c_int;
        #[c2rust::src_loc = "113:1"]
        pub fn ProcessEvents();
        #[c2rust::src_loc = "93:27"]
        pub static mut keyB2: libc::c_int;
        #[c2rust::src_loc = "93:20"]
        pub static mut keyB1: libc::c_int;
        #[c2rust::src_loc = "93:12"]
        pub static mut key: [libc::c_int; 8];
        #[c2rust::src_loc = "92:12"]
        pub static mut MouseSensitivity: libc::c_int;
        #[c2rust::src_loc = "91:50"]
        pub static mut JoyYhigh: [libc::c_int; 3];
        #[c2rust::src_loc = "91:37"]
        pub static mut JoyYlow: [libc::c_int; 3];
        #[c2rust::src_loc = "91:24"]
        pub static mut JoyXhigh: [libc::c_int; 3];
        #[c2rust::src_loc = "91:12"]
        pub static mut JoyXlow: [libc::c_int; 3];
        #[c2rust::src_loc = "90:16"]
        pub static mut keydown: [boolean; 512];
        #[c2rust::src_loc = "89:18"]
        pub static mut playermode: [inputtype; 3];
        #[c2rust::src_loc = "72:1"]
        pub fn ContinueSound();
        #[c2rust::src_loc = "71:1"]
        pub fn PauseSound();
        #[c2rust::src_loc = "63:18"]
        pub static mut soundmode: soundtype;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "565:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:55"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_joystick.h:58"]
pub mod SDL_joystick_h {
    extern "C" {
        #[c2rust::src_loc = "126:1"]
        pub fn SDL_NumJoysticks() -> libc::c_int;
    }
}
pub use self::catdefs_h::{
    boolean, byte, dirtype, dword, east, farptr, flatptr, itoa, nodir, north, northeast, northwest,
    south, southeast, southwest, sword, west, word,
};
pub use self::pcrlib_h::{
    _egaok, _vgaok, bioskey, bloadin, charptr, clearkeys, demo, drawchar, drawpic, drawwindow,
    egaplaneofs, erasewindow, expwin, get, grmode, grtype, inputtype, joystick1, joystick2, key,
    keyB1, keyB2, keyboard, keydown, leftedge, loadgrfiles, mouse, off, picptr, pictype,
    playermode, print, repaintscreen, screencenterx, screencentery, sdlib, soundmode, soundtype,
    spkr, spriteptr, spritetype, sx, sy, text, tileptr, xormask, CGAgr, CheckMouseMode,
    ContinueSound, ControlJoystick, ControlStruct, EGAgr, JoyXhigh, JoyXlow, JoyYhigh, JoyYlow,
    MouseSensitivity, PauseSound, ProbeJoysticks, ProcessEvents, ReadJoystick, ScancodeToDOS,
    UpdateScreen, VGAgr, WaitVBL,
};
pub use self::stdint_intn_h::int16_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdio_h::sprintf;
use self::stdlib_h::free;
pub use self::types_h::{__int16_t, __uint16_t, __uint32_t, __uint8_t};
use self::SDL_joystick_h::SDL_NumJoysticks;
pub use self::SDL_keycode_h::{
    C2RustUnnamed, SDLK_a, SDLK_b, SDLK_c, SDLK_d, SDLK_e, SDLK_f, SDLK_g, SDLK_h, SDLK_i, SDLK_j,
    SDLK_k, SDLK_l, SDLK_m, SDLK_n, SDLK_o, SDLK_p, SDLK_q, SDLK_r, SDLK_s, SDLK_t, SDLK_u, SDLK_v,
    SDLK_w, SDLK_x, SDLK_y, SDLK_z, SDLK_0, SDLK_1, SDLK_2, SDLK_3, SDLK_4, SDLK_5, SDLK_6, SDLK_7,
    SDLK_8, SDLK_9, SDLK_AC_BACK, SDLK_AC_BOOKMARKS, SDLK_AC_FORWARD, SDLK_AC_HOME,
    SDLK_AC_REFRESH, SDLK_AC_SEARCH, SDLK_AC_STOP, SDLK_AGAIN, SDLK_ALTERASE, SDLK_AMPERSAND,
    SDLK_APP1, SDLK_APP2, SDLK_APPLICATION, SDLK_ASTERISK, SDLK_AT, SDLK_AUDIOFASTFORWARD,
    SDLK_AUDIOMUTE, SDLK_AUDIONEXT, SDLK_AUDIOPLAY, SDLK_AUDIOPREV, SDLK_AUDIOREWIND,
    SDLK_AUDIOSTOP, SDLK_BACKQUOTE, SDLK_BACKSLASH, SDLK_BACKSPACE, SDLK_BRIGHTNESSDOWN,
    SDLK_BRIGHTNESSUP, SDLK_CALCULATOR, SDLK_CANCEL, SDLK_CAPSLOCK, SDLK_CARET, SDLK_CLEAR,
    SDLK_CLEARAGAIN, SDLK_COLON, SDLK_COMMA, SDLK_COMPUTER, SDLK_COPY, SDLK_CRSEL,
    SDLK_CURRENCYSUBUNIT, SDLK_CURRENCYUNIT, SDLK_CUT, SDLK_DECIMALSEPARATOR, SDLK_DELETE,
    SDLK_DISPLAYSWITCH, SDLK_DOLLAR, SDLK_DOWN, SDLK_EJECT, SDLK_END, SDLK_EQUALS, SDLK_ESCAPE,
    SDLK_EXCLAIM, SDLK_EXECUTE, SDLK_EXSEL, SDLK_F1, SDLK_F10, SDLK_F11, SDLK_F12, SDLK_F13,
    SDLK_F14, SDLK_F15, SDLK_F16, SDLK_F17, SDLK_F18, SDLK_F19, SDLK_F2, SDLK_F20, SDLK_F21,
    SDLK_F22, SDLK_F23, SDLK_F24, SDLK_F3, SDLK_F4, SDLK_F5, SDLK_F6, SDLK_F7, SDLK_F8, SDLK_F9,
    SDLK_FIND, SDLK_GREATER, SDLK_HASH, SDLK_HELP, SDLK_HOME, SDLK_INSERT, SDLK_KBDILLUMDOWN,
    SDLK_KBDILLUMTOGGLE, SDLK_KBDILLUMUP, SDLK_KP_0, SDLK_KP_00, SDLK_KP_000, SDLK_KP_1, SDLK_KP_2,
    SDLK_KP_3, SDLK_KP_4, SDLK_KP_5, SDLK_KP_6, SDLK_KP_7, SDLK_KP_8, SDLK_KP_9, SDLK_KP_A,
    SDLK_KP_AMPERSAND, SDLK_KP_AT, SDLK_KP_B, SDLK_KP_BACKSPACE, SDLK_KP_BINARY, SDLK_KP_C,
    SDLK_KP_CLEAR, SDLK_KP_CLEARENTRY, SDLK_KP_COLON, SDLK_KP_COMMA, SDLK_KP_D,
    SDLK_KP_DBLAMPERSAND, SDLK_KP_DBLVERTICALBAR, SDLK_KP_DECIMAL, SDLK_KP_DIVIDE, SDLK_KP_E,
    SDLK_KP_ENTER, SDLK_KP_EQUALS, SDLK_KP_EQUALSAS400, SDLK_KP_EXCLAM, SDLK_KP_F, SDLK_KP_GREATER,
    SDLK_KP_HASH, SDLK_KP_HEXADECIMAL, SDLK_KP_LEFTBRACE, SDLK_KP_LEFTPAREN, SDLK_KP_LESS,
    SDLK_KP_MEMADD, SDLK_KP_MEMCLEAR, SDLK_KP_MEMDIVIDE, SDLK_KP_MEMMULTIPLY, SDLK_KP_MEMRECALL,
    SDLK_KP_MEMSTORE, SDLK_KP_MEMSUBTRACT, SDLK_KP_MINUS, SDLK_KP_MULTIPLY, SDLK_KP_OCTAL,
    SDLK_KP_PERCENT, SDLK_KP_PERIOD, SDLK_KP_PLUS, SDLK_KP_PLUSMINUS, SDLK_KP_POWER,
    SDLK_KP_RIGHTBRACE, SDLK_KP_RIGHTPAREN, SDLK_KP_SPACE, SDLK_KP_TAB, SDLK_KP_VERTICALBAR,
    SDLK_KP_XOR, SDLK_LALT, SDLK_LCTRL, SDLK_LEFT, SDLK_LEFTBRACKET, SDLK_LEFTPAREN, SDLK_LESS,
    SDLK_LGUI, SDLK_LSHIFT, SDLK_MAIL, SDLK_MEDIASELECT, SDLK_MENU, SDLK_MINUS, SDLK_MODE,
    SDLK_MUTE, SDLK_NUMLOCKCLEAR, SDLK_OPER, SDLK_OUT, SDLK_PAGEDOWN, SDLK_PAGEUP, SDLK_PASTE,
    SDLK_PAUSE, SDLK_PERCENT, SDLK_PERIOD, SDLK_PLUS, SDLK_POWER, SDLK_PRINTSCREEN, SDLK_PRIOR,
    SDLK_QUESTION, SDLK_QUOTE, SDLK_QUOTEDBL, SDLK_RALT, SDLK_RCTRL, SDLK_RETURN, SDLK_RETURN2,
    SDLK_RGUI, SDLK_RIGHT, SDLK_RIGHTBRACKET, SDLK_RIGHTPAREN, SDLK_RSHIFT, SDLK_SCROLLLOCK,
    SDLK_SELECT, SDLK_SEMICOLON, SDLK_SEPARATOR, SDLK_SLASH, SDLK_SLEEP, SDLK_SPACE, SDLK_STOP,
    SDLK_SYSREQ, SDLK_TAB, SDLK_THOUSANDSSEPARATOR, SDLK_UNDERSCORE, SDLK_UNDO, SDLK_UNKNOWN,
    SDLK_UP, SDLK_VOLUMEDOWN, SDLK_VOLUMEUP, SDLK_WWW,
};
pub use self::SDL_scancode_h::{
    SDL_Scancode, SDL_NUM_SCANCODES, SDL_SCANCODE_0, SDL_SCANCODE_1, SDL_SCANCODE_2,
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
#[c2rust::src_loc = "610:3"]
pub type stype = [spritetype; 10];
#[c2rust::src_loc = "609:3"]
pub type ptype = [pictype; 64];
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "612:11"]
pub struct picfiletype {
    pub charptr: farptr,
    pub tileptr: farptr,
    pub picptr: farptr,
    pub spriteptr: farptr,
    pub pictableptr: farptr,
    pub spritetableptr: farptr,
    pub plane: [farptr; 4],
    pub numchars: sword,
    pub numtiles: sword,
    pub numpics: sword,
    pub numsprites: sword,
}
#[no_mangle]
#[c2rust::src_loc = "60:5"]
pub static mut rowy: [libc::c_int; 4] = [
    4 as libc::c_int,
    9 as libc::c_int,
    14 as libc::c_int,
    19 as libc::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "61:5"]
pub static mut collumnx: [libc::c_int; 4] = [
    14 as libc::c_int,
    20 as libc::c_int,
    26 as libc::c_int,
    32 as libc::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "62:5"]
pub static mut spotok: [[libc::c_int; 5]; 4] = [[0; 5]; 4];
#[no_mangle]
#[c2rust::src_loc = "64:5"]
pub static mut row: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "64:9"]
pub static mut collumn: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "65:8"]
pub static mut oldgrmode: grtype = text;
#[no_mangle]
#[c2rust::src_loc = "65:19"]
pub static mut newgrmode: grtype = text;
#[no_mangle]
#[c2rust::src_loc = "66:11"]
pub static mut oldsoundmode: soundtype = off;
#[no_mangle]
#[c2rust::src_loc = "66:25"]
pub static mut newsoundmode: soundtype = off;
#[no_mangle]
#[c2rust::src_loc = "67:11"]
pub static mut oldplayermode: [inputtype; 3] = [keyboard; 3];
#[no_mangle]
#[c2rust::src_loc = "67:29"]
pub static mut newplayermode: [inputtype; 3] = [keyboard; 3];
#[no_mangle]
#[c2rust::src_loc = "69:5"]
pub static mut joy1ok: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "69:12"]
pub static mut joy2ok: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "69:19"]
pub static mut mouseok: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn calibratejoy(mut joynum: libc::c_int) {
    let mut current_block: u64;
    let mut stage: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut xl: libc::c_int = 0;
    let mut yl: libc::c_int = 0;
    let mut xh: libc::c_int = 0;
    let mut yh: libc::c_int = 0;
    let mut ctr: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    expwin(24 as libc::c_int, 9 as libc::c_int);
    print(b" Joystick Configuration\n\r\0" as *const u8 as *const libc::c_char);
    print(b" ----------------------\n\r\0" as *const u8 as *const libc::c_char);
    print(b"Hold the joystick in the\n\r\0" as *const u8 as *const libc::c_char);
    print(b"upper left\n\r\0" as *const u8 as *const libc::c_char);
    print(b"corner and hit fire:\0" as *const u8 as *const libc::c_char);
    stage = 15 as libc::c_int;
    loop {
        drawchar(sx, sy, stage);
        UpdateScreen();
        WaitVBL();
        WaitVBL();
        WaitVBL();
        stage += 1;
        if stage == 23 as libc::c_int {
            stage = 15 as libc::c_int;
        }
        ProcessEvents();
        ReadJoystick(joynum, &mut xl, &mut yl);
        ctr = ControlJoystick(joynum);
        if keydown[SDL_SCANCODE_ESCAPE as libc::c_int as usize] != 0 {
            current_block = 15976468122069307450;
            break;
        }
        if !(ctr.button1 as libc::c_int != 1 as libc::c_int) {
            current_block = 8457315219000651999;
            break;
        }
    }
    match current_block {
        8457315219000651999 => {
            drawchar(sx, sy, ' ' as i32);
            loop {
                ctr = ControlJoystick(joynum);
                if !(ctr.button1 != 0) {
                    break;
                }
            }
            UpdateScreen();
            WaitVBL();
            WaitVBL();
            print(b"\n\n\rHold the joystick in the\n\r\0" as *const u8 as *const libc::c_char);
            print(b"lower right\n\r\0" as *const u8 as *const libc::c_char);
            print(b"corner and hit fire:\0" as *const u8 as *const libc::c_char);
            loop {
                drawchar(sx, sy, stage);
                UpdateScreen();
                WaitVBL();
                WaitVBL();
                WaitVBL();
                stage += 1;
                if stage == 23 as libc::c_int {
                    stage = 15 as libc::c_int;
                }
                ProcessEvents();
                ReadJoystick(joynum, &mut xh, &mut yh);
                ctr = ControlJoystick(joynum);
                if keydown[SDL_SCANCODE_ESCAPE as libc::c_int as usize] != 0 {
                    current_block = 15976468122069307450;
                    break;
                }
                if !(ctr.button1 as libc::c_int != 1 as libc::c_int) {
                    current_block = 15597372965620363352;
                    break;
                }
            }
            match current_block {
                15976468122069307450 => {}
                _ => {
                    drawchar(sx, sy, ' ' as i32);
                    loop {
                        ctr = ControlJoystick(joynum);
                        if !(ctr.button1 != 0) {
                            break;
                        }
                    }
                    UpdateScreen();
                    dx = (xh - xl) / 4 as libc::c_int;
                    dy = (yh - yl) / 4 as libc::c_int;
                    JoyXlow[joynum as usize] = xl + dx;
                    JoyXhigh[joynum as usize] = xh - dx;
                    JoyYlow[joynum as usize] = yl + dy;
                    JoyYhigh[joynum as usize] = yh - dy;
                }
            }
        }
        _ => {}
    }
    clearkeys();
    erasewindow();
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub unsafe extern "C" fn calibratemouse() {
    let mut ch: libc::c_char = 0;
    expwin(24 as libc::c_int, 5 as libc::c_int);
    print(b"  Mouse Configuration   \n\r\0" as *const u8 as *const libc::c_char);
    print(b"  -------------------   \n\r\0" as *const u8 as *const libc::c_char);
    print(b"Choose the sensitivity  \n\r\0" as *const u8 as *const libc::c_char);
    print(b"of the mouse, 1 being   \n\r\0" as *const u8 as *const libc::c_char);
    print(b"slow, 9 being fast:\0" as *const u8 as *const libc::c_char);
    loop {
        ch = (get() % 256 as libc::c_int) as libc::c_char;
        if ch as libc::c_int == 27 as libc::c_int {
            ch = '5' as i32 as libc::c_char;
        }
        if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
            break;
        }
    }
    MouseSensitivity = 15 as libc::c_int - (ch as libc::c_int - '0' as i32);
    erasewindow();
}
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn printscan(mut sc: libc::c_int) {
    static mut chartable: [libc::c_char; 128] = [
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        'Q' as i32 as libc::c_char,
        'W' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'Y' as i32 as libc::c_char,
        'U' as i32 as libc::c_char,
        'I' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        '[' as i32 as libc::c_char,
        ']' as i32 as libc::c_char,
        '|' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'F' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'J' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        ';' as i32 as libc::c_char,
        '"' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        'Z' as i32 as libc::c_char,
        'X' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'V' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        'N' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        ',' as i32 as libc::c_char,
        '.' as i32 as libc::c_char,
        '/' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        15 as libc::c_int as libc::c_char,
        '?' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        21 as libc::c_int as libc::c_char,
        '5' as i32 as libc::c_char,
        17 as libc::c_int as libc::c_char,
        '+' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        19 as libc::c_int as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
    ];
    sc = ScancodeToDOS(sc as SDL_Scancode);
    if sc == 1 as libc::c_int {
        print(b"ESC\0" as *const u8 as *const libc::c_char);
    } else if sc == 0xe as libc::c_int {
        print(b"BKSP\0" as *const u8 as *const libc::c_char);
    } else if sc == 0xf as libc::c_int {
        print(b"TAB\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x1d as libc::c_int {
        print(b"CTRL\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x2a as libc::c_int {
        print(b"LSHIFT\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x39 as libc::c_int {
        print(b"SPACE\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x3a as libc::c_int {
        print(b"CAPSLK\0" as *const u8 as *const libc::c_char);
    } else if sc >= 0x3b as libc::c_int && sc <= 0x44 as libc::c_int {
        let mut str: [libc::c_char; 3] = [0; 3];
        print(b"F\0" as *const u8 as *const libc::c_char);
        itoa(
            sc - 0x3a as libc::c_int,
            str.as_mut_ptr(),
            10 as libc::c_int,
        );
        print(str.as_mut_ptr());
    } else if sc == 0x57 as libc::c_int {
        print(b"F11\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x59 as libc::c_int {
        print(b"F12\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x46 as libc::c_int {
        print(b"SCRLLK\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x1c as libc::c_int {
        print(b"ENTER\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x36 as libc::c_int {
        print(b"RSHIFT\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x37 as libc::c_int {
        print(b"PRTSC\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x38 as libc::c_int {
        print(b"ALT\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x47 as libc::c_int {
        print(b"HOME\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x49 as libc::c_int {
        print(b"PGUP\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x4f as libc::c_int {
        print(b"END\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x51 as libc::c_int {
        print(b"PGDN\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x52 as libc::c_int {
        print(b"INS\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x53 as libc::c_int {
        print(b"DEL\0" as *const u8 as *const libc::c_char);
    } else if sc == 0x45 as libc::c_int {
        print(b"NUMLK\0" as *const u8 as *const libc::c_char);
    } else {
        let fresh0 = sx;
        sx = sx + 1;
        drawchar(fresh0, sy, chartable[sc as usize] as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "276:1"]
pub unsafe extern "C" fn calibratekeys() {
    let mut ch: libc::c_char = 0;
    let mut hx: libc::c_int = 0;
    let mut hy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut select: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    expwin(22 as libc::c_int, 15 as libc::c_int);
    print(b"Keyboard Configuration\n\r\0" as *const u8 as *const libc::c_char);
    print(b"----------------------\0" as *const u8 as *const libc::c_char);
    print(b"\n\r0 north    :\0" as *const u8 as *const libc::c_char);
    print(b"\n\r1 east     :\0" as *const u8 as *const libc::c_char);
    print(b"\n\r2 south    :\0" as *const u8 as *const libc::c_char);
    print(b"\n\r3 west     :\0" as *const u8 as *const libc::c_char);
    print(b"\n\r4 northeast:\0" as *const u8 as *const libc::c_char);
    print(b"\n\r5 southeast:\0" as *const u8 as *const libc::c_char);
    print(b"\n\r6 southwest:\0" as *const u8 as *const libc::c_char);
    print(b"\n\r7 northwest:\0" as *const u8 as *const libc::c_char);
    print(b"\n\r8 button1  :\0" as *const u8 as *const libc::c_char);
    print(b"\n\r9 button2  :\0" as *const u8 as *const libc::c_char);
    print(b"\n\n\rModify which action:\0" as *const u8 as *const libc::c_char);
    hx = sx;
    hy = sy;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        sx = 22 as libc::c_int;
        sy = 7 as libc::c_int + i;
        printscan(key[i as usize]);
        i += 1;
    }
    sx = 22 as libc::c_int;
    sy = 15 as libc::c_int;
    printscan(keyB1);
    sx = 22 as libc::c_int;
    sy = 16 as libc::c_int;
    printscan(keyB2);
    loop {
        sx = hx;
        sy = hy;
        ch = (get() % 256 as libc::c_int) as libc::c_char;
        if !((ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32) {
            select = ch as libc::c_int - '0' as i32;
            drawchar(sx, sy, ch as libc::c_int);
            select = ch as libc::c_int - '0' as i32;
            print(b"\n\rPress the new key:\0" as *const u8 as *const libc::c_char);
            clearkeys();
            UpdateScreen();
            loop {
                new = bioskey(1 as libc::c_int);
                if !(new == 0 as libc::c_int) {
                    break;
                }
                WaitVBL();
            }
            clearkeys();
            print(b"\r                  \0" as *const u8 as *const libc::c_char);
            if select < 8 as libc::c_int {
                key[select as usize] = new;
            }
            if select == 8 as libc::c_int {
                keyB1 = new;
            }
            if select == 9 as libc::c_int {
                keyB2 = new;
            }
            sy = select + 7 as libc::c_int;
            sx = 22 as libc::c_int;
            print(b"        \0" as *const u8 as *const libc::c_char);
            sx = 22 as libc::c_int;
            printscan(new);
            ch = '0' as i32 as libc::c_char;
            clearkeys();
        }
        if !(ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32) {
            break;
        }
    }
    erasewindow();
}
#[no_mangle]
#[c2rust::src_loc = "355:1"]
pub unsafe extern "C" fn getconfig() {
    spotok[0 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    spotok[0 as libc::c_int as usize][1 as libc::c_int as usize] = _egaok as libc::c_int;
    spotok[0 as libc::c_int as usize][2 as libc::c_int as usize] = _vgaok as libc::c_int;
    spotok[0 as libc::c_int as usize][3 as libc::c_int as usize] = 0 as libc::c_int;
    spotok[0 as libc::c_int as usize][4 as libc::c_int as usize] = 0 as libc::c_int;
    spotok[1 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    spotok[1 as libc::c_int as usize][1 as libc::c_int as usize] = 1 as libc::c_int;
    spotok[1 as libc::c_int as usize][2 as libc::c_int as usize] = 0 as libc::c_int;
    spotok[1 as libc::c_int as usize][3 as libc::c_int as usize] = 0 as libc::c_int;
    spotok[1 as libc::c_int as usize][4 as libc::c_int as usize] = 0 as libc::c_int;
    let mut numjoy: libc::c_int = SDL_NumJoysticks();
    joy1ok = (numjoy > 0 as libc::c_int) as libc::c_int;
    joy2ok = (numjoy > 1 as libc::c_int) as libc::c_int;
    mouseok = 1 as libc::c_int;
    spotok[2 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    spotok[2 as libc::c_int as usize][1 as libc::c_int as usize] = mouseok;
    spotok[2 as libc::c_int as usize][2 as libc::c_int as usize] = joy1ok;
    spotok[2 as libc::c_int as usize][3 as libc::c_int as usize] = joy2ok;
    spotok[2 as libc::c_int as usize][4 as libc::c_int as usize] = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "393:1"]
pub unsafe extern "C" fn drawpanel() {
    leftedge = 1 as libc::c_int;
    xormask = 0 as libc::c_int;
    sx = 8 as libc::c_int;
    sy = 2 as libc::c_int;
    print(b"       Control Panel      \n\r\0" as *const u8 as *const libc::c_char);
    getconfig();
    sy = rowy[0 as libc::c_int as usize] + 2 as libc::c_int;
    sx = 2 as libc::c_int;
    print(b"VIDEO:\0" as *const u8 as *const libc::c_char);
    drawpic(
        collumnx[0 as libc::c_int as usize] * 8 as libc::c_int,
        rowy[0 as libc::c_int as usize] * 8 as libc::c_int,
        0 as libc::c_int,
    );
    if _egaok != 0 {
        drawpic(
            collumnx[1 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[0 as libc::c_int as usize] * 8 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        drawpic(
            collumnx[1 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[0 as libc::c_int as usize] * 8 as libc::c_int,
            3 as libc::c_int,
        );
    }
    sy = rowy[1 as libc::c_int as usize] + 2 as libc::c_int;
    sx = 2 as libc::c_int;
    print(b"SOUND:\0" as *const u8 as *const libc::c_char);
    drawpic(
        collumnx[0 as libc::c_int as usize] * 8 as libc::c_int,
        rowy[1 as libc::c_int as usize] * 8 as libc::c_int,
        5 as libc::c_int,
    );
    drawpic(
        collumnx[1 as libc::c_int as usize] * 8 as libc::c_int,
        rowy[1 as libc::c_int as usize] * 8 as libc::c_int,
        6 as libc::c_int,
    );
    sy = rowy[2 as libc::c_int as usize] + 2 as libc::c_int;
    sx = 2 as libc::c_int;
    print(b"CONTROL:\0" as *const u8 as *const libc::c_char);
    drawpic(
        collumnx[0 as libc::c_int as usize] * 8 as libc::c_int,
        rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
        7 as libc::c_int,
    );
    if mouseok != 0 {
        drawpic(
            collumnx[1 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            10 as libc::c_int,
        );
    } else {
        drawpic(
            collumnx[1 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            12 as libc::c_int,
        );
    }
    if joy1ok != 0 {
        drawpic(
            collumnx[2 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        drawpic(
            collumnx[2 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            11 as libc::c_int,
        );
    }
    if joy2ok != 0 {
        drawpic(
            collumnx[3 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            9 as libc::c_int,
        );
    } else {
        drawpic(
            collumnx[3 as libc::c_int as usize] * 8 as libc::c_int,
            rowy[2 as libc::c_int as usize] * 8 as libc::c_int,
            11 as libc::c_int,
        );
    }
    drawchar(
        collumnx[(newgrmode as libc::c_int - 1 as libc::c_int) as usize] + 1 as libc::c_int,
        rowy[0 as libc::c_int as usize] + 3 as libc::c_int,
        15 as libc::c_int,
    );
    drawchar(
        collumnx[newsoundmode as libc::c_int as usize] + 1 as libc::c_int,
        rowy[1 as libc::c_int as usize] + 3 as libc::c_int,
        15 as libc::c_int,
    );
    drawchar(
        collumnx[newplayermode[1 as libc::c_int as usize] as libc::c_int as usize]
            + 1 as libc::c_int,
        rowy[2 as libc::c_int as usize] + 3 as libc::c_int,
        15 as libc::c_int,
    );
    sy = 21 as libc::c_int;
    sx = 1 as libc::c_int;
    print(b"  Move the cursor with the arrow keys \n\r\0" as *const u8 as *const libc::c_char);
    print(b"   Make decisions with the ENTER key  \n\r\0" as *const u8 as *const libc::c_char);
    print(b"       ESC to return to your game     \n\r\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn controlpanel() {
    let mut chf: libc::c_int = 0;
    let mut oldcenterx: libc::c_int = 0;
    let mut oldcentery: libc::c_int = 0;
    clearkeys();
    PauseSound();
    ProbeJoysticks();
    oldgrmode = grmode;
    newgrmode = oldgrmode;
    oldsoundmode = soundmode;
    newsoundmode = oldsoundmode;
    oldplayermode[1 as libc::c_int as usize] = playermode[1 as libc::c_int as usize];
    newplayermode[1 as libc::c_int as usize] = oldplayermode[1 as libc::c_int as usize];
    oldplayermode[2 as libc::c_int as usize] = playermode[2 as libc::c_int as usize];
    newplayermode[2 as libc::c_int as usize] = oldplayermode[2 as libc::c_int as usize];
    oldcenterx = screencenterx;
    oldcentery = screencentery;
    screencenterx = 19 as libc::c_int;
    screencentery = 11 as libc::c_int;
    drawwindow(
        0 as libc::c_int,
        0 as libc::c_int,
        39 as libc::c_int,
        24 as libc::c_int,
    );
    drawpanel();
    row = 0 as libc::c_int;
    collumn = grmode as libc::c_int - 1 as libc::c_int;
    loop {
        sx = collumnx[collumn as usize] + 2 as libc::c_int;
        sy = rowy[row as usize] + 3 as libc::c_int;
        chf = get();
        if chf == SDLK_UP as libc::c_int {
            row -= 1;
            if row < 0 as libc::c_int {
                row = 2 as libc::c_int;
            }
        }
        if chf == SDLK_DOWN as libc::c_int {
            row += 1;
            if row > 2 as libc::c_int {
                row = 0 as libc::c_int;
            }
        }
        while spotok[row as usize][collumn as usize] == 0 {
            collumn -= 1;
        }
        if chf == SDLK_LEFT as libc::c_int {
            if collumn == 0 as libc::c_int {
                collumn = 4 as libc::c_int;
            }
            loop {
                collumn -= 1;
                if !(spotok[row as usize][collumn as usize] == 0) {
                    break;
                }
            }
        }
        if chf == SDLK_RIGHT as libc::c_int {
            loop {
                collumn += 1;
                if !(spotok[row as usize][collumn as usize] == 0 || collumn > 3 as libc::c_int) {
                    break;
                }
                if collumn == 4 as libc::c_int {
                    collumn = -(1 as libc::c_int);
                }
            }
        }
        if chf == SDLK_RETURN as libc::c_int {
            match row {
                0 => {
                    if !(newgrmode as libc::c_int == collumn + 1 as libc::c_int) {
                        drawchar(
                            collumnx[(newgrmode as libc::c_int - 1 as libc::c_int) as usize]
                                + 1 as libc::c_int,
                            rowy[row as usize] + 3 as libc::c_int,
                            32 as libc::c_int,
                        );
                        newgrmode = (collumn as grtype as libc::c_uint)
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as grtype;
                        grmode = newgrmode;
                        loadgrfiles();
                        drawwindow(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            39 as libc::c_int,
                            24 as libc::c_int,
                        );
                        drawpanel();
                    }
                }
                1 => {
                    drawchar(
                        collumnx[newsoundmode as libc::c_int as usize] + 1 as libc::c_int,
                        rowy[row as usize] + 3 as libc::c_int,
                        32 as libc::c_int,
                    );
                    newsoundmode = collumn as soundtype;
                }
                2 => {
                    drawchar(
                        collumnx[newplayermode[1 as libc::c_int as usize] as libc::c_int as usize]
                            + 1 as libc::c_int,
                        rowy[row as usize] + 3 as libc::c_int,
                        32 as libc::c_int,
                    );
                    newplayermode[1 as libc::c_int as usize] = collumn as inputtype;
                    if newplayermode[1 as libc::c_int as usize] as libc::c_uint
                        == keyboard as libc::c_int as libc::c_uint
                    {
                        calibratekeys();
                    } else if newplayermode[1 as libc::c_int as usize] as libc::c_uint
                        == mouse as libc::c_int as libc::c_uint
                    {
                        calibratemouse();
                    } else if newplayermode[1 as libc::c_int as usize] as libc::c_uint
                        == joystick1 as libc::c_int as libc::c_uint
                    {
                        calibratejoy(1 as libc::c_int);
                    } else if newplayermode[1 as libc::c_int as usize] as libc::c_uint
                        == joystick2 as libc::c_int as libc::c_uint
                    {
                        calibratejoy(2 as libc::c_int);
                    }
                    drawpanel();
                }
                _ => {}
            }
            drawchar(
                collumnx[collumn as usize] + 1 as libc::c_int,
                rowy[row as usize] + 3 as libc::c_int,
                15 as libc::c_int,
            );
        }
        if !(chf != SDLK_ESCAPE as libc::c_int) {
            break;
        }
    }
    playermode[1 as libc::c_int as usize] = newplayermode[1 as libc::c_int as usize];
    playermode[2 as libc::c_int as usize] = newplayermode[2 as libc::c_int as usize];
    CheckMouseMode();
    grmode = newgrmode;
    screencenterx = oldcenterx;
    screencentery = oldcentery;
    soundmode = newsoundmode;
    repaintscreen();
    ContinueSound();
}
#[no_mangle]
#[c2rust::src_loc = "599:10"]
pub static mut egaplane: [libc::c_uint; 4] = [0; 4];
#[no_mangle]
#[c2rust::src_loc = "600:12"]
pub static mut image: spritetype = spritetype {
    width: 0,
    height: 0,
    shapeptr: 0,
    maskptr: 0,
    xl: 0,
    yl: 0,
    xh: 0,
    yh: 0,
    name: [0; 12],
};
#[no_mangle]
#[c2rust::src_loc = "600:19"]
pub static mut spritetable: [spritetype; 10] = [spritetype {
    width: 0,
    height: 0,
    shapeptr: 0,
    maskptr: 0,
    xl: 0,
    yl: 0,
    xh: 0,
    yh: 0,
    name: [0; 12],
}; 10];
#[no_mangle]
#[c2rust::src_loc = "601:9"]
pub static mut pictable: [pictype; 64] = [pictype {
    width: 0,
    height: 0,
    shapeptr: 0,
    name: [0; 8],
}; 64];
#[no_mangle]
#[c2rust::src_loc = "602:7"]
pub static mut lastgrpic: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "604:5"]
pub static mut numchars: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "604:14"]
pub static mut numtiles: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "604:23"]
pub static mut numpics: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "604:31"]
pub static mut numsprites: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "606:1"]
pub unsafe extern "C" fn installgrfile(
    mut filename: *mut libc::c_char,
    mut unpack: libc::c_int,
    mut inmem: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut picfile: *mut picfiletype = 0 as *mut picfiletype;
    let mut spriteinfile: *mut stype = 0 as *mut stype;
    let mut picinfile: *mut ptype = 0 as *mut ptype;
    if *filename.offset(0 as libc::c_int as isize) == 0 {
        picfile = inmem as *mut picfiletype;
    } else {
        if lastgrpic as libc::c_long != 0 {
            free(lastgrpic);
        }
        if unpack != 0 {
            picfile = bloadin(filename) as *mut picfiletype;
        } else {
            picfile = bloadin(filename) as *mut picfiletype;
        }
        lastgrpic = picfile as *mut libc::c_void;
    }
    numchars = (*picfile).numchars as libc::c_int;
    numtiles = (*picfile).numtiles as libc::c_int;
    numpics = (*picfile).numpics as libc::c_int;
    numsprites = (*picfile).numsprites as libc::c_int;
    charptr =
        (picfile as *mut byte).offset(flatptr((*picfile).charptr) as isize) as *mut libc::c_void;
    tileptr =
        (picfile as *mut byte).offset(flatptr((*picfile).tileptr) as isize) as *mut libc::c_void;
    picptr =
        (picfile as *mut byte).offset(flatptr((*picfile).picptr) as isize) as *mut libc::c_void;
    spriteptr =
        (picfile as *mut byte).offset(flatptr((*picfile).spriteptr) as isize) as *mut libc::c_void;
    egaplaneofs[0 as libc::c_int as usize] = (flatptr((*picfile).plane[0 as libc::c_int as usize]))
        .wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[1 as libc::c_int as usize] = (flatptr((*picfile).plane[1 as libc::c_int as usize]))
        .wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[2 as libc::c_int as usize] = (flatptr((*picfile).plane[2 as libc::c_int as usize]))
        .wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[3 as libc::c_int as usize] = (flatptr((*picfile).plane[3 as libc::c_int as usize]))
        .wrapping_sub(flatptr((*picfile).charptr));
    picinfile =
        (picfile as *mut byte).offset(flatptr((*picfile).pictableptr) as isize) as *mut ptype;
    spriteinfile =
        (picfile as *mut byte).offset(flatptr((*picfile).spritetableptr) as isize) as *mut stype;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        pictable[i as usize] = (*picinfile)[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        spritetable[i as usize] = (*spriteinfile)[i as usize];
        i += 1;
    }
}
