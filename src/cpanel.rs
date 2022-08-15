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

const SDLK_UP: u32 = 1073741906;
const SDLK_DOWN: u32 = 1073741905;
const SDLK_LEFT: u32 = 1073741904;
const SDLK_RIGHT: u32 = 1073741903;
const SDLK_ESCAPE: u32 = 27;
const SDLK_RETURN: u32 = 13;

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

pub static mut egaplane: [u32; 4] = [0; 4];

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
