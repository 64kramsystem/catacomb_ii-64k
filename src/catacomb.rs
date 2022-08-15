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
        _showhighscores, bar, bioskey, bloadin, centerwindow, ch, clearkeys, drawwindow, expwin,
        get, grmode, keydown, leftedge, level, port_temp_LoadFile, print, printchartile, printint,
        score, sx, sy,
    },
    rleasm::port_temp_RLEExpand,
    safe_sdl::SDL_mutex,
    scan_codes::*,
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

pub unsafe fn refresh(gs: &mut GlobalState, pas: &mut PcrlibAState) {
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
    if grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(gs);
    } else {
        egarefresh(gs);
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

unsafe fn simplerefresh(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    WaitVBL(pas);
    if grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(gs);
    } else {
        egarefresh(gs);
    };
}

pub unsafe fn loadgrfiles(gs: &mut GlobalState, cps: &mut CpanelState) {
    if !gs.picsexact.is_null() {
        free(gs.picsexact as *mut libc::c_void);
    }
    if grmode as u32 == CGAgr as i32 as u32 {
        gs.pics = bloadin("CGACHARS.CA2") as *mut i8;
        gs.picsexact = gs.pics;
        installgrfile(
            b"CGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
            cps,
        );
    } else {
        gs.pics = bloadin("EGACHARS.CA2") as *mut i8;
        gs.picsexact = gs.pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
            cps,
        );
    };
}

pub fn clearold(oldtiles: &mut [i32; 576]) {
    oldtiles.fill(0xff);
}

pub unsafe fn restore(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    clearold(&mut gs.oldtiles);
    simplerefresh(gs, pas);
}

unsafe fn wantmore(gs: &mut GlobalState, pas: &mut PcrlibAState) -> boolean {
    sx = 2;
    sy = 20;
    print(b"(space for more/esc)\0" as *const u8 as *const i8, gs);
    sx = 12;
    sy = 21;
    ch = get(gs, pas) as i8;
    if ch as i32 == 27 {
        return false as boolean;
    }
    return true as boolean;
}
unsafe fn charpic(
    mut x: i32,
    mut y: i32,
    mut c: classtype,
    mut dir: dirtype,
    mut stage: i32,
    gs: &mut GlobalState,
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
            tilenum = tilenum + 1;
            drawchartile(xx, yy, fresh0, gs);
            xx += 1;
        }
        yy += 1;
    }
}

