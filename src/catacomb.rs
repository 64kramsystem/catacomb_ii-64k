use std::convert::TryInto;

use libc::O_RDONLY;

use crate::{
    active_obj::activeobj,
    cat_play::{givebolt, givenuke, givepotion, playloop},
    catasm::{cgarefresh, drawchartile, egarefresh},
    class_type::classtype::{self, *},
    control_struct::ControlStruct,
    cpanel::controlpanel,
    demo_enum::demoenum,
    dir_type::dirtype::{self, *},
    exit_type::exittype::*,
    extra_constants::{
        blankfloor, leftoff, maxpics, numlevels, solidwall, tile2s, topoff, NUM_DEMOS, O_BINARY,
    },
    extra_types::boolean,
    global_state::GlobalState,
    gr_type::grtype::{self, *},
    indemo,
    obj_def_type::objdeftype,
    obj_type::objtype,
    objects::initobjects,
    pcrlib_c::{_checkhighscore, _setupgame, _showhighscores, centerwindow, drawwindow, expwin},
    sdl_scan_codes::*,
    state_type::statetype,
};
extern "C" {
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: u64) -> i64;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: i32) -> i32;
    fn toupper(_: i32) -> i32;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn RLEExpand(source: *mut i8, dest: *mut i8, origlen: i64);
    fn bioskey(_: i32) -> i32;
    fn _quit(_: *mut i8);
    static mut score: i32;
    fn printhighscore();
    fn printbody();
    fn printshotpower();
    fn printscore();
    static mut level: i16;
    fn bar(xl: i32, yl: i32, xh: i32, yh: i32, ch_0: i32);
    fn printint(val: i32);
    fn printchartile(str_0: *const i8);
    fn print(str_0: *const i8);
    fn get() -> i32;
    fn drawpic(x: i32, y: i32, picnum: i32);
    fn drawchar(x: i32, y: i32, charnum: i32);
    fn installgrfile(filename: *mut i8, inmem: *mut libc::c_void);
    fn WaitVBL();
    fn UpdateScreen();
    static mut leftedge: i32;
    static mut sy: i32;
    static mut sx: i32;
    static mut grmode: grtype;
    fn _Verify(filename: *mut i8) -> i64;
    fn clearkeys();
    fn rnd(_: u16) -> i32;
    fn rndt() -> i32;
    fn bloadin(filename: *mut i8) -> *mut libc::c_void;
    fn LoadFile(filename: *mut i8, buffer: *mut i8) -> u64;
    fn LoadDemo(demonum: i32);
    fn ControlPlayer(player_0: i32) -> ControlStruct;
    static mut keydown: [boolean; 512];
    fn WaitEndSound();
    fn PlaySound(sound: i32);
    static mut str: [i8; 80];
    static mut ch: i8;
}

#[inline]
unsafe extern "C" fn itoa(mut value: i32, mut str_0: *mut i8, mut base: i32) -> *mut i8 {
    if base == 16 {
        sprintf(str_0, b"%X\0" as *const u8 as *const i8, value);
    } else {
        sprintf(str_0, b"%d\0" as *const u8 as *const i8, value);
    }
    return str_0;
}

