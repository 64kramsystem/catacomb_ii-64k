use ::libc;

use crate::{
    catacomb::{dofkeys, loadlevel},
    catasm::{doall, drawobj},
    demo_enum::demoenum,
    indemo,
    sdl_scan_codes::*,
};
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static mut resetgame: boolean;
    static mut ctrl: ControlStruct;
    static mut exitdemo: boolean;
    static mut side: libc::c_int;
    static mut opposite: [dirtype; 9];
    static mut originx: libc::c_int;
    static mut originy: libc::c_int;
    static mut objdef: [objdeftype; 23];
    static mut gamexit: exittype;
    static mut altobj: objtype;
    static mut altnum: libc::c_int;
    static mut view: [[libc::c_int; 86]; 87];
    static mut chkspot: libc::c_int;
    static mut chkx: libc::c_int;
    static mut chky: libc::c_int;
    static mut background: [[libc::c_int; 86]; 87];
    static mut numobj: libc::c_int;
    fn eraseobj();
    static mut obj: objtype;
    static mut o: [activeobj; 201];
    static mut objecton: libc::c_int;
    fn restore();
    static mut leveldone: boolean;
    static mut frameon: word;
    static mut boltsleft: libc::c_int;
    static mut altmeters: [[libc::c_char; 14]; 14];
    static mut meters: [[libc::c_char; 14]; 14];
    static mut shotpower: libc::c_int;
    fn clearold();
    static mut playdone: boolean;
    fn refresh();
    static mut GODMODE: boolean;
    fn bioskey(_: libc::c_int) -> libc::c_int;
    static mut highscores: [scores; 5];
    static mut level: sword;
    static mut score: sdword;
    fn centerwindow(width: libc::c_int, height: libc::c_int);
    fn printlong(val: libc::c_long);
    fn printint(val: libc::c_int);
    fn print(str: *const libc::c_char);
    fn _inputint() -> libc::c_uint;
    fn get() -> libc::c_int;
    static mut screencenterx: libc::c_int;
    fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
    fn WaitVBL();
    fn UpdateScreen();
    static mut sy: libc::c_int;
    static mut sx: libc::c_int;
    fn clearkeys();
    fn rndt() -> libc::c_int;
    fn initrndt(randomize: boolean);
    fn RecordDemo();
    fn SaveDemo(demonum: libc::c_int);
    fn ControlPlayer(player_0: libc::c_int) -> ControlStruct;
    static mut keydown: [boolean; 512];
    fn WaitEndSound();
    fn PlaySound(sound: libc::c_int);
    static mut ch: libc::c_char;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type sbyte = int8_t;
pub type word = uint16_t;
pub type sword = int16_t;
pub type sdword = int32_t;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const nukeshot: C2RustUnnamed_1 = 4;
pub const mshot: C2RustUnnamed_1 = 3;
pub const pshot: C2RustUnnamed_1 = 2;
pub const monster: C2RustUnnamed_1 = 1;
pub const benign: C2RustUnnamed_1 = 0;
pub type classtype = libc::c_uint;
pub const lastclass: classtype = 23;
pub const guns: classtype = 22;
pub const gune: classtype = 21;
pub const secretgate: classtype = 20;
pub const torch: classtype = 19;
pub const teleporter: classtype = 18;
pub const dead6: classtype = 17;
pub const dead5: classtype = 16;
pub const dead4: classtype = 15;
pub const dead3: classtype = 14;
pub const dead2: classtype = 13;
pub const dead1: classtype = 12;
pub const rock: classtype = 11;
pub const bigshot: classtype = 10;
pub const shot: classtype = 9;
pub const wallhit: classtype = 8;
pub const turbogre: classtype = 7;
pub const dragon: classtype = 6;
pub const gargoyle: classtype = 5;
pub const ogre: classtype = 4;
pub const skeleton: classtype = 3;
pub const goblin: classtype = 2;
pub const player: classtype = 1;
pub const nothing: classtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct activeobj {
    pub active: boolean,
    pub class: word,
    pub x: byte,
    pub y: byte,
    pub stage: byte,
    pub delay: byte,
    pub dir: word,
    pub hp: sbyte,
    pub oldx: byte,
    pub oldy: byte,
    pub oldtile: sword,
    pub filler: [byte; 1],
}

