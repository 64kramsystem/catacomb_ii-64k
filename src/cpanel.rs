use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn loadgrfiles();
    fn drawwindow(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int);
    fn bioskey(_: libc::c_int) -> libc::c_int;
    fn get() -> libc::c_int;
    fn expwin(width: libc::c_int, height: libc::c_int);
    fn erasewindow();
    static mut _vgaok: boolean;
    static mut _egaok: boolean;
    fn print(str: *const libc::c_char);
    fn repaintscreen();
    static mut screencentery: libc::c_int;
    static mut screencenterx: libc::c_int;
    fn drawpic(x: libc::c_int, y: libc::c_int, picnum: libc::c_int);
    fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
    static mut egaplaneofs: [dword; 4];
    static mut spriteptr: *mut libc::c_void;
    static mut picptr: *mut libc::c_void;
    static mut tileptr: *mut libc::c_void;
    static mut charptr: *mut libc::c_void;
    fn WaitVBL();
    fn UpdateScreen();
    static mut xormask: libc::c_int;
    static mut leftedge: libc::c_int;
    static mut sy: libc::c_int;
    static mut sx: libc::c_int;
    static mut grmode: grtype;
    fn clearkeys();
    fn bloadin(filename: *mut libc::c_char) -> *mut libc::c_void;
    fn ControlJoystick(joynum: libc::c_int) -> ControlStruct;
    fn ReadJoystick(joynum: libc::c_int, xcount: *mut libc::c_int, ycount: *mut libc::c_int);
    fn ProbeJoysticks();
    fn CheckMouseMode();
    fn ScancodeToDOS(sc: SDL_Scancode) -> libc::c_int;
    fn ProcessEvents();
    static mut keyB2: libc::c_int;
    static mut keyB1: libc::c_int;
    static mut key: [libc::c_int; 8];
    static mut MouseSensitivity: libc::c_int;
    static mut JoyYhigh: [libc::c_int; 3];
    static mut JoyYlow: [libc::c_int; 3];
    static mut JoyXhigh: [libc::c_int; 3];
    static mut JoyXlow: [libc::c_int; 3];
    static mut keydown: [boolean; 512];
    static mut playermode: [inputtype; 3];
    fn ContinueSound();
    fn PauseSound();
    static mut soundmode: soundtype;
    fn SDL_NumJoysticks() -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const SDLK_AUDIOFASTFORWARD: C2RustUnnamed = 1073742110;
