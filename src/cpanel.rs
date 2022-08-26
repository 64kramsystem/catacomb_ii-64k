use std::mem;

use serdine::Deserialize;

use crate::{
    catacomb::{loadgrfiles, repaintscreen},
    control_struct::ControlStruct,
    cpanel_state::CpanelState,
    dir_type::dirtype::*,
    global_state::GlobalState,
    input_type::inputtype::*,
    pcrlib_a::{drawchar, drawpic, ContinueSound, PauseSound, WaitVBL},
    pcrlib_a_state::PcrlibAState,
    pcrlib_c::{
        ProbeJoysticks, ProcessEvents, ReadJoystick, ScancodeToDOS, UpdateScreen, _egaok, _vgaok,
        bioskey, bloadin, clearkeys, drawwindow, erasewindow, expwin, get, print_str,
        CheckMouseMode, ControlJoystick,
    },
    pcrlib_c_state::PcrlibCState,
    pic_file_type::picfiletype,
    pic_type::pictype,
    safe_sdl::safe_SDL_NumJoysticks,
    scan_codes::*,
    sprite_type::spritetype,
};

const SDLK_UP: u32 = 1073741906;
const SDLK_DOWN: u32 = 1073741905;
const SDLK_LEFT: u32 = 1073741904;
const SDLK_RIGHT: u32 = 1073741903;
const SDLK_ESCAPE: u32 = 27;
const SDLK_RETURN: u32 = 13;

const rowy: [i32; 4] = [4, 9, 14, 19];
const collumnx: [i32; 4] = [14, 20, 26, 32];

fn calibratejoy(
    joynum: i32,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
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
        button1: false,
        button2: false,
    };
    expwin(24, 9, gs, pas, pcs);
    print_str(" Joystick Configuration\n\r", gs, pcs);
    print_str(" ----------------------\n\r", gs, pcs);
    print_str("Hold the joystick in the\n\r", gs, pcs);
    print_str("upper left\n\r", gs, pcs);
    print_str("corner and hit fire:", gs, pcs);
    stage = 15;
    loop {
        drawchar(pcs.sx, pcs.sy, stage, gs, pcs);
        UpdateScreen(gs, pcs);
        WaitVBL(pas);
        WaitVBL(pas);
        WaitVBL(pas);
        stage += 1;
        if stage == 23 {
            stage = 15;
        }
        ProcessEvents(pcs);
        ReadJoystick(joynum, &mut xl, &mut yl, pcs);
        ctr = ControlJoystick(joynum, pcs);
        if pcs.keydown[SDL_SCANCODE_ESCAPE as usize] {
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
            drawchar(pcs.sx, pcs.sy, ' ' as i32, gs, pcs);
            loop {
                ctr = ControlJoystick(joynum, pcs);
                if !ctr.button1 {
                    break;
                }
            }
            UpdateScreen(gs, pcs);
            WaitVBL(pas);
            WaitVBL(pas);
            print_str("\n\n\rHold the joystick in the\n\r", gs, pcs);
            print_str("lower right\n\r", gs, pcs);
            print_str("corner and hit fire:", gs, pcs);
            loop {
                drawchar(pcs.sx, pcs.sy, stage, gs, pcs);
                UpdateScreen(gs, pcs);
                WaitVBL(pas);
                WaitVBL(pas);
                WaitVBL(pas);
                stage += 1;
                if stage == 23 {
                    stage = 15;
                }
                ProcessEvents(pcs);
                ReadJoystick(joynum, &mut xh, &mut yh, pcs);
                ctr = ControlJoystick(joynum, pcs);
                if pcs.keydown[SDL_SCANCODE_ESCAPE as usize] {
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
                    drawchar(pcs.sx, pcs.sy, ' ' as i32, gs, pcs);
                    loop {
                        ctr = ControlJoystick(joynum, pcs);
                        if !ctr.button1 {
                            break;
                        }
                    }
                    UpdateScreen(gs, pcs);
                    dx = (xh - xl) / 4;
                    dy = (yh - yl) / 4;
                    pcs.JoyXlow[joynum as usize] = xl + dx;
                    pcs.JoyXhigh[joynum as usize] = xh - dx;
                    pcs.JoyYlow[joynum as usize] = yl + dy;
                    pcs.JoyYhigh[joynum as usize] = yh - dy;
                }
            }
        }
        _ => {}
    }
    clearkeys(pcs);
    erasewindow(gs, pcs);
}