#[no_mangle]
pub static mut background: [[i32; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
pub static mut originx: i32 = 0;
#[no_mangle]
pub static mut originy: i32 = 0;

#[no_mangle]
pub static mut saveitems: [i16; 6] = [0; 6];
#[no_mangle]
pub static mut shotpower: i32 = 0;
#[no_mangle]
pub static mut boltsleft: i32 = 0;
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
pub static mut altnum: i32 = 0;
#[no_mangle]
pub static mut numobj: i32 = 0;
#[no_mangle]
pub static mut objecton: i32 = 0;
#[no_mangle]
pub static mut playdone: boolean = 0;
#[no_mangle]
pub static mut leveldone: boolean = 0;
#[no_mangle]
pub static mut tempb: boolean = 0;
#[no_mangle]
pub static mut tempp: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut chkx: i32 = 0;
#[no_mangle]
pub static mut chky: i32 = 0;
#[no_mangle]
pub static mut chkspot: i32 = 0;
#[no_mangle]
pub static mut frameon: u16 = 0;
#[no_mangle]
pub static mut grmem: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut VGAPAL: i32 = 0;
#[no_mangle]
pub static mut exitdemo: bool = false;
#[no_mangle]
pub static mut resetgame: boolean = 0;
#[no_mangle]
pub static mut gamestate: statetype = statetype::ingame;
#[no_mangle]
pub static mut ctrl: ControlStruct = ControlStruct {
    dir: north,
    button1: 0,
    button2: 0,
};
#[no_mangle]
pub static mut pics: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut picsexact: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut EGADATASTART: u32 = 0;
#[no_mangle]
pub static mut savescore: i32 = 0;
#[no_mangle]
pub static mut GODMODE: boolean = false as boolean;

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

pub unsafe fn refresh(global_state: &mut GlobalState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut basex: i32 = 0;
    let mut basey: i32 = 0;
    let mut underwin: [[u16; 16]; 5] = [[0; 16]; 5];
    basex = originx + 4;
    basey = originy + 17;
    if indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                underwin[y as usize][x as usize] =
                    global_state.view[(y + basey) as usize][(x + basex) as usize] as u16;
                global_state.view[(y + basey) as usize][(x + basex) as usize] =
                    demowin[y as usize][x as usize] as i32;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
    if grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(global_state);
    } else {
        egarefresh(global_state);
    }
    if indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                global_state.view[(y + basey) as usize][(x + basex) as usize] =
                    underwin[y as usize][x as usize] as i32;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
}

unsafe fn simplerefresh(global_state: &mut GlobalState) {
    WaitVBL();
    if grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(global_state);
    } else {
        egarefresh(global_state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn loadgrfiles() {
    if !picsexact.is_null() {
        free(picsexact as *mut libc::c_void);
    }
    if grmode as u32 == CGAgr as i32 as u32 {
        pics = bloadin(b"CGACHARS.CA2\0" as *const u8 as *const i8 as *mut i8) as *mut i8;
        picsexact = pics;
        installgrfile(
            b"CGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
        );
    } else {
        pics = bloadin(b"EGACHARS.CA2\0" as *const u8 as *const i8 as *mut i8) as *mut i8;
        picsexact = pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const i8 as *mut i8,
            0 as *mut libc::c_void,
        );
    };
}

pub unsafe fn clearold(oldtiles: &mut [i32; 576]) {
    memset(
        oldtiles as *mut [i32; 576] as *mut libc::c_void,
        0xff as i32,
        ::std::mem::size_of::<[i32; 576]>() as u64,
    );
}

pub unsafe fn restore(global_state: &mut GlobalState) {
    clearold(&mut global_state.oldtiles);
    simplerefresh(global_state);
}
#[no_mangle]
pub unsafe extern "C" fn wantmore() -> boolean {
    sx = 2;
    sy = 20;
    print(b"(space for more/esc)\0" as *const u8 as *const i8);
    sx = 12;
    sy = 21;
    ch = get() as i8;
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
    objdef: &[objdeftype],
) {
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut size: i32 = 0;
    let mut tilenum: i32 = 0;
    size = objdef[c as usize].size as i32;
    tilenum = (objdef[c as usize].firstchar as u32).wrapping_add(
        ((size * size) as u32).wrapping_mul(
            (dir as u32 & objdef[c as usize].dirmask as u32)
                .wrapping_mul(objdef[c as usize].stages as u32)
                .wrapping_add(stage as u32),
        ),
    ) as i32;
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

unsafe fn help(objdef: &[objdeftype], screencenterx: &i32, screencentery: &i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"  C A T A C O M B   \n\0" as *const u8 as *const i8);
    print(b"   - - - - - - -    \n\0" as *const u8 as *const i8);
    print(b" by John Carmack    \n\0" as *const u8 as *const i8);
    print(b"                    \n\0" as *const u8 as *const i8);
    print(b"\n\0" as *const u8 as *const i8);
    print(b"f1 = help           \n\0" as *const u8 as *const i8);
    print(b"f2 = control panel  \n\0" as *const u8 as *const i8);
    print(b"f3 = game reset     \n\0" as *const u8 as *const i8);
    print(b"f4 = save game      \n\0" as *const u8 as *const i8);
    print(b"f5 = load saved game\n\0" as *const u8 as *const i8);
    print(b"f9 = pause          \n\0" as *const u8 as *const i8);
    print(b"f10 / ESC = quit    \n\0" as *const u8 as *const i8);
    print(b"\n\0" as *const u8 as *const i8);
    print(b"hit fire at the demo\n\0" as *const u8 as *const i8);
    print(b"to begin playing.   \n\0" as *const u8 as *const i8);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"\nKeyboard controls:  \n\n\0" as *const u8 as *const i8);
    print(b"move    : arrows    \n\0" as *const u8 as *const i8);
    print(b"button1 : ctrl      \n\0" as *const u8 as *const i8);
    print(b"button2 : alt       \n\0" as *const u8 as *const i8);
    print(b"\nTo switch to mouse \n\0" as *const u8 as *const i8);
    print(b"or joystick control,\n\0" as *const u8 as *const i8);
    print(b"hit f2             \n\0" as *const u8 as *const i8);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"Button 1 / ctrl key:\n\0" as *const u8 as *const i8);
    print(b"Builds shot power.  \n\0" as *const u8 as *const i8);
    print(b"If the shot power   \n\0" as *const u8 as *const i8);
    print(b"meter is full when  \n\0" as *const u8 as *const i8);
    print(b"the button is       \n\0" as *const u8 as *const i8);
    print(b"released, a super   \n\0" as *const u8 as *const i8);
    print(b"shot will be        \n\0" as *const u8 as *const i8);
    print(b"launched.           \n\0" as *const u8 as *const i8);
    print(b"\n\0" as *const u8 as *const i8);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2, objdef);
    charpic(19, 15, shot, east, 1, objdef);
    charpic(17, 14, shot, east, 0, objdef);
    charpic(15, 15, shot, east, 1, objdef);
    charpic(8, 14, bigshot, east, 0, objdef);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"Button 2 / alt key:\n\0" as *const u8 as *const i8);
    print(b"Allows you to move  \n\0" as *const u8 as *const i8);
    print(b"without changing the\n\0" as *const u8 as *const i8);
    print(b"direction you are   \n\0" as *const u8 as *const i8);
    print(b"facing.  Good for   \n\0" as *const u8 as *const i8);
    print(b"searching walls and \n\0" as *const u8 as *const i8);
    print(b"fighting retreats.  \n\0" as *const u8 as *const i8);
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
    charpic(6, 13, player, south, 2, objdef);
    sx = 6;
    sy = 15;
    print(b"\x1D\x1D\x1E\x1E\x1F\x1F\0" as *const u8 as *const i8);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"\"P\" or \"space\" will \n\0" as *const u8 as *const i8);
    print(b"take a healing      \n\0" as *const u8 as *const i8);
    print(b"potion if you have  \n\0" as *const u8 as *const i8);
    print(b"one.  This restores \n\0" as *const u8 as *const i8);
    print(b"the body meter to   \n\0" as *const u8 as *const i8);
    print(b"full strength.  Keep\n\0" as *const u8 as *const i8);
    print(b"a sharp eye on the  \n\0" as *const u8 as *const i8);
    print(b"meter, because when \n\0" as *const u8 as *const i8);
    print(b"it runs out, you are\n\0" as *const u8 as *const i8);
    print(b"dead!               \n\n\0" as *const u8 as *const i8);
    print(b"\"B\" will cast a bolt\n\0" as *const u8 as *const i8);
    print(b"spell if you have   \n\0" as *const u8 as *const i8);
    print(b"any.  You can mow   \n\0" as *const u8 as *const i8);
    print(b"down a lot of       \n\0" as *const u8 as *const i8);
    print(b"monsters with a bit \n\0" as *const u8 as *const i8);
    print(b"of skill.           \n\0" as *const u8 as *const i8);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20, screencenterx, screencentery);
    print(b"\"N\" or \"enter\" will \n\0" as *const u8 as *const i8);
    print(b"cast a nuke spell.  \n\0" as *const u8 as *const i8);
    print(b"This usually wipes  \n\0" as *const u8 as *const i8);
    print(b"out all the monsters\n\0" as *const u8 as *const i8);
    print(b"near you.  Consider \n\0" as *const u8 as *const i8);
    print(b"it a panic button   \n\0" as *const u8 as *const i8);
    print(b"when you are being  \n\0" as *const u8 as *const i8);
    print(b"mobbed by monsters! \n\n\0" as *const u8 as *const i8);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const i8);
    printchartile(b"POTIONS:       \x80\xA2\x80\n\0" as *const u8 as *const i8);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const i8);
    printchartile(b"SCROLLS:       \x80\xA3\x80\n\0" as *const u8 as *const i8);
    printchartile(b" (BOLTS/NUKES) \x80\x80\x80\n\0" as *const u8 as *const i8);
    printchartile(b"TREASURE:      \x80\xA7\x80\n\0" as *const u8 as *const i8);
    printchartile(b" (POINTS)      \x80\x80\x80\n\0" as *const u8 as *const i8);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const i8);
    wantmore();
}