unsafe fn help(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    centerwindow(20, 20, gs);
    print(b"  C A T A C O M B   \n\0" as *const u8 as *const i8, gs);
    print(b"   - - - - - - -    \n\0" as *const u8 as *const i8, gs);
    print(b" by John Carmack    \n\0" as *const u8 as *const i8, gs);
    print(b"                    \n\0" as *const u8 as *const i8, gs);
    print(b"\n\0" as *const u8 as *const i8, gs);
    print(b"f1 = help           \n\0" as *const u8 as *const i8, gs);
    print(b"f2 = control panel  \n\0" as *const u8 as *const i8, gs);
    print(b"f3 = game reset     \n\0" as *const u8 as *const i8, gs);
    print(b"f4 = save game      \n\0" as *const u8 as *const i8, gs);
    print(b"f5 = load saved game\n\0" as *const u8 as *const i8, gs);
    print(b"f9 = pause          \n\0" as *const u8 as *const i8, gs);
    print(b"f10 / ESC = quit    \n\0" as *const u8 as *const i8, gs);
    print(b"\n\0" as *const u8 as *const i8, gs);
    print(b"hit fire at the demo\n\0" as *const u8 as *const i8, gs);
    print(b"to begin playing.   \n\0" as *const u8 as *const i8, gs);
    if wantmore(gs, pas) == 0 {
        return;
    }
    centerwindow(20, 20, gs);
    print(
        b"\nKeyboard controls:  \n\n\0" as *const u8 as *const i8,
        gs,
    );
    print(b"move    : arrows    \n\0" as *const u8 as *const i8, gs);
    print(b"button1 : ctrl      \n\0" as *const u8 as *const i8, gs);
    print(b"button2 : alt       \n\0" as *const u8 as *const i8, gs);
    print(b"\nTo switch to mouse \n\0" as *const u8 as *const i8, gs);
    print(b"or joystick control,\n\0" as *const u8 as *const i8, gs);
    print(b"hit f2             \n\0" as *const u8 as *const i8, gs);
    if wantmore(gs, pas) == 0 {
        return;
    }
    centerwindow(20, 20, gs);
    print(b"Button 1 / ctrl key:\n\0" as *const u8 as *const i8, gs);
    print(b"Builds shot power.  \n\0" as *const u8 as *const i8, gs);
    print(b"If the shot power   \n\0" as *const u8 as *const i8, gs);
    print(b"meter is full when  \n\0" as *const u8 as *const i8, gs);
    print(b"the button is       \n\0" as *const u8 as *const i8, gs);
    print(b"released, a super   \n\0" as *const u8 as *const i8, gs);
    print(b"shot will be        \n\0" as *const u8 as *const i8, gs);
    print(b"launched.           \n\0" as *const u8 as *const i8, gs);
    print(b"\n\0" as *const u8 as *const i8, gs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128, gs);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2, gs);
    charpic(19, 15, shot, east, 1, gs);
    charpic(17, 14, shot, east, 0, gs);
    charpic(15, 15, shot, east, 1, gs);
    charpic(8, 14, bigshot, east, 0, gs);
    if wantmore(gs, pas) == 0 {
        return;
    }
    centerwindow(20, 20, gs);
    print(b"Button 2 / alt key:\n\0" as *const u8 as *const i8, gs);
    print(b"Allows you to move  \n\0" as *const u8 as *const i8, gs);
    print(b"without changing the\n\0" as *const u8 as *const i8, gs);
    print(b"direction you are   \n\0" as *const u8 as *const i8, gs);
    print(b"facing.  Good for   \n\0" as *const u8 as *const i8, gs);
    print(b"searching walls and \n\0" as *const u8 as *const i8, gs);
    print(b"fighting retreats.  \n\0" as *const u8 as *const i8, gs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            if y == 15 {
                drawchartile(x, y, 129, gs);
            } else if y == 16 {
                drawchartile(x, y, 131, gs);
            } else {
                drawchartile(x, y, 128, gs);
            }
            x += 1;
        }
        y += 1;
    }
    charpic(6, 13, player, south, 2, gs);
    sx = 6;
    sy = 15;
    print(b"\x1D\x1D\x1E\x1E\x1F\x1F\0" as *const u8 as *const i8, gs);
    if wantmore(gs, pas) == 0 {
        return;
    }
    centerwindow(20, 20, gs);
    print(
        b"\"P\" or \"space\" will \n\0" as *const u8 as *const i8,
        gs,
    );
    print(b"take a healing      \n\0" as *const u8 as *const i8, gs);
    print(b"potion if you have  \n\0" as *const u8 as *const i8, gs);
    print(b"one.  This restores \n\0" as *const u8 as *const i8, gs);
    print(b"the body meter to   \n\0" as *const u8 as *const i8, gs);
    print(b"full strength.  Keep\n\0" as *const u8 as *const i8, gs);
    print(b"a sharp eye on the  \n\0" as *const u8 as *const i8, gs);
    print(b"meter, because when \n\0" as *const u8 as *const i8, gs);
    print(b"it runs out, you are\n\0" as *const u8 as *const i8, gs);
    print(b"dead!               \n\n\0" as *const u8 as *const i8, gs);
    print(b"\"B\" will cast a bolt\n\0" as *const u8 as *const i8, gs);
    print(b"spell if you have   \n\0" as *const u8 as *const i8, gs);
    print(b"any.  You can mow   \n\0" as *const u8 as *const i8, gs);
    print(b"down a lot of       \n\0" as *const u8 as *const i8, gs);
    print(b"monsters with a bit \n\0" as *const u8 as *const i8, gs);
    print(b"of skill.           \n\0" as *const u8 as *const i8, gs);
    if wantmore(gs, pas) == 0 {
        return;
    }
    centerwindow(20, 20, gs);
    print(
        b"\"N\" or \"enter\" will \n\0" as *const u8 as *const i8,
        gs,
    );
    print(b"cast a nuke spell.  \n\0" as *const u8 as *const i8, gs);
    print(b"This usually wipes  \n\0" as *const u8 as *const i8, gs);
    print(b"out all the monsters\n\0" as *const u8 as *const i8, gs);
    print(b"near you.  Consider \n\0" as *const u8 as *const i8, gs);
    print(b"it a panic button   \n\0" as *const u8 as *const i8, gs);
    print(b"when you are being  \n\0" as *const u8 as *const i8, gs);
    print(b"mobbed by monsters! \n\n\0" as *const u8 as *const i8, gs);
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b"POTIONS:       \x80\xA2\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b"SCROLLS:       \x80\xA3\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b" (BOLTS/NUKES) \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b"TREASURE:      \x80\xA7\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b" (POINTS)      \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    printchartile(
        b"               \x80\x80\x80\n\0" as *const u8 as *const i8,
        gs,
    );
    wantmore(gs, pas);
}

