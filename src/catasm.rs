use ::libc;

use crate::{cat_play::doactive, class_type::classtype::*};
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut originy: libc::c_int;
    static mut originx: libc::c_int;
    static mut oldtiles: [libc::c_int; 576];
    static mut numobj: libc::c_int;
    static mut o: [activeobj; 201];
    static mut objdef: [objdeftype; 23];
    static mut objecton: libc::c_int;
    static mut frameon: word;
    static mut leveldone: boolean;
    static mut playdone: boolean;
    static mut view: [[libc::c_int; 86]; 87];
    static mut background: [[libc::c_int; 86]; 87];
    static mut obj: objtype;
    static mut pics: *mut libc::c_char;
    fn doinactive();
    fn refresh();
    static mut grmode: grtype;
    fn UpdateScreen();
    static mut screenseg: [byte; 64000];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type sbyte = int8_t;
pub type word = uint16_t;
pub type sword = int16_t;

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
pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const screenpitch: C2RustUnnamed_0 = 320;
#[inline]
unsafe extern "C" fn EGA(mut chan: *const byte, mut ofs: byte) -> byte {
    return ((*chan.offset(3) as libc::c_int >> ofs as libc::c_int & 1) << 3
        | (*chan.offset(2) as libc::c_int >> ofs as libc::c_int & 1) << 2
        | (*chan.offset(1) as libc::c_int >> ofs as libc::c_int & 1) << 1
        | *chan.offset(0) as libc::c_int >> ofs as libc::c_int & 1) as byte;
}
#[no_mangle]
pub static mut squares: [byte; 9] = [0, 1, 4, 9, 16, 25, 36, 49, 64];
#[no_mangle]
pub static mut table86: [word; 87] = [
    0, 86, 172, 258, 344, 430, 516, 602, 688, 774, 860, 946, 1032, 1118, 1204, 1290, 1376, 1462,
    1548, 1634, 1720, 1806, 1892, 1978, 2064, 2150, 2236, 2322, 2408, 2494, 2580, 2666, 2752, 2838,
    2924, 3010, 3096, 3182, 3268, 3354, 3440, 3526, 3612, 3698, 3784, 3870, 3956, 4042, 4128, 4214,
    4300, 4386, 4472, 4558, 4644, 4730, 4816, 4902, 4988, 5074, 5160, 5246, 5332, 5418, 5504, 5590,
    5676, 5762, 5848, 5934, 6020, 6106, 6192, 6278, 6364, 6450, 6536, 6622, 6708, 6794, 6880, 6966,
    7052, 7138, 7224, 7310, 7396,
];

