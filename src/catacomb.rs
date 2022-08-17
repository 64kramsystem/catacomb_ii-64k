use std::{ffi::CString, ptr};

use libc::O_RDONLY;

use crate::{
    active_obj::activeobj,
    cat_play::{
        givebolt, givenuke, givepotion, playloop, printbody, printhighscore, printscore,
        printshotpower,
    },
    catasm::{cgarefresh, drawchartile, egarefresh},
    class_type::classtype::{self, *},
    control_struct::ControlStruct,
    cpanel::{controlpanel, installgrfile, pictype, spritetype},
    cpanel_state::CpanelState,
    demo_enum::demoenum,
    dir_type::dirtype::{self, *},
    exit_type::exittype::*,
    extra_constants::{
        blankfloor, leftoff, maxpics, numlevels, solidwall, tile2s, topoff, NUM_DEMOS, O_BINARY,
    },
    extra_types::boolean,
    global_state::GlobalState,
    gr_type::grtype::*,
    input_type::inputtype::*,
    obj_def_type::objdeftype,
    obj_type::objtype,
    objects::initobjects,
    pcrlib_a::{
        drawchar, drawpic, rnd, rndt, PlaySound, SDL_AudioSpec, SDL_sem, SavedSoundStruct,
        WaitEndSound, WaitVBL,
    },
    pcrlib_a_state::PcrlibAState,
    pcrlib_c::{
        ControlPlayer, LoadDemo, UpdateScreen, _Verify, _checkhighscore, _quit, _setupgame,
        _showhighscores, bar, bioskey, bloadin, centerwindow, clearkeys, drawwindow, expwin, get,
        joyinfo_t, port_temp_LoadFile, port_temp_print_str, printchartile, C2RustUnnamed_5,
        SDL_DisplayMode, SDL_GameController, SDL_Rect,
    },
    pcrlib_c_state::PcrlibCState,
    rleasm::RLEExpand,
    safe_sdl::{SDL_Renderer, SDL_Texture, SDL_Window, SDL_mutex},
    scan_codes::*,
    scores::scores,
    sound_type::soundtype::*,
    state_type::statetype,
    vec2::Vec2,
};
extern "C" {
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: u64) -> i64;
    fn free(_: *mut libc::c_void);
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
}

const demowin: [[i8; 16]; 5] = [
    [
        14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16,
    ],
    [
        17,
        ' ' as i32 as i8,
        '-' as i32 as i8,
        '-' as i32 as i8,
        '-' as i32 as i8,
        ' ' as i32 as i8,
        'D' as i32 as i8,
        'E' as i32 as i8,
        'M' as i32 as i8,
        'O' as i32 as i8,
        ' ' as i32 as i8,
        '-' as i32 as i8,
        '-' as i32 as i8,
        '-' as i32 as i8,
        ' ' as i32 as i8,
        18,
    ],
    [
        17,
        'S' as i32 as i8,
        'P' as i32 as i8,
        'A' as i32 as i8,
        'C' as i32 as i8,
        'E' as i32 as i8,
        ' ' as i32 as i8,
        'T' as i32 as i8,
        'O' as i32 as i8,
        ' ' as i32 as i8,
        'S' as i32 as i8,
        'T' as i32 as i8,
        'A' as i32 as i8,
        'R' as i32 as i8,
        'T' as i32 as i8,
        18,
    ],
    [
        17,
        'F' as i32 as i8,
        '1' as i32 as i8,
        ' ' as i32 as i8,
        'T' as i32 as i8,
        'O' as i32 as i8,
        ' ' as i32 as i8,
        'G' as i32 as i8,
        'E' as i32 as i8,
        'T' as i32 as i8,
        ' ' as i32 as i8,
        'H' as i32 as i8,
        'E' as i32 as i8,
        'L' as i32 as i8,
        'P' as i32 as i8,
        18,
    ],
    [
        19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21,
    ],
];

pub unsafe fn refresh(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut basex: i32 = 0;
    let mut basey: i32 = 0;
    let mut underwin: [[u16; 16]; 5] = [[0; 16]; 5];
    basex = gs.origin.x + 4;
    basey = gs.origin.y + 17;
    if gs.indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                underwin[y as usize][x as usize] =
                    gs.view[(y + basey) as usize][(x + basex) as usize] as u16;
                gs.view[(y + basey) as usize][(x + basex) as usize] =
                    demowin[y as usize][x as usize] as i32;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL(pas);
    if pcs.grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(gs, pcs);
    } else {
        egarefresh(gs, pcs);
    }
    if gs.indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                gs.view[(y + basey) as usize][(x + basex) as usize] =
                    underwin[y as usize][x as usize] as i32;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL(pas);
}

unsafe fn simplerefresh(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    WaitVBL(pas);
    if pcs.grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(gs, pcs);
    } else {
        egarefresh(gs, pcs);
    };
}