fn calibratemouse(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut ch: i8 = 0;
    expwin(24, 5, gs, pas, pcs);
    print_str("  Mouse Configuration   \n\r", gs, pcs);
    print_str("  -------------------   \n\r", gs, pcs);
    print_str("Choose the sensitivity  \n\r", gs, pcs);
    print_str("of the mouse, 1 being   \n\r", gs, pcs);
    print_str("slow, 9 being fast:", gs, pcs);
    loop {
        ch = (get(gs, pas, pcs) % 256) as i8;
        if ch as i32 == 27 {
            ch = '5' as i32 as i8;
        }
        if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
            break;
        }
    }
    pcs.MouseSensitivity = 15 - (ch as i32 - '0' as i32);
    erasewindow(gs, pcs);
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

fn printscan(mut sc: i32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    sc = ScancodeToDOS(sc as SDL_Scancode);
    if sc == 1 {
        print_str("ESC", gs, pcs);
    } else if sc == 0xe as i32 {
        print_str("BKSP", gs, pcs);
    } else if sc == 0xf as i32 {
        print_str("TAB", gs, pcs);
    } else if sc == 0x1d as i32 {
        print_str("CTRL", gs, pcs);
    } else if sc == 0x2a as i32 {
        print_str("LSHIFT", gs, pcs);
    } else if sc == 0x39 as i32 {
        print_str("SPACE", gs, pcs);
    } else if sc == 0x3a as i32 {
        print_str("CAPSLK", gs, pcs);
    } else if sc >= 0x3b as i32 && sc <= 0x44 as i32 {
        let str = format!("F{}", sc - 0x3a as i32);
        print_str(&str, gs, pcs);
    } else if sc == 0x57 as i32 {
        print_str("F11", gs, pcs);
    } else if sc == 0x59 as i32 {
        print_str("F12", gs, pcs);
    } else if sc == 0x46 as i32 {
        print_str("SCRLLK", gs, pcs);
    } else if sc == 0x1c as i32 {
        print_str("ENTER", gs, pcs);
    } else if sc == 0x36 as i32 {
        print_str("RSHIFT", gs, pcs);
    } else if sc == 0x37 as i32 {
        print_str("PRTSC", gs, pcs);
    } else if sc == 0x38 as i32 {
        print_str("ALT", gs, pcs);
    } else if sc == 0x47 as i32 {
        print_str("HOME", gs, pcs);
    } else if sc == 0x49 as i32 {
        print_str("PGUP", gs, pcs);
    } else if sc == 0x4f as i32 {
        print_str("END", gs, pcs);
    } else if sc == 0x51 as i32 {
        print_str("PGDN", gs, pcs);
    } else if sc == 0x52 as i32 {
        print_str("INS", gs, pcs);
    } else if sc == 0x53 as i32 {
        print_str("DEL", gs, pcs);
    } else if sc == 0x45 as i32 {
        print_str("NUMLK", gs, pcs);
    } else {
        let fresh0 = pcs.sx;
        pcs.sx += 1;
        drawchar(fresh0, pcs.sy, chartable[sc as usize] as i32, gs, pcs);
    };
}

