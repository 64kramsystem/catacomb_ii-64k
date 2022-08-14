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
    static mut resetgame: boolean;
    static mut ctrl: ControlStruct;
    static mut exitdemo: boolean;
    static mut altobj: objtype;
    static mut altnum: i32;
    static mut chkspot: i32;
    static mut chkx: i32;
    static mut chky: i32;
    static mut numobj: i32;
    static mut obj: objtype;
    static mut o: [activeobj; 201];
    static mut objecton: i32;
    static mut leveldone: boolean;
    static mut frameon: u16;
    static mut boltsleft: i32;
    static mut shotpower: i32;
    static mut playdone: boolean;
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

#[no_mangle]
pub unsafe extern "C" fn newobject() -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    i = 1;
    loop {
        if !(i <= numobj) {
            current_block = 10680521327981672866;
            break;
        }
        if o[i as usize].class as i32 == nothing as i32 {
            current_block = 15327184191433388822;
            break;
        }
        i += 1;
    }
    match current_block {
        10680521327981672866 => {
            if numobj < 200 {
                numobj += 1;
            }
            i = numobj;
        }
        _ => {}
    }
    o[i as usize].oldtile = -(1) as i16;
    o[i as usize].oldx = 0;
    o[i as usize].oldy = 0;
    return i;
}

pub unsafe fn printscore(global_state: &mut GlobalState) {
    sx = 31;
    sy = 3;
    printlong(score as i64, global_state);
}

pub unsafe fn printhighscore(global_state: &mut GlobalState) {
    sx = 31;
    sy = 5;
    printlong(highscores[1].score as i64, global_state);
}

pub unsafe fn printshotpower(global_state: &mut GlobalState) {
    sx = 25;
    sy = 13;
    if shotpower == 13 {
        print(altmeters[13].as_ptr(), global_state);
    } else {
        print(meters[shotpower as usize].as_ptr(), global_state);
    };
}

pub unsafe fn printbody(global_state: &mut GlobalState) {
    sx = 25;
    sy = 16;
    if o[0].hp as i32 > 6 {
        print(meters[o[0].hp as usize].as_ptr(), global_state);
    } else {
        print(altmeters[o[0].hp as usize].as_ptr(), global_state);
    };
}

unsafe fn levelcleared(global_state: &mut GlobalState) {
    let mut warp: [i8; 3] = [0; 3];
    let mut value: i32 = 0;
    leveldone = true as boolean;
    warp[0] = (global_state.background[(altobj.y as i32 + 2) as usize][altobj.x as usize] as i8
        as i32
        - 161) as i8;
    if (warp[0] as i32) < '0' as i32 || warp[0] as i32 > '9' as i32 {
        warp[0] = '0' as i32 as i8;
    }
    warp[1] = (global_state.background[(altobj.y as i32 + 2) as usize]
        [(altobj.x as i32 + 1) as usize] as i8 as i32
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
        playdone = true as boolean;
        global_state.gamexit = victorious;
    }
}

unsafe fn givekey(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    i = global_state.items[1] as i32 + 1;
    global_state.items[1] = i as i16;
    if i < 11 {
        drawchar(26 + i, 7, 31, global_state);
    }
}

pub unsafe fn givepotion(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    i = global_state.items[2] as i32 + 1;
    global_state.items[2] = i as i16;
    if i < 11 {
        drawchar(26 + i, 8, 29, global_state);
    }
}

pub unsafe fn givebolt(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    i = global_state.items[3] as i32 + 1;
    global_state.items[3] = i as i16;
    if i < 11 {
        drawchar(26 + i, 9, 30, global_state);
    }
}

pub unsafe fn givenuke(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    i = global_state.items[5] as i32 + 1;
    global_state.items[5] = i as i16;
    if i < 11 {
        drawchar(26 + i, 10, 30, global_state);
    }
}

unsafe fn takekey(global_state: &mut GlobalState) -> boolean {
    let mut i: i32 = 0;
    if global_state.items[1] as i32 > 0 {
        i = global_state.items[1] as i32 - 1;
        global_state.items[1] = i as i16;
        if i < 10 {
            drawchar(27 + i, 7, 32, global_state);
        }
        PlaySound(11);
        return true as boolean;
    } else {
        PlaySound(14);
        return false as boolean;
    };
}

