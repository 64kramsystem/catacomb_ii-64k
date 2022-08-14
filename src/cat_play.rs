use std::convert::TryInto;

use ::libc;

use crate::{
    active_obj::activeobj,
    catacomb::{clearold, dofkeys, loadlevel, refresh, restore},
    catasm::{doall, drawobj, eraseobj},
    class_type::classtype::*,
    control_struct::ControlStruct,
    demo_enum::demoenum,
    dir_type::dirtype::{self, *},
    exit_type::exittype::*,
    extra_types::boolean,
    global_state::GlobalState,
    indemo,
    obj_def_type::objdeftype,
    obj_type::objtype,
    pcrlib_a::{drawchar, WaitEndSound},
    pcrlib_c::{centerwindow, get, print, printint, printlong, UpdateScreen, _inputint},
    scores::scores,
    sdl_scan_codes::*,
    tag_type::tagtype::*,
};
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn abs(_: i32) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static mut ctrl: ControlStruct;
    static mut GODMODE: boolean;
    fn bioskey(_: i32) -> i32;
    static mut highscores: [scores; 5];
    static mut level: i16;
    static mut score: i32;
    fn WaitVBL();
    static mut sy: i32;
    static mut sx: i32;
    fn clearkeys();
    fn rndt() -> i32;
    fn initrndt(randomize: boolean);
    fn RecordDemo();
    fn SaveDemo(demonum: i32);
    fn ControlPlayer(player_0: i32) -> ControlStruct;
    static mut keydown: [boolean; 512];
    fn PlaySound(sound: i32);
    static mut ch: i8;
}