fn calibratekeys(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut ch: i8 = 0;
    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut i: i32 = 0;
    let mut select: i32 = 0;
    let mut new = 0;
    expwin(22, 15, gs, pas, pcs);
    print_str("Keyboard Configuration\n\r", gs, pcs);
    print_str("----------------------", gs, pcs);
    print_str("\n\r0 north    :", gs, pcs);
    print_str("\n\r1 east     :", gs, pcs);
    print_str("\n\r2 south    :", gs, pcs);
    print_str("\n\r3 west     :", gs, pcs);
    print_str("\n\r4 northeast:", gs, pcs);
    print_str("\n\r5 southeast:", gs, pcs);
    print_str("\n\r6 southwest:", gs, pcs);
    print_str("\n\r7 northwest:", gs, pcs);
    print_str("\n\r8 button1  :", gs, pcs);
    print_str("\n\r9 button2  :", gs, pcs);
    print_str("\n\n\rModify which action:", gs, pcs);
    hx = pcs.sx;
    hy = pcs.sy;
    i = 0;
    while i < 8 {
        pcs.sx = 22;
        pcs.sy = 7 + i;
        printscan(pcs.key[i as usize] as i32, gs, pcs);
        i += 1;
    }
    pcs.sx = 22;
    pcs.sy = 15;
    printscan(pcs.keyB1 as i32, gs, pcs);
    pcs.sx = 22;
    pcs.sy = 16;
    printscan(pcs.keyB2 as i32, gs, pcs);
    loop {
        pcs.sx = hx;
        pcs.sy = hy;
        ch = (get(gs, pas, pcs) % 256) as i8;
        if !((ch as i32) < '0' as i32 || ch as i32 > '9' as i32) {
            select = ch as i32 - '0' as i32;
            drawchar(pcs.sx, pcs.sy, ch as i32, gs, pcs);
            select = ch as i32 - '0' as i32;
            print_str("\n\rPress the new key:", gs, pcs);
            clearkeys(pcs);
            UpdateScreen(gs, pcs);
            loop {
                new = bioskey(1, pcs);
                if !(new == 0) {
                    break;
                }
                WaitVBL(pas);
            }
            clearkeys(pcs);
            print_str("\r                  ", gs, pcs);
            if select < 8 {
                pcs.key[select as usize] = new;
            }
            if select == 8 {
                pcs.keyB1 = new;
            }
            if select == 9 {
                pcs.keyB2 = new;
            }
            pcs.sy = select + 7;
            pcs.sx = 22;
            print_str("        ", gs, pcs);
            pcs.sx = 22;
            printscan(new as i32, gs, pcs);
            ch = '0' as i32 as i8;
            clearkeys(pcs);
        }
        if !(ch as i32 >= '0' as i32 && ch as i32 <= '9' as i32) {
            break;
        }
    }
    erasewindow(gs, pcs);
}

pub fn getconfig(cps: &mut CpanelState) {
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
    let numjoy: i32 = safe_SDL_NumJoysticks();
    cps.joy1ok = (numjoy > 0) as i32;
    cps.joy2ok = (numjoy > 1) as i32;
    cps.mouseok = 1;
    cps.spotok[2][0] = 1;
    cps.spotok[2][1] = cps.mouseok;
    cps.spotok[2][2] = cps.joy1ok;
    cps.spotok[2][3] = cps.joy2ok;
    cps.spotok[2][4] = 0;
}