unsafe fn reset(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    centerwindow(18, 1, gs);
    print(b"reset game (y/n)?\0" as *const u8 as *const i8, gs);
    ch = get(gs, pas) as i8;
    if ch as i32 == 'y' as i32 {
        gs.gamexit = killed;
        gs.playdone = true;
    }
}

pub unsafe fn loadlevel(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut i: i32 = 0;
    let mut tokens: [classtype; 26] = [
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
    let mut filename = format!("LEVEL{level}.CA2");
    port_temp_LoadFile(&filename, &mut rle);
    port_temp_RLEExpand(&mut rle[4..], &mut sm);
    gs.numobj = 0;
    gs.o[0].x = 13;
    gs.o[0].y = 13;
    gs.o[0].stage = 0;
    gs.o[0].delay = 0;
    gs.o[0].dir = east as i32 as u16;
    gs.o[0].oldx = 0;
    gs.o[0].oldy = 0;
    gs.o[0].oldtile = -(1) as i16;
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
                    gs.o[gs.numobj as usize].oldtile = -(1) as i16;
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
    sx = 33;
    sy = 1;
    printint(level as i32, gs);
    print(b" \0" as *const u8 as *const i8, gs);
    restore(gs, pas);
    i = 0;
    while i < 6 {
        gs.saveitems[i as usize] = gs.items[i as usize];
        i += 1;
    }
    gs.savescore = score;
    gs.saveo[0] = gs.o[0];
}

unsafe fn drawside(gs: &mut GlobalState, cps: &mut CpanelState) {
    let mut i: i32 = 0;
    sx = 0;
    while sx < 40 {
        drawchar(sx, 24, 0, gs);
        sx += 1;
    }
    sy = 0;
    while sy < 24 {
        drawchar(39, sy, 0, gs);
        sy += 1;
    }
    drawwindow(24, 0, 38, 23, gs);
    print(
        b"  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n\0" as *const u8 as *const i8,
        gs,
    );
    print(
        b" shot power\n\n\n    body\n\n\n\0" as *const u8 as *const i8,
        gs,
    );
    printhighscore(gs);
    printbody(gs);
    printshotpower(gs);
    printscore(gs);
    sx = 33;
    sy = 1;
    printint(level as i32, gs);
    drawpic(25 * 8, 17 * 8, 13, gs, cps);
    i = 1;
    while i <= gs.items[1] as i32 && i < 11 {
        drawchar(26 + i, 7, 31, gs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[2] as i32 && i < 11 {
        drawchar(26 + i, 8, 29, gs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[3] as i32 && i < 11 {
        drawchar(26 + i, 9, 30, gs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[5] as i32 && i < 11 {
        drawchar(26 + i, 10, 30, gs);
        i += 1;
    }
}

unsafe fn playsetup(gs: &mut GlobalState, cps: &mut CpanelState) {
    let mut i: i32 = 0;
    gs.shotpower = 0;
    bar(0, 0, 23, 23, 0, gs);
    if level as i32 == 0 {
        i = 1;
        while i < 6 {
            gs.items[i as usize] = 0;
            i += 1;
        }
        score = 0;
        level = 1;
        gs.o[0].active = true as boolean;
        gs.o[0].class = player;
        gs.o[0].hp = 13;
        gs.o[0].dir = west as i32 as u16;
        gs.o[0].stage = 0;
        gs.o[0].delay = 0;
        drawside(gs, cps);
        givenuke(gs);
        givenuke(gs);
        givebolt(gs);
        givebolt(gs);
        givebolt(gs);
        givepotion(gs);
        givepotion(gs);
        givepotion(gs);
    } else {
        drawside(gs, cps);
    };
}

pub unsafe fn repaintscreen(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    match gs.gamestate {
        statetype::intitle => {
            drawpic(0, 0, 14, gs, cps);
        }
        statetype::ingame => {
            restore(gs, pas);
            drawside(gs, cps);
            printscore(gs);
            sx = 33;
            sy = 1;
            printint(level as i32, gs);
        }
        statetype::inscores => {
            restore(gs, pas);
            drawside(gs, cps);
            printscore(gs);
            sx = 33;
            sy = 1;
            printint(level as i32, gs);
            gs.indemo = demoenum::demoplay;
        }
    };
}

pub unsafe fn dofkeys(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    let mut handle: i32 = 0;
    let mut key: i32 = bioskey(1);
    if key == SDL_SCANCODE_ESCAPE as i32 {
        key = SDL_SCANCODE_F10 as i32;
    }
    if key < SDL_SCANCODE_F1 as i32 || key > SDL_SCANCODE_F10 as i32 {
        return;
    }
    let mut current_block_72: u64;
    match key {
        58 => {
            clearkeys();
            help(gs, pas);
        }
        59 => {
            clearkeys();
            controlpanel(gs, cps, pas);
        }
        60 => {
            clearkeys();
            expwin(18, 1, gs, pas);
            print(b"RESET GAME (Y/N)?\0" as *const u8 as *const i8, gs);
            ch = (get(gs, pas) as u8).to_ascii_uppercase() as i8;
            if ch as i32 == 'Y' as i32 {
                gs.resetgame = true;
            }
        }
        61 => {
            clearkeys();
            expwin(22, 4, gs, pas);
            if gs.indemo != demoenum::notdemo {
                print(b"Can't save game here!\0" as *const u8 as *const i8, gs);
                get(gs, pas);
            } else {
                print(b"Save as game #(1-9):\0" as *const u8 as *const i8, gs);
                ch = (get(gs, pas) as u8).to_ascii_uppercase() as i8;
                drawchar(sx, sy, ch as i32, gs);
                if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                    let str = CString::new(format!("GAME{ch}.CA2")).unwrap();
                    if _Verify(str.as_ptr()) != 0 {
                        print(
                            b"\nGame exists,\noverwrite (Y/N)?\0" as *const u8 as *const i8,
                            gs,
                        );
                        ch = get(gs, pas) as i8;
                        if ch as i32 != 'Y' as i32 && ch as i32 != 'y' as i32 {
                            current_block_72 = 919954187481050311;
                        } else {
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const i8, gs);
                            sy -= 1;
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const i8, gs);
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
                                str.as_ptr(),
                                0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
                                0o400 as i32 | 0o200 as i32,
                            );
                            if handle == -(1) {
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
                                &mut level as *mut i16 as *const libc::c_void,
                                ::std::mem::size_of::<i16>() as u64,
                            );
                            write(
                                handle,
                                &mut *gs.saveo.as_mut_ptr().offset(0) as *mut activeobj
                                    as *const libc::c_void,
                                ::std::mem::size_of::<activeobj>() as u64,
                            );
                            close(handle);
                            print(b"\nGame saved.  Hit F5\n\0" as *const u8 as *const i8, gs);
                            print(b"when you wish to\n\0" as *const u8 as *const i8, gs);
                            print(b"restart the game.\0" as *const u8 as *const i8, gs);
                            get(gs, pas);
                        }
                    }
                }
            }
        }
        62 => {
            clearkeys();
            expwin(22, 4, gs, pas);
            print(b"Load game #(1-9):\0" as *const u8 as *const i8, gs);
            ch = (get(gs, pas) as u8).to_ascii_uppercase() as i8;
            drawchar(sx, sy, ch as i32, gs);
            if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                let str = CString::new(format!("GAME{ch}.CA2")).unwrap();
                // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
                // project.
                handle = open(
                    str.as_ptr(),
                    O_RDONLY | O_BINARY,
                    0o200 as i32 | 0o400 as i32,
                );
                if handle == -(1) {
                    print(b"\nGame not found.\0" as *const u8 as *const i8, gs);
                    get(gs, pas);
                } else {
                    read(
                        handle,
                        &mut gs.items as *mut _ as *mut libc::c_void,
                        ::std::mem::size_of::<[i16; 6]>() as u64,
                    );
                    read(
                        handle,
                        &mut score as *mut i32 as *mut libc::c_void,
                        ::std::mem::size_of::<i32>() as u64,
                    );
                    read(
                        handle,
                        &mut level as *mut i16 as *mut libc::c_void,
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
                    drawside(gs, cps);
                    gs.leveldone = true;
                }
            }
        }
        66 => {
            clearkeys();
            expwin(7, 1, gs, pas);
            print(b"PAUSED\0" as *const u8 as *const i8, gs);
            get(gs, pas);
        }
        67 => {
            clearkeys();
            expwin(12, 1, gs, pas);
            print(b"QUIT (Y/N)?\0" as *const u8 as *const i8, gs);
            ch = (get(gs, pas) as u8).to_ascii_uppercase() as i8;
            if ch as i32 == 'Y' as i32 {
                _quit(b"\0" as *const u8 as *const i8 as *mut i8, pas);
            }
        }
        _ => return,
    }
    clearold(&mut gs.oldtiles);
    clearkeys();
    repaintscreen(gs, cps, pas);
}

unsafe fn dotitlepage(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    let mut i: i32 = 0;
    drawpic(0, 0, 14, gs, cps);
    UpdateScreen(gs);
    gs.gamestate = statetype::intitle;
    i = 0;
    while i < 300 {
        WaitVBL(pas);
        gs.indemo = demoenum::notdemo;
        gs.ctrl = ControlPlayer(1, gs);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            level = 0;
            gs.exitdemo = true;
            break;
        } else {
            gs.indemo = demoenum::demoplay;
            if bioskey(1) != 0 {
                dofkeys(gs, cps, pas);
                UpdateScreen(gs);
            }
            if gs.exitdemo {
                break;
            }
            i += 1;
        }
    }
    gs.gamestate = statetype::ingame;
}

unsafe fn doendpage(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    WaitEndSound(gs, pas);
    drawpic(0, 0, 15, gs, cps);
    PlaySound(3, pas);
    WaitEndSound(gs, pas);
    PlaySound(3, pas);
    WaitEndSound(gs, pas);
    PlaySound(3, pas);
    WaitEndSound(gs, pas);
    PlaySound(3, pas);
    WaitEndSound(gs, pas);
    drawwindow(0, 0, 17, 9, gs);
    print(b"Congratulation! \n\0" as *const u8 as *const i8, gs);
    print(b"One as skilled  \n\0" as *const u8 as *const i8, gs);
    print(b"as yourself     \n\0" as *const u8 as *const i8, gs);
    print(b"deserves the    \n\0" as *const u8 as *const i8, gs);
    print(b"10,000,000 gold \n\0" as *const u8 as *const i8, gs);
    print(b"you pulled out  \n\0" as *const u8 as *const i8, gs);
    print(b"of the palace! \0" as *const u8 as *const i8, gs);
    clearkeys();
    get(gs, pas);
    drawwindow(0, 0, 17, 9, gs);
    print(b"Let us know what\n\0" as *const u8 as *const i8, gs);
    print(b"you enjoyed     \n\0" as *const u8 as *const i8, gs);
    print(b"about this game,\n\0" as *const u8 as *const i8, gs);
    print(b"so we can give  \n\0" as *const u8 as *const i8, gs);
    print(b"you more of it. \n\0" as *const u8 as *const i8, gs);
    print(b"Thank you for   \n\0" as *const u8 as *const i8, gs);
    print(b"playing!\0" as *const u8 as *const i8, gs);
    get(gs, pas);
}

unsafe fn dodemo(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    let mut i: i32 = 0;
    while !gs.exitdemo {
        dotitlepage(gs, cps, pas);
        if gs.exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1, pas) + 1;
        LoadDemo(i, gs);
        level = 0;
        playsetup(gs, cps);
        playloop(gs, cps, pas);
        if gs.exitdemo {
            break;
        }
        level = 0;
        gs.gamestate = statetype::inscores;
        gs.indemo = demoenum::demoplay;
        _showhighscores(gs);
        UpdateScreen(gs);
        i = 0;
        while i < 500 {
            WaitVBL(pas);
            gs.indemo = demoenum::notdemo;
            gs.ctrl = ControlPlayer(1, gs);
            if gs.ctrl.button1 as i32 != 0
                || gs.ctrl.button2 as i32 != 0
                || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                gs.exitdemo = true;
                break;
            } else {
                if bioskey(1) != 0 {
                    dofkeys(gs, cps, pas);
                }
                if gs.exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}

unsafe fn gameover(gs: &mut GlobalState, cps: &mut CpanelState, pas: &mut PcrlibAState) {
    let mut i: i32 = 0;
    expwin(11, 4, gs, pas);
    print(b"\n GAME OVER\n     \0" as *const u8 as *const i8, gs);
    UpdateScreen(gs);
    WaitEndSound(gs, pas);
    i = 0;
    while i < 120 {
        WaitVBL(pas);
        i += 1;
    }
    gs.gamestate = statetype::inscores;
    _checkhighscore(gs, pas);
    level = 0;
    i = 0;
    while i < 500 {
        WaitVBL(pas);
        gs.ctrl = ControlPlayer(1, gs);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            break;
        }
        if bioskey(1) != 0 {
            dofkeys(gs, cps, pas);
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

    _setupgame(&mut gs, &mut cps, &mut pas);

    unsafe {
        expwin(33, 13, &mut gs, &mut pas);
        print(
            b"  Softdisk Publishing presents\n\n\0" as *const u8 as *const i8,
            &mut gs,
        );
        print(
            b"          The Catacomb\n\n\0" as *const u8 as *const i8,
            &mut gs,
        );
        print(
            b"        By John Carmack\n\n\0" as *const u8 as *const i8,
            &mut gs,
        );
        print(
            b"       Copyright 1990-93\n\0" as *const u8 as *const i8,
            &mut gs,
        );
        print(
            b"      Softdisk Publishing\0" as *const u8 as *const i8,
            &mut gs,
        );
        print(b"\n\n\0" as *const u8 as *const i8, &mut gs);
        print(b"\n\n\0" as *const u8 as *const i8, &mut gs);
        print(
            b"         Press a key:\0" as *const u8 as *const i8,
            &mut gs,
        );
        get(&mut gs, &mut pas);

        clearkeys();

        gs.screencenter.x = 11;
        gs.screencenter.y = 11;

        gs.exitdemo = false;
        level = 0;

        // go until quit () is called
        loop {
            dodemo(&mut gs, &mut cps, &mut pas);
            playsetup(&mut gs, &mut cps);
            gs.indemo = demoenum::notdemo;
            gs.gamestate = statetype::ingame;
            playloop(&mut gs, &mut cps, &mut pas);
            if gs.indemo == demoenum::notdemo {
                gs.exitdemo = false;
                if level > numlevels {
                    doendpage(&mut gs, &mut cps, &mut pas); // finished all levels
                }
                gameover(&mut gs, &mut cps, &mut pas);
            }
        }
    }
}
