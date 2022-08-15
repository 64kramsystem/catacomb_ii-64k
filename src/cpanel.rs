use std::ffi::{CStr, CString};

use ::libc;

use crate::{
    catacomb::{loadgrfiles, repaintscreen},
    control_struct::ControlStruct,
    cpanel_state::CpanelState,
    dir_type::dirtype::*,
    global_state::GlobalState,
    input_type::inputtype::*,
    pcrlib_a::{drawchar, drawpic, soundmode, xormask, ContinueSound, PauseSound, WaitVBL},
    pcrlib_c::{
        MouseSensitivity, ProbeJoysticks, ProcessEvents, ReadJoystick, ScancodeToDOS, UpdateScreen,
        _egaok, _vgaok, bioskey, bloadin, charptr, clearkeys, drawwindow, egaplaneofs, erasewindow,
        expwin, get, grmode, key, keyB1, keyB2, keydown, leftedge, picptr, playermode, print,
        spriteptr, sx, sy, tileptr, CheckMouseMode, ControlJoystick, JoyXhigh, JoyXlow, JoyYhigh,
        JoyYlow,
    },
    safe_sdl::safe_SDL_NumJoysticks,
    sdl_scan_codes::*,
};

extern "C" {
    fn free(_: *mut libc::c_void);
}

pub type C2RustUnnamed = u32;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct farptr {
    pub ofs: u16,
    pub seg: u16,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct spritetype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub maskptr: u32,
    pub xl: i16,
    pub yl: i16,
    pub xh: i16,
    pub yh: i16,
    pub name: [i8; 12],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct pictype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub name: [i8; 8],
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
    pub numchars: i16,
    pub numtiles: i16,
    pub numpics: i16,
    pub numsprites: i16,
}

#[inline]
unsafe fn flatptr(mut ptr: farptr) -> u32 {
    return (((ptr.seg as i32) << 4) + ptr.ofs as i32) as u32;
}

const rowy: [i32; 4] = [4, 9, 14, 19];
const collumnx: [i32; 4] = [14, 20, 26, 32];