fn drawpanel(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    pcs.leftedge = 1;
    pas.xormask = 0;
    pcs.sx = 8;
    pcs.sy = 2;
    print_str("       Control Panel      \n\r", gs, pcs);
    getconfig(cps);
    pcs.sy = rowy[0] + 2;
    pcs.sx = 2;
    print_str("VIDEO:", gs, pcs);
    drawpic(collumnx[0] * 8, rowy[0] * 8, 0, gs, cps, pcs);
    if _egaok {
        drawpic(collumnx[1] * 8, rowy[0] * 8, 1, gs, cps, pcs);
    } else {
        drawpic(collumnx[1] * 8, rowy[0] * 8, 3, gs, cps, pcs);
    }
    pcs.sy = rowy[1] + 2;
    pcs.sx = 2;
    print_str("SOUND:", gs, pcs);
    drawpic(collumnx[0] * 8, rowy[1] * 8, 5, gs, cps, pcs);
    drawpic(collumnx[1] * 8, rowy[1] * 8, 6, gs, cps, pcs);
    pcs.sy = rowy[2] + 2;
    pcs.sx = 2;
    print_str("CONTROL:", gs, pcs);
    drawpic(collumnx[0] * 8, rowy[2] * 8, 7, gs, cps, pcs);
    if cps.mouseok != 0 {
        drawpic(collumnx[1] * 8, rowy[2] * 8, 10, gs, cps, pcs);
    } else {
        drawpic(collumnx[1] * 8, rowy[2] * 8, 12, gs, cps, pcs);
    }
    if cps.joy1ok != 0 {
        drawpic(collumnx[2] * 8, rowy[2] * 8, 8, gs, cps, pcs);
    } else {
        drawpic(collumnx[2] * 8, rowy[2] * 8, 11, gs, cps, pcs);
    }
    if cps.joy2ok != 0 {
        drawpic(collumnx[3] * 8, rowy[2] * 8, 9, gs, cps, pcs);
    } else {
        drawpic(collumnx[3] * 8, rowy[2] * 8, 11, gs, cps, pcs);
    }
    drawchar(
        collumnx[(cps.newgrmode as i32 - 1) as usize] + 1,
        rowy[0] + 3,
        15,
        gs,
        pcs,
    );
    drawchar(
        collumnx[cps.newsoundmode as i32 as usize] + 1,
        rowy[1] + 3,
        15,
        gs,
        pcs,
    );
    drawchar(
        collumnx[cps.newplayermode[1] as i32 as usize] + 1,
        rowy[2] + 3,
        15,
        gs,
        pcs,
    );
    pcs.sy = 21;
    pcs.sx = 1;
    print_str("  Move the cursor with the arrow keys \n\r", gs, pcs);
    print_str("   Make decisions with the ENTER key  \n\r", gs, pcs);
    print_str("       ESC to return to your game     \n\r", gs, pcs);
}