unsafe fn reset(global_state: &mut GlobalState) {
    centerwindow(
        18,
        1,
        &global_state.screencenterx,
        &global_state.screencentery,
    );
    print(b"reset game (y/n)?\0" as *const u8 as *const i8);
    ch = get() as i8;
    if ch as i32 == 'y' as i32 {
        global_state.gamexit = killed;
        playdone = true as boolean;
    }
}

pub unsafe fn loadlevel(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    let mut tokens: [classtype; 26] = [
        player, teleporter, goblin, skeleton, ogre, gargoyle, dragon, turbogre, guns, gune,
        secretgate, nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing,
        nothing, nothing, nothing, nothing, nothing, nothing, nothing,
    ];
    let mut filename: [i8; 64] = [0; 64];
    let mut st: [i8; 64] = [0; 64];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut btile: u8 = 0;
    let mut sm: [i8; 4096] = [0; 4096];
    let mut rle: [i8; 4096] = [0; 4096];
    strcpy(filename.as_mut_ptr(), b"LEVEL\0" as *const u8 as *const i8);
    itoa(level as i32, st.as_mut_ptr(), 10);
    strcat(filename.as_mut_ptr(), st.as_mut_ptr());
    strcat(filename.as_mut_ptr(), b".CA2\0" as *const u8 as *const i8);
    LoadFile(filename.as_mut_ptr(), rle.as_mut_ptr());
    RLEExpand(&mut *rle.as_mut_ptr().offset(4), sm.as_mut_ptr(), 4096);
    numobj = 0;
    o[0].x = 13;
    o[0].y = 13;
    o[0].stage = 0;
    o[0].delay = 0;
    o[0].dir = east as i32 as u16;
    o[0].oldx = 0;
    o[0].oldy = 0;
    o[0].oldtile = -(1) as i16;
    yy = 0;
    while yy < 64 {
        xx = 0;
        while xx < 64 {
            btile = sm[(yy * 64 + xx) as usize] as u8;
            if (btile as i32) < 230 {
                background[(yy + 11) as usize][(xx + 11) as usize] = btile as i32;
            } else {
                background[(yy + 11) as usize][(xx + 11) as usize] = 128;
                if tokens[(btile as i32 - 230) as usize] as u32 == player as i32 as u32 {
                    o[0].x = (xx + 11) as u8;
                    o[0].y = (yy + 11) as u8;
                } else {
                    numobj += 1;
                    o[numobj as usize].active = false as boolean;
                    o[numobj as usize].class = tokens[(btile as i32 - 230) as usize] as u16;
                    o[numobj as usize].x = (xx + 11) as u8;
                    o[numobj as usize].y = (yy + 11) as u8;
                    o[numobj as usize].stage = 0;
                    o[numobj as usize].delay = 0;
                    // Ugly defensive typecast.
                    o[numobj as usize].dir =
                        TryInto::<dirtype>::try_into(rndt() / 64).unwrap() as u16;
                    o[numobj as usize].hp =
                        global_state.objdef[o[numobj as usize].class as usize].hitpoints as i8;
                    o[numobj as usize].oldx = o[numobj as usize].x;
                    o[numobj as usize].oldy = o[numobj as usize].y;
                    o[numobj as usize].oldtile = -(1) as i16;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    originx = o[0].x as i32 - 11;
    originy = o[0].y as i32 - 11;
    shotpower = 0;
    y = 11 - 1;
    while y < 65 + 11 {
        x = 11 - 1;
        while x < 64 + 11 {
            global_state.view[y as usize][x as usize] = background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    sx = 33;
    sy = 1;
    printint(level as i32);
    print(b" \0" as *const u8 as *const i8);
    restore(global_state);
    i = 0;
    while i < 6 {
        saveitems[i as usize] = global_state.items[i as usize];
        i += 1;
    }
    savescore = score;
    saveo[0] = o[0];
}

unsafe fn drawside(items: &mut [i16]) {
    let mut i: i32 = 0;
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
    print(b"  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n\0" as *const u8 as *const i8);
    print(b" shot power\n\n\n    body\n\n\n\0" as *const u8 as *const i8);
    printhighscore();
    printbody();
    printshotpower();
    printscore();
    sx = 33;
    sy = 1;
    printint(level as i32);
    drawpic(25 * 8, 17 * 8, 13);
    i = 1;
    while i <= items[1] as i32 && i < 11 {
        drawchar(26 + i, 7, 31);
        i += 1;
    }
    i = 1;
    while i <= items[2] as i32 && i < 11 {
        drawchar(26 + i, 8, 29);
        i += 1;
    }
    i = 1;
    while i <= items[3] as i32 && i < 11 {
        drawchar(26 + i, 9, 30);
        i += 1;
    }
    i = 1;
    while i <= items[5] as i32 && i < 11 {
        drawchar(26 + i, 10, 30);
        i += 1;
    }
}

unsafe fn playsetup(items: &mut [i16]) {
    let mut i: i32 = 0;
    shotpower = 0;
    bar(0, 0, 23, 23, 0);
    if level as i32 == 0 {
        i = 1;
        while i < 6 {
            items[i as usize] = 0;
            i += 1;
        }
        score = 0;
        level = 1;
        o[0].active = true as boolean;
        o[0].class = player as i32 as u16;
        o[0].hp = 13;
        o[0].dir = west as i32 as u16;
        o[0].stage = 0;
        o[0].delay = 0;
        drawside(items);
        givenuke(items);
        givenuke(items);
        givebolt(items);
        givebolt(items);
        givebolt(items);
        givepotion(items);
        givepotion(items);
        givepotion(items);
    } else {
        drawside(items);
    };
}

pub unsafe fn repaintscreen(global_state: &mut GlobalState) {
    match gamestate {
        statetype::intitle => {
            drawpic(0, 0, 14);
        }
        statetype::ingame => {
            restore(global_state);
            drawside(&mut global_state.items);
            printscore();
            sx = 33;
            sy = 1;
            printint(level as i32);
        }
        statetype::inscores => {
            restore(global_state);
            drawside(&mut global_state.items);
            printscore();
            sx = 33;
            sy = 1;
            printint(level as i32);
            indemo = demoenum::demoplay;
        }
    };
}

pub unsafe fn dofkeys(global_state: &mut GlobalState) {
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
            help(
                &global_state.objdef,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
        }
        59 => {
            clearkeys();
            controlpanel(global_state);
        }
        60 => {
            clearkeys();
            expwin(
                18,
                1,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
            print(b"RESET GAME (Y/N)?\0" as *const u8 as *const i8);
            ch = toupper(get()) as i8;
            if ch as i32 == 'Y' as i32 {
                resetgame = true as boolean;
            }
        }
        61 => {
            clearkeys();
            expwin(
                22,
                4,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
            if indemo != demoenum::notdemo {
                print(b"Can't save game here!\0" as *const u8 as *const i8);
                get();
            } else {
                print(b"Save as game #(1-9):\0" as *const u8 as *const i8);
                ch = toupper(get()) as i8;
                drawchar(sx, sy, ch as i32);
                if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                    strcpy(str.as_mut_ptr(), b"GAME0.CA2\0" as *const u8 as *const i8);
                    str[4] = ch;
                    if _Verify(str.as_mut_ptr()) != 0 {
                        print(b"\nGame exists,\noverwrite (Y/N)?\0" as *const u8 as *const i8);
                        ch = get() as i8;
                        if ch as i32 != 'Y' as i32 && ch as i32 != 'y' as i32 {
                            current_block_72 = 919954187481050311;
                        } else {
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const i8);
                            sy -= 1;
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const i8);
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
                                0o1 as i32 | 0 | 0o100 as i32 | 0o1000 as i32,
                                0o400 as i32 | 0o200 as i32,
                            );
                            if handle == -(1) {
                                return;
                            }
                            write(
                                handle,
                                &mut saveitems as *mut [i16; 6] as *const libc::c_void,
                                ::std::mem::size_of::<[i16; 6]>() as u64,
                            );
                            write(
                                handle,
                                &mut savescore as *mut i32 as *const libc::c_void,
                                ::std::mem::size_of::<i32>() as u64,
                            );
                            write(
                                handle,
                                &mut level as *mut i16 as *const libc::c_void,
                                ::std::mem::size_of::<i16>() as u64,
                            );
                            write(
                                handle,
                                &mut *saveo.as_mut_ptr().offset(0) as *mut activeobj
                                    as *const libc::c_void,
                                ::std::mem::size_of::<activeobj>() as u64,
                            );
                            close(handle);
                            print(b"\nGame saved.  Hit F5\n\0" as *const u8 as *const i8);
                            print(b"when you wish to\n\0" as *const u8 as *const i8);
                            print(b"restart the game.\0" as *const u8 as *const i8);
                            get();
                        }
                    }
                }
            }
        }
        62 => {
            clearkeys();
            expwin(
                22,
                4,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
            print(b"Load game #(1-9):\0" as *const u8 as *const i8);
            ch = toupper(get()) as i8;
            drawchar(sx, sy, ch as i32);
            if !((ch as i32) < '1' as i32 || ch as i32 > '9' as i32) {
                strcpy(str.as_mut_ptr(), b"GAME0.CA2\0" as *const u8 as *const i8);
                str[4] = ch;
                // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
                // project.
                handle = open(
                    str.as_mut_ptr(),
                    O_RDONLY | O_BINARY,
                    0o200 as i32 | 0o400 as i32,
                );
                if handle == -(1) {
                    print(b"\nGame not found.\0" as *const u8 as *const i8);
                    get();
                } else {
                    read(
                        handle,
                        &mut global_state.items as *mut _ as *mut libc::c_void,
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
                        &mut *o.as_mut_ptr().offset(0) as *mut activeobj as *mut libc::c_void,
                        ::std::mem::size_of::<activeobj>() as u64,
                    );
                    close(handle);
                    exitdemo = true;
                    if indemo != demoenum::notdemo {
                        playdone = true as boolean;
                    }
                    drawside(&mut global_state.items);
                    leveldone = true as boolean;
                }
            }
        }
        66 => {
            clearkeys();
            expwin(
                7,
                1,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
            print(b"PAUSED\0" as *const u8 as *const i8);
            get();
        }
        67 => {
            clearkeys();
            expwin(
                12,
                1,
                &global_state.screencenterx,
                &global_state.screencentery,
            );
            print(b"QUIT (Y/N)?\0" as *const u8 as *const i8);
            ch = toupper(get()) as i8;
            if ch as i32 == 'Y' as i32 {
                _quit(b"\0" as *const u8 as *const i8 as *mut i8);
            }
        }
        _ => return,
    }
    clearold(&mut global_state.oldtiles);
    clearkeys();
    repaintscreen(global_state);
}

unsafe fn dotitlepage(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    drawpic(0, 0, 14);
    UpdateScreen();
    gamestate = statetype::intitle;
    i = 0;
    while i < 300 {
        WaitVBL();
        indemo = demoenum::notdemo;
        ctrl = ControlPlayer(1);
        if ctrl.button1 as i32 != 0
            || ctrl.button2 as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            level = 0;
            exitdemo = true;
            break;
        } else {
            indemo = demoenum::demoplay;
            if bioskey(1) != 0 {
                dofkeys(global_state);
                UpdateScreen();
            }
            if exitdemo {
                break;
            }
            i += 1;
        }
    }
    gamestate = statetype::ingame;
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
    print(b"Congratulation! \n\0" as *const u8 as *const i8);
    print(b"One as skilled  \n\0" as *const u8 as *const i8);
    print(b"as yourself     \n\0" as *const u8 as *const i8);
    print(b"deserves the    \n\0" as *const u8 as *const i8);
    print(b"10,000,000 gold \n\0" as *const u8 as *const i8);
    print(b"you pulled out  \n\0" as *const u8 as *const i8);
    print(b"of the palace! \0" as *const u8 as *const i8);
    clearkeys();
    get();
    drawwindow(0, 0, 17, 9);
    print(b"Let us know what\n\0" as *const u8 as *const i8);
    print(b"you enjoyed     \n\0" as *const u8 as *const i8);
    print(b"about this game,\n\0" as *const u8 as *const i8);
    print(b"so we can give  \n\0" as *const u8 as *const i8);
    print(b"you more of it. \n\0" as *const u8 as *const i8);
    print(b"Thank you for   \n\0" as *const u8 as *const i8);
    print(b"playing!\0" as *const u8 as *const i8);
    get();
}

unsafe fn dodemo(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    while !exitdemo {
        dotitlepage(global_state);
        if exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1) + 1;
        LoadDemo(i);
        level = 0;
        playsetup(&mut global_state.items);
        playloop(global_state);
        if exitdemo {
            break;
        }
        level = 0;
        gamestate = statetype::inscores;
        indemo = demoenum::demoplay;
        _showhighscores(&global_state.screencenterx, &global_state.screencentery);
        UpdateScreen();
        i = 0;
        while i < 500 {
            WaitVBL();
            indemo = demoenum::notdemo;
            ctrl = ControlPlayer(1);
            if ctrl.button1 as i32 != 0
                || ctrl.button2 as i32 != 0
                || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                exitdemo = true;
                break;
            } else {
                if bioskey(1) != 0 {
                    dofkeys(global_state);
                }
                if exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}

unsafe fn gameover(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    expwin(
        11,
        4,
        &global_state.screencenterx,
        &global_state.screencentery,
    );
    print(b"\n GAME OVER\n     \0" as *const u8 as *const i8);
    UpdateScreen();
    WaitEndSound();
    i = 0;
    while i < 120 {
        WaitVBL();
        i += 1;
    }
    gamestate = statetype::inscores;
    _checkhighscore(&global_state.screencenterx, &global_state.screencentery);
    level = 0;
    i = 0;
    while i < 500 {
        WaitVBL();
        ctrl = ControlPlayer(1);
        if ctrl.button1 as i32 != 0
            || ctrl.button2 as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            break;
        }
        if bioskey(1) != 0 {
            dofkeys(global_state);
        }
        if exitdemo as i32 != 0 || indemo == demoenum::demoplay {
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
    let mut global_state = GlobalState::new(
        [0; 2048],
        [0; 6],
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
        0,
        [[0; 86]; 87],
        19,
        11,
        quited,
        [0; 576],
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

    initobjects(&mut global_state.objdef);

    global_state.priority.fill(99);

    global_state.priority[blankfloor] = 0;
    for i in global_state.objdef[teleporter as usize].firstchar
        ..=global_state.objdef[teleporter as usize].firstchar + 20
    {
        global_state.priority[i as usize] = 0;
    }
    for clvar in (dead2 as usize)..=(dead5 as usize) {
        for i in global_state.objdef[clvar].firstchar
            ..=(global_state.objdef[clvar].firstchar
                + global_state.objdef[clvar].size as u16 * global_state.objdef[clvar].size as u16)
        {
            global_state.priority[i as usize] = 0; /*deadthing*/
        }
    }
    for i in 152..=161 {
        global_state.priority[i] = 2; /*shots*/
    }
    for i in global_state.objdef[bigshot as usize].firstchar
        ..=(global_state.objdef[bigshot as usize].firstchar + 31)
    {
        global_state.priority[i as usize] = 2; /*bigshot*/
    }
    for i in 0..=(tile2s - 1) {
        if global_state.priority[i] == 99 {
            global_state.priority[i] = 3; /*most 1*1 tiles are walls, etc*/
        }
    }
    global_state.priority[167] = 1; // chest
    for i in tile2s..=maxpics {
        if global_state.priority[i] as i32 == 99 {
            global_state.priority[i] = 4; /*most bigger tiles are monsters*/
        }
    }
    for i in global_state.objdef[player as usize].firstchar
        ..=(global_state.objdef[player as usize].firstchar + 63)
    {
        global_state.priority[i as usize] = 5; /*player*/
    }

    unsafe {
        global_state.side = 0;

        for x in 0..=85 {
            for y in 0..=(topoff - 1) {
                global_state.view[x][y] = solidwall;
                global_state.view[x][(85 - y)] = solidwall;
                background[x][y] = solidwall;
                background[x][(85 - y)] = solidwall;
            }
            global_state.view[86][x] = solidwall;
        }
        for y in 11..=74 {
            for x in 0..=(leftoff - 1) {
                global_state.view[x][y] = solidwall;
                global_state.view[(85 - x)][y] = solidwall;
                background[x][y] = solidwall;
                background[(85 - x)][y] = solidwall;
            }
        }

        //   puts ("CATACOMB II is executing");

        //  _dontplay = 1;	// no sounds for debugging and profiling

        _setupgame();

        expwin(
            33,
            13,
            &global_state.screencenterx,
            &global_state.screencentery,
        );
        print(b"  Softdisk Publishing presents\n\n\0" as *const u8 as *const i8);
        print(b"          The Catacomb\n\n\0" as *const u8 as *const i8);
        print(b"        By John Carmack\n\n\0" as *const u8 as *const i8);
        print(b"       Copyright 1990-93\n\0" as *const u8 as *const i8);
        print(b"      Softdisk Publishing\0" as *const u8 as *const i8);
        print(b"\n\n\0" as *const u8 as *const i8);
        print(b"\n\n\0" as *const u8 as *const i8);
        print(b"         Press a key:\0" as *const u8 as *const i8);
        get();

        clearkeys();

        global_state.screencentery = 11;
        global_state.screencenterx = 11;

        exitdemo = false;
        level = 0;

        // go until quit () is called
        loop {
            dodemo(&mut global_state);
            playsetup(&mut global_state.items);
            indemo = demoenum::notdemo;
            gamestate = statetype::ingame;
            playloop(&mut global_state);
            if indemo == demoenum::notdemo {
                exitdemo = false;
                if level > numlevels {
                    doendpage(); // finished all levels
                }
                gameover(&mut global_state);
            }
        }
    }
}