unsafe fn takepotion(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    if global_state.items[2] as i32 > 0 {
        i = global_state.items[2] as i32 - 1;
        global_state.items[2] = i as i16;
        if i < 11 {
            drawchar(27 + i, 8, 32, global_state);
        }
        PlaySound(12);
        o[0].hp = 13;
        obj.hp = 13;
        printbody(global_state);
    } else {
        PlaySound(14);
    };
}

unsafe fn castbolt(global_state: &mut GlobalState) {
    let mut i: i32 = 0;
    if global_state.items[3] as i32 > 0 {
        i = global_state.items[3] as i32 - 1;
        global_state.items[3] = i as i16;
        if i < 11 {
            drawchar(27 + i, 9, 32, global_state);
        }
        boltsleft = 8;
        PlaySound(13);
    } else {
        PlaySound(14);
    };
}

unsafe fn castnuke(global_state: &mut GlobalState) {
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
    if global_state.items[5] as i32 == 0 {
        PlaySound(14);
        return;
    }
    i = global_state.items[5] as i32 - 1;
    global_state.items[5] = i as i16;
    if i < 11 {
        drawchar(27 + i, 10, 32, global_state);
    }
    base.delay = 0;
    base.stage = 0;
    base.active = true as boolean;
    base.x = obj.x;
    base.y = obj.y;
    base.oldx = base.x;
    base.oldy = base.y;
    base.oldtile = -(1) as i16;
    base.class = bigshot as i32 as u16;
    x = -(1);
    while x <= 1 {
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as i32 + x * 2) as u8;
        o[n as usize].dir = north as i32 as u16;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as i32 + x * 2) as u8;
        o[n as usize].dir = south as i32 as u16;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as i32 + x * 2) as u8;
        o[n as usize].dir = east as i32 as u16;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as i32 + x * 2) as u8;
        o[n as usize].dir = west as i32 as u16;
        x += 1;
    }
    PlaySound(13);
    obj.stage = 2;
    obj.delay = 4;
}

