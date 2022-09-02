use ::libc;
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
    static mut priority: [byte; 2048];
    static mut view: [[libc::c_int; 86]; 87];
    static mut background: [[libc::c_int; 86]; 87];
    static mut obj: objtype;
    static mut pics: *mut libc::c_char;
    fn doinactive();
    fn doactive();
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
pub type C2RustUnnamed = libc::c_uint;
pub const lastclass: C2RustUnnamed = 23;
pub const guns: C2RustUnnamed = 22;
pub const gune: C2RustUnnamed = 21;
pub const secretgate: C2RustUnnamed = 20;
pub const torch: C2RustUnnamed = 19;
pub const teleporter: C2RustUnnamed = 18;
pub const dead6: C2RustUnnamed = 17;
pub const dead5: C2RustUnnamed = 16;
pub const dead4: C2RustUnnamed = 15;
pub const dead3: C2RustUnnamed = 14;
pub const dead2: C2RustUnnamed = 13;
pub const dead1: C2RustUnnamed = 12;
pub const rock: C2RustUnnamed = 11;
pub const bigshot: C2RustUnnamed = 10;
pub const shot: C2RustUnnamed = 9;
pub const wallhit: C2RustUnnamed = 8;
pub const turbogre: C2RustUnnamed = 7;
pub const dragon: C2RustUnnamed = 6;
pub const gargoyle: C2RustUnnamed = 5;
pub const ogre: C2RustUnnamed = 4;
pub const skeleton: C2RustUnnamed = 3;
pub const goblin: C2RustUnnamed = 2;
pub const player: C2RustUnnamed = 1;
pub const nothing: C2RustUnnamed = 0;
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
    return ((*chan.offset(3 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
        & 1 as libc::c_int)
        << 3 as libc::c_int
        | (*chan.offset(2 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int)
            << 2 as libc::c_int
        | (*chan.offset(1 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int)
            << 1 as libc::c_int
        | *chan.offset(0 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int) as byte;
}
#[no_mangle]
pub static mut squares: [byte; 9] = [
    0 as libc::c_int as byte,
    1 as libc::c_int as byte,
    4 as libc::c_int as byte,
    9 as libc::c_int as byte,
    16 as libc::c_int as byte,
    25 as libc::c_int as byte,
    36 as libc::c_int as byte,
    49 as libc::c_int as byte,
    64 as libc::c_int as byte,
];
#[no_mangle]
pub static mut table86: [word; 87] = [
    0 as libc::c_int as word,
    86 as libc::c_int as word,
    172 as libc::c_int as word,
    258 as libc::c_int as word,
    344 as libc::c_int as word,
    430 as libc::c_int as word,
    516 as libc::c_int as word,
    602 as libc::c_int as word,
    688 as libc::c_int as word,
    774 as libc::c_int as word,
    860 as libc::c_int as word,
    946 as libc::c_int as word,
    1032 as libc::c_int as word,
    1118 as libc::c_int as word,
    1204 as libc::c_int as word,
    1290 as libc::c_int as word,
    1376 as libc::c_int as word,
    1462 as libc::c_int as word,
    1548 as libc::c_int as word,
    1634 as libc::c_int as word,
    1720 as libc::c_int as word,
    1806 as libc::c_int as word,
    1892 as libc::c_int as word,
    1978 as libc::c_int as word,
    2064 as libc::c_int as word,
    2150 as libc::c_int as word,
    2236 as libc::c_int as word,
    2322 as libc::c_int as word,
    2408 as libc::c_int as word,
    2494 as libc::c_int as word,
    2580 as libc::c_int as word,
    2666 as libc::c_int as word,
    2752 as libc::c_int as word,
    2838 as libc::c_int as word,
    2924 as libc::c_int as word,
    3010 as libc::c_int as word,
    3096 as libc::c_int as word,
    3182 as libc::c_int as word,
    3268 as libc::c_int as word,
    3354 as libc::c_int as word,
    3440 as libc::c_int as word,
    3526 as libc::c_int as word,
    3612 as libc::c_int as word,
    3698 as libc::c_int as word,
    3784 as libc::c_int as word,
    3870 as libc::c_int as word,
    3956 as libc::c_int as word,
    4042 as libc::c_int as word,
    4128 as libc::c_int as word,
    4214 as libc::c_int as word,
    4300 as libc::c_int as word,
    4386 as libc::c_int as word,
    4472 as libc::c_int as word,
    4558 as libc::c_int as word,
    4644 as libc::c_int as word,
    4730 as libc::c_int as word,
    4816 as libc::c_int as word,
    4902 as libc::c_int as word,
    4988 as libc::c_int as word,
    5074 as libc::c_int as word,
    5160 as libc::c_int as word,
    5246 as libc::c_int as word,
    5332 as libc::c_int as word,
    5418 as libc::c_int as word,
    5504 as libc::c_int as word,
    5590 as libc::c_int as word,
    5676 as libc::c_int as word,
    5762 as libc::c_int as word,
    5848 as libc::c_int as word,
    5934 as libc::c_int as word,
    6020 as libc::c_int as word,
    6106 as libc::c_int as word,
    6192 as libc::c_int as word,
    6278 as libc::c_int as word,
    6364 as libc::c_int as word,
    6450 as libc::c_int as word,
    6536 as libc::c_int as word,
    6622 as libc::c_int as word,
    6708 as libc::c_int as word,
    6794 as libc::c_int as word,
    6880 as libc::c_int as word,
    6966 as libc::c_int as word,
    7052 as libc::c_int as word,
    7138 as libc::c_int as word,
    7224 as libc::c_int as word,
    7310 as libc::c_int as word,
    7396 as libc::c_int as word,
];
#[no_mangle]
pub unsafe extern "C" fn drawobj() {
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
        if !(fresh0 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        x = obj.size as libc::c_uint;
        loop {
            let fresh1 = x;
            x = x.wrapping_sub(1);
            if !(fresh1 > 0 as libc::c_int as libc::c_uint) {
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
        ofs = ofs.wrapping_add((86 as libc::c_int - obj.size as libc::c_int) as libc::c_uint);
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
        if !(fresh2 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        x = obj.size as libc::c_uint;
        loop {
            let fresh3 = x;
            x = x.wrapping_sub(1);
            if !(fresh3 > 0 as libc::c_int as libc::c_uint) {
                break;
            }
            if *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) == tilenum {
                *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize) =
                    *(background.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
            }
            tilenum += 1;
            ofs = ofs.wrapping_add(1);
        }
        ofs = ofs.wrapping_add((86 as libc::c_int - obj.size as libc::c_int) as libc::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn doall() {
    if numobj > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"numobj > 0\0" as *const u8 as *const libc::c_char,
            b"/home/saverio/code/catacomb_ii-64k/source_project/catasm.c\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void doall()\0")).as_ptr(),
        );
    }
    loop {
        objecton = numobj;
        loop {
            memcpy(
                &mut obj as *mut objtype as *mut libc::c_void,
                &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj
                    as *const libc::c_void,
                ::core::mem::size_of::<activeobj>() as libc::c_ulong,
            );
            if obj.class as libc::c_int != nothing as libc::c_int {
                memcpy(
                    &mut obj.think as *mut byte as *mut libc::c_void,
                    &mut *objdef.as_mut_ptr().offset(obj.class as isize) as *mut objdeftype
                        as *const libc::c_void,
                    ::core::mem::size_of::<objdeftype>() as libc::c_ulong,
                );
                if obj.active != 0 {
                    doactive();
                } else {
                    doinactive();
                }
            }
            if leveldone as libc::c_int != 0 || playdone as libc::c_int != 0 {
                return;
            }
            objecton -= 1;
            if !(objecton >= 0 as libc::c_int) {
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
    let mut src: *mut byte = (pics as *mut byte).offset((tile << 4 as libc::c_int) as isize);
    let mut r: libc::c_uint = 0;
    r = 0 as libc::c_int as libc::c_uint;
    while r < 8 as libc::c_int as libc::c_uint {
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = (*src.offset(0 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = (*src.offset(0 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = (*src.offset(0 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = (*src.offset(0 as libc::c_int as isize) as libc::c_int >> 0 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh8 = dest;
        dest = dest.offset(1);
        *fresh8 = (*src.offset(1 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = (*src.offset(1 as libc::c_int as isize) as libc::c_int >> 4 as libc::c_int
            & 3 as libc::c_int) as byte;
        let fresh10 = dest;
        dest = dest.offset(1);
        *fresh10 = (*src.offset(1 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int
            & 3 as libc::c_int) as byte;
        *dest = (*src.offset(1 as libc::c_int as isize) as libc::c_int >> 0 as libc::c_int
            & 3 as libc::c_int) as byte;
        dest = dest.offset((screenpitch as libc::c_int - 7 as libc::c_int) as isize);
        r = r.wrapping_add(1);
        src = src.offset(2 as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cgarefresh() {
    let mut ofs: libc::c_uint = (originy * 86 as libc::c_int + originx) as libc::c_uint;
    let mut tile: libc::c_int = 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut endofrow: libc::c_uint = ofs.wrapping_add(24 as libc::c_int as libc::c_uint);
    let mut vbuf: *mut byte = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
        if tile != oldtiles[i as usize] {
            oldtiles[i as usize] = tile;
            drawcgachartile(vbuf, tile);
        }
        i = i.wrapping_add(1);
        ofs = ofs.wrapping_add(1);
        vbuf = vbuf.offset(8 as libc::c_int as isize);
        if !(ofs == endofrow) {
            continue;
        }
        if i == (24 as libc::c_int * 24 as libc::c_int) as libc::c_uint {
            break;
        }
        ofs = ofs.wrapping_add((86 as libc::c_int - 24 as libc::c_int) as libc::c_uint);
        endofrow = endofrow.wrapping_add(86 as libc::c_int as libc::c_uint);
        vbuf = vbuf.offset(
            (screenpitch as libc::c_int * 8 as libc::c_int - 24 as libc::c_int * 8 as libc::c_int)
                as isize,
        );
    }
    UpdateScreen();
}
unsafe extern "C" fn drawegachartile(mut dest: *mut byte, mut tile: libc::c_int) {
    let mut src: *mut byte = (pics as *mut byte).offset((tile << 5 as libc::c_int) as isize);
    let mut r: libc::c_uint = 0;
    r = 0 as libc::c_int as libc::c_uint;
    while r < 8 as libc::c_int as libc::c_uint {
        let chan: [byte; 4] = [
            *src.offset(0 as libc::c_int as isize),
            *src.offset(8 as libc::c_int as isize),
            *src.offset(16 as libc::c_int as isize),
            *src.offset(24 as libc::c_int as isize),
        ];
        let fresh11 = dest;
        dest = dest.offset(1);
        *fresh11 = EGA(chan.as_ptr(), 7 as libc::c_int as byte);
        let fresh12 = dest;
        dest = dest.offset(1);
        *fresh12 = EGA(chan.as_ptr(), 6 as libc::c_int as byte);
        let fresh13 = dest;
        dest = dest.offset(1);
        *fresh13 = EGA(chan.as_ptr(), 5 as libc::c_int as byte);
        let fresh14 = dest;
        dest = dest.offset(1);
        *fresh14 = EGA(chan.as_ptr(), 4 as libc::c_int as byte);
        let fresh15 = dest;
        dest = dest.offset(1);
        *fresh15 = EGA(chan.as_ptr(), 3 as libc::c_int as byte);
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = EGA(chan.as_ptr(), 2 as libc::c_int as byte);
        let fresh17 = dest;
        dest = dest.offset(1);
        *fresh17 = EGA(chan.as_ptr(), 1 as libc::c_int as byte);
        *dest = EGA(chan.as_ptr(), 0 as libc::c_int as byte);
        dest = dest.offset((screenpitch as libc::c_int - 7 as libc::c_int) as isize);
        r = r.wrapping_add(1);
        src = src.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn egarefresh() {
    let mut ofs: libc::c_uint = (originy * 86 as libc::c_int + originx) as libc::c_uint;
    let mut tile: libc::c_int = 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut endofrow: libc::c_uint = ofs.wrapping_add(24 as libc::c_int as libc::c_uint);
    let mut vbuf: *mut byte = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut libc::c_int).offset(ofs as isize);
        if tile != oldtiles[i as usize] {
            oldtiles[i as usize] = tile;
            drawegachartile(vbuf, tile);
        }
        i = i.wrapping_add(1);
        ofs = ofs.wrapping_add(1);
        vbuf = vbuf.offset(8 as libc::c_int as isize);
        if !(ofs == endofrow) {
            continue;
        }
        if i == (24 as libc::c_int * 24 as libc::c_int) as libc::c_uint {
            break;
        }
        ofs = ofs.wrapping_add((86 as libc::c_int - 24 as libc::c_int) as libc::c_uint);
        endofrow = endofrow.wrapping_add(86 as libc::c_int as libc::c_uint);
        vbuf = vbuf.offset(
            (screenpitch as libc::c_int * 8 as libc::c_int - 24 as libc::c_int * 8 as libc::c_int)
                as isize,
        );
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
                    .offset(((y << 3 as libc::c_int) * screenpitch as libc::c_int) as isize)
                    .offset((x << 3 as libc::c_int) as isize),
                tile,
            );
        }
        2 | _ => {
            drawegachartile(
                screenseg
                    .as_mut_ptr()
                    .offset(((y << 3 as libc::c_int) * screenpitch as libc::c_int) as isize)
                    .offset((x << 3 as libc::c_int) as isize),
                tile,
            );
        }
    };
}
