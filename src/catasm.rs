use ::libc;

use crate::{
    active_obj::activeobj, cat_play::doactive, catacomb::refresh, class_type::classtype::*,
    extra_types::boolean, gr_type::grtype, obj_def_type::objdeftype, obj_type::objtype,
};
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut originy: i32;
    static mut originx: i32;
    static mut oldtiles: [i32; 576];
    static mut numobj: i32;
    static mut o: [activeobj; 201];
    static mut objecton: i32;
    static mut frameon: u16;
    static mut leveldone: boolean;
    static mut playdone: boolean;
    static mut background: [[i32; 86]; 87];
    static mut obj: objtype;
    static mut pics: *mut i8;
    fn doinactive();
    static mut grmode: grtype;
    fn UpdateScreen();
    static mut screenseg: [u8; 64000];
}

pub type C2RustUnnamed_0 = u32;
pub const screenpitch: C2RustUnnamed_0 = 320;
#[inline]
unsafe extern "C" fn EGA(mut chan: *const u8, mut ofs: u8) -> u8 {
    return ((*chan.offset(3) as i32 >> ofs as i32 & 1) << 3
        | (*chan.offset(2) as i32 >> ofs as i32 & 1) << 2
        | (*chan.offset(1) as i32 >> ofs as i32 & 1) << 1
        | *chan.offset(0) as i32 >> ofs as i32 & 1) as u8;
}

const squares: [u8; 9] = [0, 1, 4, 9, 16, 25, 36, 49, 64];

const table86: [u16; 87] = [
    0, 86, 172, 258, 344, 430, 516, 602, 688, 774, 860, 946, 1032, 1118, 1204, 1290, 1376, 1462,
    1548, 1634, 1720, 1806, 1892, 1978, 2064, 2150, 2236, 2322, 2408, 2494, 2580, 2666, 2752, 2838,
    2924, 3010, 3096, 3182, 3268, 3354, 3440, 3526, 3612, 3698, 3784, 3870, 3956, 4042, 4128, 4214,
    4300, 4386, 4472, 4558, 4644, 4730, 4816, 4902, 4988, 5074, 5160, 5246, 5332, 5418, 5504, 5590,
    5676, 5762, 5848, 5934, 6020, 6106, 6192, 6278, 6364, 6450, 6536, 6622, 6708, 6794, 6880, 6966,
    7052, 7138, 7224, 7310, 7396,
];

pub unsafe fn drawobj(priority: &[u8], view: &mut [[i32; 86]]) {
    let mut tilenum: i32 = obj.firstchar as i32
        + squares[obj.size as usize] as i32
            * ((obj.dir as i32 & obj.dirmask as i32) * obj.stages as i32 + obj.stage as i32);
    obj.oldtile = tilenum as i16;
    obj.oldy = obj.y;
    obj.oldx = obj.x;
    let objpri: u8 = priority[tilenum as usize];
    let mut ofs: u32 = (table86[obj.oldy as usize] as i32 + obj.oldx as i32) as u32;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    y = obj.size as u32;
    loop {
        let fresh0 = y;
        y = y.wrapping_sub(1);
        if !(fresh0 > 0) {
            break;
        }
        x = obj.size as u32;
        loop {
            let fresh1 = x;
            x = x.wrapping_sub(1);
            if !(fresh1 > 0) {
                break;
            }
            if priority[*(view.as_mut_ptr() as *mut i32).offset(ofs as isize) as usize] as i32
                <= objpri as i32
            {
                *(view.as_mut_ptr() as *mut i32).offset(ofs as isize) = tilenum;
            }
            tilenum += 1;
            ofs = ofs.wrapping_add(1);
        }
        ofs = ofs.wrapping_add((86 - obj.size as i32) as u32);
    }
}

pub unsafe fn eraseobj(view: &mut [[i32; 86]]) {
    let mut tilenum: i32 = obj.oldtile as i32;
    let mut ofs: u32 = (table86[obj.oldy as usize] as i32 + obj.oldx as i32) as u32;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    y = obj.size as u32;
    loop {
        let fresh2 = y;
        y = y.wrapping_sub(1);
        if !(fresh2 > 0) {
            break;
        }
        x = obj.size as u32;
        loop {
            let fresh3 = x;
            x = x.wrapping_sub(1);
            if !(fresh3 > 0) {
                break;
            }
            if *(view.as_mut_ptr() as *mut i32).offset(ofs as isize) == tilenum {
                *(view.as_mut_ptr() as *mut i32).offset(ofs as isize) =
                    *(background.as_mut_ptr() as *mut i32).offset(ofs as isize);
            }
            tilenum += 1;
            ofs = ofs.wrapping_add(1);
        }
        ofs = ofs.wrapping_add((86 - obj.size as i32) as u32);
    }
}