unsafe fn playshoot(side: &mut i32) {
    let mut new: i32 = 0;
    obj.stage = 2;
    obj.delay = 4;
    PlaySound(5);
    new = newobject();
    o[new as usize].class = shot as i32 as u16;
    *side = *side ^ 1;
    o[new as usize].delay = 0;
    o[new as usize].stage = 0;
    o[new as usize].active = true as boolean;
    o[new as usize].dir = obj.dir;
    match o[new as usize].dir as i32 {
        0 => {
            o[new as usize].x = (obj.x as i32 + *side) as u8;
            o[new as usize].y = obj.y;
        }
        1 => {
            o[new as usize].x = (obj.x as i32 + 1) as u8;
            o[new as usize].y = (obj.y as i32 + *side) as u8;
        }
        2 => {
            o[new as usize].x = (obj.x as i32 + *side) as u8;
            o[new as usize].y = (obj.y as i32 + 1) as u8;
        }
        3 => {
            o[new as usize].x = obj.x;
            o[new as usize].y = (obj.y as i32 + *side) as u8;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn playbigshoot() {
    let mut new: i32 = 0;
    obj.stage = 2;
    if boltsleft == 0 {
        obj.delay = 4;
    }
    PlaySound(4);
    new = newobject();
    o[new as usize].delay = 0;
    o[new as usize].stage = 0;
    o[new as usize].active = true as boolean;
    o[new as usize].dir = obj.dir;
    o[new as usize].x = obj.x;
    o[new as usize].y = obj.y;
    o[new as usize].class = bigshot as i32 as u16;
}

unsafe fn givescroll(global_state: &mut GlobalState) {
    if rndt() < 128 {
        givebolt(global_state);
    } else {
        givenuke(global_state);
    };
}

unsafe fn opendoor(global_state: &mut GlobalState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    PlaySound(11);
    x = chkx;
    y = chky;
    if chkspot == 165 {
        loop {
            global_state.view[y as usize][x as usize] = 128;
            global_state.background[y as usize][x as usize] = 128;
            y -= 1;
            if !(global_state.view[y as usize][x as usize] == 165) {
                break;
            }
        }
        y = chky + 1;
        while global_state.view[y as usize][x as usize] == 165 {
            global_state.view[y as usize][x as usize] = 128;
            global_state.background[y as usize][x as usize] = 128;
            y += 1;
        }
    } else {
        loop {
            global_state.view[y as usize][x as usize] = 128;
            global_state.background[y as usize][x as usize] = 128;
            x -= 1;
            if !(global_state.view[y as usize][x as usize] == 166) {
                break;
            }
        }
        x = chkx + 1;
        while global_state.view[y as usize][x as usize] == 166 {
            global_state.view[y as usize][x as usize] = 128;
            global_state.background[y as usize][x as usize] = 128;
            x += 1;
        }
    };
}

unsafe fn tagobject(global_state: &mut GlobalState) {
    let mut i: i32 = altobj.hp as i32;
    if GODMODE as i32 != 0 && altobj.class as i32 == player as i32 {
        return;
    }
    altobj.hp = (altobj.hp as i32 - obj.damage as i32) as i8;
    if i <= obj.damage as i32 {
        if altobj.class as i32 == player as i32 {
            o[0].hp = 0;
            altobj.hp = o[0].hp;
            printbody(global_state);
            PlaySound(10);
            playdone = true as boolean;
            global_state.gamexit = killed;
        } else {
            score = score + altobj.points as i32;
            printscore(global_state);
            PlaySound(9);
        }
        o[altnum as usize].class = (dead1 as i32 - 1 + altobj.size as i32) as u16;
        o[altnum as usize].delay = 2;
        o[altnum as usize].stage = 0;
    } else {
        if o[altnum as usize].class as i32 == guns as i32
            || o[altnum as usize].class as i32 == gune as i32
        {
            return;
        }
        o[altnum as usize].hp = altobj.hp;
        o[altnum as usize].stage = 3;
        if altnum == 0 {
            o[0].delay = 2;
            printbody(global_state);
            PlaySound(8);
        } else {
            o[altnum as usize].delay = 4;
            PlaySound(7);
        }
    };
}

unsafe fn intomonster(global_state: &mut GlobalState) -> boolean {
    let mut gotit: boolean = 0;
    altnum = 0;
    gotit = false as boolean;
    loop {
        altobj.update_from_active(o[altnum as usize]);
        if altobj.class as i32 > nothing as i32 && altnum != objecton {
            memcpy(
                &mut altobj.think as *mut u8 as *mut libc::c_void,
                &mut *global_state
                    .objdef
                    .as_mut_ptr()
                    .offset(altobj.class as isize) as *mut objdeftype
                    as *const libc::c_void,
                ::std::mem::size_of::<objdeftype>() as u64,
            );
            if chkx >= altobj.x as i32
                && (chkx - altobj.x as i32) < altobj.size as i32
                && chky >= altobj.y as i32
                && (chky - altobj.y as i32) < altobj.size as i32
            {
                if altobj.solid != 0 {
                    gotit = true as boolean;
                } else if objecton == 0
                    && (altobj.class as i32 == teleporter as i32
                        || altobj.class as i32 == secretgate as i32)
                {
                    levelcleared(global_state);
                }
            }
        }
        if gotit == 0 {
            altnum += 1;
        }
        if gotit as i32 != 0 || altnum > numobj {
            break;
        }
    }
    if gotit == 0 {
        return true as boolean;
    }
    match obj.contact as i32 {
        0 => return false as boolean,
        1 | 3 => {
            if altnum == 0 {
                tagobject(global_state);
                obj.stage = 2;
                obj.delay = 20;
            } else if altobj.class as i32 == shot as i32 {
                return true as boolean;
            }
            return false as boolean;
        }
        2 => {
            if altnum > 0 {
                tagobject(global_state);
            }
            return false as boolean;
        }
        4 => {
            tagobject(global_state);
            return true as boolean;
        }
        _ => {}
    }
    return false as boolean;
}

unsafe fn walkthrough(global_state: &mut GlobalState) -> boolean {
    let mut new: i32 = 0;
    if chkspot == 128 {
        return true as boolean;
    }
    if chkspot >= 256 && chkspot <= 256 + 67 * 4 + 35 * 9 + 19 * 16 + 19 * 25 {
        return intomonster(global_state);
    }
    if chkspot >= 129 && chkspot <= 135 {
        if obj.contact as i32 == pshot as i32
            || obj.contact as i32 == nukeshot as i32
            || obj.contact as i32 == mshot as i32
        {
            new = newobject();
            o[new as usize].active = true as boolean;
            o[new as usize].x = chkx as u8;
            o[new as usize].y = chky as u8;
            o[new as usize].stage = 0;
            o[new as usize].delay = 2;
            o[new as usize].class = wallhit as i32 as u16;
            PlaySound(6);
        }
        return false as boolean;
    }
    if chkspot >= 136 && chkspot <= 145 {
        if obj.contact as i32 == pshot as i32 || obj.contact as i32 == nukeshot as i32 {
            PlaySound(6);
            if chkspot < 143 {
                global_state.background[chky as usize][chkx as usize] = 128;
            } else {
                global_state.background[chky as usize][chkx as usize] = chkspot + 19;
            }
            new = newobject();
            o[new as usize].active = true as boolean;
            o[new as usize].x = chkx as u8;
            o[new as usize].y = chky as u8;
            o[new as usize].stage = 0;
            o[new as usize].delay = 2;
            o[new as usize].class = dead1 as i32 as u16;
            if obj.contact as i32 == pshot as i32 {
                return false as boolean;
            } else {
                return true as boolean;
            }
        } else {
            return false as boolean;
        }
    }
    if chkspot == 162 {
        if obj.class as i32 == player as i32 {
            givepotion(global_state);
            global_state.view[chky as usize][chkx as usize] = 128;
            global_state.background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 163 {
        if obj.class as i32 == player as i32 {
            givescroll(global_state);
            global_state.view[chky as usize][chkx as usize] = 128;
            global_state.background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 164 {
        if obj.class as i32 == player as i32 {
            givekey(global_state);
            global_state.view[chky as usize][chkx as usize] = 128;
            global_state.background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 165 || chkspot == 166 {
        if obj.class as i32 == player as i32 {
            if takekey(global_state) != 0 {
                opendoor(global_state);
                return true as boolean;
            }
        }
        return false as boolean;
    }
    if chkspot == 167 {
        if obj.class as i32 == player as i32 {
            score += 500;
            printscore(global_state);
            global_state.background[chky as usize][chkx as usize] = 128;
            global_state.view[chky as usize][chkx as usize] = 128;
            PlaySound(3);
        }
        return true as boolean;
    }
    if chkspot >= 29 && chkspot <= 31 {
        return true as boolean;
    }
    return false as boolean;
}

unsafe fn walk(global_state: &mut GlobalState) -> boolean {
    let mut i: i32 = 0;
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut deltay: i32 = 0;
    let mut deltax: i32 = 0;
    let mut try_0: boolean = 0;
    match obj.dir as i32 {
        0 => {
            newx = obj.x as i32;
            newy = obj.y as i32 - 1;
            chkx = newx;
            chky = newy;
            deltax = 1;
            deltay = 0;
        }
        1 => {
            newx = obj.x as i32 + 1;
            newy = obj.y as i32;
            chkx = obj.x as i32 + obj.size as i32;
            chky = newy;
            deltax = 0;
            deltay = 1;
        }
        2 => {
            newx = obj.x as i32;
            newy = obj.y as i32 + 1;
            chkx = newx;
            chky = obj.y as i32 + obj.size as i32;
            deltax = 1;
            deltay = 0;
        }
        3 => {
            newx = obj.x as i32 - 1;
            newy = obj.y as i32;
            chkx = newx;
            chky = newy;
            deltax = 0;
            deltay = 1;
        }
        _ => return false as boolean,
    }
    i = 1;
    while i <= obj.size as i32 {
        chkspot = global_state.view[chky as usize][chkx as usize];
        if chkspot != 128 {
            try_0 = walkthrough(global_state);
            if leveldone != 0 {
                return true as boolean;
            }
            if obj.stage as i32 == 2 {
                return true as boolean;
            }
            if try_0 == 0 {
                return false as boolean;
            }
        }
        chkx = chkx + deltax;
        chky = chky + deltay;
        i += 1;
    }
    obj.x = newx as u8;
    obj.y = newy as u8;
    obj.stage = (obj.stage as i32 ^ 1) as u8;
    return true as boolean;
}

unsafe fn playercmdthink(global_state: &mut GlobalState) {
    let mut olddir: dirtype = north;
    let mut c: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    c = ControlPlayer(1);
    obj.stage = (obj.stage as i32 & 1) as u8;
    if c.button1 as i32 != 0
        && c.button2 as i32 != 0
        && keydown[SDL_SCANCODE_Q as usize] as i32 != 0
    {
        givepotion(global_state);
        givescroll(global_state);
        givekey(global_state);
    }
    if (c.dir as u32) < nodir as i32 as u32 && frameon as i32 % 2 != 0 {
        if c.button2 != 0 {
            olddir = obj.dir.try_into().unwrap();
        }
        if c.dir as u32 > west as i32 as u32 {
            if frameon as i32 / 2 % 2 != 0 {
                match c.dir as u32 {
                    4 => {
                        obj.dir = east as i32 as u16;
                        walk(global_state);
                        c.dir = north;
                    }
                    5 => {
                        obj.dir = south as i32 as u16;
                        walk(global_state);
                        c.dir = east;
                    }
                    6 => {
                        obj.dir = west as i32 as u16;
                        walk(global_state);
                        c.dir = south;
                    }
                    7 => {
                        obj.dir = north as i32 as u16;
                        walk(global_state);
                        c.dir = west;
                    }
                    _ => {}
                }
            } else {
                match c.dir as u32 {
                    4 => {
                        obj.dir = north as i32 as u16;
                        walk(global_state);
                        c.dir = east;
                    }
                    5 => {
                        obj.dir = east as i32 as u16;
                        walk(global_state);
                        c.dir = south;
                    }
                    6 => {
                        obj.dir = south as i32 as u16;
                        walk(global_state);
                        c.dir = west;
                    }
                    7 => {
                        obj.dir = west as i32 as u16;
                        walk(global_state);
                        c.dir = north;
                    }
                    _ => {}
                }
            }
        }
        obj.dir = c.dir as u16;
        if walk(global_state) == 0 {
            PlaySound(1);
        }
        if c.button2 != 0 {
            obj.dir = olddir as u16;
        }
    } else if c.button2 == 0 {
        match c.dir as u32 {
            7 | 0 => {
                obj.dir = north as i32 as u16;
            }
            4 | 1 => {
                obj.dir = east as i32 as u16;
            }
            5 | 2 => {
                obj.dir = south as i32 as u16;
            }
            6 | 3 => {
                obj.dir = west as i32 as u16;
            }
            8 | _ => {}
        }
    }
    global_state.origin.x = obj.x as i32 - 11;
    global_state.origin.y = obj.y as i32 - 11;
    if boltsleft > 0 {
        if frameon as i32 % 3 == 0 {
            playbigshoot();
            boltsleft -= 1;
        }
    } else if c.button1 != 0 {
        if shotpower == 0 {
            shotpower = 1;
        } else if shotpower < 13 && frameon as i32 % 2 != 0 {
            shotpower += 1;
        }
        printshotpower(global_state);
    } else if shotpower > 0 {
        if shotpower == 13 {
            playbigshoot();
        } else {
            playshoot(&mut global_state.side);
        }
        shotpower = 0;
        printshotpower(global_state);
    }
    if indemo == demoenum::notdemo {
        if keydown[SDL_SCANCODE_P as usize] as i32 != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            if (obj.hp as i32) < 13 {
                takepotion(global_state);
                keydown[SDL_SCANCODE_Q as usize] = false as boolean;
                keydown[SDL_SCANCODE_SPACE as usize] = false as boolean;
            }
        } else if keydown[SDL_SCANCODE_B as usize] != 0 {
            castbolt(global_state);
            keydown[SDL_SCANCODE_B as usize] = false as boolean;
        } else if keydown[SDL_SCANCODE_N as usize] as i32 != 0
            || keydown[SDL_SCANCODE_RETURN as usize] as i32 != 0
        {
            castnuke(global_state);
            keydown[SDL_SCANCODE_N as usize] = false as boolean;
            keydown[SDL_SCANCODE_RETURN as usize] = false as boolean;
        }
    }
    dofkeys(global_state);
    if resetgame != 0 {
        resetgame = false as boolean;
        playdone = true as boolean;
        return;
    }
    match indemo {
        demoenum::notdemo => {
            if keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                centerwindow(16, 2, global_state);
                print(
                    b"warp to which\nlevel (1-99)?\0" as *const u8 as *const i8,
                    global_state,
                );
                clearkeys();
                level = _inputint(global_state) as i16;
                if (level as i32) < 1 {
                    level = 1;
                }
                if level as i32 > 30 {
                    level = 30;
                }
                restore(global_state);
                leveldone = true as boolean;
            }
            if keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && keydown[SDL_SCANCODE_TAB as usize] as i32 != 0
            {
                if GODMODE != 0 {
                    centerwindow(13, 1, global_state);
                    print(b"God Mode Off\0" as *const u8 as *const i8, global_state);
                    GODMODE = false as boolean;
                } else {
                    centerwindow(12, 1, global_state);
                    print(b"God Mode On\0" as *const u8 as *const i8, global_state);
                    GODMODE = true as boolean;
                }
                UpdateScreen(global_state);
                clearkeys();
                while bioskey(0) == 0 {
                    WaitVBL();
                }
                restore(global_state);
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
                exitdemo = true as boolean;
                leveldone = true as boolean;
                level = 0;
                return;
            }
            indemo = demoenum::demoplay;
        }
        _ => {}
    };
}

unsafe fn chasethink(mut diagonal: boolean, global_state: &mut GlobalState) {
    let mut deltax: i32 = 0;
    let mut deltay: i32 = 0;
    let mut d: [dirtype; 3] = [north; 3];
    let mut tdir: i32 = 0;
    let mut olddir: i32 = 0;
    let mut turnaround: i32 = 0;
    obj.stage = (obj.stage as i32 & 1) as u8;
    olddir = obj.dir as i32;
    turnaround = opposite[olddir as usize] as i32;
    deltax = o[0].x as i32 - obj.x as i32;
    deltay = o[0].y as i32 - obj.y as i32;
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
            obj.dir = d[1] as u16;
            if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                return;
            }
        }
        if d[2] as u32 != nodir as i32 as u32 {
            obj.dir = d[2] as u16;
            if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                return;
            }
        }
    } else {
        if d[2] as u32 != nodir as i32 as u32 {
            obj.dir = d[2] as u16;
            if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                return;
            }
        }
        if d[1] as u32 != nodir as i32 as u32 {
            obj.dir = d[1] as u16;
            if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                return;
            }
        }
    }
    obj.dir = olddir as u16;
    if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
        return;
    }
    if rndt() > 128 {
        tdir = north as i32;
        while tdir <= west as i32 {
            if tdir != turnaround {
                obj.dir = tdir as u16;
                if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = west as i32;
        while tdir >= north as i32 {
            if tdir != turnaround {
                obj.dir = tdir as u16;
                if walk(global_state) as i32 != 0 || obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    obj.dir = turnaround as u16;
    walk(global_state);
}

unsafe fn gargthink(global_state: &mut GlobalState) {
    let mut n: i32 = 0;
    if rndt() > 220 {
        obj.stage = 2;
        obj.delay = 6;
        PlaySound(5);
        n = newobject();
        o[n as usize].class = rock as i32 as u16;
        o[n as usize].delay = 0;
        o[n as usize].stage = 0;
        o[n as usize].active = true as boolean;
        o[n as usize].dir = obj.dir;
        match obj.dir as i32 {
            0 => {
                o[n as usize].x = (obj.x as i32 + 1 + global_state.side) as u8;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as i32 + 3) as u8;
                o[n as usize].y = (obj.y as i32 + 1 + global_state.side) as u8;
            }
            2 => {
                o[n as usize].x = (obj.x as i32 + 1 + global_state.side) as u8;
                o[n as usize].y = (obj.y as i32 + 3) as u8;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as i32 + 1 + global_state.side) as u8;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, global_state);
    };
}

unsafe fn dragonthink(global_state: &mut GlobalState) {
    let mut n: i32 = 0;
    if rndt() > 220 {
        obj.stage = 2;
        obj.delay = 6;
        PlaySound(5);
        n = newobject();
        o[n as usize].class = bigshot as i32 as u16;
        o[n as usize].delay = 0;
        o[n as usize].stage = 0;
        o[n as usize].active = true as boolean;
        o[n as usize].dir = obj.dir;
        match o[n as usize].dir as i32 {
            0 => {
                o[n as usize].x = (obj.x as i32 + 1 + global_state.side) as u8;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as i32 + 3) as u8;
                o[n as usize].y = (obj.y as i32 + 1 + global_state.side) as u8;
            }
            2 => {
                o[n as usize].x = (obj.x as i32 + 1 + global_state.side) as u8;
                o[n as usize].y = (obj.y as i32 + 3) as u8;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as i32 + 1 + global_state.side) as u8;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, global_state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gunthink(mut dir: i32) {
    let mut n: i32 = 0;
    PlaySound(5);
    obj.stage = 0;
    n = newobject();
    o[n as usize].class = bigshot as i32 as u16;
    o[n as usize].delay = 0;
    o[n as usize].stage = 0;
    o[n as usize].active = true as boolean;
    o[n as usize].dir = dir as u16;
    o[n as usize].x = obj.x;
    o[n as usize].y = obj.y;
}

unsafe fn shooterthink(global_state: &mut GlobalState) {
    if (obj.x as i32) < global_state.origin.x - 1
        || (obj.y as i32) < global_state.origin.y - 1
        || obj.x as i32 > global_state.origin.x + 22
        || obj.y as i32 > global_state.origin.y + 22
        || walk(global_state) == 0
        || obj.stage as i32 == 2
    {
        obj.class = nothing as i32 as u16;
    }
}
#[no_mangle]
pub unsafe extern "C" fn idlethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2;
    if obj.stage as i32 == obj.stages as i32 {
        obj.stage = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fadethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2;
    if obj.stage as i32 == obj.stages as i32 {
        obj.class = nothing as i32 as u16;
    }
}

unsafe fn killnear(mut chkx_0: i32, mut chky_0: i32, global_state: &mut GlobalState) {
    let mut spot: i32 = 0;
    let mut new: i32 = 0;
    spot = global_state.background[chky_0 as usize][chkx_0 as usize];
    if spot < 136 || spot > 145 {
        return;
    }
    PlaySound(6);
    if spot < 143 {
        global_state.background[chky_0 as usize][chkx_0 as usize] = 128;
    } else {
        global_state.background[chky_0 as usize][chkx_0 as usize] = spot + 19;
    }
    new = newobject();
    o[new as usize].active = true as boolean;
    o[new as usize].x = chkx_0 as u8;
    o[new as usize].y = chky_0 as u8;
    o[new as usize].stage = 0;
    o[new as usize].delay = 2;
    o[new as usize].class = dead1 as i32 as u16;
}

unsafe fn explodethink(global_state: &mut GlobalState) {
    obj.stage = (obj.stage).wrapping_add(1);
    if obj.stage as i32 == 1 {
        killnear(obj.x as i32 - 1, obj.y as i32, global_state);
        killnear(obj.x as i32, obj.y as i32 - 1, global_state);
        killnear(obj.x as i32 + 1, obj.y as i32, global_state);
        killnear(obj.x as i32, obj.y as i32 + 1, global_state);
    }
    obj.delay = 2;
    if obj.stage as i32 == obj.stages as i32 {
        obj.class = nothing as i32 as u16;
    }
}

unsafe fn think(global_state: &mut GlobalState) {
    if obj.delay as i32 > 0 {
        obj.delay = (obj.delay).wrapping_sub(1);
    } else if rndt() < obj.speed as i32 {
        match obj.think as i32 {
            0 => {
                playercmdthink(global_state);
            }
            3 => {
                chasethink(false as boolean, global_state);
            }
            4 => {
                chasethink(true as boolean, global_state);
            }
            1 => {
                gargthink(global_state);
            }
            2 => {
                dragonthink(global_state);
            }
            5 => {
                shooterthink(global_state);
            }
            6 => {
                idlethink();
            }
            7 => {
                fadethink();
            }
            8 => {
                explodethink(global_state);
            }
            9 => {
                gunthink(west as i32);
            }
            10 => {
                gunthink(north as i32);
            }
            _ => {}
        }
    }
}

pub unsafe fn doactive(global_state: &mut GlobalState) {
    if obj.class as i32 != dead1 as i32
        && ((obj.x as i32) < global_state.origin.x - 10
            || obj.x as i32 > global_state.origin.x + 34
            || (obj.y as i32) < global_state.origin.y - 10
            || obj.y as i32 > global_state.origin.y + 34)
    {
        o[objecton as usize].active = false as boolean;
    } else {
        think(global_state);
        eraseobj(global_state);
        if playdone != 0 {
            return;
        }
        if obj.class as i32 > nothing as i32 {
            drawobj(&global_state.priority, &mut global_state.view);
        }
        memcpy(
            &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj as *mut libc::c_void,
            &mut obj as *mut objtype as *const libc::c_void,
            ::std::mem::size_of::<activeobj>() as u64,
        );
    };
}

pub unsafe fn doinactive(global_state: &mut GlobalState) {
    if obj.x as i32 + obj.size as i32 >= global_state.origin.x
        && (obj.x as i32) < global_state.origin.x + 24
        && obj.y as i32 + obj.size as i32 >= global_state.origin.y
        && (obj.y as i32) < global_state.origin.y + 24
    {
        obj.active = true as boolean;
        obj.dir = north as i32 as u16;
        o[objecton as usize] = obj.into();
    }
}

pub unsafe fn playloop(global_state: &mut GlobalState) {
    global_state.screencenter.x = 11;
    loop {
        if indemo == demoenum::notdemo {
            centerwindow(11, 2, global_state);
            print(
                b" Entering\nlevel \0" as *const u8 as *const i8,
                global_state,
            );
            printint(level as i32, global_state);
            print(b"...\0" as *const u8 as *const i8, global_state);
            PlaySound(17);
            WaitEndSound(global_state);
        }
        clearold(&mut global_state.oldtiles);
        loadlevel(global_state);
        leveldone = false as boolean;
        if keydown[SDL_SCANCODE_F7 as usize] as i32 != 0
            && keydown[SDL_SCANCODE_D as usize] as i32 != 0
        {
            clearold(&mut global_state.oldtiles);
            refresh(global_state);
            refresh(global_state);
            clearkeys();
            centerwindow(12, 1, global_state);
            print(b"RECORD DEMO\0" as *const u8 as *const i8, global_state);
            loop {
                ch = get(global_state) as i8;
                if !(ch as i32 != 13) {
                    break;
                }
            }
            RecordDemo();
            clearold(&mut global_state.oldtiles);
            clearkeys();
        }
        playdone = false as boolean;
        frameon = 0;
        boltsleft = 0;
        shotpower = 0;
        initrndt(false as boolean);
        printshotpower(global_state);
        doall(global_state);
        if indemo == demoenum::recording {
            clearkeys();
            centerwindow(15, 1, global_state);
            print(b"SAVE AS DEMO#:\0" as *const u8 as *const i8, global_state);
            loop {
                ch = get(global_state) as i8;
                if !((ch as i32) < '0' as i32 || ch as i32 > '9' as i32) {
                    break;
                }
            }
            SaveDemo(ch as i32 - '0' as i32);
            clearold(&mut global_state.oldtiles);
            refresh(global_state);
            refresh(global_state);
        }
        if indemo != demoenum::notdemo {
            playdone = true as boolean;
        }
        if !(playdone == 0) {
            break;
        }
    }
}