const altmeters: [[i8; 14]; 14] = [
    [
        127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        23, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [23, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 127, 0],
    [23, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 25, 0],
];

const meters: [[i8; 14]; 14] = [
    [
        127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [
        26, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 127, 0,
    ],
    [26, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 127, 0],
    [26, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 28, 0],
];

const opposite: [dirtype; 9] = [
    south, west, north, east, southwest, northwest, northeast, southeast, nodir,
];

unsafe fn newobject(gs: &mut GlobalState) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    i = 1;
    loop {
        if !(i <= gs.numobj) {
            current_block = 10680521327981672866;
            break;
        }
        if gs.o[i as usize].class as i32 == nothing as i32 {
            current_block = 15327184191433388822;
            break;
        }
        i += 1;
    }
    match current_block {
        10680521327981672866 => {
            if gs.numobj < 200 {
                gs.numobj += 1;
            }
            i = gs.numobj;
        }
        _ => {}
    }
    gs.o[i as usize].oldtile = -(1) as i16;
    gs.o[i as usize].oldx = 0;
    gs.o[i as usize].oldy = 0;
    return i;
}

pub unsafe fn printscore(gs: &mut GlobalState) {
    sx = 31;
    sy = 3;
    printlong(score as i64, gs);
}

pub unsafe fn printhighscore(gs: &mut GlobalState) {
    sx = 31;
    sy = 5;
    printlong(highscores[1].score as i64, gs);
}

pub unsafe fn printshotpower(gs: &mut GlobalState) {
    sx = 25;
    sy = 13;
    if gs.shotpower == 13 {
        print(altmeters[13].as_ptr(), gs);
    } else {
        print(meters[gs.shotpower as usize].as_ptr(), gs);
    };
}

pub unsafe fn printbody(gs: &mut GlobalState) {
    sx = 25;
    sy = 16;
    if gs.o[0].hp as i32 > 6 {
        print(meters[gs.o[0].hp as usize].as_ptr(), gs);
    } else {
        print(altmeters[gs.o[0].hp as usize].as_ptr(), gs);
    };
}

unsafe fn levelcleared(gs: &mut GlobalState) {
    let mut warp: [i8; 3] = [0; 3];
    let mut value: i32 = 0;
    gs.leveldone = true;
    warp[0] = (gs.background[(gs.altobj.y as i32 + 2) as usize][gs.altobj.x as usize] as i8 as i32
        - 161) as i8;
    if (warp[0] as i32) < '0' as i32 || warp[0] as i32 > '9' as i32 {
        warp[0] = '0' as i32 as i8;
    }
    warp[1] = (gs.background[(gs.altobj.y as i32 + 2) as usize][(gs.altobj.x as i32 + 1) as usize]
        as i8 as i32
        - 161) as i8;
    if (warp[1] as i32) < '0' as i32 || warp[1] as i32 > '9' as i32 {
        warp[2] = ' ' as i32 as i8;
    }
    value = atoi(warp.as_mut_ptr());
    if value > 0 {
        level = value as i16;
    } else {
        level += 1;
    }
    if level as i32 > 30 {
        gs.playdone = true;
        gs.gamexit = victorious;
    }
}

unsafe fn givekey(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    i = gs.items[1] as i32 + 1;
    gs.items[1] = i as i16;
    if i < 11 {
        drawchar(26 + i, 7, 31, gs);
    }
}

pub unsafe fn givepotion(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    i = gs.items[2] as i32 + 1;
    gs.items[2] = i as i16;
    if i < 11 {
        drawchar(26 + i, 8, 29, gs);
    }
}

pub unsafe fn givebolt(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    i = gs.items[3] as i32 + 1;
    gs.items[3] = i as i16;
    if i < 11 {
        drawchar(26 + i, 9, 30, gs);
    }
}

pub unsafe fn givenuke(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    i = gs.items[5] as i32 + 1;
    gs.items[5] = i as i16;
    if i < 11 {
        drawchar(26 + i, 10, 30, gs);
    }
}

unsafe fn takekey(gs: &mut GlobalState) -> boolean {
    let mut i: i32 = 0;
    if gs.items[1] as i32 > 0 {
        i = gs.items[1] as i32 - 1;
        gs.items[1] = i as i16;
        if i < 10 {
            drawchar(27 + i, 7, 32, gs);
        }
        PlaySound(11);
        return true as boolean;
    } else {
        PlaySound(14);
        return false as boolean;
    };
}

unsafe fn takepotion(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    if gs.items[2] as i32 > 0 {
        i = gs.items[2] as i32 - 1;
        gs.items[2] = i as i16;
        if i < 11 {
            drawchar(27 + i, 8, 32, gs);
        }
        PlaySound(12);
        gs.o[0].hp = 13;
        gs.obj.hp = 13;
        printbody(gs);
    } else {
        PlaySound(14);
    };
}

unsafe fn castbolt(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    if gs.items[3] as i32 > 0 {
        i = gs.items[3] as i32 - 1;
        gs.items[3] = i as i16;
        if i < 11 {
            drawchar(27 + i, 9, 32, gs);
        }
        gs.boltsleft = 8;
        PlaySound(13);
    } else {
        PlaySound(14);
    };
}

unsafe fn castnuke(gs: &mut GlobalState) {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut n: i32 = 0;
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
    if gs.items[5] as i32 == 0 {
        PlaySound(14);
        return;
    }
    i = gs.items[5] as i32 - 1;
    gs.items[5] = i as i16;
    if i < 11 {
        drawchar(27 + i, 10, 32, gs);
    }
    base.delay = 0;
    base.stage = 0;
    base.active = true as boolean;
    base.x = gs.obj.x;
    base.y = gs.obj.y;
    base.oldx = base.x;
    base.oldy = base.y;
    base.oldtile = -(1) as i16;
    base.class = bigshot as i32 as u16;
    x = -(1);
    while x <= 1 {
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].x = (gs.o[n as usize].x as i32 + x * 2) as u8;
        gs.o[n as usize].dir = north as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].x = (gs.o[n as usize].x as i32 + x * 2) as u8;
        gs.o[n as usize].dir = south as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].y = (gs.o[n as usize].y as i32 + x * 2) as u8;
        gs.o[n as usize].dir = east as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].y = (gs.o[n as usize].y as i32 + x * 2) as u8;
        gs.o[n as usize].dir = west as i32 as u16;
        x += 1;
    }
    PlaySound(13);
    gs.obj.stage = 2;
    gs.obj.delay = 4;
}

unsafe fn playshoot(gs: &mut GlobalState) {
    let mut new: i32 = 0;
    gs.obj.stage = 2;
    gs.obj.delay = 4;
    PlaySound(5);
    new = newobject(gs);
    gs.o[new as usize].class = shot as i32 as u16;
    gs.side ^= 1;
    gs.o[new as usize].delay = 0;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].active = true as boolean;
    gs.o[new as usize].dir = gs.obj.dir;
    match gs.o[new as usize].dir as i32 {
        0 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + gs.side) as u8;
            gs.o[new as usize].y = gs.obj.y;
        }
        1 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + 1) as u8;
            gs.o[new as usize].y = (gs.obj.y as i32 + gs.side) as u8;
        }
        2 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + gs.side) as u8;
            gs.o[new as usize].y = (gs.obj.y as i32 + 1) as u8;
        }
        3 => {
            gs.o[new as usize].x = gs.obj.x;
            gs.o[new as usize].y = (gs.obj.y as i32 + gs.side) as u8;
        }
        _ => {}
    };
}

