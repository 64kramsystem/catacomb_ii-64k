use libc::O_RDONLY;

use crate::{
    cat_play::{givebolt, givenuke, givepotion, playloop},
    cpanel::controlpanel,
    demo_enum::demoenum,
    extra_constants::{
        blankfloor, leftoff, maxpics, numlevels, solidwall, tile2s, topoff, NUM_DEMOS, O_BINARY,
    },
    indemo,
    pcrlib_c::_setupgame,
    sdl_scan_codes::*,
};
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn initobjects();
    fn RLEExpand(source: *mut libc::c_char, dest: *mut libc::c_char, origlen: libc::c_long);
    fn bioskey(_: libc::c_int) -> libc::c_int;
    fn _quit(_: *mut libc::c_char);
    fn _checkhighscore();
    fn _showhighscores();
    static mut _extension: *const libc::c_char;
    static mut score: sdword;
    fn printhighscore();
    fn printbody();
    fn printshotpower();
    fn printscore();
    static mut level: sword;
    fn expwin(width: libc::c_int, height: libc::c_int);
    fn centerwindow(width: libc::c_int, height: libc::c_int);
    fn bar(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int, ch_0: libc::c_int);
    fn drawwindow(xl: libc::c_int, yl: libc::c_int, xh: libc::c_int, yh: libc::c_int);
    fn printint(val: libc::c_int);
    fn printchartile(str_0: *const libc::c_char);
    fn print(str_0: *const libc::c_char);
    fn get() -> libc::c_int;
    static mut screencentery: libc::c_int;
    static mut screencenterx: libc::c_int;
    fn drawchartile(x: libc::c_int, y: libc::c_int, tile: libc::c_int);
    fn drawpic(x: libc::c_int, y: libc::c_int, picnum: libc::c_int);
    fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
    fn installgrfile(filename: *mut libc::c_char, inmem: *mut libc::c_void);
    fn WaitVBL();
    fn UpdateScreen();
    static mut leftedge: libc::c_int;
    static mut sy: libc::c_int;
    static mut sx: libc::c_int;
    static mut grmode: grtype;
    fn _Verify(filename: *mut libc::c_char) -> libc::c_long;
    fn clearkeys();
    fn rnd(_: word) -> libc::c_int;
    fn rndt() -> libc::c_int;
    fn bloadin(filename: *mut libc::c_char) -> *mut libc::c_void;
    fn LoadFile(filename: *mut libc::c_char, buffer: *mut libc::c_char) -> libc::c_ulong;
    fn LoadDemo(demonum: libc::c_int);
    fn ControlPlayer(player_0: libc::c_int) -> ControlStruct;
    static mut keydown: [boolean; 512];
    fn WaitEndSound();
    fn PlaySound(sound: libc::c_int);
    static mut str: [libc::c_char; 80];
    static mut ch: libc::c_char;
    fn cgarefresh();
    fn egarefresh();
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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

pub enum statetype {
    inscores, // 2
    intitle,  // 1
    ingame,   // 0
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: boolean,
    pub button2: boolean,
}

pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
pub type exittype = libc::c_uint;
pub const victorious: exittype = 3;
pub const reseted: exittype = 2;
pub const killed: exittype = 1;
pub const quited: exittype = 0;
#[inline]
unsafe extern "C" fn itoa(
    mut value: libc::c_int,
    mut str_0: *mut libc::c_char,
    mut base: libc::c_int,
) -> *mut libc::c_char {
    if base == 16 {
        sprintf(str_0, b"%X\0" as *const u8 as *const libc::c_char, value);
    } else {
        sprintf(str_0, b"%d\0" as *const u8 as *const libc::c_char, value);
    }
    return str_0;
}
#[no_mangle]
pub static mut altmeters: [[libc::c_char; 14]; 14] = [
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
#[no_mangle]
pub static mut meters: [[libc::c_char; 14]; 14] = [
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
#[no_mangle]
pub static mut opposite: [dirtype; 9] = [
    south, west, north, east, southwest, northwest, northeast, southeast, nodir,
];
#[no_mangle]
pub static mut gamexit: exittype = quited;
#[no_mangle]
pub static mut oldtiles: [libc::c_int; 576] = [0; 576];
#[no_mangle]
pub static mut background: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
pub static mut view: [[libc::c_int; 86]; 87] = [[0; 86]; 87];
#[no_mangle]
pub static mut originx: libc::c_int = 0;
#[no_mangle]
pub static mut originy: libc::c_int = 0;

#[no_mangle]
pub static mut saveitems: [sword; 6] = [0; 6];
#[no_mangle]
pub static mut shotpower: libc::c_int = 0;
#[no_mangle]
pub static mut side: libc::c_int = 0;
#[no_mangle]
pub static mut boltsleft: libc::c_int = 0;
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
pub static mut altnum: libc::c_int = 0;
#[no_mangle]
pub static mut numobj: libc::c_int = 0;
#[no_mangle]
pub static mut objecton: libc::c_int = 0;
#[no_mangle]
pub static mut objdef: [objdeftype; 23] = [objdeftype {
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
}; 23];
#[no_mangle]
pub static mut playdone: boolean = 0;
#[no_mangle]
pub static mut leveldone: boolean = 0;
#[no_mangle]
pub static mut tempb: boolean = 0;
#[no_mangle]
pub static mut tempp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut chkx: libc::c_int = 0;
#[no_mangle]
pub static mut chky: libc::c_int = 0;
#[no_mangle]
pub static mut chkspot: libc::c_int = 0;
#[no_mangle]
pub static mut frameon: word = 0;
#[no_mangle]
pub static mut grmem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut VGAPAL: libc::c_int = 0;
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
pub static mut pics: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut picsexact: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut EGADATASTART: libc::c_uint = 0;
#[no_mangle]
pub static mut savescore: sdword = 0;
#[no_mangle]
pub static mut GODMODE: boolean = false as boolean;
#[no_mangle]
pub static mut demowin: [[libc::c_char; 16]; 5] = [
    [
        14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16,
    ],
    [
        17,
        ' ' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        18,
    ],
    [
        17,
        'S' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        18,
    ],
    [
        17,
        'F' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        18,
    ],
    [
        19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn refresh() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut basex: libc::c_int = 0;
    let mut basey: libc::c_int = 0;
    let mut underwin: [[word; 16]; 5] = [[0; 16]; 5];
    basex = originx + 4;
    basey = originy + 17;
    if indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                underwin[y as usize][x as usize] =
                    view[(y + basey) as usize][(x + basex) as usize] as word;
                view[(y + basey) as usize][(x + basex) as usize] =
                    demowin[y as usize][x as usize] as libc::c_int;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        cgarefresh();
    } else {
        egarefresh();
    }
    if indemo != demoenum::notdemo {
        y = 0;
        while y <= 4 {
            x = 0;
            while x <= 15 {
                view[(y + basey) as usize][(x + basex) as usize] =
                    underwin[y as usize][x as usize] as libc::c_int;
                x += 1;
            }
            y += 1;
        }
    }
    WaitVBL();
}
#[no_mangle]
pub unsafe extern "C" fn simplerefresh() {
    WaitVBL();
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        cgarefresh();
    } else {
        egarefresh();
    };
}
#[no_mangle]
pub unsafe extern "C" fn loadgrfiles() {
    if !picsexact.is_null() {
        free(picsexact as *mut libc::c_void);
    }
    if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        pics = bloadin(b"CGACHARS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
        picsexact = pics;
        installgrfile(
            b"CGAPICS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    } else {
        pics = bloadin(b"EGACHARS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as *mut libc::c_char;
        picsexact = pics;
        installgrfile(
            b"EGAPICS.CA2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn clearold() {
    memset(
        &mut oldtiles as *mut [libc::c_int; 576] as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 576]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn restore() {
    clearold();
    simplerefresh();
}
#[no_mangle]
pub unsafe extern "C" fn wantmore() -> boolean {
    sx = 2;
    sy = 20;
    print(b"(space for more/esc)\0" as *const u8 as *const libc::c_char);
    sx = 12;
    sy = 21;
    ch = get() as libc::c_char;
    if ch as libc::c_int == 27 {
        return false as boolean;
    }
    return true as boolean;
}
unsafe extern "C" fn charpic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut c: classtype,
    mut dir: dirtype,
    mut stage: libc::c_int,
) {
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tilenum: libc::c_int = 0;
    size = objdef[c as usize].size as libc::c_int;
    tilenum = (objdef[c as usize].firstchar as libc::c_uint).wrapping_add(
        ((size * size) as libc::c_uint).wrapping_mul(
            (dir as libc::c_uint & objdef[c as usize].dirmask as libc::c_uint)
                .wrapping_mul(objdef[c as usize].stages as libc::c_uint)
                .wrapping_add(stage as libc::c_uint),
        ),
    ) as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn help() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    centerwindow(20, 20);
    print(b"  C A T A C O M B   \n\0" as *const u8 as *const libc::c_char);
    print(b"   - - - - - - -    \n\0" as *const u8 as *const libc::c_char);
    print(b" by John Carmack    \n\0" as *const u8 as *const libc::c_char);
    print(b"                    \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    print(b"f1 = help           \n\0" as *const u8 as *const libc::c_char);
    print(b"f2 = control panel  \n\0" as *const u8 as *const libc::c_char);
    print(b"f3 = game reset     \n\0" as *const u8 as *const libc::c_char);
    print(b"f4 = save game      \n\0" as *const u8 as *const libc::c_char);
    print(b"f5 = load saved game\n\0" as *const u8 as *const libc::c_char);
    print(b"f9 = pause          \n\0" as *const u8 as *const libc::c_char);
    print(b"f10 / ESC = quit    \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    print(b"hit fire at the demo\n\0" as *const u8 as *const libc::c_char);
    print(b"to begin playing.   \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\nKeyboard controls:  \n\n\0" as *const u8 as *const libc::c_char);
    print(b"move    : arrows    \n\0" as *const u8 as *const libc::c_char);
    print(b"button1 : ctrl      \n\0" as *const u8 as *const libc::c_char);
    print(b"button2 : alt       \n\0" as *const u8 as *const libc::c_char);
    print(b"\nTo switch to mouse \n\0" as *const u8 as *const libc::c_char);
    print(b"or joystick control,\n\0" as *const u8 as *const libc::c_char);
    print(b"hit f2             \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"Button 1 / ctrl key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Builds shot power.  \n\0" as *const u8 as *const libc::c_char);
    print(b"If the shot power   \n\0" as *const u8 as *const libc::c_char);
    print(b"meter is full when  \n\0" as *const u8 as *const libc::c_char);
    print(b"the button is       \n\0" as *const u8 as *const libc::c_char);
    print(b"released, a super   \n\0" as *const u8 as *const libc::c_char);
    print(b"shot will be        \n\0" as *const u8 as *const libc::c_char);
    print(b"launched.           \n\0" as *const u8 as *const libc::c_char);
    print(b"\n\0" as *const u8 as *const libc::c_char);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2);
    charpic(19, 15, shot, east, 1);
    charpic(17, 14, shot, east, 0);
    charpic(15, 15, shot, east, 1);
    charpic(8, 14, bigshot, east, 0);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"Button 2 / alt key:\n\0" as *const u8 as *const libc::c_char);
    print(b"Allows you to move  \n\0" as *const u8 as *const libc::c_char);
    print(b"without changing the\n\0" as *const u8 as *const libc::c_char);
    print(b"direction you are   \n\0" as *const u8 as *const libc::c_char);
    print(b"facing.  Good for   \n\0" as *const u8 as *const libc::c_char);
    print(b"searching walls and \n\0" as *const u8 as *const libc::c_char);
    print(b"fighting retreats.  \n\0" as *const u8 as *const libc::c_char);
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
    charpic(6, 13, player, south, 2);
    sx = 6;
    sy = 15;
    print(b"\x1D\x1D\x1E\x1E\x1F\x1F\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\"P\" or \"space\" will \n\0" as *const u8 as *const libc::c_char);
    print(b"take a healing      \n\0" as *const u8 as *const libc::c_char);
    print(b"potion if you have  \n\0" as *const u8 as *const libc::c_char);
    print(b"one.  This restores \n\0" as *const u8 as *const libc::c_char);
    print(b"the body meter to   \n\0" as *const u8 as *const libc::c_char);
    print(b"full strength.  Keep\n\0" as *const u8 as *const libc::c_char);
    print(b"a sharp eye on the  \n\0" as *const u8 as *const libc::c_char);
    print(b"meter, because when \n\0" as *const u8 as *const libc::c_char);
    print(b"it runs out, you are\n\0" as *const u8 as *const libc::c_char);
    print(b"dead!               \n\n\0" as *const u8 as *const libc::c_char);
    print(b"\"B\" will cast a bolt\n\0" as *const u8 as *const libc::c_char);
    print(b"spell if you have   \n\0" as *const u8 as *const libc::c_char);
    print(b"any.  You can mow   \n\0" as *const u8 as *const libc::c_char);
    print(b"down a lot of       \n\0" as *const u8 as *const libc::c_char);
    print(b"monsters with a bit \n\0" as *const u8 as *const libc::c_char);
    print(b"of skill.           \n\0" as *const u8 as *const libc::c_char);
    if wantmore() == 0 {
        return;
    }
    centerwindow(20, 20);
    print(b"\"N\" or \"enter\" will \n\0" as *const u8 as *const libc::c_char);
    print(b"cast a nuke spell.  \n\0" as *const u8 as *const libc::c_char);
    print(b"This usually wipes  \n\0" as *const u8 as *const libc::c_char);
    print(b"out all the monsters\n\0" as *const u8 as *const libc::c_char);
    print(b"near you.  Consider \n\0" as *const u8 as *const libc::c_char);
    print(b"it a panic button   \n\0" as *const u8 as *const libc::c_char);
    print(b"when you are being  \n\0" as *const u8 as *const libc::c_char);
    print(b"mobbed by monsters! \n\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"POTIONS:       \x80\xA2\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"SCROLLS:       \x80\xA3\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b" (BOLTS/NUKES) \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"TREASURE:      \x80\xA7\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b" (POINTS)      \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    printchartile(b"               \x80\x80\x80\n\0" as *const u8 as *const libc::c_char);
    wantmore();
}
#[no_mangle]
pub unsafe extern "C" fn reset() {
    centerwindow(18, 1);
    print(b"reset game (y/n)?\0" as *const u8 as *const libc::c_char);
    ch = get() as libc::c_char;
    if ch as libc::c_int == 'y' as i32 {
        gamexit = killed;
        playdone = true as boolean;
    }
}

pub unsafe fn loadlevel(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    let mut tokens: [classtype; 26] = [
        player, teleporter, goblin, skeleton, ogre, gargoyle, dragon, turbogre, guns, gune,
        secretgate, nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing,
        nothing, nothing, nothing, nothing, nothing, nothing, nothing,
    ];
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut st: [libc::c_char; 64] = [0; 64];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut btile: byte = 0;
    let mut sm: [libc::c_char; 4096] = [0; 4096];
    let mut rle: [libc::c_char; 4096] = [0; 4096];
    strcpy(
        filename.as_mut_ptr(),
        b"LEVEL\0" as *const u8 as *const libc::c_char,
    );
    itoa(level as libc::c_int, st.as_mut_ptr(), 10);
    strcat(filename.as_mut_ptr(), st.as_mut_ptr());
    strcat(
        filename.as_mut_ptr(),
        b".CA2\0" as *const u8 as *const libc::c_char,
    );
    LoadFile(filename.as_mut_ptr(), rle.as_mut_ptr());
    RLEExpand(&mut *rle.as_mut_ptr().offset(4), sm.as_mut_ptr(), 4096);
    numobj = 0;
    o[0].x = 13;
    o[0].y = 13;
    o[0].stage = 0;
    o[0].delay = 0;
    o[0].dir = east as libc::c_int as word;
    o[0].oldx = 0;
    o[0].oldy = 0;
    o[0].oldtile = -(1) as sword;
    yy = 0;
    while yy < 64 {
        xx = 0;
        while xx < 64 {
            btile = sm[(yy * 64 + xx) as usize] as byte;
            if (btile as libc::c_int) < 230 {
                background[(yy + 11) as usize][(xx + 11) as usize] = btile as libc::c_int;
            } else {
                background[(yy + 11) as usize][(xx + 11) as usize] = 128;
                if tokens[(btile as libc::c_int - 230) as usize] as libc::c_uint
                    == player as libc::c_int as libc::c_uint
                {
                    o[0].x = (xx + 11) as byte;
                    o[0].y = (yy + 11) as byte;
                } else {
                    numobj += 1;
                    o[numobj as usize].active = false as boolean;
                    o[numobj as usize].class =
                        tokens[(btile as libc::c_int - 230) as usize] as word;
                    o[numobj as usize].x = (xx + 11) as byte;
                    o[numobj as usize].y = (yy + 11) as byte;
                    o[numobj as usize].stage = 0;
                    o[numobj as usize].delay = 0;
                    o[numobj as usize].dir = (rndt() / 64) as dirtype as word;
                    o[numobj as usize].hp =
                        objdef[o[numobj as usize].class as usize].hitpoints as sbyte;
                    o[numobj as usize].oldx = o[numobj as usize].x;
                    o[numobj as usize].oldy = o[numobj as usize].y;
                    o[numobj as usize].oldtile = -(1) as sword;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    originx = o[0].x as libc::c_int - 11;
    originy = o[0].y as libc::c_int - 11;
    shotpower = 0;
    y = 11 - 1;
    while y < 65 + 11 {
        x = 11 - 1;
        while x < 64 + 11 {
            view[y as usize][x as usize] = background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    sx = 33;
    sy = 1;
    printint(level as libc::c_int);
    print(b" \0" as *const u8 as *const libc::c_char);
    restore();
    i = 0;
    while i < 6 {
        saveitems[i as usize] = items[i as usize];
        i += 1;
    }
    savescore = score;
    saveo[0] = o[0];
}

unsafe fn drawside(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
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
    print(
        b"  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n\0" as *const u8 as *const libc::c_char,
    );
    print(b" shot power\n\n\n    body\n\n\n\0" as *const u8 as *const libc::c_char);
    printhighscore();
    printbody();
    printshotpower();
    printscore();
    sx = 33;
    sy = 1;
    printint(level as libc::c_int);
    drawpic(25 * 8, 17 * 8, 13);
    i = 1;
    while i <= items[1] as libc::c_int && i < 11 {
        drawchar(26 + i, 7, 31);
        i += 1;
    }
    i = 1;
    while i <= items[2] as libc::c_int && i < 11 {
        drawchar(26 + i, 8, 29);
        i += 1;
    }
    i = 1;
    while i <= items[3] as libc::c_int && i < 11 {
        drawchar(26 + i, 9, 30);
        i += 1;
    }
    i = 1;
    while i <= items[5] as libc::c_int && i < 11 {
        drawchar(26 + i, 10, 30);
        i += 1;
    }
}

unsafe fn playsetup(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    shotpower = 0;
    bar(0, 0, 23, 23, 0);
    if level as libc::c_int == 0 {
        i = 1;
        while i < 6 {
            items[i as usize] = 0;
            i += 1;
        }
        score = 0;
        level = 1;
        o[0].active = true as boolean;
        o[0].class = player as libc::c_int as word;
        o[0].hp = 13;
        o[0].dir = west as libc::c_int as word;
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

pub unsafe fn repaintscreen(items: &mut [sword]) {
    match gamestate {
        statetype::intitle => {
            drawpic(0, 0, 14);
        }
        statetype::ingame => {
            restore();
            drawside(items);
            printscore();
            sx = 33;
            sy = 1;
            printint(level as libc::c_int);
        }
        statetype::inscores => {
            restore();
            drawside(items);
            printscore();
            sx = 33;
            sy = 1;
            printint(level as libc::c_int);
            indemo = demoenum::demoplay;
        }
    };
}

pub unsafe fn dofkeys(items: &mut [sword]) {
    let mut handle: libc::c_int = 0;
    let mut key: libc::c_int = bioskey(1);
    if key == SDL_SCANCODE_ESCAPE as libc::c_int {
        key = SDL_SCANCODE_F10 as libc::c_int;
    }
    if key < SDL_SCANCODE_F1 as libc::c_int || key > SDL_SCANCODE_F10 as libc::c_int {
        return;
    }
    let mut current_block_72: u64;
    match key {
        58 => {
            clearkeys();
            help();
        }
        59 => {
            clearkeys();
            controlpanel(items);
        }
        60 => {
            clearkeys();
            expwin(18, 1);
            print(b"RESET GAME (Y/N)?\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            if ch as libc::c_int == 'Y' as i32 {
                resetgame = true as boolean;
            }
        }
        61 => {
            clearkeys();
            expwin(22, 4);
            if indemo != demoenum::notdemo {
                print(b"Can't save game here!\0" as *const u8 as *const libc::c_char);
                get();
            } else {
                print(b"Save as game #(1-9):\0" as *const u8 as *const libc::c_char);
                ch = toupper(get()) as libc::c_char;
                drawchar(sx, sy, ch as libc::c_int);
                if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
                    strcpy(
                        str.as_mut_ptr(),
                        b"GAME0.CA2\0" as *const u8 as *const libc::c_char,
                    );
                    str[4] = ch;
                    if _Verify(str.as_mut_ptr()) != 0 {
                        print(
                            b"\nGame exists,\noverwrite (Y/N)?\0" as *const u8
                                as *const libc::c_char,
                        );
                        ch = get() as libc::c_char;
                        if ch as libc::c_int != 'Y' as i32 && ch as libc::c_int != 'y' as i32 {
                            current_block_72 = 919954187481050311;
                        } else {
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
                            sy -= 1;
                            sx = leftedge;
                            print(b"                    \0" as *const u8 as *const libc::c_char);
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
                                0o1 as libc::c_int
                                    | 0
                                    | 0o100 as libc::c_int
                                    | 0o1000 as libc::c_int,
                                0o400 as libc::c_int | 0o200 as libc::c_int,
                            );
                            if handle == -(1) {
                                return;
                            }
                            write(
                                handle,
                                &mut saveitems as *mut [sword; 6] as *const libc::c_void,
                                ::std::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut savescore as *mut sdword as *const libc::c_void,
                                ::std::mem::size_of::<sdword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut level as *mut sword as *const libc::c_void,
                                ::std::mem::size_of::<sword>() as libc::c_ulong,
                            );
                            write(
                                handle,
                                &mut *saveo.as_mut_ptr().offset(0) as *mut activeobj
                                    as *const libc::c_void,
                                ::std::mem::size_of::<activeobj>() as libc::c_ulong,
                            );
                            close(handle);
                            print(b"\nGame saved.  Hit F5\n\0" as *const u8 as *const libc::c_char);
                            print(b"when you wish to\n\0" as *const u8 as *const libc::c_char);
                            print(b"restart the game.\0" as *const u8 as *const libc::c_char);
                            get();
                        }
                    }
                }
            }
        }
        62 => {
            clearkeys();
            expwin(22, 4);
            print(b"Load game #(1-9):\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            drawchar(sx, sy, ch as libc::c_int);
            if !((ch as libc::c_int) < '1' as i32 || ch as libc::c_int > '9' as i32) {
                strcpy(
                    str.as_mut_ptr(),
                    b"GAME0.CA2\0" as *const u8 as *const libc::c_char,
                );
                str[4] = ch;
                // The flags don't make much sense, as O_RDONLY == O_BINARY == 0; this comes from the original
                // project.
                handle = open(
                    str.as_mut_ptr(),
                    O_RDONLY | O_BINARY,
                    0o200 as libc::c_int | 0o400 as libc::c_int,
                );
                if handle == -(1) {
                    print(b"\nGame not found.\0" as *const u8 as *const libc::c_char);
                    get();
                } else {
                    read(
                        handle,
                        items as *mut _ as *mut libc::c_void,
                        ::std::mem::size_of::<[sword; 6]>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut score as *mut sdword as *mut libc::c_void,
                        ::std::mem::size_of::<sdword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut level as *mut sword as *mut libc::c_void,
                        ::std::mem::size_of::<sword>() as libc::c_ulong,
                    );
                    read(
                        handle,
                        &mut *o.as_mut_ptr().offset(0) as *mut activeobj as *mut libc::c_void,
                        ::std::mem::size_of::<activeobj>() as libc::c_ulong,
                    );
                    close(handle);
                    exitdemo = true;
                    if indemo != demoenum::notdemo {
                        playdone = true as boolean;
                    }
                    drawside(items);
                    leveldone = true as boolean;
                }
            }
        }
        66 => {
            clearkeys();
            expwin(7, 1);
            print(b"PAUSED\0" as *const u8 as *const libc::c_char);
            get();
        }
        67 => {
            clearkeys();
            expwin(12, 1);
            print(b"QUIT (Y/N)?\0" as *const u8 as *const libc::c_char);
            ch = toupper(get()) as libc::c_char;
            if ch as libc::c_int == 'Y' as i32 {
                _quit(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            }
        }
        _ => return,
    }
    clearold();
    clearkeys();
    repaintscreen(items);
}

unsafe fn dotitlepage(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    drawpic(0, 0, 14);
    UpdateScreen();
    gamestate = statetype::intitle;
    i = 0;
    while i < 300 {
        WaitVBL();
        indemo = demoenum::notdemo;
        ctrl = ControlPlayer(1);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
        {
            level = 0;
            exitdemo = true;
            break;
        } else {
            indemo = demoenum::demoplay;
            if bioskey(1) != 0 {
                dofkeys(items);
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
    print(b"Congratulation! \n\0" as *const u8 as *const libc::c_char);
    print(b"One as skilled  \n\0" as *const u8 as *const libc::c_char);
    print(b"as yourself     \n\0" as *const u8 as *const libc::c_char);
    print(b"deserves the    \n\0" as *const u8 as *const libc::c_char);
    print(b"10,000,000 gold \n\0" as *const u8 as *const libc::c_char);
    print(b"you pulled out  \n\0" as *const u8 as *const libc::c_char);
    print(b"of the palace! \0" as *const u8 as *const libc::c_char);
    clearkeys();
    get();
    drawwindow(0, 0, 17, 9);
    print(b"Let us know what\n\0" as *const u8 as *const libc::c_char);
    print(b"you enjoyed     \n\0" as *const u8 as *const libc::c_char);
    print(b"about this game,\n\0" as *const u8 as *const libc::c_char);
    print(b"so we can give  \n\0" as *const u8 as *const libc::c_char);
    print(b"you more of it. \n\0" as *const u8 as *const libc::c_char);
    print(b"Thank you for   \n\0" as *const u8 as *const libc::c_char);
    print(b"playing!\0" as *const u8 as *const libc::c_char);
    get();
}

unsafe fn dodemo(priority: &[byte], items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    while !exitdemo {
        dotitlepage(items);
        if exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1) + 1;
        LoadDemo(i);
        level = 0;
        playsetup(items);
        playloop(priority, items);
        if exitdemo {
            break;
        }
        level = 0;
        gamestate = statetype::inscores;
        indemo = demoenum::demoplay;
        _showhighscores();
        UpdateScreen();
        i = 0;
        while i < 500 {
            WaitVBL();
            indemo = demoenum::notdemo;
            ctrl = ControlPlayer(1);
            if ctrl.button1 as libc::c_int != 0
                || ctrl.button2 as libc::c_int != 0
                || keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
            {
                exitdemo = true;
                break;
            } else {
                if bioskey(1) != 0 {
                    dofkeys(items);
                }
                if exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}

unsafe fn gameover(items: &mut [sword]) {
    let mut i: libc::c_int = 0;
    expwin(11, 4);
    print(b"\n GAME OVER\n     \0" as *const u8 as *const libc::c_char);
    UpdateScreen();
    WaitEndSound();
    i = 0;
    while i < 120 {
        WaitVBL();
        i += 1;
    }
    gamestate = statetype::inscores;
    _checkhighscore();
    level = 0;
    i = 0;
    while i < 500 {
        WaitVBL();
        ctrl = ControlPlayer(1);
        if ctrl.button1 as libc::c_int != 0
            || ctrl.button2 as libc::c_int != 0
            || keydown[SDL_SCANCODE_SPACE as usize] as libc::c_int != 0
        {
            break;
        }
        if bioskey(1) != 0 {
            dofkeys(items);
        }
        if exitdemo as libc::c_int != 0 || indemo == demoenum::demoplay {
            break;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn US_CheckParm(
    mut parm: *mut libc::c_char,
    mut strings: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: libc::c_char = 0;
    let mut cs: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    while *(*__ctype_b_loc()).offset(*parm as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        parm = parm.offset(1);
    }
    i = 0;
    while !(*strings).is_null() && **strings as libc::c_int != 0 {
        let fresh1 = strings;
        strings = strings.offset(1);
        s = *fresh1;
        p = parm;
        cp = 0;
        cs = cp;
        while cs as libc::c_int == cp as libc::c_int {
            let fresh2 = s;
            s = s.offset(1);
            cs = *fresh2;
            if cs == 0 {
                return i;
            }
            let fresh3 = p;
            p = p.offset(1);
            cp = *fresh3;
            if *(*__ctype_b_loc()).offset(cs as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cs = tolower(cs as libc::c_int) as libc::c_char;
            }
            if *(*__ctype_b_loc()).offset(cp as libc::c_int as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cp = tolower(cp as libc::c_int) as libc::c_char;
            }
        }
        i += 1;
    }
    return -(1);
}

/***************************************************************************/
/***************************************************************************/

/*=========================*/
/*			   */
/* m a i n   p r o g r a m */
/*			   */
/*=========================*/

pub fn original_main() {
    /***************************************************************************/
    // Ex-globals

    let mut priority: [byte; 2048] = [0; 2048];
    let mut items: [sword; 6] = [0; 6];

    /***************************************************************************/

    let mut args: Vec<*mut libc::c_char> = Vec::new();

    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }

    unsafe {
        if args.len() > 1 && strcasecmp(args[1], b"/VER\0" as *const u8 as *const libc::c_char) == 0
        {
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
            exit(0);
        }

        initobjects();

        priority.fill(99);

        priority[blankfloor] = 0;
        for i in objdef[teleporter as usize].firstchar..=objdef[teleporter as usize].firstchar + 20
        {
            priority[i as usize] = 0;
        }
        for clvar in dead2..=dead5 {
            for i in objdef[clvar as usize].firstchar
                ..=(objdef[clvar as usize].firstchar
                    + objdef[clvar as usize].size as u16 * objdef[clvar as usize].size as u16)
            {
                priority[i as usize] = 0; /*deadthing*/
            }
        }
        for i in 152..=161 {
            priority[i] = 2; /*shots*/
        }
        for i in objdef[bigshot as usize].firstchar..=(objdef[bigshot as usize].firstchar + 31) {
            priority[i as usize] = 2; /*bigshot*/
        }
        for i in 0..=(tile2s - 1) {
            if priority[i] == 99 {
                priority[i] = 3; /*most 1*1 tiles are walls, etc*/
            }
        }
        priority[167] = 1; // chest
        for i in tile2s..=maxpics {
            if priority[i] as libc::c_int == 99 {
                priority[i] = 4; /*most bigger tiles are monsters*/
            }
        }
        for i in objdef[player as usize].firstchar..=(objdef[player as usize].firstchar + 63) {
            priority[i as usize] = 5; /*player*/
        }

        side = 0;

        for x in 0..=85 {
            for y in 0..=(topoff - 1) {
                view[x][y] = solidwall;
                view[x][(85 - y)] = solidwall;
                background[x][y] = solidwall;
                background[x][(85 - y)] = solidwall;
            }
            view[86][x] = solidwall;
        }
        for y in 11..=74 {
            for x in 0..=(leftoff - 1) {
                view[x][y] = solidwall;
                view[(85 - x)][y] = solidwall;
                background[x][y] = solidwall;
                background[(85 - x)][y] = solidwall;
            }
        }

        //   puts ("CATACOMB II is executing");

        //  _dontplay = 1;	// no sounds for debugging and profiling

        _extension = b"CA2\0" as *const u8 as *const libc::c_char;

        _setupgame(args);

        expwin(33, 13);
        print(b"  Softdisk Publishing presents\n\n\0" as *const u8 as *const libc::c_char);
        print(b"          The Catacomb\n\n\0" as *const u8 as *const libc::c_char);
        print(b"        By John Carmack\n\n\0" as *const u8 as *const libc::c_char);
        print(b"       Copyright 1990-93\n\0" as *const u8 as *const libc::c_char);
        print(b"      Softdisk Publishing\0" as *const u8 as *const libc::c_char);
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        print(b"         Press a key:\0" as *const u8 as *const libc::c_char);
        get();

        clearkeys();

        screencentery = 11;
        screencenterx = 11;

        exitdemo = false;
        level = 0;

        // go until quit () is called
        loop {
            dodemo(&priority, &mut items);
            playsetup(&mut items);
            indemo = demoenum::notdemo;
            gamestate = statetype::ingame;
            playloop(&priority, &mut items);
            if indemo == demoenum::notdemo {
                exitdemo = false;
                if level > numlevels {
                    doendpage(); // finished all levels
                }
                gameover(&mut items);
            }
        }
    }
}