pub fn controlpanel(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut chf: i32 = 0;
    let mut oldcenterx: i32 = 0;
    let mut oldcentery: i32 = 0;
    clearkeys(pcs);
    PauseSound(pas);
    ProbeJoysticks(pcs);
    cps.oldgrmode = pcs.grmode;
    cps.newgrmode = cps.oldgrmode;
    cps.oldsoundmode = pas.soundmode;
    cps.newsoundmode = cps.oldsoundmode;
    cps.oldplayermode[1] = pcs.playermode[1];
    cps.newplayermode[1] = cps.oldplayermode[1];
    cps.oldplayermode[2] = pcs.playermode[2];
    cps.newplayermode[2] = cps.oldplayermode[2];
    oldcenterx = gs.screencenter.x;
    oldcentery = gs.screencenter.y;
    gs.screencenter.x = 19;
    gs.screencenter.y = 11;
    drawwindow(0, 0, 39, 24, gs, pcs);
    drawpanel(gs, cps, pas, pcs);
    cps.row = 0;
    cps.collumn = pcs.grmode as i32 - 1;
    loop {
        pcs.sx = collumnx[cps.collumn as usize] + 2;
        pcs.sy = rowy[cps.row as usize] + 3;
        chf = get(gs, pas, pcs);
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
                    cps.collumn = -1;
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
                            pcs,
                        );
                        cps.newgrmode = (cps.collumn + 1).into();
                        pcs.grmode = cps.newgrmode;
                        loadgrfiles(gs, cps, pcs);
                        drawwindow(0, 0, 39, 24, gs, pcs);
                        drawpanel(gs, cps, pas, pcs);
                    }
                }
                1 => {
                    drawchar(
                        collumnx[cps.newsoundmode as i32 as usize] + 1,
                        rowy[cps.row as usize] + 3,
                        32,
                        gs,
                        pcs,
                    );
                    cps.newsoundmode = cps.collumn.into();
                }
                2 => {
                    drawchar(
                        collumnx[cps.newplayermode[1] as i32 as usize] + 1,
                        rowy[cps.row as usize] + 3,
                        32,
                        gs,
                        pcs,
                    );
                    cps.newplayermode[1] = cps.collumn.into();
                    if cps.newplayermode[1] as u32 == keyboard as i32 as u32 {
                        calibratekeys(gs, pas, pcs);
                    } else if cps.newplayermode[1] as u32 == mouse as i32 as u32 {
                        calibratemouse(gs, pas, pcs);
                    } else if cps.newplayermode[1] as u32 == joystick1 as i32 as u32 {
                        calibratejoy(1, gs, pas, pcs);
                    } else if cps.newplayermode[1] as u32 == joystick2 as i32 as u32 {
                        calibratejoy(2, gs, pas, pcs);
                    }
                    drawpanel(gs, cps, pas, pcs);
                }
                _ => {}
            }
            drawchar(
                collumnx[cps.collumn as usize] + 1,
                rowy[cps.row as usize] + 3,
                15,
                gs,
                pcs,
            );
        }
        if !(chf != SDLK_ESCAPE as i32) {
            break;
        }
    }
    pcs.playermode[1] = cps.newplayermode[1];
    pcs.playermode[2] = cps.newplayermode[2];
    CheckMouseMode(pcs);
    pcs.grmode = cps.newgrmode;
    gs.screencenter.x = oldcenterx;
    gs.screencenter.y = oldcentery;
    pas.soundmode = cps.newsoundmode;
    repaintscreen(gs, cps, pas, pcs);
    ContinueSound(pas);
}

pub fn installgrfile(filename: &str, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    // Rust port: we divide the pic data in parts:
    //
    // - metadata (picfiletype)
    // - char data (currently retained inside picfile_data)
    // - pic data (currently retained inside picfile_data)
    //
    let picfile_data = bloadin(&filename).unwrap();
    let picfile: picfiletype = Deserialize::deserialize(&picfile_data[..]);
    cps.numchars = picfile.numchars as i32;
    cps.numtiles = picfile.numtiles as i32;
    cps.numpics = picfile.numpics as i32;
    cps.numsprites = picfile.numsprites as i32;
    pcs.picfile_data = picfile_data;
    pcs.charptr = picfile.charptr.flatptr() as usize;
    pcs.picptr = picfile.picptr.flatptr() as usize;
    pcs.egaplaneofs[0] = (picfile.plane[0].flatptr() - picfile.charptr.flatptr()) as u32;
    pcs.egaplaneofs[1] = (picfile.plane[1].flatptr() - picfile.charptr.flatptr()) as u32;
    pcs.egaplaneofs[2] = (picfile.plane[2].flatptr() - picfile.charptr.flatptr()) as u32;
    pcs.egaplaneofs[3] = (picfile.plane[3].flatptr() - picfile.charptr.flatptr()) as u32;

    for i in 0..64 {
        let start = picfile.pictableptr.flatptr() as usize + i * mem::size_of::<pictype>();
        let end = start + mem::size_of::<pictype>();

        let picinfile = Deserialize::deserialize(&pcs.picfile_data[start..end]);
        cps.pictable[i as usize] = picinfile;
    }

    for i in 0..10 {
        let start = picfile.spritetableptr.flatptr() as usize + i * mem::size_of::<spritetype>();
        let end = start + mem::size_of::<spritetype>();

        let spriteinfile = Deserialize::deserialize(&pcs.picfile_data[start..end]);
        cps.spritetable[i] = spriteinfile
    }
}