unsafe fn playbigshoot(gs: &mut GlobalState) {
    let mut new: i32 = 0;
    gs.obj.stage = 2;
    if gs.boltsleft == 0 {
        gs.obj.delay = 4;
    }
    PlaySound(4);
    new = newobject(gs);
    gs.o[new as usize].delay = 0;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].active = true as boolean;
    gs.o[new as usize].dir = gs.obj.dir;
    gs.o[new as usize].x = gs.obj.x;
    gs.o[new as usize].y = gs.obj.y;
    gs.o[new as usize].class = bigshot as i32 as u16;
}

unsafe fn givescroll(gs: &mut GlobalState) {
    if rndt() < 128 {
        givebolt(gs);
    } else {
        givenuke(gs);
    };
}

unsafe fn opendoor(gs: &mut GlobalState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    PlaySound(11);
    x = gs.chkx;
    y = gs.chky;
    if gs.chkspot == 165 {
        loop {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            y -= 1;
            if !(gs.view[y as usize][x as usize] == 165) {
                break;
            }
        }
        y = gs.chky + 1;
        while gs.view[y as usize][x as usize] == 165 {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            y += 1;
        }
    } else {
        loop {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            x -= 1;
            if !(gs.view[y as usize][x as usize] == 166) {
                break;
            }
        }
        x = gs.chkx + 1;
        while gs.view[y as usize][x as usize] == 166 {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            x += 1;
        }
    };
}

unsafe fn tagobject(gs: &mut GlobalState) {
    let mut i: i32 = gs.altobj.hp as i32;
    if GODMODE as i32 != 0 && gs.altobj.class as i32 == player as i32 {
        return;
    }
    gs.altobj.hp = (gs.altobj.hp as i32 - gs.obj.damage as i32) as i8;
    if i <= gs.obj.damage as i32 {
        if gs.altobj.class as i32 == player as i32 {
            gs.o[0].hp = 0;
            gs.altobj.hp = gs.o[0].hp;
            printbody(gs);
            PlaySound(10);
            gs.playdone = true;
            gs.gamexit = killed;
        } else {
            score = score + gs.altobj.points as i32;
            printscore(gs);
            PlaySound(9);
        }
        gs.o[gs.altnum as usize].class = (dead1 as i32 - 1 + gs.altobj.size as i32) as u16;
        gs.o[gs.altnum as usize].delay = 2;
        gs.o[gs.altnum as usize].stage = 0;
    } else {
        if gs.o[gs.altnum as usize].class as i32 == guns as i32
            || gs.o[gs.altnum as usize].class as i32 == gune as i32
        {
            return;
        }
        gs.o[gs.altnum as usize].hp = gs.altobj.hp;
        gs.o[gs.altnum as usize].stage = 3;
        if gs.altnum == 0 {
            gs.o[0].delay = 2;
            printbody(gs);
            PlaySound(8);
        } else {
            gs.o[gs.altnum as usize].delay = 4;
            PlaySound(7);
        }
    };
}

unsafe fn intomonster(gs: &mut GlobalState) -> boolean {
    let mut gotit: boolean = 0;
    gs.altnum = 0;
    gotit = false as boolean;
    loop {
        gs.altobj.update_from_active(gs.o[gs.altnum as usize]);
        if gs.altobj.class as i32 > nothing as i32 && gs.altnum != gs.objecton {
            memcpy(
                &mut gs.altobj.think as *mut u8 as *mut libc::c_void,
                &mut *gs.objdef.as_mut_ptr().offset(gs.altobj.class as isize) as *mut objdeftype
                    as *const libc::c_void,
                ::std::mem::size_of::<objdeftype>() as u64,
            );
            if gs.chkx >= gs.altobj.x as i32
                && (gs.chkx - gs.altobj.x as i32) < gs.altobj.size as i32
                && gs.chky >= gs.altobj.y as i32
                && (gs.chky - gs.altobj.y as i32) < gs.altobj.size as i32
            {
                if gs.altobj.solid != 0 {
                    gotit = true as boolean;
                } else if gs.objecton == 0
                    && (gs.altobj.class as i32 == teleporter as i32
                        || gs.altobj.class as i32 == secretgate as i32)
                {
                    levelcleared(gs);
                }
            }
        }
        if gotit == 0 {
            gs.altnum += 1;
        }
        if gotit as i32 != 0 || gs.altnum > gs.numobj {
            break;
        }
    }
    if gotit == 0 {
        return true as boolean;
    }
    match gs.obj.contact as i32 {
        0 => return false as boolean,
        1 | 3 => {
            if gs.altnum == 0 {
                tagobject(gs);
                gs.obj.stage = 2;
                gs.obj.delay = 20;
            } else if gs.altobj.class as i32 == shot as i32 {
                return true as boolean;
            }
            return false as boolean;
        }
        2 => {
            if gs.altnum > 0 {
                tagobject(gs);
            }
            return false as boolean;
        }
        4 => {
            tagobject(gs);
            return true as boolean;
        }
        _ => {}
    }
    return false as boolean;
}