pub unsafe fn loadgrfiles(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    if !gs.picsexact.is_null() {
        free(gs.picsexact as *mut libc::c_void);
    }
    if pcs.grmode as u32 == CGAgr as i32 as u32 {
        gs.pics = bloadin("CGACHARS.CA2") as *mut i8;
        gs.picsexact = gs.pics;
        installgrfile(
            b"CGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
            cps,
            pcs,
        );
    } else {
        gs.pics = bloadin("EGACHARS.CA2") as *mut i8;
        gs.picsexact = gs.pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
            cps,
            pcs,
        );
    };
}

pub fn clearold(oldtiles: &mut [i32; 576]) {
    oldtiles.fill(0xff);
}

pub unsafe fn restore(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    clearold(&mut gs.oldtiles);
    simplerefresh(gs, pas, pcs);
}

unsafe fn wantmore(
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) -> boolean {
    pcs.sx = 2;
    pcs.sy = 20;
    port_temp_print_str("(space for more/esc)", gs, pcs);
    pcs.sx = 12;
    pcs.sy = 21;
    let ch = get(gs, pas, pcs) as i8;
    if ch == 27 {
        return false as boolean;
    }
    return true as boolean;
}
unsafe fn charpic(
    x: i32,
    y: i32,
    c: classtype,
    dir: dirtype,
    stage: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
) {
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut size: i32 = 0;
    let mut tilenum: i32 = 0;
    size = gs.objdef[c as usize].size as i32;
    tilenum = (gs.objdef[c as usize].firstchar as u32).wrapping_add(
        ((size * size) as u32).wrapping_mul(
            (dir as u32 & gs.objdef[c as usize].dirmask as u32)
                .wrapping_mul(gs.objdef[c as usize].stages as u32)
                .wrapping_add(stage as u32),
        ),
    ) as i32;
    yy = y;
    while yy <= y + size - 1 {
        xx = x;
        while xx <= x + size - 1 {
            let fresh0 = tilenum;
            tilenum += 1;
            drawchartile(xx, yy, fresh0, gs, pcs);
            xx += 1;
        }
        yy += 1;
    }
}