pub const SDLK_AUDIOREWIND: C2RustUnnamed = 1073742109;
pub const SDLK_APP2: C2RustUnnamed = 1073742108;
pub const SDLK_APP1: C2RustUnnamed = 1073742107;
pub const SDLK_SLEEP: C2RustUnnamed = 1073742106;
pub const SDLK_EJECT: C2RustUnnamed = 1073742105;
pub const SDLK_KBDILLUMUP: C2RustUnnamed = 1073742104;
pub const SDLK_KBDILLUMDOWN: C2RustUnnamed = 1073742103;
pub const SDLK_KBDILLUMTOGGLE: C2RustUnnamed = 1073742102;
pub const SDLK_DISPLAYSWITCH: C2RustUnnamed = 1073742101;
pub const SDLK_BRIGHTNESSUP: C2RustUnnamed = 1073742100;
pub const SDLK_BRIGHTNESSDOWN: C2RustUnnamed = 1073742099;
pub const SDLK_AC_BOOKMARKS: C2RustUnnamed = 1073742098;
pub const SDLK_AC_REFRESH: C2RustUnnamed = 1073742097;
pub const SDLK_AC_STOP: C2RustUnnamed = 1073742096;
pub const SDLK_AC_FORWARD: C2RustUnnamed = 1073742095;
pub const SDLK_AC_BACK: C2RustUnnamed = 1073742094;
pub const SDLK_AC_HOME: C2RustUnnamed = 1073742093;
pub const SDLK_AC_SEARCH: C2RustUnnamed = 1073742092;
pub const SDLK_COMPUTER: C2RustUnnamed = 1073742091;
pub const SDLK_CALCULATOR: C2RustUnnamed = 1073742090;
pub const SDLK_MAIL: C2RustUnnamed = 1073742089;
pub const SDLK_WWW: C2RustUnnamed = 1073742088;
pub const SDLK_MEDIASELECT: C2RustUnnamed = 1073742087;
pub const SDLK_AUDIOMUTE: C2RustUnnamed = 1073742086;
pub const SDLK_AUDIOPLAY: C2RustUnnamed = 1073742085;
pub const SDLK_AUDIOSTOP: C2RustUnnamed = 1073742084;
pub const SDLK_AUDIOPREV: C2RustUnnamed = 1073742083;
pub const SDLK_AUDIONEXT: C2RustUnnamed = 1073742082;
pub const SDLK_MODE: C2RustUnnamed = 1073742081;
pub const SDLK_RGUI: C2RustUnnamed = 1073742055;
pub const SDLK_RALT: C2RustUnnamed = 1073742054;
pub const SDLK_RSHIFT: C2RustUnnamed = 1073742053;
pub const SDLK_RCTRL: C2RustUnnamed = 1073742052;
pub const SDLK_LGUI: C2RustUnnamed = 1073742051;
pub const SDLK_LALT: C2RustUnnamed = 1073742050;
pub const SDLK_LSHIFT: C2RustUnnamed = 1073742049;
pub const SDLK_LCTRL: C2RustUnnamed = 1073742048;
pub const SDLK_KP_HEXADECIMAL: C2RustUnnamed = 1073742045;
pub const SDLK_KP_DECIMAL: C2RustUnnamed = 1073742044;
pub const SDLK_KP_OCTAL: C2RustUnnamed = 1073742043;
pub const SDLK_KP_BINARY: C2RustUnnamed = 1073742042;
pub const SDLK_KP_CLEARENTRY: C2RustUnnamed = 1073742041;
pub const SDLK_KP_CLEAR: C2RustUnnamed = 1073742040;
pub const SDLK_KP_PLUSMINUS: C2RustUnnamed = 1073742039;
pub const SDLK_KP_MEMDIVIDE: C2RustUnnamed = 1073742038;
pub const SDLK_KP_MEMMULTIPLY: C2RustUnnamed = 1073742037;
pub const SDLK_KP_MEMSUBTRACT: C2RustUnnamed = 1073742036;
pub const SDLK_KP_MEMADD: C2RustUnnamed = 1073742035;
pub const SDLK_KP_MEMCLEAR: C2RustUnnamed = 1073742034;
pub const SDLK_KP_MEMRECALL: C2RustUnnamed = 1073742033;
pub const SDLK_KP_MEMSTORE: C2RustUnnamed = 1073742032;
pub const SDLK_KP_EXCLAM: C2RustUnnamed = 1073742031;
pub const SDLK_KP_AT: C2RustUnnamed = 1073742030;
pub const SDLK_KP_SPACE: C2RustUnnamed = 1073742029;
pub const SDLK_KP_HASH: C2RustUnnamed = 1073742028;
pub const SDLK_KP_COLON: C2RustUnnamed = 1073742027;
pub const SDLK_KP_DBLVERTICALBAR: C2RustUnnamed = 1073742026;
pub const SDLK_KP_VERTICALBAR: C2RustUnnamed = 1073742025;
pub const SDLK_KP_DBLAMPERSAND: C2RustUnnamed = 1073742024;
pub const SDLK_KP_AMPERSAND: C2RustUnnamed = 1073742023;
pub const SDLK_KP_GREATER: C2RustUnnamed = 1073742022;
pub const SDLK_KP_LESS: C2RustUnnamed = 1073742021;
pub const SDLK_KP_PERCENT: C2RustUnnamed = 1073742020;
pub const SDLK_KP_POWER: C2RustUnnamed = 1073742019;
pub const SDLK_KP_XOR: C2RustUnnamed = 1073742018;
pub const SDLK_KP_F: C2RustUnnamed = 1073742017;
pub const SDLK_KP_E: C2RustUnnamed = 1073742016;
pub const SDLK_KP_D: C2RustUnnamed = 1073742015;
pub const SDLK_KP_C: C2RustUnnamed = 1073742014;
pub const SDLK_KP_B: C2RustUnnamed = 1073742013;
pub const SDLK_KP_A: C2RustUnnamed = 1073742012;
pub const SDLK_KP_BACKSPACE: C2RustUnnamed = 1073742011;
pub const SDLK_KP_TAB: C2RustUnnamed = 1073742010;
pub const SDLK_KP_RIGHTBRACE: C2RustUnnamed = 1073742009;
pub const SDLK_KP_LEFTBRACE: C2RustUnnamed = 1073742008;
pub const SDLK_KP_RIGHTPAREN: C2RustUnnamed = 1073742007;
pub const SDLK_KP_LEFTPAREN: C2RustUnnamed = 1073742006;
pub const SDLK_CURRENCYSUBUNIT: C2RustUnnamed = 1073742005;
pub const SDLK_CURRENCYUNIT: C2RustUnnamed = 1073742004;
pub const SDLK_DECIMALSEPARATOR: C2RustUnnamed = 1073742003;
pub const SDLK_THOUSANDSSEPARATOR: C2RustUnnamed = 1073742002;
pub const SDLK_KP_000: C2RustUnnamed = 1073742001;
pub const SDLK_KP_00: C2RustUnnamed = 1073742000;
pub const SDLK_EXSEL: C2RustUnnamed = 1073741988;
pub const SDLK_CRSEL: C2RustUnnamed = 1073741987;
pub const SDLK_CLEARAGAIN: C2RustUnnamed = 1073741986;
pub const SDLK_OPER: C2RustUnnamed = 1073741985;
pub const SDLK_OUT: C2RustUnnamed = 1073741984;
pub const SDLK_SEPARATOR: C2RustUnnamed = 1073741983;
pub const SDLK_RETURN2: C2RustUnnamed = 1073741982;
pub const SDLK_PRIOR: C2RustUnnamed = 1073741981;
pub const SDLK_CLEAR: C2RustUnnamed = 1073741980;
pub const SDLK_CANCEL: C2RustUnnamed = 1073741979;
pub const SDLK_SYSREQ: C2RustUnnamed = 1073741978;
pub const SDLK_ALTERASE: C2RustUnnamed = 1073741977;
pub const SDLK_KP_EQUALSAS400: C2RustUnnamed = 1073741958;
pub const SDLK_KP_COMMA: C2RustUnnamed = 1073741957;
pub const SDLK_VOLUMEDOWN: C2RustUnnamed = 1073741953;
pub const SDLK_VOLUMEUP: C2RustUnnamed = 1073741952;
pub const SDLK_MUTE: C2RustUnnamed = 1073741951;
pub const SDLK_FIND: C2RustUnnamed = 1073741950;
pub const SDLK_PASTE: C2RustUnnamed = 1073741949;
pub const SDLK_COPY: C2RustUnnamed = 1073741948;
pub const SDLK_CUT: C2RustUnnamed = 1073741947;
pub const SDLK_UNDO: C2RustUnnamed = 1073741946;
pub const SDLK_AGAIN: C2RustUnnamed = 1073741945;
pub const SDLK_STOP: C2RustUnnamed = 1073741944;
pub const SDLK_SELECT: C2RustUnnamed = 1073741943;
pub const SDLK_MENU: C2RustUnnamed = 1073741942;
pub const SDLK_HELP: C2RustUnnamed = 1073741941;
pub const SDLK_EXECUTE: C2RustUnnamed = 1073741940;
pub const SDLK_F24: C2RustUnnamed = 1073741939;
pub const SDLK_F23: C2RustUnnamed = 1073741938;
pub const SDLK_F22: C2RustUnnamed = 1073741937;
pub const SDLK_F21: C2RustUnnamed = 1073741936;
pub const SDLK_F20: C2RustUnnamed = 1073741935;
pub const SDLK_F19: C2RustUnnamed = 1073741934;
pub const SDLK_F18: C2RustUnnamed = 1073741933;
pub const SDLK_F17: C2RustUnnamed = 1073741932;
pub const SDLK_F16: C2RustUnnamed = 1073741931;
pub const SDLK_F15: C2RustUnnamed = 1073741930;
pub const SDLK_F14: C2RustUnnamed = 1073741929;
pub const SDLK_F13: C2RustUnnamed = 1073741928;
pub const SDLK_KP_EQUALS: C2RustUnnamed = 1073741927;
pub const SDLK_POWER: C2RustUnnamed = 1073741926;
pub const SDLK_APPLICATION: C2RustUnnamed = 1073741925;
pub const SDLK_KP_PERIOD: C2RustUnnamed = 1073741923;
pub const SDLK_KP_0: C2RustUnnamed = 1073741922;
pub const SDLK_KP_9: C2RustUnnamed = 1073741921;
pub const SDLK_KP_8: C2RustUnnamed = 1073741920;
pub const SDLK_KP_7: C2RustUnnamed = 1073741919;
pub const SDLK_KP_6: C2RustUnnamed = 1073741918;
pub const SDLK_KP_5: C2RustUnnamed = 1073741917;
pub const SDLK_KP_4: C2RustUnnamed = 1073741916;
pub const SDLK_KP_3: C2RustUnnamed = 1073741915;
pub const SDLK_KP_2: C2RustUnnamed = 1073741914;
pub const SDLK_KP_1: C2RustUnnamed = 1073741913;
pub const SDLK_KP_ENTER: C2RustUnnamed = 1073741912;
pub const SDLK_KP_PLUS: C2RustUnnamed = 1073741911;
pub const SDLK_KP_MINUS: C2RustUnnamed = 1073741910;
pub const SDLK_KP_MULTIPLY: C2RustUnnamed = 1073741909;
pub const SDLK_KP_DIVIDE: C2RustUnnamed = 1073741908;
pub const SDLK_NUMLOCKCLEAR: C2RustUnnamed = 1073741907;
pub const SDLK_UP: C2RustUnnamed = 1073741906;
pub const SDLK_DOWN: C2RustUnnamed = 1073741905;
pub const SDLK_LEFT: C2RustUnnamed = 1073741904;
pub const SDLK_RIGHT: C2RustUnnamed = 1073741903;
pub const SDLK_PAGEDOWN: C2RustUnnamed = 1073741902;
pub const SDLK_END: C2RustUnnamed = 1073741901;
pub const SDLK_DELETE: C2RustUnnamed = 127;
pub const SDLK_PAGEUP: C2RustUnnamed = 1073741899;
pub const SDLK_HOME: C2RustUnnamed = 1073741898;
pub const SDLK_INSERT: C2RustUnnamed = 1073741897;
pub const SDLK_PAUSE: C2RustUnnamed = 1073741896;
pub const SDLK_SCROLLLOCK: C2RustUnnamed = 1073741895;
pub const SDLK_PRINTSCREEN: C2RustUnnamed = 1073741894;
pub const SDLK_F12: C2RustUnnamed = 1073741893;
pub const SDLK_F11: C2RustUnnamed = 1073741892;
pub const SDLK_F10: C2RustUnnamed = 1073741891;
pub const SDLK_F9: C2RustUnnamed = 1073741890;
pub const SDLK_F8: C2RustUnnamed = 1073741889;
pub const SDLK_F7: C2RustUnnamed = 1073741888;
pub const SDLK_F6: C2RustUnnamed = 1073741887;
pub const SDLK_F5: C2RustUnnamed = 1073741886;
pub const SDLK_F4: C2RustUnnamed = 1073741885;
pub const SDLK_F3: C2RustUnnamed = 1073741884;
pub const SDLK_F2: C2RustUnnamed = 1073741883;
pub const SDLK_F1: C2RustUnnamed = 1073741882;
pub const SDLK_CAPSLOCK: C2RustUnnamed = 1073741881;
pub const SDLK_z: C2RustUnnamed = 122;
pub const SDLK_y: C2RustUnnamed = 121;
pub const SDLK_x: C2RustUnnamed = 120;
pub const SDLK_w: C2RustUnnamed = 119;
pub const SDLK_v: C2RustUnnamed = 118;
pub const SDLK_u: C2RustUnnamed = 117;
pub const SDLK_t: C2RustUnnamed = 116;
pub const SDLK_s: C2RustUnnamed = 115;
pub const SDLK_r: C2RustUnnamed = 114;
pub const SDLK_q: C2RustUnnamed = 113;
pub const SDLK_p: C2RustUnnamed = 112;
pub const SDLK_o: C2RustUnnamed = 111;
pub const SDLK_n: C2RustUnnamed = 110;
pub const SDLK_m: C2RustUnnamed = 109;
pub const SDLK_l: C2RustUnnamed = 108;
pub const SDLK_k: C2RustUnnamed = 107;
pub const SDLK_j: C2RustUnnamed = 106;
pub const SDLK_i: C2RustUnnamed = 105;
pub const SDLK_h: C2RustUnnamed = 104;
pub const SDLK_g: C2RustUnnamed = 103;
pub const SDLK_f: C2RustUnnamed = 102;
pub const SDLK_e: C2RustUnnamed = 101;
pub const SDLK_d: C2RustUnnamed = 100;
pub const SDLK_c: C2RustUnnamed = 99;
pub const SDLK_b: C2RustUnnamed = 98;
pub const SDLK_a: C2RustUnnamed = 97;
pub const SDLK_BACKQUOTE: C2RustUnnamed = 96;
pub const SDLK_UNDERSCORE: C2RustUnnamed = 95;
pub const SDLK_CARET: C2RustUnnamed = 94;
pub const SDLK_RIGHTBRACKET: C2RustUnnamed = 93;
pub const SDLK_BACKSLASH: C2RustUnnamed = 92;
pub const SDLK_LEFTBRACKET: C2RustUnnamed = 91;
pub const SDLK_AT: C2RustUnnamed = 64;
pub const SDLK_QUESTION: C2RustUnnamed = 63;
pub const SDLK_GREATER: C2RustUnnamed = 62;
pub const SDLK_EQUALS: C2RustUnnamed = 61;
pub const SDLK_LESS: C2RustUnnamed = 60;
pub const SDLK_SEMICOLON: C2RustUnnamed = 59;
pub const SDLK_COLON: C2RustUnnamed = 58;
pub const SDLK_9: C2RustUnnamed = 57;
pub const SDLK_8: C2RustUnnamed = 56;
pub const SDLK_7: C2RustUnnamed = 55;
pub const SDLK_6: C2RustUnnamed = 54;
pub const SDLK_5: C2RustUnnamed = 53;
pub const SDLK_4: C2RustUnnamed = 52;
pub const SDLK_3: C2RustUnnamed = 51;
pub const SDLK_2: C2RustUnnamed = 50;
pub const SDLK_1: C2RustUnnamed = 49;
pub const SDLK_0: C2RustUnnamed = 48;
pub const SDLK_SLASH: C2RustUnnamed = 47;
pub const SDLK_PERIOD: C2RustUnnamed = 46;
pub const SDLK_MINUS: C2RustUnnamed = 45;
pub const SDLK_COMMA: C2RustUnnamed = 44;
pub const SDLK_PLUS: C2RustUnnamed = 43;
pub const SDLK_ASTERISK: C2RustUnnamed = 42;
pub const SDLK_RIGHTPAREN: C2RustUnnamed = 41;
pub const SDLK_LEFTPAREN: C2RustUnnamed = 40;
pub const SDLK_QUOTE: C2RustUnnamed = 39;
pub const SDLK_AMPERSAND: C2RustUnnamed = 38;
pub const SDLK_DOLLAR: C2RustUnnamed = 36;
pub const SDLK_PERCENT: C2RustUnnamed = 37;
pub const SDLK_HASH: C2RustUnnamed = 35;
pub const SDLK_QUOTEDBL: C2RustUnnamed = 34;
pub const SDLK_EXCLAIM: C2RustUnnamed = 33;
pub const SDLK_SPACE: C2RustUnnamed = 32;
pub const SDLK_TAB: C2RustUnnamed = 9;
pub const SDLK_BACKSPACE: C2RustUnnamed = 8;
pub const SDLK_ESCAPE: C2RustUnnamed = 27;
pub const SDLK_RETURN: C2RustUnnamed = 13;
pub const SDLK_UNKNOWN: C2RustUnnamed = 0;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type word = uint16_t;
pub type sword = int16_t;
pub type dword = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct farptr {
    pub ofs: word,
    pub seg: word,
}
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
pub type soundtype = libc::c_uint;
pub const sdlib: soundtype = 2;
pub const spkr: soundtype = 1;
pub const off: soundtype = 0;
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
pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
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
pub struct pictype {
    pub width: sword,
    pub height: sword,
    pub shapeptr: dword,
    pub name: [libc::c_char; 8],
}
pub type stype = [spritetype; 10];
pub type ptype = [pictype; 64];
#[derive(Copy, Clone)]
#[repr(C)]
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
#[inline]
unsafe extern "C" fn flatptr(mut ptr: farptr) -> dword {
    return (((ptr.seg as libc::c_int) << 4 as libc::c_int) + ptr.ofs as libc::c_int) as dword;
}
#[inline]
unsafe extern "C" fn itoa(
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
#[no_mangle]
pub static mut rowy: [libc::c_int; 4] = [
    4 as libc::c_int,
    9 as libc::c_int,
    14 as libc::c_int,
    19 as libc::c_int,
];
#[no_mangle]
pub static mut collumnx: [libc::c_int; 4] = [
    14 as libc::c_int,
    20 as libc::c_int,
    26 as libc::c_int,
    32 as libc::c_int,
];
#[no_mangle]
pub static mut spotok: [[libc::c_int; 5]; 4] = [[0; 5]; 4];
#[no_mangle]
pub static mut row: libc::c_int = 0;
#[no_mangle]
pub static mut collumn: libc::c_int = 0;
#[no_mangle]
pub static mut oldgrmode: grtype = text;
#[no_mangle]
pub static mut newgrmode: grtype = text;
#[no_mangle]
pub static mut oldsoundmode: soundtype = off;
#[no_mangle]
pub static mut newsoundmode: soundtype = off;
#[no_mangle]
pub static mut oldplayermode: [inputtype; 3] = [keyboard; 3];
#[no_mangle]
pub static mut newplayermode: [inputtype; 3] = [keyboard; 3];
#[no_mangle]
pub static mut joy1ok: libc::c_int = 0;
#[no_mangle]
pub static mut joy2ok: libc::c_int = 0;
#[no_mangle]
pub static mut mouseok: libc::c_int = 0;
#[no_mangle]
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
pub static mut egaplane: [libc::c_uint; 4] = [0; 4];
#[no_mangle]
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
pub static mut pictable: [pictype; 64] = [pictype {
    width: 0,
    height: 0,
    shapeptr: 0,
    name: [0; 8],
}; 64];
#[no_mangle]
pub static mut lastgrpic: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut numchars: libc::c_int = 0;
#[no_mangle]
pub static mut numtiles: libc::c_int = 0;
#[no_mangle]
pub static mut numpics: libc::c_int = 0;
#[no_mangle]
pub static mut numsprites: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn installgrfile(
    mut filename: *mut libc::c_char,
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
        picfile = bloadin(filename) as *mut picfiletype;
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