pub unsafe fn doall(
    priority: &[u8],
    items: &mut [i16],
    objdef: &mut [objdeftype],
    side: &mut i32,
    view: &mut [[i32; 86]],
    screencenterx: &mut i32,
    screencentery: &mut i32,
) {
    assert!(numobj > 0);

    loop {
        objecton = numobj;
        loop {
            memcpy(
                &mut obj as *mut objtype as *mut libc::c_void,
                &mut *o.as_mut_ptr().offset(objecton as isize) as *mut activeobj
                    as *const libc::c_void,
                ::std::mem::size_of::<activeobj>() as u64,
            );
            if obj.class as i32 != nothing as i32 {
                memcpy(
                    &mut obj.think as *mut u8 as *mut libc::c_void,
                    &mut *objdef.as_mut_ptr().offset(obj.class as isize) as *mut objdeftype
                        as *const libc::c_void,
                    ::std::mem::size_of::<objdeftype>() as u64,
                );
                if obj.active != 0 {
                    doactive(
                        priority,
                        items,
                        objdef,
                        side,
                        view,
                        screencenterx,
                        screencentery,
                    );
                } else {
                    doinactive();
                }
            }
            if leveldone as i32 != 0 || playdone as i32 != 0 {
                return;
            }
            objecton -= 1;
            if !(objecton >= 0) {
                break;
            }
        }
        refresh(view);
        frameon = frameon.wrapping_add(1);
        if leveldone != 0 {
            return;
        }
        if !(playdone == 0) {
            break;
        }
    }
}
unsafe extern "C" fn drawcgachartile(mut dest: *mut u8, mut tile: i32) {
    let mut src: *mut u8 = (pics as *mut u8).offset((tile << 4) as isize);
    let mut r: u32 = 0;
    r = 0;
    while r < 8 {
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = (*src.offset(0) as i32 >> 6 & 3) as u8;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = (*src.offset(0) as i32 >> 4 & 3) as u8;
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = (*src.offset(0) as i32 >> 2 & 3) as u8;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = (*src.offset(0) as i32 >> 0 & 3) as u8;
        let fresh8 = dest;
        dest = dest.offset(1);
        *fresh8 = (*src.offset(1) as i32 >> 6 & 3) as u8;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = (*src.offset(1) as i32 >> 4 & 3) as u8;
        let fresh10 = dest;
        dest = dest.offset(1);
        *fresh10 = (*src.offset(1) as i32 >> 2 & 3) as u8;
        *dest = (*src.offset(1) as i32 >> 0 & 3) as u8;
        dest = dest.offset((screenpitch as i32 - 7) as isize);
        r = r.wrapping_add(1);
        src = src.offset(2);
    }
}

pub unsafe fn cgarefresh(view: &mut [[i32; 86]]) {
    let mut ofs: u32 = (originy * 86 + originx) as u32;
    let mut tile: i32 = 0;
    let mut i: u32 = 0;
    let mut endofrow: u32 = ofs.wrapping_add(24);
    let mut vbuf: *mut u8 = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut i32).offset(ofs as isize);
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
        if i == (24 * 24) as u32 {
            break;
        }
        ofs = ofs.wrapping_add((86 - 24) as u32);
        endofrow = endofrow.wrapping_add(86);
        vbuf = vbuf.offset((screenpitch as i32 * 8 - 24 * 8) as isize);
    }
    UpdateScreen();
}
unsafe extern "C" fn drawegachartile(mut dest: *mut u8, mut tile: i32) {
    let mut src: *mut u8 = (pics as *mut u8).offset((tile << 5) as isize);
    let mut r: u32 = 0;
    r = 0;
    while r < 8 {
        let chan: [u8; 4] = [
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
        dest = dest.offset((screenpitch as i32 - 7) as isize);
        r = r.wrapping_add(1);
        src = src.offset(1);
    }
}

pub unsafe fn egarefresh(view: &mut [[i32; 86]]) {
    let mut ofs: u32 = (originy * 86 + originx) as u32;
    let mut tile: i32 = 0;
    let mut i: u32 = 0;
    let mut endofrow: u32 = ofs.wrapping_add(24);
    let mut vbuf: *mut u8 = screenseg.as_mut_ptr();
    loop {
        tile = *(view.as_mut_ptr() as *mut i32).offset(ofs as isize);
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
        if i == (24 * 24) as u32 {
            break;
        }
        ofs = ofs.wrapping_add((86 - 24) as u32);
        endofrow = endofrow.wrapping_add(86);
        vbuf = vbuf.offset((screenpitch as i32 * 8 - 24 * 8) as isize);
    }
    UpdateScreen();
}
#[no_mangle]
pub unsafe extern "C" fn drawchartile(mut x: i32, mut y: i32, mut tile: i32) {
    match grmode as u32 {
        1 => {
            drawcgachartile(
                screenseg
                    .as_mut_ptr()
                    .offset(((y << 3) * screenpitch as i32) as isize)
                    .offset((x << 3) as isize),
                tile,
            );
        }
        2 | _ => {
            drawegachartile(
                screenseg
                    .as_mut_ptr()
                    .offset(((y << 3) * screenpitch as i32) as isize)
                    .offset((x << 3) as isize),
                tile,
            );
        }
    };
}