unsafe fn walkthrough(gs: &mut GlobalState) -> boolean {
    let mut new: i32 = 0;
    if gs.chkspot == 128 {
        return true as boolean;
    }
    if gs.chkspot >= 256 && gs.chkspot <= 256 + 67 * 4 + 35 * 9 + 19 * 16 + 19 * 25 {
        return intomonster(gs);
    }
    if gs.chkspot >= 129 && gs.chkspot <= 135 {
        if gs.obj.contact as i32 == pshot as i32
            || gs.obj.contact as i32 == nukeshot as i32
            || gs.obj.contact as i32 == mshot as i32
        {
            new = newobject(gs);
            gs.o[new as usize].active = true as boolean;
            gs.o[new as usize].x = gs.chkx as u8;
            gs.o[new as usize].y = gs.chky as u8;
            gs.o[new as usize].stage = 0;
            gs.o[new as usize].delay = 2;
            gs.o[new as usize].class = wallhit as i32 as u16;
            PlaySound(6);
        }
        return false as boolean;
    }
    if gs.chkspot >= 136 && gs.chkspot <= 145 {
        if gs.obj.contact as i32 == pshot as i32 || gs.obj.contact as i32 == nukeshot as i32 {
            PlaySound(6);
            if gs.chkspot < 143 {
                gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            } else {
                gs.background[gs.chky as usize][gs.chkx as usize] = gs.chkspot + 19;
            }
            new = newobject(gs);
            gs.o[new as usize].active = true as boolean;
            gs.o[new as usize].x = gs.chkx as u8;
            gs.o[new as usize].y = gs.chky as u8;
            gs.o[new as usize].stage = 0;
            gs.o[new as usize].delay = 2;
            gs.o[new as usize].class = dead1 as i32 as u16;
            if gs.obj.contact as i32 == pshot as i32 {
                return false as boolean;
            } else {
                return true as boolean;
            }
        } else {
            return false as boolean;
        }
    }
    if gs.chkspot == 162 {
        if gs.obj.class as i32 == player as i32 {
            givepotion(gs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if gs.chkspot == 163 {
        if gs.obj.class as i32 == player as i32 {
            givescroll(gs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if gs.chkspot == 164 {
        if gs.obj.class as i32 == player as i32 {
            givekey(gs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if gs.chkspot == 165 || gs.chkspot == 166 {
        if gs.obj.class as i32 == player as i32 {
            if takekey(gs) != 0 {
                opendoor(gs);
                return true as boolean;
            }
        }
        return false as boolean;
    }
    if gs.chkspot == 167 {
        if gs.obj.class as i32 == player as i32 {
            score += 500;
            printscore(gs);
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(3);
        }
        return true as boolean;
    }
    if gs.chkspot >= 29 && gs.chkspot <= 31 {
        return true as boolean;
    }
    return false as boolean;
}

unsafe fn walk(gs: &mut GlobalState) -> boolean {
    let mut i: i32 = 0;
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut deltay: i32 = 0;
    let mut deltax: i32 = 0;
    let mut try_0: boolean = 0;
    match gs.obj.dir as i32 {
        0 => {
            newx = gs.obj.x as i32;
            newy = gs.obj.y as i32 - 1;
            gs.chkx = newx;
            gs.chky = newy;
            deltax = 1;
            deltay = 0;
        }
        1 => {
            newx = gs.obj.x as i32 + 1;
            newy = gs.obj.y as i32;
            gs.chkx = gs.obj.x as i32 + gs.obj.size as i32;
            gs.chky = newy;
            deltax = 0;
            deltay = 1;
        }
        2 => {
            newx = gs.obj.x as i32;
            newy = gs.obj.y as i32 + 1;
            gs.chkx = newx;
            gs.chky = gs.obj.y as i32 + gs.obj.size as i32;
            deltax = 1;
            deltay = 0;
        }
        3 => {
            newx = gs.obj.x as i32 - 1;
            newy = gs.obj.y as i32;
            gs.chkx = newx;
            gs.chky = newy;
            deltax = 0;
            deltay = 1;
        }
        _ => return false as boolean,
    }
    i = 1;
    while i <= gs.obj.size as i32 {
        gs.chkspot = gs.view[gs.chky as usize][gs.chkx as usize];
        if gs.chkspot != 128 {
            try_0 = walkthrough(gs);
            if gs.leveldone {
                return true as boolean;
            }
            if gs.obj.stage as i32 == 2 {
                return true as boolean;
            }
            if try_0 == 0 {
                return false as boolean;
            }
        }
        gs.chkx = gs.chkx + deltax;
        gs.chky = gs.chky + deltay;
        i += 1;
    }
    gs.obj.x = newx as u8;
    gs.obj.y = newy as u8;
    gs.obj.stage = (gs.obj.stage as i32 ^ 1) as u8;
    return true as boolean;
}

unsafe fn playercmdthink(gs: &mut GlobalState) {
    let mut olddir: dirtype = north;
    let mut c: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    c = ControlPlayer(1);
    gs.obj.stage = (gs.obj.stage as i32 & 1) as u8;
    if c.button1 as i32 != 0
        && c.button2 as i32 != 0
        && keydown[SDL_SCANCODE_Q as usize] as i32 != 0
    {
        givepotion(gs);
        givescroll(gs);
        givekey(gs);
    }
    if (c.dir as u32) < nodir as i32 as u32 && gs.frameon as i32 % 2 != 0 {
        if c.button2 != 0 {
            olddir = gs.obj.dir.try_into().unwrap();
        }
        if c.dir as u32 > west as i32 as u32 {
            if gs.frameon as i32 / 2 % 2 != 0 {
                match c.dir as u32 {
                    4 => {
                        gs.obj.dir = east as i32 as u16;
                        walk(gs);
                        c.dir = north;
                    }
                    5 => {
                        gs.obj.dir = south as i32 as u16;
                        walk(gs);
                        c.dir = east;
                    }
                    6 => {
                        gs.obj.dir = west as i32 as u16;
                        walk(gs);
                        c.dir = south;
                    }
                    7 => {
                        gs.obj.dir = north as i32 as u16;
                        walk(gs);
                        c.dir = west;
                    }
                    _ => {}
                }
            } else {
                match c.dir as u32 {
                    4 => {
                        gs.obj.dir = north as i32 as u16;
                        walk(gs);
                        c.dir = east;
                    }
                    5 => {
                        gs.obj.dir = east as i32 as u16;
                        walk(gs);
                        c.dir = south;
                    }
                    6 => {
                        gs.obj.dir = south as i32 as u16;
                        walk(gs);
                        c.dir = west;
                    }
                    7 => {
                        gs.obj.dir = west as i32 as u16;
                        walk(gs);
                        c.dir = north;
                    }
                    _ => {}
                }
            }
        }
        gs.obj.dir = c.dir as u16;
        if walk(gs) == 0 {
            PlaySound(1);
        }
        if c.button2 != 0 {
            gs.obj.dir = olddir as u16;
        }
    } else if c.button2 == 0 {
        match c.dir as u32 {
            7 | 0 => {
                gs.obj.dir = north as i32 as u16;
            }
            4 | 1 => {
                gs.obj.dir = east as i32 as u16;
            }
            5 | 2 => {
                gs.obj.dir = south as i32 as u16;
            }
            6 | 3 => {
                gs.obj.dir = west as i32 as u16;
            }
            8 | _ => {}
        }
    }
    gs.origin.x = gs.obj.x as i32 - 11;
    gs.origin.y = gs.obj.y as i32 - 11;
    if gs.boltsleft > 0 {
        if gs.frameon as i32 % 3 == 0 {
            playbigshoot(gs);
            gs.boltsleft -= 1;
        }
    } else if c.button1 != 0 {
        if gs.shotpower == 0 {
            gs.shotpower = 1;
        } else if gs.shotpower < 13 && gs.frameon as i32 % 2 != 0 {
            gs.shotpower += 1;
        }
        printshotpower(gs);
    } else if gs.shotpower > 0 {
        if gs.shotpower == 13 {
            playbigshoot(gs);
        } else {
            playshoot(gs);
        }
        gs.shotpower = 0;
        printshotpower(gs);
    }
    if indemo == demoenum::notdemo {
        if keydown[SDL_SCANCODE_P as usize] as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            if (gs.obj.hp as i32) < 13 {
                takepotion(gs);
                keydown[SDL_SCANCODE_Q as usize] = false as boolean;
                keydown[SDL_SCANCODE_SPACE as usize] = false as boolean;
            }
        } else if keydown[SDL_SCANCODE_B as usize] != 0 {
            castbolt(gs);
            keydown[SDL_SCANCODE_B as usize] = false as boolean;
        } else if keydown[SDL_SCANCODE_N as usize] as i32 != 0
            || keydown[SDL_SCANCODE_RETURN as usize] as i32 != 0
        {
            castnuke(gs);
            keydown[SDL_SCANCODE_N as usize] = false as boolean;
            keydown[SDL_SCANCODE_RETURN as usize] = false as boolean;
        }
    }
    dofkeys(gs);
    if gs.resetgame {
        gs.resetgame = false;
        gs.playdone = true;
        return;
    }
    match indemo {
        demoenum::notdemo => {
            if keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                centerwindow(16, 2, gs);
                print(
                    b"warp to which\nlevel (1-99)?\0" as *const u8 as *const i8,
                    gs,
                );
                clearkeys();
                level = _inputint(gs) as i16;
                if (level as i32) < 1 {
                    level = 1;
                }
                if level as i32 > 30 {
                    level = 30;
                }
                restore(gs);
                gs.leveldone = true;
            }
            if keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && keydown[SDL_SCANCODE_TAB as usize] as i32 != 0
            {
                if GODMODE != 0 {
                    centerwindow(13, 1, gs);
                    print(b"God Mode Off\0" as *const u8 as *const i8, gs);
                    GODMODE = false as boolean;
                } else {
                    centerwindow(12, 1, gs);
                    print(b"God Mode On\0" as *const u8 as *const i8, gs);
                    GODMODE = true as boolean;
                }
                UpdateScreen(gs);
                clearkeys();
                while bioskey(0) == 0 {
                    WaitVBL();
                }
                restore(gs);
                clearkeys();
            }
        }
        demoenum::demoplay => {
            indemo = demoenum::notdemo;
            ctrl = ControlPlayer(1);
            if ctrl.button1 as i32 != 0
                || ctrl.button2 as i32 != 0
                || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                indemo = demoenum::demoplay;
                gs.exitdemo = true;
                gs.leveldone = true;
                level = 0;
                return;
            }
            indemo = demoenum::demoplay;
        }
        _ => {}
    };
}

unsafe fn chasethink(mut diagonal: boolean, gs: &mut GlobalState) {
    let mut deltax: i32 = 0;
    let mut deltay: i32 = 0;
    let mut d: [dirtype; 3] = [north; 3];
    let mut tdir: i32 = 0;
    let mut olddir: i32 = 0;
    let mut turnaround: i32 = 0;
    gs.obj.stage = (gs.obj.stage as i32 & 1) as u8;
    olddir = gs.obj.dir as i32;
    turnaround = opposite[olddir as usize] as i32;
    deltax = gs.o[0].x as i32 - gs.obj.x as i32;
    deltay = gs.o[0].y as i32 - gs.obj.y as i32;
    d[1] = nodir;
    d[2] = nodir;
    if deltax > 0 {
        d[1] = east;
    }
    if deltax < 0 {
        d[1] = west;
    }
    if deltay > 0 {
        d[2] = south;
    }
    if deltay < 0 {
        d[2] = north;
    }
    if abs(deltay) > abs(deltax) {
        tdir = d[1] as i32;
        d[1] = d[2];
        d[2] = tdir.try_into().unwrap();
    }
    if d[1] as u32 == turnaround as u32 {
        d[1] = nodir;
    }
    if d[2] as u32 == turnaround as u32 {
        d[2] = nodir;
    }
    if diagonal != 0 {
        if d[1] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[1] as u16;
            if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
        if d[2] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[2] as u16;
            if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
    } else {
        if d[2] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[2] as u16;
            if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
        if d[1] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[1] as u16;
            if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
    }
    gs.obj.dir = olddir as u16;
    if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
        return;
    }
    if rndt() > 128 {
        tdir = north as i32;
        while tdir <= west as i32 {
            if tdir != turnaround {
                gs.obj.dir = tdir as u16;
                if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = west as i32;
        while tdir >= north as i32 {
            if tdir != turnaround {
                gs.obj.dir = tdir as u16;
                if walk(gs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    gs.obj.dir = turnaround as u16;
    walk(gs);
}

unsafe fn gargthink(gs: &mut GlobalState) {
    let mut n: i32 = 0;
    if rndt() > 220 {
        gs.obj.stage = 2;
        gs.obj.delay = 6;
        PlaySound(5);
        n = newobject(gs);
        gs.o[n as usize].class = rock as i32 as u16;
        gs.o[n as usize].delay = 0;
        gs.o[n as usize].stage = 0;
        gs.o[n as usize].active = true as boolean;
        gs.o[n as usize].dir = gs.obj.dir;
        match gs.obj.dir as i32 {
            0 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = gs.obj.y;
            }
            1 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 3) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            2 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 3) as u8;
            }
            3 => {
                gs.o[n as usize].x = gs.obj.x;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, gs);
    };
}

unsafe fn dragonthink(gs: &mut GlobalState) {
    let mut n: i32 = 0;
    if rndt() > 220 {
        gs.obj.stage = 2;
        gs.obj.delay = 6;
        PlaySound(5);
        n = newobject(gs);
        gs.o[n as usize].class = bigshot as i32 as u16;
        gs.o[n as usize].delay = 0;
        gs.o[n as usize].stage = 0;
        gs.o[n as usize].active = true as boolean;
        gs.o[n as usize].dir = gs.obj.dir;
        match gs.o[n as usize].dir as i32 {
            0 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = gs.obj.y;
            }
            1 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 3) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            2 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 3) as u8;
            }
            3 => {
                gs.o[n as usize].x = gs.obj.x;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, gs);
    };
}

unsafe fn gunthink(mut dir: i32, gs: &mut GlobalState) {
    let mut n: i32 = 0;
    PlaySound(5);
    gs.obj.stage = 0;
    n = newobject(gs);
    gs.o[n as usize].class = bigshot as i32 as u16;
    gs.o[n as usize].delay = 0;
    gs.o[n as usize].stage = 0;
    gs.o[n as usize].active = true as boolean;
    gs.o[n as usize].dir = dir as u16;
    gs.o[n as usize].x = gs.obj.x;
    gs.o[n as usize].y = gs.obj.y;
}

unsafe fn shooterthink(gs: &mut GlobalState) {
    if (gs.obj.x as i32) < gs.origin.x - 1
        || (gs.obj.y as i32) < gs.origin.y - 1
        || gs.obj.x as i32 > gs.origin.x + 22
        || gs.obj.y as i32 > gs.origin.y + 22
        || walk(gs) == 0
        || gs.obj.stage as i32 == 2
    {
        gs.obj.class = nothing as i32 as u16;
    }
}

unsafe fn idlethink(gs: &mut GlobalState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.stage = 0;
    }
}

unsafe fn fadethink(gs: &mut GlobalState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.class = nothing as i32 as u16;
    }
}

unsafe fn killnear(mut chkx_0: i32, mut chky_0: i32, gs: &mut GlobalState) {
    let mut spot: i32 = 0;
    let mut new: i32 = 0;
    spot = gs.background[chky_0 as usize][chkx_0 as usize];
    if spot < 136 || spot > 145 {
        return;
    }
    PlaySound(6);
    if spot < 143 {
        gs.background[chky_0 as usize][chkx_0 as usize] = 128;
    } else {
        gs.background[chky_0 as usize][chkx_0 as usize] = spot + 19;
    }
    new = newobject(gs);
    gs.o[new as usize].active = true as boolean;
    gs.o[new as usize].x = chkx_0 as u8;
    gs.o[new as usize].y = chky_0 as u8;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].delay = 2;
    gs.o[new as usize].class = dead1 as i32 as u16;
}

unsafe fn explodethink(gs: &mut GlobalState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    if gs.obj.stage as i32 == 1 {
        killnear(gs.obj.x as i32 - 1, gs.obj.y as i32, gs);
        killnear(gs.obj.x as i32, gs.obj.y as i32 - 1, gs);
        killnear(gs.obj.x as i32 + 1, gs.obj.y as i32, gs);
        killnear(gs.obj.x as i32, gs.obj.y as i32 + 1, gs);
    }
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.class = nothing as i32 as u16;
    }
}

unsafe fn think(gs: &mut GlobalState) {
    if gs.obj.delay as i32 > 0 {
        gs.obj.delay = (gs.obj.delay).wrapping_sub(1);
    } else if rndt() < gs.obj.speed as i32 {
        match gs.obj.think as i32 {
            0 => {
                playercmdthink(gs);
            }
            3 => {
                chasethink(false as boolean, gs);
            }
            4 => {
                chasethink(true as boolean, gs);
            }
            1 => {
                gargthink(gs);
            }
            2 => {
                dragonthink(gs);
            }
            5 => {
                shooterthink(gs);
            }
            6 => {
                idlethink(gs);
            }
            7 => {
                fadethink(gs);
            }
            8 => {
                explodethink(gs);
            }
            9 => {
                gunthink(west as i32, gs);
            }
            10 => {
                gunthink(north as i32, gs);
            }
            _ => {}
        }
    }
}

pub unsafe fn doactive(gs: &mut GlobalState) {
    if gs.obj.class as i32 != dead1 as i32
        && ((gs.obj.x as i32) < gs.origin.x - 10
            || gs.obj.x as i32 > gs.origin.x + 34
            || (gs.obj.y as i32) < gs.origin.y - 10
            || gs.obj.y as i32 > gs.origin.y + 34)
    {
        gs.o[gs.objecton as usize].active = false as boolean;
    } else {
        think(gs);
        eraseobj(gs);
        if gs.playdone {
            return;
        }
        if gs.obj.class as i32 > nothing as i32 {
            drawobj(gs);
        }
        memcpy(
            &mut *gs.o.as_mut_ptr().offset(gs.objecton as isize) as *mut activeobj
                as *mut libc::c_void,
            &mut gs.obj as *mut objtype as *const libc::c_void,
            ::std::mem::size_of::<activeobj>() as u64,
        );
    };
}

pub unsafe fn doinactive(gs: &mut GlobalState) {
    if gs.obj.x as i32 + gs.obj.size as i32 >= gs.origin.x
        && (gs.obj.x as i32) < gs.origin.x + 24
        && gs.obj.y as i32 + gs.obj.size as i32 >= gs.origin.y
        && (gs.obj.y as i32) < gs.origin.y + 24
    {
        gs.obj.active = true as boolean;
        gs.obj.dir = north as i32 as u16;
        gs.o[gs.objecton as usize] = gs.obj.into();
    }
}

pub unsafe fn playloop(gs: &mut GlobalState) {
    gs.screencenter.x = 11;
    loop {
        if indemo == demoenum::notdemo {
            centerwindow(11, 2, gs);
            print(b" Entering\nlevel \0" as *const u8 as *const i8, gs);
            printint(level as i32, gs);
            print(b"...\0" as *const u8 as *const i8, gs);
            PlaySound(17);
            WaitEndSound(gs);
        }
        clearold(&mut gs.oldtiles);
        loadlevel(gs);
        gs.leveldone = false;
        if keydown[SDL_SCANCODE_F7 as usize] as i32 != 0
            && keydown[SDL_SCANCODE_D as usize] as i32 != 0
        {
            clearold(&mut gs.oldtiles);
            refresh(gs);
            refresh(gs);
            clearkeys();
            centerwindow(12, 1, gs);
            print(b"RECORD DEMO\0" as *const u8 as *const i8, gs);
            loop {
                ch = get(gs) as i8;
                if !(ch as i32 != 13) {
                    break;
                }
            }
            RecordDemo();
            clearold(&mut gs.oldtiles);
            clearkeys();
        }
        gs.playdone = false;
        gs.frameon = 0;
        gs.boltsleft = 0;
        gs.shotpower = 0;
        initrndt(false as boolean);
        printshotpower(gs);
        doall(gs);
        if indemo == demoenum::recording {
            clearkeys();
            centerwindow(15, 1, gs);
            print(b"SAVE AS DEMO#:\0" as *const u8 as *const i8, gs);
            loop {
                ch = get(gs) as i8;
                if !((ch as i32) < '0' as i32 || ch as i32 > '9' as i32) {
                    break;
                }
            }
            SaveDemo(ch as i32 - '0' as i32);
            clearold(&mut gs.oldtiles);
            refresh(gs);
            refresh(gs);
        }
        if indemo != demoenum::notdemo {
            gs.playdone = true;
        }
        if gs.playdone {
            break;
        }
    }
}