impl From<objtype> for activeobj {
    fn from(source: objtype) -> Self {
        Self {
            active: source.active,
            class: source.class,
            x: source.x,
            y: source.y,
            stage: source.stage,
            delay: source.delay,
            dir: source.dir,
            hp: source.hp,
            oldx: source.oldx,
            oldy: source.oldy,
            oldtile: source.oldtile,
            filler: source.filler,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objdeftype {
    pub think: byte,
    pub contact: byte,
    pub solid: byte,
    pub firstchar: word,
    pub size: byte,
    pub stages: byte,
    pub dirmask: byte,
    pub speed: word,
    pub hitpoints: byte,
    pub damage: byte,
    pub points: word,
    pub filler: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objtype {
    pub active: boolean,
    pub class: word,
    pub x: byte,
    pub y: byte,
    pub stage: byte,
    pub delay: byte,
    pub dir: word,
    pub hp: sbyte,
    pub oldx: byte,
    pub oldy: byte,
    pub oldtile: sword,
    pub filler: [byte; 1],
    pub think: byte,
    pub contact: byte,
    pub solid: byte,
    pub firstchar: word,
    pub size: byte,
    pub stages: byte,
    pub dirmask: byte,
    pub speed: word,
    pub hitpoints: byte,
    pub damage: byte,
    pub points: word,
    pub filler2: [byte; 2],
}

impl objtype {
    pub fn update_from_active(&mut self, active_o: activeobj) {
        self.active = active_o.active;
        self.class = active_o.class;
        self.x = active_o.x;
        self.y = active_o.y;
        self.stage = active_o.stage;
        self.delay = active_o.delay;
        self.dir = active_o.dir;
        self.hp = active_o.hp;
        self.oldx = active_o.oldx;
        self.oldy = active_o.oldy;
        self.oldtile = active_o.oldtile;
        self.filler = active_o.filler;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: boolean,
    pub button2: boolean,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct scores {
    pub score: sdword,
    pub level: sword,
    pub initials: [libc::c_char; 4],
}
pub type exittype = libc::c_uint;
pub const victorious: exittype = 3;
pub const reseted: exittype = 2;
pub const killed: exittype = 1;
pub const quited: exittype = 0;
#[no_mangle]
pub unsafe extern "C" fn newobject() -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    i = 1;
    loop {
        if !(i <= numobj) {
            current_block = 10680521327981672866;
            break;
        }
        if o[i as usize].class as libc::c_int == nothing as libc::c_int {
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
    o[i as usize].oldtile = -(1) as sword;
    o[i as usize].oldx = 0;
    o[i as usize].oldy = 0;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn printscore() {
    sx = 31;
    sy = 3;
    printlong(score as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn printhighscore() {
    sx = 31;
    sy = 5;
    printlong(highscores[1].score as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn printshotpower() {
    sx = 25;
    sy = 13;
    if shotpower == 13 {
        print((altmeters[13]).as_mut_ptr());
    } else {
        print((meters[shotpower as usize]).as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn printbody() {
    sx = 25;
    sy = 16;
    if o[0].hp as libc::c_int > 6 {
        print((meters[o[0].hp as usize]).as_mut_ptr());
    } else {
        print((altmeters[o[0].hp as usize]).as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn levelcleared() {
    let mut warp: [libc::c_char; 3] = [0; 3];
    let mut value: libc::c_int = 0;
    leveldone = true as boolean;
    warp[0] = (background[(altobj.y as libc::c_int + 2) as usize][altobj.x as usize] as libc::c_char
        as libc::c_int
        - 161) as libc::c_char;
    if (warp[0] as libc::c_int) < '0' as i32 || warp[0] as libc::c_int > '9' as i32 {
        warp[0] = '0' as i32 as libc::c_char;
    }
    warp[1] = (background[(altobj.y as libc::c_int + 2) as usize]
        [(altobj.x as libc::c_int + 1) as usize] as libc::c_char as libc::c_int
        - 161) as libc::c_char;
    if (warp[1] as libc::c_int) < '0' as i32 || warp[1] as libc::c_int > '9' as i32 {
        warp[2] = ' ' as i32 as libc::c_char;
    }
    value = atoi(warp.as_mut_ptr());
    if value > 0 {
        level = value as sword;
    } else {
        level += 1;
    }
    if level as libc::c_int > 30 {
        playdone = true as boolean;
        gamexit = victorious;
    }
}

unsafe fn givekey(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    i = items[1] as libc::c_int + 1;
    items[1] = i as sword;
    if i < 11 {
        drawchar(26 + i, 7, 31);
    }
}

pub unsafe fn givepotion(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    i = items[2] as libc::c_int + 1;
    items[2] = i as sword;
    if i < 11 {
        drawchar(26 + i, 8, 29);
    }
}

pub unsafe fn givebolt(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    i = items[3] as libc::c_int + 1;
    items[3] = i as sword;
    if i < 11 {
        drawchar(26 + i, 9, 30);
    }
}

pub unsafe fn givenuke(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    i = items[5] as libc::c_int + 1;
    items[5] = i as sword;
    if i < 11 {
        drawchar(26 + i, 10, 30);
    }
}

unsafe fn takekey(items: &mut [sword]) -> boolean {
    let mut i: libc::c_int = 0;
    if items[1] as libc::c_int > 0 {
        i = items[1] as libc::c_int - 1;
        items[1] = i as sword;
        if i < 10 {
            drawchar(27 + i, 7, 32);
        }
        PlaySound(11);
        return true as boolean;
    } else {
        PlaySound(14);
        return false as boolean;
    };
}

unsafe fn takepotion(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    if items[2] as libc::c_int > 0 {
        i = items[2] as libc::c_int - 1;
        items[2] = i as sword;
        if i < 11 {
            drawchar(27 + i, 8, 32);
        }
        PlaySound(12);
        o[0].hp = 13;
        obj.hp = 13;
        printbody();
    } else {
        PlaySound(14);
    };
}

unsafe fn castbolt(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    if items[3] as libc::c_int > 0 {
        i = items[3] as libc::c_int - 1;
        items[3] = i as sword;
        if i < 11 {
            drawchar(27 + i, 9, 32);
        }
        boltsleft = 8;
        PlaySound(13);
    } else {
        PlaySound(14);
    };
}

unsafe fn castnuke(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
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
    if items[5] as libc::c_int == 0 {
        PlaySound(14);
        return;
    }
    i = items[5] as libc::c_int - 1;
    items[5] = i as sword;
    if i < 11 {
        drawchar(27 + i, 10, 32);
    }
    base.delay = 0;
    base.stage = 0;
    base.active = true as boolean;
    base.x = obj.x;
    base.y = obj.y;
    base.oldx = base.x;
    base.oldy = base.y;
    base.oldtile = -(1) as sword;
    base.class = bigshot as libc::c_int as word;
    x = -(1);
    while x <= 1 {
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as libc::c_int + x * 2) as byte;
        o[n as usize].dir = north as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].x = (o[n as usize].x as libc::c_int + x * 2) as byte;
        o[n as usize].dir = south as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as libc::c_int + x * 2) as byte;
        o[n as usize].dir = east as libc::c_int as word;
        n = newobject();
        o[n as usize] = base;
        o[n as usize].y = (o[n as usize].y as libc::c_int + x * 2) as byte;
        o[n as usize].dir = west as libc::c_int as word;
        x += 1;
    }
    PlaySound(13);
    obj.stage = 2;
    obj.delay = 4;
}
#[no_mangle]
pub unsafe extern "C" fn playshoot() {
    let mut new: libc::c_int = 0;
    obj.stage = 2;
    obj.delay = 4;
    PlaySound(5);
    new = newobject();
    o[new as usize].class = shot as libc::c_int as word;
    side ^= 1;
    o[new as usize].delay = 0;
    o[new as usize].stage = 0;
    o[new as usize].active = true as boolean;
    o[new as usize].dir = obj.dir;
    match o[new as usize].dir as libc::c_int {
        0 => {
            o[new as usize].x = (obj.x as libc::c_int + side) as byte;
            o[new as usize].y = obj.y;
        }
        1 => {
            o[new as usize].x = (obj.x as libc::c_int + 1) as byte;
            o[new as usize].y = (obj.y as libc::c_int + side) as byte;
        }
        2 => {
            o[new as usize].x = (obj.x as libc::c_int + side) as byte;
            o[new as usize].y = (obj.y as libc::c_int + 1) as byte;
        }
        3 => {
            o[new as usize].x = obj.x;
            o[new as usize].y = (obj.y as libc::c_int + side) as byte;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn playbigshoot() {
    let mut new: libc::c_int = 0;
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
    o[new as usize].class = bigshot as libc::c_int as word;
}

unsafe fn givescroll(items: &mut [sword]) {
    if rndt() < 128 {
        givebolt(items);
    } else {
        givenuke(items);
    };
}
#[no_mangle]
pub unsafe extern "C" fn opendoor() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    PlaySound(11);
    x = chkx;
    y = chky;
    if chkspot == 165 {
        loop {
            view[y as usize][x as usize] = 128;
            background[y as usize][x as usize] = 128;
            y -= 1;
            if !(view[y as usize][x as usize] == 165) {
                break;
            }
        }
        y = chky + 1;
        while view[y as usize][x as usize] == 165 {
            view[y as usize][x as usize] = 128;
            background[y as usize][x as usize] = 128;
            y += 1;
        }
    } else {
        loop {
            view[y as usize][x as usize] = 128;
            background[y as usize][x as usize] = 128;
            x -= 1;
            if !(view[y as usize][x as usize] == 166) {
                break;
            }
        }
        x = chkx + 1;
        while view[y as usize][x as usize] == 166 {
            view[y as usize][x as usize] = 128;
            background[y as usize][x as usize] = 128;
            x += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tagobject() {
    let mut i: libc::c_int = altobj.hp as libc::c_int;
    if GODMODE as libc::c_int != 0 && altobj.class as libc::c_int == player as libc::c_int {
        return;
    }
    altobj.hp = (altobj.hp as libc::c_int - obj.damage as libc::c_int) as sbyte;
    if i <= obj.damage as libc::c_int {
        if altobj.class as libc::c_int == player as libc::c_int {
            o[0].hp = 0;
            altobj.hp = o[0].hp;
            printbody();
            PlaySound(10);
            playdone = true as boolean;
            gamexit = killed;
        } else {
            score = score + altobj.points as libc::c_int;
            printscore();
            PlaySound(9);
        }
        o[altnum as usize].class =
            (dead1 as libc::c_int - 1 + altobj.size as libc::c_int) as classtype as word;
        o[altnum as usize].delay = 2;
        o[altnum as usize].stage = 0;
    } else {
        if o[altnum as usize].class as libc::c_int == guns as libc::c_int
            || o[altnum as usize].class as libc::c_int == gune as libc::c_int
        {
            return;
        }
        o[altnum as usize].hp = altobj.hp;
        o[altnum as usize].stage = 3;
        if altnum == 0 {
            o[0].delay = 2;
            printbody();
            PlaySound(8);
        } else {
            o[altnum as usize].delay = 4;
            PlaySound(7);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn intomonster() -> boolean {
    let mut gotit: boolean = 0;
    altnum = 0;
    gotit = false as boolean;
    loop {
        altobj.update_from_active(o[altnum as usize]);
        if altobj.class as libc::c_int > nothing as libc::c_int && altnum != objecton {
            memcpy(
                &mut altobj.think as *mut byte as *mut libc::c_void,
                &mut *objdef.as_mut_ptr().offset(altobj.class as isize) as *mut objdeftype
                    as *const libc::c_void,
                ::std::mem::size_of::<objdeftype>() as libc::c_ulong,
            );
            if chkx >= altobj.x as libc::c_int
                && (chkx - altobj.x as libc::c_int) < altobj.size as libc::c_int
                && chky >= altobj.y as libc::c_int
                && (chky - altobj.y as libc::c_int) < altobj.size as libc::c_int
            {
                if altobj.solid != 0 {
                    gotit = true as boolean;
                } else if objecton == 0
                    && (altobj.class as libc::c_int == teleporter as libc::c_int
                        || altobj.class as libc::c_int == secretgate as libc::c_int)
                {
                    levelcleared();
                }
            }
        }
        if gotit == 0 {
            altnum += 1;
        }
        if gotit as libc::c_int != 0 || altnum > numobj {
            break;
        }
    }
    if gotit == 0 {
        return true as boolean;
    }
    match obj.contact as libc::c_int {
        0 => return false as boolean,
        1 | 3 => {
            if altnum == 0 {
                tagobject();
                obj.stage = 2;
                obj.delay = 20;
            } else if altobj.class as libc::c_int == shot as libc::c_int {
                return true as boolean;
            }
            return false as boolean;
        }
        2 => {
            if altnum > 0 {
                tagobject();
            }
            return false as boolean;
        }
        4 => {
            tagobject();
            return true as boolean;
        }
        _ => {}
    }
    return false as boolean;
}

unsafe fn walkthrough(items: &mut [sword]) -> boolean {
    let mut new: libc::c_int = 0;
    if chkspot == 128 {
        return true as boolean;
    }
    if chkspot >= 256 && chkspot <= 256 + 67 * 4 + 35 * 9 + 19 * 16 + 19 * 25 {
        return intomonster();
    }
    if chkspot >= 129 && chkspot <= 135 {
        if obj.contact as libc::c_int == pshot as libc::c_int
            || obj.contact as libc::c_int == nukeshot as libc::c_int
            || obj.contact as libc::c_int == mshot as libc::c_int
        {
            new = newobject();
            o[new as usize].active = true as boolean;
            o[new as usize].x = chkx as byte;
            o[new as usize].y = chky as byte;
            o[new as usize].stage = 0;
            o[new as usize].delay = 2;
            o[new as usize].class = wallhit as libc::c_int as word;
            PlaySound(6);
        }
        return false as boolean;
    }
    if chkspot >= 136 && chkspot <= 145 {
        if obj.contact as libc::c_int == pshot as libc::c_int
            || obj.contact as libc::c_int == nukeshot as libc::c_int
        {
            PlaySound(6);
            if chkspot < 143 {
                background[chky as usize][chkx as usize] = 128;
            } else {
                background[chky as usize][chkx as usize] = chkspot + 19;
            }
            new = newobject();
            o[new as usize].active = true as boolean;
            o[new as usize].x = chkx as byte;
            o[new as usize].y = chky as byte;
            o[new as usize].stage = 0;
            o[new as usize].delay = 2;
            o[new as usize].class = dead1 as libc::c_int as word;
            if obj.contact as libc::c_int == pshot as libc::c_int {
                return false as boolean;
            } else {
                return true as boolean;
            }
        } else {
            return false as boolean;
        }
    }
    if chkspot == 162 {
        if obj.class as libc::c_int == player as libc::c_int {
            givepotion(items);
            view[chky as usize][chkx as usize] = 128;
            background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 163 {
        if obj.class as libc::c_int == player as libc::c_int {
            givescroll(items);
            view[chky as usize][chkx as usize] = 128;
            background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 164 {
        if obj.class as libc::c_int == player as libc::c_int {
            givekey(items);
            view[chky as usize][chkx as usize] = 128;
            background[chky as usize][chkx as usize] = 128;
            PlaySound(2);
        }
        return true as boolean;
    }
    if chkspot == 165 || chkspot == 166 {
        if obj.class as libc::c_int == player as libc::c_int {
            if takekey(items) != 0 {
                opendoor();
                return true as boolean;
            }
        }
        return false as boolean;
    }
    if chkspot == 167 {
        if obj.class as libc::c_int == player as libc::c_int {
            score += 500;
            printscore();
            background[chky as usize][chkx as usize] = 128;
            view[chky as usize][chkx as usize] = 128;
            PlaySound(3);
        }
        return true as boolean;
    }
    if chkspot >= 29 && chkspot <= 31 {
        return true as boolean;
    }
    return false as boolean;
}

unsafe fn walk(items: &mut [sword]) -> boolean {
    let mut i: libc::c_int = 0;
    let mut newx: libc::c_int = 0;
    let mut newy: libc::c_int = 0;
    let mut deltay: libc::c_int = 0;
    let mut deltax: libc::c_int = 0;
    let mut try_0: boolean = 0;
    match obj.dir as libc::c_int {
        0 => {
            newx = obj.x as libc::c_int;
            newy = obj.y as libc::c_int - 1;
            chkx = newx;
            chky = newy;
            deltax = 1;
            deltay = 0;
        }
        1 => {
            newx = obj.x as libc::c_int + 1;
            newy = obj.y as libc::c_int;
            chkx = obj.x as libc::c_int + obj.size as libc::c_int;
            chky = newy;
            deltax = 0;
            deltay = 1;
        }
        2 => {
            newx = obj.x as libc::c_int;
            newy = obj.y as libc::c_int + 1;
            chkx = newx;
            chky = obj.y as libc::c_int + obj.size as libc::c_int;
            deltax = 1;
            deltay = 0;
        }
        3 => {
            newx = obj.x as libc::c_int - 1;
            newy = obj.y as libc::c_int;
            chkx = newx;
            chky = newy;
            deltax = 0;
            deltay = 1;
        }
        _ => return false as boolean,
    }
    i = 1;
    while i <= obj.size as libc::c_int {
        chkspot = view[chky as usize][chkx as usize];
        if chkspot != 128 {
            try_0 = walkthrough(items);
            if leveldone != 0 {
                return true as boolean;
            }
            if obj.stage as libc::c_int == 2 {
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
    obj.x = newx as byte;
    obj.y = newy as byte;
    obj.stage = (obj.stage as libc::c_int ^ 1) as byte;
    return true as boolean;
}

unsafe fn playercmdthink(items: &mut [sword]) {
    let mut olddir: dirtype = north;
    let mut c: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    c = ControlPlayer(1);
    obj.stage = (obj.stage as libc::c_int & 1) as byte;
    if c.button1 as libc::c_int != 0
        && c.button2 as libc::c_int != 0
        && keydown[SDL_SCANCODE_Q as usize] as libc::c_int != 0
    {
        givepotion(items);
        givescroll(items);
        givekey(items);
    }
    if (c.dir as libc::c_uint) < nodir as libc::c_int as libc::c_uint
        && frameon as libc::c_int % 2 != 0
    {
        if c.button2 != 0 {
            olddir = obj.dir as dirtype;
        }
        if c.dir as libc::c_uint > west as libc::c_int as libc::c_uint {
            if frameon as libc::c_int / 2 % 2 != 0 {
                match c.dir as libc::c_uint {
                    4 => {
                        obj.dir = east as libc::c_int as word;
                        walk(items);
                        c.dir = north;
                    }
                    5 => {
                        obj.dir = south as libc::c_int as word;
                        walk(items);
                        c.dir = east;
                    }
                    6 => {
                        obj.dir = west as libc::c_int as word;
                        walk(items);
                        c.dir = south;
                    }
                    7 => {
                        obj.dir = north as libc::c_int as word;
                        walk(items);
                        c.dir = west;
                    }
                    _ => {}
                }
            } else {
                match c.dir as libc::c_uint {
                    4 => {
                        obj.dir = north as libc::c_int as word;
                        walk(items);
                        c.dir = east;
                    }
                    5 => {
                        obj.dir = east as libc::c_int as word;
                        walk(items);
                        c.dir = south;
                    }
                    6 => {
                        obj.dir = south as libc::c_int as word;
                        walk(items);
                        c.dir = west;
                    }
                    7 => {
                        obj.dir = west as libc::c_int as word;
                        walk(items);
                        c.dir = north;
                    }
                    _ => {}
                }
            }
        }
        obj.dir = c.dir as word;
        if walk(items) == 0 {
            PlaySound(1);
        }
        if c.button2 != 0 {
            obj.dir = olddir as word;
        }
    } else if c.button2 == 0 {
        match c.dir as libc::c_uint {
            7 | 0 => {
                obj.dir = north as libc::c_int as word;
            }
            4 | 1 => {
                obj.dir = east as libc::c_int as word;
            }
            5 | 2 => {
                obj.dir = south as libc::c_int as word;
            }
            6 | 3 => {
                obj.dir = west as libc::c_int as word;
            }
            8 | _ => {}
        }
    }
    originx = obj.x as libc::c_int - 11;
    originy = obj.y as libc::c_int - 11;
    if boltsleft > 0 {
        if frameon as libc::c_int % 3 == 0 {
            playbigshoot();
            boltsleft -= 1;
        }
    } else if c.button1 != 0 {
        if shotpower == 0 {
            shotpower = 1;
        } else if shotpower < 13 && frameon as libc::c_int % 2 != 0 {
            shotpower += 1;
        }
        printshotpower();
    } else if shotpower > 0 {
        if shotpower == 13 {
            playbigshoot();
        } else {
            playshoot();
        }
        shotpower = 0;
        printshotpower();
    }
    if indemo == demoenum::notdemo {
        if keydown[SDL_SCANCODE_P as usize] as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
        {
            if (obj.hp as libc::c_int) < 13 {
                takepotion(items);
                keydown[SDL_SCANCODE_Q as usize] = false as boolean;
                keydown[SDL_SCANCODE_SPACE as usize] = false as boolean;
            }
        } else if keydown[SDL_SCANCODE_B as usize] != 0 {
            castbolt(items);
            keydown[SDL_SCANCODE_B as usize] = false as boolean;
        } else if keydown[SDL_SCANCODE_N as usize] as libc::c_int != 0
            || keydown[SDL_SCANCODE_RETURN as usize] as libc::c_int != 0
        {
            castnuke(items);
            keydown[SDL_SCANCODE_N as usize] = false as boolean;
            keydown[SDL_SCANCODE_RETURN as usize] = false as boolean;
        }
    }
    dofkeys(items);
    if resetgame != 0 {
        resetgame = false as boolean;
        playdone = true as boolean;
        return;
    }
    match indemo {
        demoenum::notdemo => {
            if keydown[SDL_SCANCODE_C as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_T as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
            {
                centerwindow(16, 2);
                print(b"warp to which\nlevel (1-99)?\0" as *const u8 as *const libc::c_char);
                clearkeys();
                level = _inputint() as sword;
                if (level as libc::c_int) < 1 {
                    level = 1;
                }
                if level as libc::c_int > 30 {
                    level = 30;
                }
                restore();
                leveldone = true as boolean;
            }
            if keydown[SDL_SCANCODE_C as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_T as usize] as libc::c_int != 0
                && keydown[SDL_SCANCODE_TAB as usize] as libc::c_int != 0
            {
                if GODMODE != 0 {
                    centerwindow(13, 1);
                    print(b"God Mode Off\0" as *const u8 as *const libc::c_char);
                    GODMODE = false as boolean;
                } else {
                    centerwindow(12, 1);
                    print(b"God Mode On\0" as *const u8 as *const libc::c_char);
                    GODMODE = true as boolean;
                }
                UpdateScreen();
                clearkeys();
                while bioskey(0) == 0 {
                    WaitVBL();
                }
                restore();
                clearkeys();
            }
        }
        demoenum::demoplay => {
            indemo = demoenum::notdemo;
            ctrl = ControlPlayer(1);
            if ctrl.button1 as libc::c_int != 0
                || ctrl.button2 as libc::c_int != 0
                || keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
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

unsafe fn chasethink(mut diagonal: boolean, items: &mut [sword]) {
    let mut deltax: libc::c_int = 0;
    let mut deltay: libc::c_int = 0;
    let mut d: [dirtype; 3] = [north; 3];
    let mut tdir: libc::c_int = 0;
    let mut olddir: libc::c_int = 0;
    let mut turnaround: libc::c_int = 0;
    obj.stage = (obj.stage as libc::c_int & 1) as byte;
    olddir = obj.dir as libc::c_int;
    turnaround = opposite[olddir as usize] as libc::c_int;
    deltax = o[0].x as libc::c_int - obj.x as libc::c_int;
    deltay = o[0].y as libc::c_int - obj.y as libc::c_int;
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
        tdir = d[1] as libc::c_int;
        d[1] = d[2];
        d[2] = tdir as dirtype;
    }
    if d[1] as libc::c_uint == turnaround as libc::c_uint {
        d[1] = nodir;
    }
    if d[2] as libc::c_uint == turnaround as libc::c_uint {
        d[2] = nodir;
    }
    if diagonal != 0 {
        if d[1] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[1] as word;
            if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                return;
            }
        }
        if d[2] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[2] as word;
            if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                return;
            }
        }
    } else {
        if d[2] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[2] as word;
            if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                return;
            }
        }
        if d[1] as libc::c_uint != nodir as libc::c_int as libc::c_uint {
            obj.dir = d[1] as word;
            if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                return;
            }
        }
    }
    obj.dir = olddir as word;
    if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
        return;
    }
    if rndt() > 128 {
        tdir = north as libc::c_int;
        while tdir <= west as libc::c_int {
            if tdir != turnaround {
                obj.dir = tdir as word;
                if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = west as libc::c_int;
        while tdir >= north as libc::c_int {
            if tdir != turnaround {
                obj.dir = tdir as word;
                if walk(items) as libc::c_int != 0 || obj.stage as libc::c_int == 3 {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    obj.dir = turnaround as word;
    walk(items);
}

unsafe fn gargthink(items: &mut [sword]) {
    let mut n: libc::c_int = 0;
    if rndt() > 220 {
        obj.stage = 2;
        obj.delay = 6;
        PlaySound(5);
        n = newobject();
        o[n as usize].class = rock as libc::c_int as word;
        o[n as usize].delay = 0;
        o[n as usize].stage = 0;
        o[n as usize].active = true as boolean;
        o[n as usize].dir = obj.dir;
        match obj.dir as libc::c_int {
            0 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 + side) as byte;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as libc::c_int + 3) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 1 + side) as byte;
            }
            2 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 + side) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 3) as byte;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as libc::c_int + 1 + side) as byte;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, items);
    };
}

unsafe fn dragonthink(items: &mut [sword]) {
    let mut n: libc::c_int = 0;
    if rndt() > 220 {
        obj.stage = 2;
        obj.delay = 6;
        PlaySound(5);
        n = newobject();
        o[n as usize].class = bigshot as libc::c_int as word;
        o[n as usize].delay = 0;
        o[n as usize].stage = 0;
        o[n as usize].active = true as boolean;
        o[n as usize].dir = obj.dir;
        match o[n as usize].dir as libc::c_int {
            0 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 + side) as byte;
                o[n as usize].y = obj.y;
            }
            1 => {
                o[n as usize].x = (obj.x as libc::c_int + 3) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 1 + side) as byte;
            }
            2 => {
                o[n as usize].x = (obj.x as libc::c_int + 1 + side) as byte;
                o[n as usize].y = (obj.y as libc::c_int + 3) as byte;
            }
            3 => {
                o[n as usize].x = obj.x;
                o[n as usize].y = (obj.y as libc::c_int + 1 + side) as byte;
            }
            _ => {}
        }
        return;
    } else {
        chasethink(false as boolean, items);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gunthink(mut dir: libc::c_int) {
    let mut n: libc::c_int = 0;
    PlaySound(5);
    obj.stage = 0;
    n = newobject();
    o[n as usize].class = bigshot as libc::c_int as word;
    o[n as usize].delay = 0;
    o[n as usize].stage = 0;
    o[n as usize].active = true as boolean;
    o[n as usize].dir = dir as word;
    o[n as usize].x = obj.x;
    o[n as usize].y = obj.y;
}

unsafe fn shooterthink(items: &mut [sword]) {
    if (obj.x as libc::c_int) < originx - 1
        || (obj.y as libc::c_int) < originy - 1
        || obj.x as libc::c_int > originx + 22
        || obj.y as libc::c_int > originy + 22
        || walk(items) == 0
        || obj.stage as libc::c_int == 2
    {
        obj.class = nothing as libc::c_int as word;
    }
}
#[no_mangle]
pub unsafe extern "C" fn idlethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.stage = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fadethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    obj.delay = 2;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.class = nothing as libc::c_int as word;
    }
}
#[no_mangle]
pub unsafe extern "C" fn killnear(mut chkx_0: libc::c_int, mut chky_0: libc::c_int) {
    let mut spot: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    spot = background[chky_0 as usize][chkx_0 as usize];
    if spot < 136 || spot > 145 {
        return;
    }
    PlaySound(6);
    if spot < 143 {
        background[chky_0 as usize][chkx_0 as usize] = 128;
    } else {
        background[chky_0 as usize][chkx_0 as usize] = spot + 19;
    }
    new = newobject();
    o[new as usize].active = true as boolean;
    o[new as usize].x = chkx_0 as byte;
    o[new as usize].y = chky_0 as byte;
    o[new as usize].stage = 0;
    o[new as usize].delay = 2;
    o[new as usize].class = dead1 as libc::c_int as word;
}
#[no_mangle]
pub unsafe extern "C" fn explodethink() {
    obj.stage = (obj.stage).wrapping_add(1);
    if obj.stage as libc::c_int == 1 {
        killnear(obj.x as libc::c_int - 1, obj.y as libc::c_int);
        killnear(obj.x as libc::c_int, obj.y as libc::c_int - 1);
        killnear(obj.x as libc::c_int + 1, obj.y as libc::c_int);
        killnear(obj.x as libc::c_int, obj.y as libc::c_int + 1);
    }
    obj.delay = 2;
    if obj.stage as libc::c_int == obj.stages as libc::c_int {
        obj.class = nothing as libc::c_int as word;
    }
}

unsafe fn think(items: &mut [sword]) {
    if obj.delay as libc::c_int > 0 {
        obj.delay = (obj.delay).wrapping_sub(1);
    } else if rndt() < obj.speed as libc::c_int {
        match obj.think as libc::c_int {
            0 => {
                playercmdthink(items);
            }
            3 => {
                chasethink(false as boolean, items);
            }
            4 => {
                chasethink(true as boolean, items);
            }
            1 => {
                gargthink(items);
            }
            2 => {
                dragonthink(items);
            }
            5 => {
                shooterthink(items);
            }
            6 => {
                idlethink();
            }
            7 => {
                fadethink();
            }
            8 => {
                explodethink();
            }
            9 => {
                gunthink(west as libc::c_int);
            }
            10 => {
                gunthink(north as libc::c_int);
            }
            _ => {}
        }
    }
}

pub unsafe fn doactive(priority: &[byte], items: &mut [sword]) {
    if obj.class as libc::c_int != dead1 as libc::c_int
        && ((obj.x as libc::c_int) < originx - 10
            || obj.x as libc::c_int > originx + 34
            || (obj.y as libc::c_int) < originy - 10
            || obj.y as libc::c_int > originy + 34)
    {
        o[objecton as usize].active = false as boolean;
    } else {
        think(items);
        eraseobj();
        if playdone != 0 {
            return;
        }
        if obj.class as libc::c_int > nothing as libc::c_int {
            drawobj(priority);
        }
        memcpy(
            &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj as *mut libc::c_void,
            &mut obj as *mut objtype as *const libc::c_void,
            ::std::mem::size_of::<activeobj>() as libc::c_ulong,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn doinactive() {
    if obj.x as libc::c_int + obj.size as libc::c_int >= originx
        && (obj.x as libc::c_int) < originx + 24
        && obj.y as libc::c_int + obj.size as libc::c_int >= originy
        && (obj.y as libc::c_int) < originy + 24
    {
        obj.active = true as boolean;
        obj.dir = north as libc::c_int as word;
        o[objecton as usize] = obj.into();
    }
}

pub unsafe fn playloop(priority: &[byte], items: &mut [sword]) {
    screencenterx = 11;
    loop {
        if indemo == demoenum::notdemo {
            centerwindow(11, 2);
            print(b" Entering\nlevel \0" as *const u8 as *const libc::c_char);
            printint(level as libc::c_int);
            print(b"...\0" as *const u8 as *const libc::c_char);
            PlaySound(17);
            WaitEndSound();
        }
        clearold();
        loadlevel(items);
        leveldone = false as boolean;
        if keydown[SDL_SCANCODE_F7 as usize] as libc::c_int != 0
            && keydown[SDL_SCANCODE_D as usize] as libc::c_int != 0
        {
            clearold();
            refresh();
            refresh();
            clearkeys();
            centerwindow(12, 1);
            print(b"RECORD DEMO\0" as *const u8 as *const libc::c_char);
            loop {
                ch = get() as libc::c_char;
                if !(ch as libc::c_int != 13) {
                    break;
                }
            }
            RecordDemo();
            clearold();
            clearkeys();
        }
        playdone = false as boolean;
        frameon = 0;
        boltsleft = 0;
        shotpower = 0;
        initrndt(false as boolean);
        printshotpower();
        doall(priority, items);
        if indemo == demoenum::recording {
            clearkeys();
            centerwindow(15, 1);
            print(b"SAVE AS DEMO#:\0" as *const u8 as *const libc::c_char);
            loop {
                ch = get() as libc::c_char;
                if !((ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32) {
                    break;
                }
            }
            SaveDemo(ch as libc::c_int - '0' as i32);
            clearold();
            refresh();
            refresh();
        }
        if indemo != demoenum::notdemo {
            playdone = true as boolean;
        }
        if !(playdone == 0) {
            break;
        }
    }
}