unsafe fn calibratejoy(mut joynum: i32, gs: &mut GlobalState) {
    let mut current_block: u64;
    let mut stage: i32 = 0;
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    let mut xl: i32 = 0;
    let mut yl: i32 = 0;
    let mut xh: i32 = 0;
    let mut yh: i32 = 0;
    let mut ctr: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    expwin(24, 9, gs);
    print(
        b" Joystick Configuration\n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b" ----------------------\n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"Hold the joystick in the\n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(b"upper left\n\r\0" as *const u8 as *const i8, gs);
    print(b"corner and hit fire:\0" as *const u8 as *const i8, gs);
    stage = 15;
    loop {
        drawchar(sx, sy, stage, gs);
        UpdateScreen(gs);
        WaitVBL();
        WaitVBL();
        WaitVBL();
        stage += 1;
        if stage == 23 {
            stage = 15;
        }
        ProcessEvents();
        ReadJoystick(joynum, &mut xl, &mut yl);
        ctr = ControlJoystick(joynum);
        if keydown[SDL_SCANCODE_ESCAPE as usize] != 0 {
            current_block = 15976468122069307450;
            break;
        }
        if !(ctr.button1 as i32 != 1) {
            current_block = 8457315219000651999;
            break;
        }
    }
    match current_block {
        8457315219000651999 => {
            drawchar(sx, sy, ' ' as i32, gs);
            loop {
                ctr = ControlJoystick(joynum);
                if !(ctr.button1 != 0) {
                    break;
                }
            }
            UpdateScreen(gs);
            WaitVBL();
            WaitVBL();
            print(
                b"\n\n\rHold the joystick in the\n\r\0" as *const u8 as *const i8,
                gs,
            );
            print(b"lower right\n\r\0" as *const u8 as *const i8, gs);
            print(b"corner and hit fire:\0" as *const u8 as *const i8, gs);
            loop {
                drawchar(sx, sy, stage, gs);
                UpdateScreen(gs);
                WaitVBL();
                WaitVBL();
                WaitVBL();
                stage += 1;
                if stage == 23 {
                    stage = 15;
                }
                ProcessEvents();
                ReadJoystick(joynum, &mut xh, &mut yh);
                ctr = ControlJoystick(joynum);
                if keydown[SDL_SCANCODE_ESCAPE as usize] != 0 {
                    current_block = 15976468122069307450;
                    break;
                }
                if !(ctr.button1 as i32 != 1) {
                    current_block = 15597372965620363352;
                    break;
                }
            }
            match current_block {
                15976468122069307450 => {}
                _ => {
                    drawchar(sx, sy, ' ' as i32, gs);
                    loop {
                        ctr = ControlJoystick(joynum);
                        if !(ctr.button1 != 0) {
                            break;
                        }
                    }
                    UpdateScreen(gs);
                    dx = (xh - xl) / 4;
                    dy = (yh - yl) / 4;
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
    erasewindow(gs);
}

unsafe fn calibratemouse(gs: &mut GlobalState) {
    let mut ch: i8 = 0;
    expwin(24, 5, gs);
    print(
        b"  Mouse Configuration   \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"  -------------------   \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"Choose the sensitivity  \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"of the mouse, 1 being   \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(b"slow, 9 being fast:\0" as *const u8 as *const i8, gs);
    loop {
        ch = (get(gs) % 256) as i8;
        if ch as i32 == 27 {
            ch = '5' as i32 as i8;
        }
        if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
            break;
        }
    }
    MouseSensitivity = 15 - (ch as i32 - '0' as i32);
    erasewindow(gs);
}

const chartable: [char; 128] = [
    '?', '?', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '+', '?', '?', 'Q', 'W', 'E',
    'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '|', '?', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K',
    'L', ';', '"', '?', '?', '?', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', '?', '?', '?',
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', 15 as char, '?',
    '-', 21 as char, '5', 17 as char, '+', '?', 19 as char, '?', '?', '?', '?', '?', '?', '?', '?',
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
    '?',
];

unsafe fn printscan(mut sc: i32, gs: &mut GlobalState) {
    sc = ScancodeToDOS(sc as SDL_Scancode);
    if sc == 1 {
        print(b"ESC\0" as *const u8 as *const i8, gs);
    } else if sc == 0xe as i32 {
        print(b"BKSP\0" as *const u8 as *const i8, gs);
    } else if sc == 0xf as i32 {
        print(b"TAB\0" as *const u8 as *const i8, gs);
    } else if sc == 0x1d as i32 {
        print(b"CTRL\0" as *const u8 as *const i8, gs);
    } else if sc == 0x2a as i32 {
        print(b"LSHIFT\0" as *const u8 as *const i8, gs);
    } else if sc == 0x39 as i32 {
        print(b"SPACE\0" as *const u8 as *const i8, gs);
    } else if sc == 0x3a as i32 {
        print(b"CAPSLK\0" as *const u8 as *const i8, gs);
    } else if sc >= 0x3b as i32 && sc <= 0x44 as i32 {
        let str = CString::new(format!("F{}", sc - 0x3a as i32)).unwrap();
        print(str.as_ptr(), gs);
    } else if sc == 0x57 as i32 {
        print(b"F11\0" as *const u8 as *const i8, gs);
    } else if sc == 0x59 as i32 {
        print(b"F12\0" as *const u8 as *const i8, gs);
    } else if sc == 0x46 as i32 {
        print(b"SCRLLK\0" as *const u8 as *const i8, gs);
    } else if sc == 0x1c as i32 {
        print(b"ENTER\0" as *const u8 as *const i8, gs);
    } else if sc == 0x36 as i32 {
        print(b"RSHIFT\0" as *const u8 as *const i8, gs);
    } else if sc == 0x37 as i32 {
        print(b"PRTSC\0" as *const u8 as *const i8, gs);
    } else if sc == 0x38 as i32 {
        print(b"ALT\0" as *const u8 as *const i8, gs);
    } else if sc == 0x47 as i32 {
        print(b"HOME\0" as *const u8 as *const i8, gs);
    } else if sc == 0x49 as i32 {
        print(b"PGUP\0" as *const u8 as *const i8, gs);
    } else if sc == 0x4f as i32 {
        print(b"END\0" as *const u8 as *const i8, gs);
    } else if sc == 0x51 as i32 {
        print(b"PGDN\0" as *const u8 as *const i8, gs);
    } else if sc == 0x52 as i32 {
        print(b"INS\0" as *const u8 as *const i8, gs);
    } else if sc == 0x53 as i32 {
        print(b"DEL\0" as *const u8 as *const i8, gs);
    } else if sc == 0x45 as i32 {
        print(b"NUMLK\0" as *const u8 as *const i8, gs);
    } else {
        let fresh0 = sx;
        sx = sx + 1;
        drawchar(fresh0, sy, chartable[sc as usize] as i32, gs);
    };
}

unsafe fn calibratekeys(gs: &mut GlobalState) {
    let mut ch: i8 = 0;
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut i: i32 = 0;
    let mut select: i32 = 0;
    let mut new: i32 = 0;
    expwin(22, 15, gs);
    print(
        b"Keyboard Configuration\n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(b"----------------------\0" as *const u8 as *const i8, gs);
    print(b"\n\r0 north    :\0" as *const u8 as *const i8, gs);
    print(b"\n\r1 east     :\0" as *const u8 as *const i8, gs);
    print(b"\n\r2 south    :\0" as *const u8 as *const i8, gs);
    print(b"\n\r3 west     :\0" as *const u8 as *const i8, gs);
    print(b"\n\r4 northeast:\0" as *const u8 as *const i8, gs);
    print(b"\n\r5 southeast:\0" as *const u8 as *const i8, gs);
    print(b"\n\r6 southwest:\0" as *const u8 as *const i8, gs);
    print(b"\n\r7 northwest:\0" as *const u8 as *const i8, gs);
    print(b"\n\r8 button1  :\0" as *const u8 as *const i8, gs);
    print(b"\n\r9 button2  :\0" as *const u8 as *const i8, gs);
    print(
        b"\n\n\rModify which action:\0" as *const u8 as *const i8,
        gs,
    );
    hx = sx;
    hy = sy;
    i = 0;
    while i < 8 {
        sx = 22;
        sy = 7 + i;
        printscan(key[i as usize], gs);
        i += 1;
    }
    sx = 22;
    sy = 15;
    printscan(keyB1, gs);
    sx = 22;
    sy = 16;
    printscan(keyB2, gs);
    loop {
        sx = hx;
        sy = hy;
        ch = (get(gs) % 256) as i8;
        if !((ch as i32) < '0' as i32 || ch as i32 > '9' as i32) {
            select = ch as i32 - '0' as i32;
            drawchar(sx, sy, ch as i32, gs);
            select = ch as i32 - '0' as i32;
            print(b"\n\rPress the new key:\0" as *const u8 as *const i8, gs);
            clearkeys();
            UpdateScreen(gs);
            loop {
                new = bioskey(1);
                if !(new == 0) {
                    break;
                }
                WaitVBL();
            }
            clearkeys();
            print(b"\r                  \0" as *const u8 as *const i8, gs);
            if select < 8 {
                key[select as usize] = new;
            }
            if select == 8 {
                keyB1 = new;
            }
            if select == 9 {
                keyB2 = new;
            }
            sy = select + 7;
            sx = 22;
            print(b"        \0" as *const u8 as *const i8, gs);
            sx = 22;
            printscan(new, gs);
            ch = '0' as i32 as i8;
            clearkeys();
        }
        if !(ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32) {
            break;
        }
    }
    erasewindow(gs);
}

#[no_mangle]
pub unsafe fn getconfig(cps: &mut CpanelState) {
    cps.spotok[0][0] = 1;
    cps.spotok[0][1] = _egaok as i32;
    cps.spotok[0][2] = _vgaok as i32;
    cps.spotok[0][3] = 0;
    cps.spotok[0][4] = 0;
    cps.spotok[1][0] = 1;
    cps.spotok[1][1] = 1;
    cps.spotok[1][2] = 0;
    cps.spotok[1][3] = 0;
    cps.spotok[1][4] = 0;
    let mut numjoy: i32 = safe_SDL_NumJoysticks();
    cps.joy1ok = (numjoy > 0) as i32;
    cps.joy2ok = (numjoy > 1) as i32;
    cps.mouseok = 1;
    cps.spotok[2][0] = 1;
    cps.spotok[2][1] = cps.mouseok;
    cps.spotok[2][2] = cps.joy1ok;
    cps.spotok[2][3] = cps.joy2ok;
    cps.spotok[2][4] = 0;
}

unsafe fn drawpanel(gs: &mut GlobalState, cps: &mut CpanelState) {
    leftedge = 1;
    xormask = 0;
    sx = 8;
    sy = 2;
    print(
        b"       Control Panel      \n\r\0" as *const u8 as *const i8,
        gs,
    );
    getconfig(cps);
    sy = rowy[0] + 2;
    sx = 2;
    print(b"VIDEO:\0" as *const u8 as *const i8, gs);
    drawpic(collumnx[0] * 8, rowy[0] * 8, 0, gs);
    if _egaok != 0 {
        drawpic(collumnx[1] * 8, rowy[0] * 8, 1, gs);
    } else {
        drawpic(collumnx[1] * 8, rowy[0] * 8, 3, gs);
    }
    sy = rowy[1] + 2;
    sx = 2;
    print(b"SOUND:\0" as *const u8 as *const i8, gs);
    drawpic(collumnx[0] * 8, rowy[1] * 8, 5, gs);
    drawpic(collumnx[1] * 8, rowy[1] * 8, 6, gs);
    sy = rowy[2] + 2;
    sx = 2;
    print(b"CONTROL:\0" as *const u8 as *const i8, gs);
    drawpic(collumnx[0] * 8, rowy[2] * 8, 7, gs);
    if cps.mouseok != 0 {
        drawpic(collumnx[1] * 8, rowy[2] * 8, 10, gs);
    } else {
        drawpic(collumnx[1] * 8, rowy[2] * 8, 12, gs);
    }
    if cps.joy1ok != 0 {
        drawpic(collumnx[2] * 8, rowy[2] * 8, 8, gs);
    } else {
        drawpic(collumnx[2] * 8, rowy[2] * 8, 11, gs);
    }
    if cps.joy2ok != 0 {
        drawpic(collumnx[3] * 8, rowy[2] * 8, 9, gs);
    } else {
        drawpic(collumnx[3] * 8, rowy[2] * 8, 11, gs);
    }
    drawchar(
        collumnx[(cps.newgrmode as i32 - 1) as usize] + 1,
        rowy[0] + 3,
        15,
        gs,
    );
    drawchar(
        collumnx[cps.newsoundmode as i32 as usize] + 1,
        rowy[1] + 3,
        15,
        gs,
    );
    drawchar(
        collumnx[cps.newplayermode[1] as i32 as usize] + 1,
        rowy[2] + 3,
        15,
        gs,
    );
    sy = 21;
    sx = 1;
    print(
        b"  Move the cursor with the arrow keys \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"   Make decisions with the ENTER key  \n\r\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b"       ESC to return to your game     \n\r\0" as *const u8 as *const i8,
        gs,
    );
}

pub unsafe fn controlpanel(gs: &mut GlobalState, cps: &mut CpanelState) {
    let mut chf: i32 = 0;
    let mut oldcenterx: i32 = 0;
    let mut oldcentery: i32 = 0;
    clearkeys();
    PauseSound();
    ProbeJoysticks();
    cps.oldgrmode = grmode;
    cps.newgrmode = cps.oldgrmode;
    cps.oldsoundmode = soundmode;
    cps.newsoundmode = cps.oldsoundmode;
    cps.oldplayermode[1] = playermode[1];
    cps.newplayermode[1] = cps.oldplayermode[1];
    cps.oldplayermode[2] = playermode[2];
    cps.newplayermode[2] = cps.oldplayermode[2];
    oldcenterx = gs.screencenter.x;
    oldcentery = gs.screencenter.y;
    gs.screencenter.x = 19;
    gs.screencenter.y = 11;
    drawwindow(0, 0, 39, 24, gs);
    drawpanel(gs, cps);
    cps.row = 0;
    cps.collumn = grmode as i32 - 1;
    loop {
        sx = collumnx[cps.collumn as usize] + 2;
        sy = rowy[cps.row as usize] + 3;
        chf = get(gs);
        if chf == SDLK_UP as i32 {
            cps.row -= 1;
            if cps.row < 0 {
                cps.row = 2;
            }
        }
        if chf == SDLK_DOWN as i32 {
            cps.row += 1;
            if cps.row > 2 {
                cps.row = 0;
            }
        }
        while cps.spotok[cps.row as usize][cps.collumn as usize] == 0 {
            cps.collumn -= 1;
        }
        if chf == SDLK_LEFT as i32 {
            if cps.collumn == 0 {
                cps.collumn = 4;
            }
            loop {
                cps.collumn -= 1;
                if !(cps.spotok[cps.row as usize][cps.collumn as usize] == 0) {
                    break;
                }
            }
        }
        if chf == SDLK_RIGHT as i32 {
            loop {
                cps.collumn += 1;
                if !(cps.spotok[cps.row as usize][cps.collumn as usize] == 0 || cps.collumn > 3) {
                    break;
                }
                if cps.collumn == 4 {
                    cps.collumn = -(1);
                }
            }
        }
        if chf == SDLK_RETURN as i32 {
            match cps.row {
                0 => {
                    if !(cps.newgrmode as i32 == cps.collumn + 1) {
                        drawchar(
                            collumnx[(cps.newgrmode as i32 - 1) as usize] + 1,
                            rowy[cps.row as usize] + 3,
                            32,
                            gs,
                        );
                        cps.newgrmode = (cps.collumn + 1).into();
                        grmode = cps.newgrmode;
                        loadgrfiles(gs);
                        drawwindow(0, 0, 39, 24, gs);
                        drawpanel(gs, cps);
                    }
                }
                1 => {
                    drawchar(
                        collumnx[cps.newsoundmode as i32 as usize] + 1,
                        rowy[cps.row as usize] + 3,
                        32,
                        gs,
                    );
                    cps.newsoundmode = cps.collumn.into();
                }
                2 => {
                    drawchar(
                        collumnx[cps.newplayermode[1] as i32 as usize] + 1,
                        rowy[cps.row as usize] + 3,
                        32,
                        gs,
                    );
                    cps.newplayermode[1] = cps.collumn.into();
                    if cps.newplayermode[1] as u32 == keyboard as i32 as u32 {
                        calibratekeys(gs);
                    } else if cps.newplayermode[1] as u32 == mouse as i32 as u32 {
                        calibratemouse(gs);
                    } else if cps.newplayermode[1] as u32 == joystick1 as i32 as u32 {
                        calibratejoy(1, gs);
                    } else if cps.newplayermode[1] as u32 == joystick2 as i32 as u32 {
                        calibratejoy(2, gs);
                    }
                    drawpanel(gs, cps);
                }
                _ => {}
            }
            drawchar(
                collumnx[cps.collumn as usize] + 1,
                rowy[cps.row as usize] + 3,
                15,
                gs,
            );
        }
        if !(chf != SDLK_ESCAPE as i32) {
            break;
        }
    }
    playermode[1] = cps.newplayermode[1];
    playermode[2] = cps.newplayermode[2];
    CheckMouseMode();
    grmode = cps.newgrmode;
    gs.screencenter.x = oldcenterx;
    gs.screencenter.y = oldcentery;
    soundmode = cps.newsoundmode;
    repaintscreen(gs);
    ContinueSound();
}
#[no_mangle]
pub static mut egaplane: [u32; 4] = [0; 4];
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

pub static mut lastgrpic: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
pub static mut numchars: i32 = 0;
pub static mut numtiles: i32 = 0;
pub static mut numpics: i32 = 0;
pub static mut numsprites: i32 = 0;

pub unsafe fn installgrfile(mut filename: *const i8, mut inmem: *mut libc::c_void) {
    let mut i: i32 = 0;
    let mut picfile: *mut picfiletype = 0 as *mut picfiletype;
    let mut spriteinfile: *mut stype = 0 as *mut stype;
    let mut picinfile: *mut ptype = 0 as *mut ptype;
    if *filename.offset(0) == 0 {
        picfile = inmem as *mut picfiletype;
    } else {
        if lastgrpic as i64 != 0 {
            free(lastgrpic);
        }
        let filename = CStr::from_ptr(filename).to_string_lossy().to_string();
        picfile = bloadin(&filename) as *mut picfiletype;
        lastgrpic = picfile as *mut libc::c_void;
    }
    numchars = (*picfile).numchars as i32;
    numtiles = (*picfile).numtiles as i32;
    numpics = (*picfile).numpics as i32;
    numsprites = (*picfile).numsprites as i32;
    charptr =
        (picfile as *mut u8).offset(flatptr((*picfile).charptr) as isize) as *mut libc::c_void;
    tileptr =
        (picfile as *mut u8).offset(flatptr((*picfile).tileptr) as isize) as *mut libc::c_void;
    picptr = (picfile as *mut u8).offset(flatptr((*picfile).picptr) as isize) as *mut libc::c_void;
    spriteptr =
        (picfile as *mut u8).offset(flatptr((*picfile).spriteptr) as isize) as *mut libc::c_void;
    egaplaneofs[0] = (flatptr((*picfile).plane[0])).wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[1] = (flatptr((*picfile).plane[1])).wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[2] = (flatptr((*picfile).plane[2])).wrapping_sub(flatptr((*picfile).charptr));
    egaplaneofs[3] = (flatptr((*picfile).plane[3])).wrapping_sub(flatptr((*picfile).charptr));
    picinfile = (picfile as *mut u8).offset(flatptr((*picfile).pictableptr) as isize) as *mut ptype;
    spriteinfile =
        (picfile as *mut u8).offset(flatptr((*picfile).spritetableptr) as isize) as *mut stype;
    i = 0;
    while i < 64 {
        pictable[i as usize] = (*picinfile)[i as usize];
        i += 1;
    }
    i = 0;
    while i < 10 {
        spritetable[i as usize] = (*spriteinfile)[i as usize];
        i += 1;
    }
}