unsafe fn help(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("  C A T A C O M B   \n", gs, pcs);
    port_temp_print_str("   - - - - - - -    \n", gs, pcs);
    port_temp_print_str(" by John Carmack    \n", gs, pcs);
    port_temp_print_str("                    \n", gs, pcs);
    port_temp_print_str("\n", gs, pcs);
    port_temp_print_str("f1 = help           \n", gs, pcs);
    port_temp_print_str("f2 = control panel  \n", gs, pcs);
    port_temp_print_str("f3 = game reset     \n", gs, pcs);
    port_temp_print_str("f4 = save game      \n", gs, pcs);
    port_temp_print_str("f5 = load saved game\n", gs, pcs);
    port_temp_print_str("f9 = pause          \n", gs, pcs);
    port_temp_print_str("f10 / ESC = quit    \n", gs, pcs);
    port_temp_print_str("\n", gs, pcs);
    port_temp_print_str("hit fire at the demo\n", gs, pcs);
    port_temp_print_str("to begin playing.   \n", gs, pcs);
    if wantmore(gs, pas, pcs) == 0 {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("\nKeyboard controls:  \n\n", gs, pcs);
    port_temp_print_str("move    : arrows    \n", gs, pcs);
    port_temp_print_str("button1 : ctrl      \n", gs, pcs);
    port_temp_print_str("button2 : alt       \n", gs, pcs);
    port_temp_print_str("\nTo switch to mouse \n", gs, pcs);
    port_temp_print_str("or joystick control,\n", gs, pcs);
    port_temp_print_str("hit f2             \n", gs, pcs);
    if wantmore(gs, pas, pcs) == 0 {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("Button 1 / ctrl key:\n", gs, pcs);
    port_temp_print_str("Builds shot power.  \n", gs, pcs);
    port_temp_print_str("If the shot power   \n", gs, pcs);
    port_temp_print_str("meter is full when  \n", gs, pcs);
    port_temp_print_str("the button is       \n", gs, pcs);
    port_temp_print_str("released, a super   \n", gs, pcs);
    port_temp_print_str("shot will be        \n", gs, pcs);
    port_temp_print_str("launched.           \n", gs, pcs);
    port_temp_print_str("\n", gs, pcs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128, gs, pcs);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2, gs, pcs);
    charpic(19, 15, shot, east, 1, gs, pcs);
    charpic(17, 14, shot, east, 0, gs, pcs);
    charpic(15, 15, shot, east, 1, gs, pcs);
    charpic(8, 14, bigshot, east, 0, gs, pcs);
    if wantmore(gs, pas, pcs) == 0 {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("Button 2 / alt key:\n", gs, pcs);
    port_temp_print_str("Allows you to move  \n", gs, pcs);
    port_temp_print_str("without changing the\n", gs, pcs);
    port_temp_print_str("direction you are   \n", gs, pcs);
    port_temp_print_str("facing.  Good for   \n", gs, pcs);
    port_temp_print_str("searching walls and \n", gs, pcs);
    port_temp_print_str("fighting retreats.  \n", gs, pcs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            if y == 15 {
                drawchartile(x, y, 129, gs, pcs);
            } else if y == 16 {
                drawchartile(x, y, 131, gs, pcs);
            } else {
                drawchartile(x, y, 128, gs, pcs);
            }
            x += 1;
        }
        y += 1;
    }
    charpic(6, 13, player, south, 2, gs, pcs);
    pcs.sx = 6;
    pcs.sy = 15;
    port_temp_print_str("\x1D\x1D\x1E\x1E\x1F\x1F", gs, pcs);
    if wantmore(gs, pas, pcs) == 0 {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("\"P\" or \"space\" will \n", gs, pcs);
    port_temp_print_str("take a healing      \n", gs, pcs);
    port_temp_print_str("potion if you have  \n", gs, pcs);
    port_temp_print_str("one.  This restores \n", gs, pcs);
    port_temp_print_str("the body meter to   \n", gs, pcs);
    port_temp_print_str("full strength.  Keep\n", gs, pcs);
    port_temp_print_str("a sharp eye on the  \n", gs, pcs);
    port_temp_print_str("meter, because when \n", gs, pcs);
    port_temp_print_str("it runs out, you are\n", gs, pcs);
    port_temp_print_str("dead!               \n\n", gs, pcs);
    port_temp_print_str("\"B\" will cast a bolt\n", gs, pcs);
    port_temp_print_str("spell if you have   \n", gs, pcs);
    port_temp_print_str("any.  You can mow   \n", gs, pcs);
    port_temp_print_str("down a lot of       \n", gs, pcs);
    port_temp_print_str("monsters with a bit \n", gs, pcs);
    port_temp_print_str("of skill.           \n", gs, pcs);
    if wantmore(gs, pas, pcs) == 0 {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    port_temp_print_str("\"N\" or \"enter\" will \n", gs, pcs);
    port_temp_print_str("cast a nuke spell.  \n", gs, pcs);
    port_temp_print_str("This usually wipes  \n", gs, pcs);
    port_temp_print_str("out all the monsters\n", gs, pcs);
    port_temp_print_str("near you.  Consider \n", gs, pcs);
    port_temp_print_str("it a panic button   \n", gs, pcs);
    port_temp_print_str("when you are being  \n", gs, pcs);
    port_temp_print_str("mobbed by monsters! \n\n", gs, pcs);
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b"POTIONS:       \x80\xA2\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b"SCROLLS:       \x80\xA3\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b" (BOLTS/NUKES) \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b"TREASURE:      \x80\xA7\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b" (POINTS)      \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
        pcs,
    );
    wantmore(gs, pas, pcs);
}

#[allow(dead_code)]
unsafe fn reset(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    centerwindow(18, 1, gs, pcs);
    port_temp_print_str("reset game (y/n)?", gs, pcs);
    let ch = get(gs, pas, pcs) as i8;
    if ch == 'y' as i8 {
        gs.gamexit = killed;
        gs.playdone = true;
    }
}

pub unsafe fn loadlevel(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    let tokens: [classtype; 26] = [
        player, teleporter, goblin, skeleton, ogre, gargoyle, dragon, turbogre, guns, gune,
        secretgate, nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing,
        nothing, nothing, nothing, nothing, nothing, nothing, nothing,
    ];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut btile: u8 = 0;
    let mut sm = [0; 4096];
    let mut rle = [0; 4096];
    let filename = format!("LEVEL{}.CA2", pcs.level);
    port_temp_LoadFile(&filename, &mut rle);
    RLEExpand(&rle[4..], &mut sm);
    gs.numobj = 0;
    gs.o[0].x = 13;
    gs.o[0].y = 13;
    gs.o[0].stage = 0;
    gs.o[0].delay = 0;
    gs.o[0].dir = east as i32 as u16;
    gs.o[0].oldx = 0;
    gs.o[0].oldy = 0;
    gs.o[0].oldtile = -1 as i16;
    yy = 0;
    while yy < 64 {
        xx = 0;
        while xx < 64 {
            btile = sm[(yy * 64 + xx) as usize] as u8;
            if (btile as i32) < 230 {
                gs.background[(yy + 11) as usize][(xx + 11) as usize] = btile as i32;
            } else {
                gs.background[(yy + 11) as usize][(xx + 11) as usize] = 128;
                if tokens[(btile as i32 - 230) as usize] as u32 == player as i32 as u32 {
                    gs.o[0].x = (xx + 11) as u8;
                    gs.o[0].y = (yy + 11) as u8;
                } else {
                    gs.numobj += 1;
                    gs.o[gs.numobj as usize].active = false as boolean;
                    gs.o[gs.numobj as usize].class = tokens[(btile as i32 - 230) as usize];
                    gs.o[gs.numobj as usize].x = (xx + 11) as u8;
                    gs.o[gs.numobj as usize].y = (yy + 11) as u8;
                    gs.o[gs.numobj as usize].stage = 0;
                    gs.o[gs.numobj as usize].delay = 0;
                    // Defensive typecast.
                    gs.o[gs.numobj as usize].dir = Into::<dirtype>::into(rndt(pas) / 64) as u16;
                    gs.o[gs.numobj as usize].hp =
                        gs.objdef[gs.o[gs.numobj as usize].class as usize].hitpoints as i8;
                    gs.o[gs.numobj as usize].oldx = gs.o[gs.numobj as usize].x;
                    gs.o[gs.numobj as usize].oldy = gs.o[gs.numobj as usize].y;
                    gs.o[gs.numobj as usize].oldtile = -1 as i16;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    gs.origin.x = gs.o[0].x as i32 - 11;
    gs.origin.y = gs.o[0].y as i32 - 11;
    gs.shotpower = 0;
    y = 11 - 1;
    while y < 65 + 11 {
        x = 11 - 1;
        while x < 64 + 11 {
            gs.view[y as usize][x as usize] = gs.background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    pcs.sx = 33;
    pcs.sy = 1;
    port_temp_print_str(&pcs.level.to_string(), gs, pcs);
    port_temp_print_str(" ", gs, pcs);
    restore(gs, pas, pcs);
    i = 0;
    while i < 6 {
        gs.saveitems[i as usize] = gs.items[i as usize];
        i += 1;
    }
    gs.savescore = pcs.score;
    gs.saveo[0] = gs.o[0];
}

unsafe fn drawside(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    pcs.sx = 0;
    while pcs.sx < 40 {
        drawchar(pcs.sx, 24, 0, gs, pcs);
        pcs.sx += 1;
    }
    pcs.sy = 0;
    while pcs.sy < 24 {
        drawchar(39, pcs.sy, 0, gs, pcs);
        pcs.sy += 1;
    }
    drawwindow(24, 0, 38, 23, gs, pcs);
    port_temp_print_str(
        "  pcs.level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n",
        gs,
        pcs,
    );
    port_temp_print_str(" shot power\n\n\n    body\n\n\n", gs, pcs);
    printhighscore(gs, pcs);
    printbody(gs, pcs);
    printshotpower(gs, pcs);
    printscore(gs, pcs);
    pcs.sx = 33;
    pcs.sy = 1;
    port_temp_print_str(&pcs.level.to_string(), gs, pcs);
    drawpic(25 * 8, 17 * 8, 13, gs, cps, pcs);
    i = 1;
    while i <= gs.items[1] as i32 && i < 11 {
        drawchar(26 + i, 7, 31, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[2] as i32 && i < 11 {
        drawchar(26 + i, 8, 29, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[3] as i32 && i < 11 {
        drawchar(26 + i, 9, 30, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[5] as i32 && i < 11 {
        drawchar(26 + i, 10, 30, gs, pcs);
        i += 1;
    }
}

unsafe fn playsetup(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    gs.shotpower = 0;
    bar(0, 0, 23, 23, 0, gs, pcs);
    if pcs.level as i32 == 0 {
        i = 1;
        while i < 6 {
            gs.items[i as usize] = 0;
            i += 1;
        }
        pcs.score = 0;
        pcs.level = 1;
        gs.o[0].active = true as boolean;
        gs.o[0].class = player;
        gs.o[0].hp = 13;
        gs.o[0].dir = west as i32 as u16;
        gs.o[0].stage = 0;
        gs.o[0].delay = 0;
        drawside(gs, cps, pcs);
        givenuke(gs, pcs);
        givenuke(gs, pcs);
        givebolt(gs, pcs);
        givebolt(gs, pcs);
        givebolt(gs, pcs);
        givepotion(gs, pcs);
        givepotion(gs, pcs);
        givepotion(gs, pcs);
    } else {
        drawside(gs, cps, pcs);
    };
}

pub unsafe fn repaintscreen(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    match gs.gamestate {
        statetype::intitle => {
            drawpic(0, 0, 14, gs, cps, pcs);
        }
        statetype::ingame => {
            restore(gs, pas, pcs);
            drawside(gs, cps, pcs);
            printscore(gs, pcs);
            pcs.sx = 33;
            pcs.sy = 1;
            port_temp_print_str(&pcs.level.to_string(), gs, pcs);
        }
        statetype::inscores => {
            restore(gs, pas, pcs);
            drawside(gs, cps, pcs);
            printscore(gs, pcs);
            pcs.sx = 33;
            pcs.sy = 1;
            port_temp_print_str(&pcs.level.to_string(), gs, pcs);
            gs.indemo = demoenum::demoplay;
        }
    };
}

pub unsafe fn dofkeys(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut handle: i32 = 0;
    let mut key: i32 = bioskey(1, pcs);
    if key == SDL_SCANCODE_ESCAPE as i32 {
        key = SDL_SCANCODE_F10 as i32;
    }
    if key < SDL_SCANCODE_F1 as i32 || key > SDL_SCANCODE_F10 as i32 {
        return;
    }
    let current_block_72: u64;
    match key {
        58 => {
            clearkeys(pcs);
            help(gs, pas, pcs);
        }
        59 => {
            clearkeys(pcs);
            controlpanel(gs, cps, pas, pcs);
        }
        60 => {
            clearkeys(pcs);
            expwin(18, 1, gs, pas, pcs);
            port_temp_print_str("RESET GAME (Y/N)?", gs, pcs);
            let ch = (get(gs, pas, pcs) as u8).to_ascii_uppercase() as i8;
            if ch as i32 == 'Y' as i32 {
                gs.resetgame = true;
            }
        }
        61 => {
            clearkeys(pcs);
            expwin(22, 4, gs, pas, pcs);
            if gs.indemo != demoenum::notdemo {
                port_temp_print_str("Can't save game here!", gs, pcs);
                get(gs, pas, pcs);
            } else {
                port_temp_print_str("Save as game #(1-9):", gs, pcs);
                let mut ch = (get(gs, pas, pcs) as u8).to_ascii_uppercase() as i8;
                drawchar(pcs.sx, pcs.sy, ch as i32, gs, pcs);
                if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                    let str = CString::new(format!("GAME{ch}.CA2")).unwrap();
                    if _Verify(str.as_ptr()) != 0 {
                        port_temp_print_str("\nGame exists,\noverwrite (Y/N)?", gs, pcs);
                        ch = get(gs, pas, pcs) as i8;
                        if ch as i32 != 'Y' as i32 && ch as i32 != 'y' as i32 {
                            current_block_72 = 919954187481050311;
                        } else {
                            pcs.sx = pcs.leftedge;
                            port_temp_print_str("                    ", gs, pcs);
                            pcs.sy -= 1;
                            pcs.sx = pcs.leftedge;
                            port_temp_print_str("                    ", gs, pcs);
                            pcs.sx = pcs.leftedge;
                            pcs.sy -= 1;
                            current_block_72 = 1836292691772056875;
                        }
                    } else {
                        current_block_72 = 1836292691772056875;
                    }
                    match current_block_72 {
                        919954187481050311 => {}
                        _ => {
                            handle = open(
                                str.as_ptr(),
                                0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
                                0o400 as i32 | 0o200 as i32,
                            );
                            if handle == -1 {
                                return;
                            }
                            write(
                                handle,
                                &mut gs.saveitems as *mut [i16; 6] as *const libc::c_void,
                                ::std::mem::size_of::<[i16; 6]>() as u64,
                            );
                            write(
                                handle,
                                &mut gs.savescore as *mut i32 as *const libc::c_void,
                                ::std::mem::size_of::<i32>() as u64,
                            );
                            write(
                                handle,
                                &mut pcs.level as *mut i16 as *const libc::c_void,
                                ::std::mem::size_of::<i16>() as u64,
                            );
                            write(
                                handle,
                                &mut *gs.saveo.as_mut_ptr().offset(0) as *mut activeobj
                                    as *const libc::c_void,
                                ::std::mem::size_of::<activeobj>() as u64,
                            );
                            close(handle);
                            port_temp_print_str("\nGame saved.  Hit F5\n", gs, pcs);
                            port_temp_print_str("when you wish to\n", gs, pcs);
                            port_temp_print_str("restart the game.", gs, pcs);
                            get(gs, pas, pcs);
                        }
                    }
                }
            }
        }
        62 => {
            clearkeys(pcs);
            expwin(22, 4, gs, pas, pcs);
            port_temp_print_str("Load game #(1-9):", gs, pcs);
            let ch = (get(gs, pas, pcs) as u8).to_ascii_uppercase() as i8;
            drawchar(pcs.sx, pcs.sy, ch as i32, gs, pcs);
            if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                let str = CString::new(format!("GAME{ch}.CA2")).unwrap();
                // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
                // project.
                handle = open(
                    str.as_ptr(),
                    O_RDONLY | O_BINARY,
                    0o200 as i32 | 0o400 as i32,
                );
                if handle == -1 {
                    port_temp_print_str("\nGame not found.", gs, pcs);
                    get(gs, pas, pcs);
                } else {
                    read(
                        handle,
                        &mut gs.items as *mut _ as *mut libc::c_void,
                        ::std::mem::size_of::<[i16; 6]>() as u64,
                    );
                    read(
                        handle,
                        &mut pcs.score as *mut i32 as *mut libc::c_void,
                        ::std::mem::size_of::<i32>() as u64,
                    );
                    read(
                        handle,
                        &mut pcs.level as *mut i16 as *mut libc::c_void,
                        ::std::mem::size_of::<i16>() as u64,
                    );
                    read(
                        handle,
                        &mut *gs.o.as_mut_ptr().offset(0) as *mut activeobj as *mut libc::c_void,
                        ::std::mem::size_of::<activeobj>() as u64,
                    );
                    close(handle);
                    gs.exitdemo = true;
                    if gs.indemo != demoenum::notdemo {
                        gs.playdone = true;
                    }
                    drawside(gs, cps, pcs);
                    gs.leveldone = true;
                }
            }
        }
        66 => {
            clearkeys(pcs);
            expwin(7, 1, gs, pas, pcs);
            port_temp_print_str("PAUSED", gs, pcs);
            get(gs, pas, pcs);
        }
        67 => {
            clearkeys(pcs);
            expwin(12, 1, gs, pas, pcs);
            port_temp_print_str("QUIT (Y/N)?", gs, pcs);
            let ch = (get(gs, pas, pcs) as u8).to_ascii_uppercase() as i8;
            if ch == 'Y' as i8 {
                _quit(b"\0" as *const u8 as *const i8 as *mut i8, pas, pcs);
            }
        }
        _ => return,
    }
    clearold(&mut gs.oldtiles);
    clearkeys(pcs);
    repaintscreen(gs, cps, pas, pcs);
}

unsafe fn dotitlepage(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut i: i32 = 0;
    drawpic(0, 0, 14, gs, cps, pcs);
    UpdateScreen(gs, pcs);
    gs.gamestate = statetype::intitle;
    i = 0;
    while i < 300 {
        WaitVBL(pas);
        gs.indemo = demoenum::notdemo;
        gs.ctrl = ControlPlayer(1, gs, pcs);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            pcs.level = 0;
            gs.exitdemo = true;
            break;
        } else {
            gs.indemo = demoenum::demoplay;
            if bioskey(1, pcs) != 0 {
                dofkeys(gs, cps, pas, pcs);
                UpdateScreen(gs, pcs);
            }
            if gs.exitdemo {
                break;
            }
            i += 1;
        }
    }
    gs.gamestate = statetype::ingame;
}

unsafe fn doendpage(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    WaitEndSound(gs, pas, pcs);
    drawpic(0, 0, 15, gs, cps, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    drawwindow(0, 0, 17, 9, gs, pcs);
    port_temp_print_str("Congratulation! \n", gs, pcs);
    port_temp_print_str("One as skilled  \n", gs, pcs);
    port_temp_print_str("as yourself     \n", gs, pcs);
    port_temp_print_str("deserves the    \n", gs, pcs);
    port_temp_print_str("10,000,000 gold \n", gs, pcs);
    port_temp_print_str("you pulled out  \n", gs, pcs);
    port_temp_print_str("of the palace! ", gs, pcs);
    clearkeys(pcs);
    get(gs, pas, pcs);
    drawwindow(0, 0, 17, 9, gs, pcs);
    port_temp_print_str("Let us know what\n", gs, pcs);
    port_temp_print_str("you enjoyed     \n", gs, pcs);
    port_temp_print_str("about this game,\n", gs, pcs);
    port_temp_print_str("so we can give  \n", gs, pcs);
    port_temp_print_str("you more of it. \n", gs, pcs);
    port_temp_print_str("Thank you for   \n", gs, pcs);
    port_temp_print_str("playing!", gs, pcs);
    get(gs, pas, pcs);
}

unsafe fn dodemo(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut i: i32 = 0;
    while !gs.exitdemo {
        dotitlepage(gs, cps, pas, pcs);
        if gs.exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1, pas) + 1;
        LoadDemo(i, gs, pcs);
        pcs.level = 0;
        playsetup(gs, cps, pcs);
        playloop(gs, cps, pas, pcs);
        if gs.exitdemo {
            break;
        }
        pcs.level = 0;
        gs.gamestate = statetype::inscores;
        gs.indemo = demoenum::demoplay;
        _showhighscores(gs, pcs);
        UpdateScreen(gs, pcs);
        i = 0;
        while i < 500 {
            WaitVBL(pas);
            gs.indemo = demoenum::notdemo;
            gs.ctrl = ControlPlayer(1, gs, pcs);
            if gs.ctrl.button1 as i32 != 0
                || gs.ctrl.button2 as i32 != 0
                || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                gs.exitdemo = true;
                break;
            } else {
                if bioskey(1, pcs) != 0 {
                    dofkeys(gs, cps, pas, pcs);
                }
                if gs.exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}

unsafe fn gameover(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut i: i32 = 0;
    expwin(11, 4, gs, pas, pcs);
    port_temp_print_str("\n GAME OVER\n     ", gs, pcs);
    UpdateScreen(gs, pcs);
    WaitEndSound(gs, pas, pcs);
    i = 0;
    while i < 120 {
        WaitVBL(pas);
        i += 1;
    }
    gs.gamestate = statetype::inscores;
    _checkhighscore(gs, pas, pcs);
    pcs.level = 0;
    i = 0;
    while i < 500 {
        WaitVBL(pas);
        gs.ctrl = ControlPlayer(1, gs, pcs);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            break;
        }
        if bioskey(1, pcs) != 0 {
            dofkeys(gs, cps, pas, pcs);
        }
        if gs.exitdemo as i32 != 0 || gs.indemo == demoenum::demoplay {
            break;
        }
        i += 1;
    }
}

/***************************************************************************/
/***************************************************************************/

/*=========================*/
/*			   */
/* m a i n   p r o g r a m */
/*			   */
/*=========================*/

pub fn original_main() {
    let mut gs = GlobalState::new(
        [0; 2048],
        [0; 6],
        [0; 6],
        0,
        [activeobj {
            active: 0,
            class: nothing,
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
        }; 201],
        [activeobj {
            active: 0,
            class: nothing,
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
        }; 1],
        objtype {
            active: 0,
            class: nothing,
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
        },
        objtype {
            active: 0,
            class: nothing,
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
        },
        0,
        0,
        0,
        [objdeftype {
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
        }; 23],
        false,
        false,
        0,
        0,
        0,
        0,
        false,
        false,
        statetype::ingame,
        ControlStruct {
            dir: north,
            button1: 0,
            button2: 0,
        },
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        false,
        0,
        0,
        [[0; 86]; 87],
        Vec2::new(19, 11),
        quited,
        [0; 576],
        [8; 64000],
        [[0; 86]; 87],
        Vec2::new(0, 0),
        demoenum::notdemo,
    );

    let mut cps = CpanelState::new(
        [[0; 5]; 4],
        0,
        0,
        text,
        text,
        off,
        off,
        [keyboard; 3],
        [keyboard; 3],
        0,
        0,
        0,
        [0; 4],
        spritetype {
            width: 0,
            height: 0,
            shapeptr: 0,
            maskptr: 0,
            xl: 0,
            yl: 0,
            xh: 0,
            yh: 0,
            name: [0; 12],
        },
        [spritetype {
            width: 0,
            height: 0,
            shapeptr: 0,
            maskptr: 0,
            xl: 0,
            yl: 0,
            xh: 0,
            yh: 0,
            name: [0; 12],
        }; 10],
        ptr::null_mut(),
        0,
        0,
        0,
        0,
        [pictype {
            width: 0,
            height: 0,
            shapeptr: 0,
            name: [0; 8],
        }; 64],
    );

    let mut pas = PcrlibAState::new(
        0,
        0,
        0 as *mut SDL_mutex,
        SDL_AudioSpec {
            freq: 0,
            format: 0,
            channels: 0,
            silence: 0,
            samples: 0,
            padding: 0,
            size: 0,
            callback: None,
            userdata: ptr::null_mut(),
        },
        0,
        5000,
        0,
        0,
        false as boolean,
        0,
        0,
        0,
        0,
        ptr::null_mut(),
        SavedSoundStruct {
            SndPriority: 0,
            pcSamplesPerTick: 0,
            pcLengthLeft: 0,
            pcSound: 0 as *const u16 as *mut u16,
        },
        0,
        0,
        0,
        0,
        [0; 17],
        0 as *mut SDL_sem,
        0,
        ptr::null_mut(),
        spkr,
        0,
    );

    let mut pcs = PcrlibCState::new(
        0,
        [0; 5000],
        0,
        0,
        0,
        SDL_SCANCODE_UNKNOWN,
        0 as *const SDL_Window as *mut SDL_Window,
        0 as *const SDL_Renderer as *mut SDL_Renderer,
        0 as *const SDL_Texture as *mut SDL_Texture,
        SDL_Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        },
        [keyboard, keyboard, joystick1],
        [0; 512],
        [0; 3],
        [0; 3],
        [0; 3],
        [0; 3],
        0,
        [0; 8],
        0,
        0,
        SDL_DisplayMode {
            format: 0,
            w: 0,
            h: 0,
            refresh_rate: 0,
            driverdata: 0 as *const libc::c_void as *mut libc::c_void,
        },
        [joyinfo_t {
            c2rust_unnamed: C2RustUnnamed_5 {
                controller: 0 as *const SDL_GameController as *mut SDL_GameController,
            },
            device: 0,
            isgamecontroller: 0,
        }; 3],
        true as boolean,
        0,
        0,
        0,
        0,
        [0; 64000],
        text,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        [0; 4],
        0,
        0,
        0,
        [scores {
            score: 0,
            level: 0,
            initials: [0; 4],
        }; 5],
        0,
        0,
    );

    /***************************************************************************/

    let ver_arg_position = std::env::args().position(|arg| arg == "/VER");

    if let Some(1) = ver_arg_position {
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
        std::process::exit(0);
    }

    initobjects(&mut gs.objdef);

    gs.priority.fill(99);

    gs.priority[blankfloor] = 0;
    for i in
        gs.objdef[teleporter as usize].firstchar..=gs.objdef[teleporter as usize].firstchar + 20
    {
        gs.priority[i as usize] = 0;
    }
    for clvar in (dead2 as usize)..=(dead5 as usize) {
        for i in gs.objdef[clvar].firstchar
            ..=(gs.objdef[clvar].firstchar
                + gs.objdef[clvar].size as u16 * gs.objdef[clvar].size as u16)
        {
            gs.priority[i as usize] = 0; /*deadthing*/
        }
    }
    for i in 152..=161 {
        gs.priority[i] = 2; /*shots*/
    }
    for i in gs.objdef[bigshot as usize].firstchar..=(gs.objdef[bigshot as usize].firstchar + 31) {
        gs.priority[i as usize] = 2; /*bigshot*/
    }
    for i in 0..=(tile2s - 1) {
        if gs.priority[i] == 99 {
            gs.priority[i] = 3; /*most 1*1 tiles are walls, etc*/
        }
    }
    gs.priority[167] = 1; // chest
    for i in tile2s..=maxpics {
        if gs.priority[i] as i32 == 99 {
            gs.priority[i] = 4; /*most bigger tiles are monsters*/
        }
    }
    for i in gs.objdef[player as usize].firstchar..=(gs.objdef[player as usize].firstchar + 63) {
        gs.priority[i as usize] = 5; /*player*/
    }

    gs.side = 0;

    for x in 0..=85 {
        for y in 0..=(topoff - 1) {
            gs.view[x][y] = solidwall;
            gs.view[x][(85 - y)] = solidwall;
            gs.background[x][y] = solidwall;
            gs.background[x][(85 - y)] = solidwall;
        }
        gs.view[86][x] = solidwall;
    }
    for y in 11..=74 {
        for x in 0..=(leftoff - 1) {
            gs.view[x][y] = solidwall;
            gs.view[(85 - x)][y] = solidwall;
            gs.background[x][y] = solidwall;
            gs.background[(85 - x)][y] = solidwall;
        }
    }

    //   puts ("CATACOMB II is executing");

    //  _dontplay = 1;	// no sounds for debugging and profiling

    _setupgame(&mut gs, &mut cps, &mut pas, &mut pcs);

    unsafe {
        expwin(33, 13, &mut gs, &mut pas, &mut pcs);
        port_temp_print_str("  Softdisk Publishing presents\n\n", &mut gs, &mut pcs);
        port_temp_print_str("          The Catacomb\n\n", &mut gs, &mut pcs);
        port_temp_print_str("        By John Carmack\n\n", &mut gs, &mut pcs);
        port_temp_print_str("       Copyright 1990-93\n", &mut gs, &mut pcs);
        port_temp_print_str("      Softdisk Publishing", &mut gs, &mut pcs);
        port_temp_print_str("\n\n", &mut gs, &mut pcs);
        port_temp_print_str("\n\n", &mut gs, &mut pcs);
        port_temp_print_str("         Press a key:", &mut gs, &mut pcs);
        get(&mut gs, &mut pas, &mut pcs);

        clearkeys(&mut pcs);

        gs.screencenter.x = 11;
        gs.screencenter.y = 11;

        gs.exitdemo = false;
        pcs.level = 0;

        // go until quit () is called
        loop {
            dodemo(&mut gs, &mut cps, &mut pas, &mut pcs);
            playsetup(&mut gs, &mut cps, &mut pcs);
            gs.indemo = demoenum::notdemo;
            gs.gamestate = statetype::ingame;
            playloop(&mut gs, &mut cps, &mut pas, &mut pcs);
            if gs.indemo == demoenum::notdemo {
                gs.exitdemo = false;
                if pcs.level > numlevels {
                    doendpage(&mut gs, &mut cps, &mut pas, &mut pcs); // finished all levels
                }
                gameover(&mut gs, &mut cps, &mut pas, &mut pcs);
            }
        }
    }
}