pub unsafe fn drawobj(priority: &[byte]) {
    let mut tilenum: libc::c_int = obj.firstchar as libc::c_int
        + squares[obj.size as usize] as libc::c_int
            * ((obj.dir as libc::c_int & obj.dirmask as libc::c_int) * obj.stages as libc::c_int
                + obj.stage as libc::c_int);
    obj.oldtile = tilenum as sword;
    obj.oldy = obj.y;
    obj.oldx = obj.x;
    let objpri: byte = priority[tilenum as usize];
    let mut ofs: libc::c_uint =
        (table86[obj.oldy as usize] as libc::c_int + obj.oldx as libc::c_int) as libc::c_uint;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    y = obj.size as libc::c_uint;
    loop {
        let fresh0 = y;
        y = y.wrapping_sub(1);
        if !(fresh0 > 0) {
            break;
        }
        x = obj.size as libc::c_uint;
        loop {
            let fresh1 = x;
            x = x.wrapping_sub(1);
            if !(fresh1 > 0) {
                break;
            }
            if priority[*(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) as usize]
                as libc::c_int
                <= objpri as libc::c_int
            {
                *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) = tilenum;
            }
            tilenum += 1;
            ofs = ofs.wrapping_add(1);
        }
        ofs = ofs.wrapping_add((86 - obj.size as libc::c_int) as libc::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn eraseobj() {
    let mut tilenum: libc::c_int = obj.oldtile as libc::c_int;
    let mut ofs: libc::c_uint =
        (table86[obj.oldy as usize] as libc::c_int + obj.oldx as libc::c_int) as libc::c_uint;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    y = obj.size as libc::c_uint;
    loop {
        let fresh2 = y;
        y = y.wrapping_sub(1);
        if !(fresh2 > 0) {
            break;
        }
        x = obj.size as libc::c_uint;
        loop {
            let fresh3 = x;
            x = x.wrapping_sub(1);
            if !(fresh3 > 0) {
                break;
            }
            if *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) == tilenum {
                *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) =
                    *(background.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
            }
            tilenum += 1;
            ofs = ofs.wrapping_add(1);
        }
        ofs = ofs.wrapping_add((86 - obj.size as libc::c_int) as libc::c_uint);
    }
}

pub unsafe fn doall(priority: &[byte], items: &mut [sword]) {
    assert!(numobj > 0);

    loop {
        objecton = numobj;
        loop {
            memcpy(
                &mut obj as *mut objtype as *mut libc::c_void,
                &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj
                    as *const libc::c_void,
                ::std::mem::size_of::<activeobj>() as libc::c_ulong,
            );
            if obj.class as libc::c_int != nothing as libc::c_int {
                memcpy(
                    &mut obj.think as *mut byte as *mut libc::c_void,
                    &mut *objdef.as_mut_ptr().offset(obj.class as isize) as *mut objdeftype
                        as *const libc::c_void,
                    ::std::mem::size_of::<objdeftype>() as libc::c_ulong,
                );
                if obj.active != 0 {
                    doactive(priority, items);
                } else {
                    doinactive();
                }
            }
            if leveldone as libc::c_int != 0 || playdone as libc::c_int != 0 {
                return;
            }
            objecton -= 1;
            if !(objecton >= 0) {
                break;
            }
        }
        refresh();
        frameon = frameon.wrapping_add(1);
        if leveldone != 0 {
            return;
        }
        if !(playdone == 0) {
            break;
        }
    }
}
unsafe extern "C" fn drawcgachartile(mut dest: *mut byte, mut tile: libc::c_int) {
    let mut src: *mut byte = (pics as *mut byte).offset((tile << 4) as isize);
    let mut r: libc::c_uint = 0;
    r = 0;
    while r < 8 {
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = (*src.offset(0) as libc::c_int >> 6 & 3) as byte;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = (*src.offset(0) as libc::c_int >> 4 & 3) as byte;
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = (*src.offset(0) as libc::c_int >> 2 & 3) as byte;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = (*src.offset(0) as libc::c_int >> 0 & 3) as byte;
        let fresh8 = dest;
        dest = dest.offset(1);
        *fresh8 = (*src.offset(1) as libc::c_int >> 6 & 3) as byte;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = (*src.offset(1) as libc::c_int >> 4 & 3) as byte;
        let fresh10 = dest;
        dest = dest.offset(1);
        *fresh10 = (*src.offset(1) as libc::c_int >> 2 & 3) as byte;
        *dest = (*src.offset(1) as libc::c_int >> 0 & 3) as byte;
        dest = dest.offset((screenpitch as libc::c_int - 7) as isize);
        r = r.wrapping_add(1);
        src = src.offset(2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cgarefresh() {
    let mut ofs: libc::c_uint = (originy * 86 + originx) as libc::c_uint;
    let mut tile: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut endofrow: libc::c_uint = ofs.wrapping_add(24);
    let mut vbuf: *mut byte = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
        if tile != oldtiles[i as usize] {
            oldtiles[i as usize] = tile;
            drawcgachartile(vbuf, tile);
        }
        i = i.wrapping_add(1);
        ofs = ofs.wrapping_add(1);
        vbuf = vbuf.offset(8);
        if !(ofs == endofrow) {
            continue;
        }
        if i == (24 * 24) as libc::c_uint {
            break;
        }
        ofs = ofs.wrapping_add((86 - 24) as libc::c_uint);
        endofrow = endofrow.wrapping_add(86);
        vbuf = vbuf.offset((screenpitch as libc::c_int * 8 - 24 * 8) as isize);
    }
    UpdateScreen();
}
unsafe extern "C" fn drawegachartile(mut dest: *mut byte, mut tile: libc::c_int) {
    let mut src: *mut byte = (pics as *mut byte).offset((tile << 5) as isize);
    let mut r: libc::c_uint = 0;
    r = 0;
    while r < 8 {
        let chan: [byte; 4] = [
            *src.offset(0),
            *src.offset(8),
            *src.offset(16),
            *src.offset(24),
        ];
        let fresh11 = dest;
        dest = dest.offset(1);
        *fresh11 = EGA(chan.as_ptr(), 7);
        let fresh12 = dest;
        dest = dest.offset(1);
        *fresh12 = EGA(chan.as_ptr(), 6);
        let fresh13 = dest;
        dest = dest.offset(1);
        *fresh13 = EGA(chan.as_ptr(), 5);
        let fresh14 = dest;
        dest = dest.offset(1);
        *fresh14 = EGA(chan.as_ptr(), 4);
        let fresh15 = dest;
        dest = dest.offset(1);
        *fresh15 = EGA(chan.as_ptr(), 3);
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = EGA(chan.as_ptr(), 2);
        let fresh17 = dest;
        dest = dest.offset(1);
        *fresh17 = EGA(chan.as_ptr(), 1);
        *dest = EGA(chan.as_ptr(), 0);
        dest = dest.offset((screenpitch as libc::c_int - 7) as isize);
        r = r.wrapping_add(1);
        src = src.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn egarefresh() {
    let mut ofs: libc::c_uint = (originy * 86 + originx) as libc::c_uint;
    let mut tile: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut endofrow: libc::c_uint = ofs.wrapping_add(24);
    let mut vbuf: *mut byte = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
        if tile != oldtiles[i as usize] {
            oldtiles[i as usize] = tile;
            drawegachartile(vbuf, tile);
        }
        i = i.wrapping_add(1);
        ofs = ofs.wrapping_add(1);
        vbuf = vbuf.offset(8);
        if !(ofs == endofrow) {
            continue;
        }
        if i == (24 * 24) as libc::c_uint {
            break;
        }
        ofs = ofs.wrapping_add((86 - 24) as libc::c_uint);
        endofrow = endofrow.wrapping_add(86);
        vbuf = vbuf.offset((screenpitch as libc::c_int * 8 - 24 * 8) as isize);
    }
    UpdateScreen();
}
#[no_mangle]
pub unsafe extern "C" fn drawchartile(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut tile: libc::c_int,
) {
    match grmode as libc::c_uint {
        1 => {
            drawcgachartile(
                screenseg
                    .as_mut_ptr()
                    .offset(((y << 3) * screenpitch as libc::c_int) as isize)
                    .offset((x << 3) as isize),
                tile,
            );
        }
        2 | _ => {
            drawegachartile(
                screenseg
                    .as_mut_ptr()
                    .offset(((y << 3) * screenpitch as libc::c_int) as isize)
                    .offset((x << 3) as isize),
                tile,
            );
        }
    };
}
