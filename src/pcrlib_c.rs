use ::libc;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/unistd.h:26"]
pub mod unistd_h {
    #[c2rust::src_loc = "220:1"]
    pub type ssize_t = __ssize_t;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.6/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h:28"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stat.h:28"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h:29"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:33"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:33"]
pub mod catdefs_h {
    #[c2rust::src_loc = "34:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "34:13"]
    pub const true_0: C2RustUnnamed = 1;
    #[c2rust::src_loc = "34:7"]
    pub const false_0: C2RustUnnamed = 0;
    #[c2rust::src_loc = "35:1"]
    pub type boolean = uint16_t;
    #[c2rust::src_loc = "37:1"]
    pub type byte = uint8_t;
    #[c2rust::src_loc = "39:1"]
    pub type word = uint16_t;
    #[c2rust::src_loc = "40:1"]
    pub type sword = int16_t;
    #[c2rust::src_loc = "41:1"]
    pub type dword = uint32_t;
    #[c2rust::src_loc = "42:1"]
    pub type sdword = int32_t;
    #[c2rust::src_loc = "80:9"]
    pub type dirtype = libc::c_uint;
    #[c2rust::src_loc = "81:18"]
    pub const nodir: dirtype = 8;
    #[c2rust::src_loc = "81:8"]
    pub const northwest: dirtype = 7;
    #[c2rust::src_loc = "80:57"]
    pub const southwest: dirtype = 6;
    #[c2rust::src_loc = "80:47"]
    pub const southeast: dirtype = 5;
    #[c2rust::src_loc = "80:37"]
    pub const northeast: dirtype = 4;
    #[c2rust::src_loc = "80:32"]
    pub const west: dirtype = 3;
    #[c2rust::src_loc = "80:26"]
    pub const south: dirtype = 2;
    #[c2rust::src_loc = "80:21"]
    pub const east: dirtype = 1;
    #[c2rust::src_loc = "80:15"]
    pub const north: dirtype = 0;
    #[inline]
    #[c2rust::src_loc = "53:1"]
    pub unsafe extern "C" fn itoa(
        mut value: libc::c_int,
        mut str_0: *mut libc::c_char,
        mut base: libc::c_int,
    ) -> *mut libc::c_char {
        if base == 16 as libc::c_int {
            sprintf(str_0, b"%X\0" as *const u8 as *const libc::c_char, value);
        } else {
            sprintf(str_0, b"%d\0" as *const u8 as *const libc::c_char, value);
        }
        return str_0;
    }
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn ltoa(
        mut value: libc::c_int,
        mut str_0: *mut libc::c_char,
        mut base: libc::c_int,
    ) -> *mut libc::c_char {
        return itoa(value, str_0, base);
    }
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
    use super::stdio_h::sprintf;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_stdinc.h:34"]
pub mod SDL_stdinc_h {
    #[c2rust::src_loc = "161:9"]
    pub type SDL_bool = libc::c_uint;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_TRUE: SDL_bool = 1;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_FALSE: SDL_bool = 0;
    #[c2rust::src_loc = "179:1"]
    pub type Uint8 = uint8_t;
    #[c2rust::src_loc = "185:1"]
    pub type Sint16 = int16_t;
    #[c2rust::src_loc = "191:1"]
    pub type Uint16 = uint16_t;
    #[c2rust::src_loc = "197:1"]
    pub type Sint32 = int32_t;
    #[c2rust::src_loc = "203:1"]
    pub type Uint32 = uint32_t;
    #[c2rust::src_loc = "210:1"]
    pub type Sint64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_pixels.h:34"]
pub mod SDL_pixels_h {
    #[c2rust::src_loc = "171:9"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "291:5"]
    pub const SDL_PIXELFORMAT_EXTERNAL_OES: C2RustUnnamed_0 = 542328143;
    #[c2rust::src_loc = "289:5"]
    pub const SDL_PIXELFORMAT_NV21: C2RustUnnamed_0 = 825382478;
    #[c2rust::src_loc = "287:5"]
    pub const SDL_PIXELFORMAT_NV12: C2RustUnnamed_0 = 842094158;
    #[c2rust::src_loc = "285:5"]
    pub const SDL_PIXELFORMAT_YVYU: C2RustUnnamed_0 = 1431918169;
    #[c2rust::src_loc = "283:5"]
    pub const SDL_PIXELFORMAT_UYVY: C2RustUnnamed_0 = 1498831189;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_PIXELFORMAT_YUY2: C2RustUnnamed_0 = 844715353;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_PIXELFORMAT_IYUV: C2RustUnnamed_0 = 1448433993;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_PIXELFORMAT_YV12: C2RustUnnamed_0 = 842094169;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_PIXELFORMAT_ABGR32: C2RustUnnamed_0 = 373694468;
    #[c2rust::src_loc = "273:5"]
    pub const SDL_PIXELFORMAT_BGRA32: C2RustUnnamed_0 = 372645892;
    #[c2rust::src_loc = "272:5"]
    pub const SDL_PIXELFORMAT_ARGB32: C2RustUnnamed_0 = 377888772;
    #[c2rust::src_loc = "271:5"]
    pub const SDL_PIXELFORMAT_RGBA32: C2RustUnnamed_0 = 376840196;
    #[c2rust::src_loc = "260:5"]
    pub const SDL_PIXELFORMAT_ARGB2101010: C2RustUnnamed_0 = 372711428;
    #[c2rust::src_loc = "257:5"]
    pub const SDL_PIXELFORMAT_BGRA8888: C2RustUnnamed_0 = 377888772;
    #[c2rust::src_loc = "254:5"]
    pub const SDL_PIXELFORMAT_ABGR8888: C2RustUnnamed_0 = 376840196;
    #[c2rust::src_loc = "251:5"]
    pub const SDL_PIXELFORMAT_RGBA8888: C2RustUnnamed_0 = 373694468;
    #[c2rust::src_loc = "248:5"]
    pub const SDL_PIXELFORMAT_ARGB8888: C2RustUnnamed_0 = 372645892;
    #[c2rust::src_loc = "245:5"]
    pub const SDL_PIXELFORMAT_BGRX8888: C2RustUnnamed_0 = 375789572;
    #[c2rust::src_loc = "242:5"]
    pub const SDL_PIXELFORMAT_BGR888: C2RustUnnamed_0 = 374740996;
    #[c2rust::src_loc = "239:5"]
    pub const SDL_PIXELFORMAT_RGBX8888: C2RustUnnamed_0 = 371595268;
    #[c2rust::src_loc = "236:5"]
    pub const SDL_PIXELFORMAT_RGB888: C2RustUnnamed_0 = 370546692;
    #[c2rust::src_loc = "233:5"]
    pub const SDL_PIXELFORMAT_BGR24: C2RustUnnamed_0 = 390076419;
    #[c2rust::src_loc = "230:5"]
    pub const SDL_PIXELFORMAT_RGB24: C2RustUnnamed_0 = 386930691;
    #[c2rust::src_loc = "227:5"]
    pub const SDL_PIXELFORMAT_BGR565: C2RustUnnamed_0 = 357896194;
    #[c2rust::src_loc = "224:5"]
    pub const SDL_PIXELFORMAT_RGB565: C2RustUnnamed_0 = 353701890;
    #[c2rust::src_loc = "221:5"]
    pub const SDL_PIXELFORMAT_BGRA5551: C2RustUnnamed_0 = 360976386;
    #[c2rust::src_loc = "218:5"]
    pub const SDL_PIXELFORMAT_ABGR1555: C2RustUnnamed_0 = 359862274;
    #[c2rust::src_loc = "215:5"]
    pub const SDL_PIXELFORMAT_RGBA5551: C2RustUnnamed_0 = 356782082;
    #[c2rust::src_loc = "212:5"]
    pub const SDL_PIXELFORMAT_ARGB1555: C2RustUnnamed_0 = 355667970;
    #[c2rust::src_loc = "209:5"]
    pub const SDL_PIXELFORMAT_BGRA4444: C2RustUnnamed_0 = 360845314;
    #[c2rust::src_loc = "206:5"]
    pub const SDL_PIXELFORMAT_ABGR4444: C2RustUnnamed_0 = 359796738;
    #[c2rust::src_loc = "203:5"]
    pub const SDL_PIXELFORMAT_RGBA4444: C2RustUnnamed_0 = 356651010;
    #[c2rust::src_loc = "200:5"]
    pub const SDL_PIXELFORMAT_ARGB4444: C2RustUnnamed_0 = 355602434;
    #[c2rust::src_loc = "197:5"]
    pub const SDL_PIXELFORMAT_BGR555: C2RustUnnamed_0 = 357764866;
    #[c2rust::src_loc = "194:5"]
    pub const SDL_PIXELFORMAT_RGB555: C2RustUnnamed_0 = 353570562;
    #[c2rust::src_loc = "191:5"]
    pub const SDL_PIXELFORMAT_RGB444: C2RustUnnamed_0 = 353504258;
    #[c2rust::src_loc = "188:5"]
    pub const SDL_PIXELFORMAT_RGB332: C2RustUnnamed_0 = 336660481;
    #[c2rust::src_loc = "186:5"]
    pub const SDL_PIXELFORMAT_INDEX8: C2RustUnnamed_0 = 318769153;
    #[c2rust::src_loc = "183:5"]
    pub const SDL_PIXELFORMAT_INDEX4MSB: C2RustUnnamed_0 = 304088064;
    #[c2rust::src_loc = "180:5"]
    pub const SDL_PIXELFORMAT_INDEX4LSB: C2RustUnnamed_0 = 303039488;
    #[c2rust::src_loc = "177:5"]
    pub const SDL_PIXELFORMAT_INDEX1MSB: C2RustUnnamed_0 = 287310080;
    #[c2rust::src_loc = "174:5"]
    pub const SDL_PIXELFORMAT_INDEX1LSB: C2RustUnnamed_0 = 286261504;
    #[c2rust::src_loc = "173:5"]
    pub const SDL_PIXELFORMAT_UNKNOWN: C2RustUnnamed_0 = 0;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_rect.h:34"]
pub mod SDL_rect_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct SDL_Rect {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub w: libc::c_int,
        pub h: libc::c_int,
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_video.h:34"]
pub mod SDL_video_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:9"]
    pub struct SDL_DisplayMode {
        pub format: Uint32,
        pub w: libc::c_int,
        pub h: libc::c_int,
        pub refresh_rate: libc::c_int,
        pub driverdata: *mut libc::c_void,
    }
    #[c2rust::src_loc = "97:9"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "122:5"]
    pub const SDL_WINDOW_VULKAN: C2RustUnnamed_1 = 268435456;
    #[c2rust::src_loc = "121:5"]
    pub const SDL_WINDOW_POPUP_MENU: C2RustUnnamed_1 = 524288;
    #[c2rust::src_loc = "120:5"]
    pub const SDL_WINDOW_TOOLTIP: C2RustUnnamed_1 = 262144;
    #[c2rust::src_loc = "119:5"]
    pub const SDL_WINDOW_UTILITY: C2RustUnnamed_1 = 131072;
    #[c2rust::src_loc = "118:5"]
    pub const SDL_WINDOW_SKIP_TASKBAR: C2RustUnnamed_1 = 65536;
    #[c2rust::src_loc = "117:5"]
    pub const SDL_WINDOW_ALWAYS_ON_TOP: C2RustUnnamed_1 = 32768;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_WINDOW_MOUSE_CAPTURE: C2RustUnnamed_1 = 16384;
    #[c2rust::src_loc = "113:5"]
    pub const SDL_WINDOW_ALLOW_HIGHDPI: C2RustUnnamed_1 = 8192;
    #[c2rust::src_loc = "112:5"]
    pub const SDL_WINDOW_FOREIGN: C2RustUnnamed_1 = 2048;
    #[c2rust::src_loc = "111:5"]
    pub const SDL_WINDOW_FULLSCREEN_DESKTOP: C2RustUnnamed_1 = 4097;
    #[c2rust::src_loc = "110:5"]
    pub const SDL_WINDOW_MOUSE_FOCUS: C2RustUnnamed_1 = 1024;
    #[c2rust::src_loc = "109:5"]
    pub const SDL_WINDOW_INPUT_FOCUS: C2RustUnnamed_1 = 512;
    #[c2rust::src_loc = "108:5"]
    pub const SDL_WINDOW_INPUT_GRABBED: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "107:5"]
    pub const SDL_WINDOW_MAXIMIZED: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "106:5"]
    pub const SDL_WINDOW_MINIMIZED: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "105:5"]
    pub const SDL_WINDOW_RESIZABLE: C2RustUnnamed_1 = 32;
    #[c2rust::src_loc = "104:5"]
    pub const SDL_WINDOW_BORDERLESS: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "103:5"]
    pub const SDL_WINDOW_HIDDEN: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "102:5"]
    pub const SDL_WINDOW_SHOWN: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "101:5"]
    pub const SDL_WINDOW_OPENGL: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_WINDOW_FULLSCREEN: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "146:9"]
    pub type C2RustUnnamed_2 = libc::c_uint;
    #[c2rust::src_loc = "169:5"]
    pub const SDL_WINDOWEVENT_HIT_TEST: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "168:5"]
    pub const SDL_WINDOWEVENT_TAKE_FOCUS: C2RustUnnamed_2 = 15;
    #[c2rust::src_loc = "167:5"]
    pub const SDL_WINDOWEVENT_CLOSE: C2RustUnnamed_2 = 14;
    #[c2rust::src_loc = "166:5"]
    pub const SDL_WINDOWEVENT_FOCUS_LOST: C2RustUnnamed_2 = 13;
    #[c2rust::src_loc = "165:5"]
    pub const SDL_WINDOWEVENT_FOCUS_GAINED: C2RustUnnamed_2 = 12;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_WINDOWEVENT_LEAVE: C2RustUnnamed_2 = 11;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_WINDOWEVENT_ENTER: C2RustUnnamed_2 = 10;
    #[c2rust::src_loc = "161:5"]
    pub const SDL_WINDOWEVENT_RESTORED: C2RustUnnamed_2 = 9;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_WINDOWEVENT_MAXIMIZED: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "159:5"]
    pub const SDL_WINDOWEVENT_MINIMIZED: C2RustUnnamed_2 = 7;
    #[c2rust::src_loc = "156:5"]
    pub const SDL_WINDOWEVENT_SIZE_CHANGED: C2RustUnnamed_2 = 6;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_WINDOWEVENT_RESIZED: C2RustUnnamed_2 = 5;
    #[c2rust::src_loc = "153:5"]
    pub const SDL_WINDOWEVENT_MOVED: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_WINDOWEVENT_EXPOSED: C2RustUnnamed_2 = 3;
    #[c2rust::src_loc = "150:5"]
    pub const SDL_WINDOWEVENT_HIDDEN: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "149:5"]
    pub const SDL_WINDOWEVENT_SHOWN: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_WINDOWEVENT_NONE: C2RustUnnamed_2 = 0;
    use super::SDL_rect_h::SDL_Rect;
    use super::SDL_stdinc_h::Uint32;
    extern "C" {
        #[c2rust::src_loc = "90:16"]
        pub type SDL_Window;
        #[c2rust::src_loc = "335:1"]
        pub fn SDL_GetDisplayBounds(displayIndex: libc::c_int, rect: *mut SDL_Rect) -> libc::c_int;
        #[c2rust::src_loc = "406:1"]
        pub fn SDL_GetCurrentDisplayMode(
            displayIndex: libc::c_int,
            mode_0: *mut SDL_DisplayMode,
        ) -> libc::c_int;
        #[c2rust::src_loc = "515:1"]
        pub fn SDL_CreateWindow(
            title: *const libc::c_char,
            x: libc::c_int,
            y: libc::c_int,
            w: libc::c_int,
            h: libc::c_int,
            flags: Uint32,
        ) -> *mut SDL_Window;
        #[c2rust::src_loc = "1087:1"]
        pub fn SDL_DestroyWindow(window_0: *mut SDL_Window);
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_scancode.h:34"]
pub mod SDL_scancode_h {
    #[c2rust::src_loc = "43:9"]
    pub type SDL_Scancode = libc::c_uint;
    #[c2rust::src_loc = "407:5"]
    pub const SDL_NUM_SCANCODES: SDL_Scancode = 512;
    #[c2rust::src_loc = "401:5"]
    pub const SDL_SCANCODE_AUDIOFASTFORWARD: SDL_Scancode = 286;
    #[c2rust::src_loc = "400:5"]
    pub const SDL_SCANCODE_AUDIOREWIND: SDL_Scancode = 285;
    #[c2rust::src_loc = "389:5"]
    pub const SDL_SCANCODE_APP2: SDL_Scancode = 284;
    #[c2rust::src_loc = "388:5"]
    pub const SDL_SCANCODE_APP1: SDL_Scancode = 283;
    #[c2rust::src_loc = "386:5"]
    pub const SDL_SCANCODE_SLEEP: SDL_Scancode = 282;
    #[c2rust::src_loc = "385:5"]
    pub const SDL_SCANCODE_EJECT: SDL_Scancode = 281;
    #[c2rust::src_loc = "384:5"]
    pub const SDL_SCANCODE_KBDILLUMUP: SDL_Scancode = 280;
    #[c2rust::src_loc = "383:5"]
    pub const SDL_SCANCODE_KBDILLUMDOWN: SDL_Scancode = 279;
    #[c2rust::src_loc = "382:5"]
    pub const SDL_SCANCODE_KBDILLUMTOGGLE: SDL_Scancode = 278;
    #[c2rust::src_loc = "380:5"]
    pub const SDL_SCANCODE_DISPLAYSWITCH: SDL_Scancode = 277;
    #[c2rust::src_loc = "379:5"]
    pub const SDL_SCANCODE_BRIGHTNESSUP: SDL_Scancode = 276;
    #[c2rust::src_loc = "378:5"]
    pub const SDL_SCANCODE_BRIGHTNESSDOWN: SDL_Scancode = 275;
    #[c2rust::src_loc = "367:5"]
    pub const SDL_SCANCODE_AC_BOOKMARKS: SDL_Scancode = 274;
    #[c2rust::src_loc = "366:5"]
    pub const SDL_SCANCODE_AC_REFRESH: SDL_Scancode = 273;
    #[c2rust::src_loc = "365:5"]
    pub const SDL_SCANCODE_AC_STOP: SDL_Scancode = 272;
    #[c2rust::src_loc = "364:5"]
    pub const SDL_SCANCODE_AC_FORWARD: SDL_Scancode = 271;
    #[c2rust::src_loc = "363:5"]
    pub const SDL_SCANCODE_AC_BACK: SDL_Scancode = 270;
    #[c2rust::src_loc = "362:5"]
    pub const SDL_SCANCODE_AC_HOME: SDL_Scancode = 269;
    #[c2rust::src_loc = "361:5"]
    pub const SDL_SCANCODE_AC_SEARCH: SDL_Scancode = 268;
    #[c2rust::src_loc = "360:5"]
    pub const SDL_SCANCODE_COMPUTER: SDL_Scancode = 267;
    #[c2rust::src_loc = "359:5"]
    pub const SDL_SCANCODE_CALCULATOR: SDL_Scancode = 266;
    #[c2rust::src_loc = "358:5"]
    pub const SDL_SCANCODE_MAIL: SDL_Scancode = 265;
    #[c2rust::src_loc = "357:5"]
    pub const SDL_SCANCODE_WWW: SDL_Scancode = 264;
    #[c2rust::src_loc = "356:5"]
    pub const SDL_SCANCODE_MEDIASELECT: SDL_Scancode = 263;
    #[c2rust::src_loc = "355:5"]
    pub const SDL_SCANCODE_AUDIOMUTE: SDL_Scancode = 262;
    #[c2rust::src_loc = "354:5"]
    pub const SDL_SCANCODE_AUDIOPLAY: SDL_Scancode = 261;
    #[c2rust::src_loc = "353:5"]
    pub const SDL_SCANCODE_AUDIOSTOP: SDL_Scancode = 260;
    #[c2rust::src_loc = "352:5"]
    pub const SDL_SCANCODE_AUDIOPREV: SDL_Scancode = 259;
    #[c2rust::src_loc = "351:5"]
    pub const SDL_SCANCODE_AUDIONEXT: SDL_Scancode = 258;
    #[c2rust::src_loc = "337:5"]
    pub const SDL_SCANCODE_MODE: SDL_Scancode = 257;
    #[c2rust::src_loc = "335:5"]
    pub const SDL_SCANCODE_RGUI: SDL_Scancode = 231;
    #[c2rust::src_loc = "334:5"]
    pub const SDL_SCANCODE_RALT: SDL_Scancode = 230;
    #[c2rust::src_loc = "333:5"]
    pub const SDL_SCANCODE_RSHIFT: SDL_Scancode = 229;
    #[c2rust::src_loc = "332:5"]
    pub const SDL_SCANCODE_RCTRL: SDL_Scancode = 228;
    #[c2rust::src_loc = "331:5"]
    pub const SDL_SCANCODE_LGUI: SDL_Scancode = 227;
    #[c2rust::src_loc = "330:5"]
    pub const SDL_SCANCODE_LALT: SDL_Scancode = 226;
    #[c2rust::src_loc = "329:5"]
    pub const SDL_SCANCODE_LSHIFT: SDL_Scancode = 225;
    #[c2rust::src_loc = "328:5"]
    pub const SDL_SCANCODE_LCTRL: SDL_Scancode = 224;
    #[c2rust::src_loc = "326:5"]
    pub const SDL_SCANCODE_KP_HEXADECIMAL: SDL_Scancode = 221;
    #[c2rust::src_loc = "325:5"]
    pub const SDL_SCANCODE_KP_DECIMAL: SDL_Scancode = 220;
    #[c2rust::src_loc = "324:5"]
    pub const SDL_SCANCODE_KP_OCTAL: SDL_Scancode = 219;
    #[c2rust::src_loc = "323:5"]
    pub const SDL_SCANCODE_KP_BINARY: SDL_Scancode = 218;
    #[c2rust::src_loc = "322:5"]
    pub const SDL_SCANCODE_KP_CLEARENTRY: SDL_Scancode = 217;
    #[c2rust::src_loc = "321:5"]
    pub const SDL_SCANCODE_KP_CLEAR: SDL_Scancode = 216;
    #[c2rust::src_loc = "320:5"]
    pub const SDL_SCANCODE_KP_PLUSMINUS: SDL_Scancode = 215;
    #[c2rust::src_loc = "319:5"]
    pub const SDL_SCANCODE_KP_MEMDIVIDE: SDL_Scancode = 214;
    #[c2rust::src_loc = "318:5"]
    pub const SDL_SCANCODE_KP_MEMMULTIPLY: SDL_Scancode = 213;
    #[c2rust::src_loc = "317:5"]
    pub const SDL_SCANCODE_KP_MEMSUBTRACT: SDL_Scancode = 212;
    #[c2rust::src_loc = "316:5"]
    pub const SDL_SCANCODE_KP_MEMADD: SDL_Scancode = 211;
    #[c2rust::src_loc = "315:5"]
    pub const SDL_SCANCODE_KP_MEMCLEAR: SDL_Scancode = 210;
    #[c2rust::src_loc = "314:5"]
    pub const SDL_SCANCODE_KP_MEMRECALL: SDL_Scancode = 209;
    #[c2rust::src_loc = "313:5"]
    pub const SDL_SCANCODE_KP_MEMSTORE: SDL_Scancode = 208;
    #[c2rust::src_loc = "312:5"]
    pub const SDL_SCANCODE_KP_EXCLAM: SDL_Scancode = 207;
    #[c2rust::src_loc = "311:5"]
    pub const SDL_SCANCODE_KP_AT: SDL_Scancode = 206;
    #[c2rust::src_loc = "310:5"]
    pub const SDL_SCANCODE_KP_SPACE: SDL_Scancode = 205;
    #[c2rust::src_loc = "309:5"]
    pub const SDL_SCANCODE_KP_HASH: SDL_Scancode = 204;
    #[c2rust::src_loc = "308:5"]
    pub const SDL_SCANCODE_KP_COLON: SDL_Scancode = 203;
    #[c2rust::src_loc = "307:5"]
    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: SDL_Scancode = 202;
    #[c2rust::src_loc = "306:5"]
    pub const SDL_SCANCODE_KP_VERTICALBAR: SDL_Scancode = 201;
    #[c2rust::src_loc = "305:5"]
    pub const SDL_SCANCODE_KP_DBLAMPERSAND: SDL_Scancode = 200;
    #[c2rust::src_loc = "304:5"]
    pub const SDL_SCANCODE_KP_AMPERSAND: SDL_Scancode = 199;
    #[c2rust::src_loc = "303:5"]
    pub const SDL_SCANCODE_KP_GREATER: SDL_Scancode = 198;
    #[c2rust::src_loc = "302:5"]
    pub const SDL_SCANCODE_KP_LESS: SDL_Scancode = 197;
    #[c2rust::src_loc = "301:5"]
    pub const SDL_SCANCODE_KP_PERCENT: SDL_Scancode = 196;
    #[c2rust::src_loc = "300:5"]
    pub const SDL_SCANCODE_KP_POWER: SDL_Scancode = 195;
    #[c2rust::src_loc = "299:5"]
    pub const SDL_SCANCODE_KP_XOR: SDL_Scancode = 194;
    #[c2rust::src_loc = "298:5"]
    pub const SDL_SCANCODE_KP_F: SDL_Scancode = 193;
    #[c2rust::src_loc = "297:5"]
    pub const SDL_SCANCODE_KP_E: SDL_Scancode = 192;
    #[c2rust::src_loc = "296:5"]
    pub const SDL_SCANCODE_KP_D: SDL_Scancode = 191;
    #[c2rust::src_loc = "295:5"]
    pub const SDL_SCANCODE_KP_C: SDL_Scancode = 190;
    #[c2rust::src_loc = "294:5"]
    pub const SDL_SCANCODE_KP_B: SDL_Scancode = 189;
    #[c2rust::src_loc = "293:5"]
    pub const SDL_SCANCODE_KP_A: SDL_Scancode = 188;
    #[c2rust::src_loc = "292:5"]
    pub const SDL_SCANCODE_KP_BACKSPACE: SDL_Scancode = 187;
    #[c2rust::src_loc = "291:5"]
    pub const SDL_SCANCODE_KP_TAB: SDL_Scancode = 186;
    #[c2rust::src_loc = "290:5"]
    pub const SDL_SCANCODE_KP_RIGHTBRACE: SDL_Scancode = 185;
    #[c2rust::src_loc = "289:5"]
    pub const SDL_SCANCODE_KP_LEFTBRACE: SDL_Scancode = 184;
    #[c2rust::src_loc = "288:5"]
    pub const SDL_SCANCODE_KP_RIGHTPAREN: SDL_Scancode = 183;
    #[c2rust::src_loc = "287:5"]
    pub const SDL_SCANCODE_KP_LEFTPAREN: SDL_Scancode = 182;
    #[c2rust::src_loc = "286:5"]
    pub const SDL_SCANCODE_CURRENCYSUBUNIT: SDL_Scancode = 181;
    #[c2rust::src_loc = "285:5"]
    pub const SDL_SCANCODE_CURRENCYUNIT: SDL_Scancode = 180;
    #[c2rust::src_loc = "284:5"]
    pub const SDL_SCANCODE_DECIMALSEPARATOR: SDL_Scancode = 179;
    #[c2rust::src_loc = "283:5"]
    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: SDL_Scancode = 178;
    #[c2rust::src_loc = "282:5"]
    pub const SDL_SCANCODE_KP_000: SDL_Scancode = 177;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_SCANCODE_KP_00: SDL_Scancode = 176;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_SCANCODE_EXSEL: SDL_Scancode = 164;
    #[c2rust::src_loc = "278:5"]
    pub const SDL_SCANCODE_CRSEL: SDL_Scancode = 163;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_SCANCODE_CLEARAGAIN: SDL_Scancode = 162;
    #[c2rust::src_loc = "276:5"]
    pub const SDL_SCANCODE_OPER: SDL_Scancode = 161;
    #[c2rust::src_loc = "275:5"]
    pub const SDL_SCANCODE_OUT: SDL_Scancode = 160;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_SCANCODE_SEPARATOR: SDL_Scancode = 159;
    #[c2rust::src_loc = "273:5"]
    pub const SDL_SCANCODE_RETURN2: SDL_Scancode = 158;
    #[c2rust::src_loc = "272:5"]
    pub const SDL_SCANCODE_PRIOR: SDL_Scancode = 157;
    #[c2rust::src_loc = "271:5"]
    pub const SDL_SCANCODE_CLEAR: SDL_Scancode = 156;
    #[c2rust::src_loc = "270:5"]
    pub const SDL_SCANCODE_CANCEL: SDL_Scancode = 155;
    #[c2rust::src_loc = "269:5"]
    pub const SDL_SCANCODE_SYSREQ: SDL_Scancode = 154;
    #[c2rust::src_loc = "268:5"]
    pub const SDL_SCANCODE_ALTERASE: SDL_Scancode = 153;
    #[c2rust::src_loc = "266:5"]
    pub const SDL_SCANCODE_LANG9: SDL_Scancode = 152;
    #[c2rust::src_loc = "265:5"]
    pub const SDL_SCANCODE_LANG8: SDL_Scancode = 151;
    #[c2rust::src_loc = "264:5"]
    pub const SDL_SCANCODE_LANG7: SDL_Scancode = 150;
    #[c2rust::src_loc = "263:5"]
    pub const SDL_SCANCODE_LANG6: SDL_Scancode = 149;
    #[c2rust::src_loc = "262:5"]
    pub const SDL_SCANCODE_LANG5: SDL_Scancode = 148;
    #[c2rust::src_loc = "261:5"]
    pub const SDL_SCANCODE_LANG4: SDL_Scancode = 147;
    #[c2rust::src_loc = "260:5"]
    pub const SDL_SCANCODE_LANG3: SDL_Scancode = 146;
    #[c2rust::src_loc = "259:5"]
    pub const SDL_SCANCODE_LANG2: SDL_Scancode = 145;
    #[c2rust::src_loc = "258:5"]
    pub const SDL_SCANCODE_LANG1: SDL_Scancode = 144;
    #[c2rust::src_loc = "257:5"]
    pub const SDL_SCANCODE_INTERNATIONAL9: SDL_Scancode = 143;
    #[c2rust::src_loc = "256:5"]
    pub const SDL_SCANCODE_INTERNATIONAL8: SDL_Scancode = 142;
    #[c2rust::src_loc = "255:5"]
    pub const SDL_SCANCODE_INTERNATIONAL7: SDL_Scancode = 141;
    #[c2rust::src_loc = "254:5"]
    pub const SDL_SCANCODE_INTERNATIONAL6: SDL_Scancode = 140;
    #[c2rust::src_loc = "253:5"]
    pub const SDL_SCANCODE_INTERNATIONAL5: SDL_Scancode = 139;
    #[c2rust::src_loc = "252:5"]
    pub const SDL_SCANCODE_INTERNATIONAL4: SDL_Scancode = 138;
    #[c2rust::src_loc = "251:5"]
    pub const SDL_SCANCODE_INTERNATIONAL3: SDL_Scancode = 137;
    #[c2rust::src_loc = "250:5"]
    pub const SDL_SCANCODE_INTERNATIONAL2: SDL_Scancode = 136;
    #[c2rust::src_loc = "248:5"]
    pub const SDL_SCANCODE_INTERNATIONAL1: SDL_Scancode = 135;
    #[c2rust::src_loc = "246:5"]
    pub const SDL_SCANCODE_KP_EQUALSAS400: SDL_Scancode = 134;
    #[c2rust::src_loc = "245:5"]
    pub const SDL_SCANCODE_KP_COMMA: SDL_Scancode = 133;
    #[c2rust::src_loc = "240:5"]
    pub const SDL_SCANCODE_VOLUMEDOWN: SDL_Scancode = 129;
    #[c2rust::src_loc = "239:5"]
    pub const SDL_SCANCODE_VOLUMEUP: SDL_Scancode = 128;
    #[c2rust::src_loc = "238:5"]
    pub const SDL_SCANCODE_MUTE: SDL_Scancode = 127;
    #[c2rust::src_loc = "237:5"]
    pub const SDL_SCANCODE_FIND: SDL_Scancode = 126;
    #[c2rust::src_loc = "236:5"]
    pub const SDL_SCANCODE_PASTE: SDL_Scancode = 125;
    #[c2rust::src_loc = "235:5"]
    pub const SDL_SCANCODE_COPY: SDL_Scancode = 124;
    #[c2rust::src_loc = "234:5"]
    pub const SDL_SCANCODE_CUT: SDL_Scancode = 123;
    #[c2rust::src_loc = "233:5"]
    pub const SDL_SCANCODE_UNDO: SDL_Scancode = 122;
    #[c2rust::src_loc = "232:5"]
    pub const SDL_SCANCODE_AGAIN: SDL_Scancode = 121;
    #[c2rust::src_loc = "231:5"]
    pub const SDL_SCANCODE_STOP: SDL_Scancode = 120;
    #[c2rust::src_loc = "230:5"]
    pub const SDL_SCANCODE_SELECT: SDL_Scancode = 119;
    #[c2rust::src_loc = "229:5"]
    pub const SDL_SCANCODE_MENU: SDL_Scancode = 118;
    #[c2rust::src_loc = "228:5"]
    pub const SDL_SCANCODE_HELP: SDL_Scancode = 117;
    #[c2rust::src_loc = "227:5"]
    pub const SDL_SCANCODE_EXECUTE: SDL_Scancode = 116;
    #[c2rust::src_loc = "226:5"]
    pub const SDL_SCANCODE_F24: SDL_Scancode = 115;
    #[c2rust::src_loc = "225:5"]
    pub const SDL_SCANCODE_F23: SDL_Scancode = 114;
    #[c2rust::src_loc = "224:5"]
    pub const SDL_SCANCODE_F22: SDL_Scancode = 113;
    #[c2rust::src_loc = "223:5"]
    pub const SDL_SCANCODE_F21: SDL_Scancode = 112;
    #[c2rust::src_loc = "222:5"]
    pub const SDL_SCANCODE_F20: SDL_Scancode = 111;
    #[c2rust::src_loc = "221:5"]
    pub const SDL_SCANCODE_F19: SDL_Scancode = 110;
    #[c2rust::src_loc = "220:5"]
    pub const SDL_SCANCODE_F18: SDL_Scancode = 109;
    #[c2rust::src_loc = "219:5"]
    pub const SDL_SCANCODE_F17: SDL_Scancode = 108;
    #[c2rust::src_loc = "218:5"]
    pub const SDL_SCANCODE_F16: SDL_Scancode = 107;
    #[c2rust::src_loc = "217:5"]
    pub const SDL_SCANCODE_F15: SDL_Scancode = 106;
    #[c2rust::src_loc = "216:5"]
    pub const SDL_SCANCODE_F14: SDL_Scancode = 105;
    #[c2rust::src_loc = "215:5"]
    pub const SDL_SCANCODE_F13: SDL_Scancode = 104;
    #[c2rust::src_loc = "214:5"]
    pub const SDL_SCANCODE_KP_EQUALS: SDL_Scancode = 103;
    #[c2rust::src_loc = "211:5"]
    pub const SDL_SCANCODE_POWER: SDL_Scancode = 102;
    #[c2rust::src_loc = "210:5"]
    pub const SDL_SCANCODE_APPLICATION: SDL_Scancode = 101;
    #[c2rust::src_loc = "200:5"]
    pub const SDL_SCANCODE_NONUSBACKSLASH: SDL_Scancode = 100;
    #[c2rust::src_loc = "198:5"]
    pub const SDL_SCANCODE_KP_PERIOD: SDL_Scancode = 99;
    #[c2rust::src_loc = "197:5"]
    pub const SDL_SCANCODE_KP_0: SDL_Scancode = 98;
    #[c2rust::src_loc = "196:5"]
    pub const SDL_SCANCODE_KP_9: SDL_Scancode = 97;
    #[c2rust::src_loc = "195:5"]
    pub const SDL_SCANCODE_KP_8: SDL_Scancode = 96;
    #[c2rust::src_loc = "194:5"]
    pub const SDL_SCANCODE_KP_7: SDL_Scancode = 95;
    #[c2rust::src_loc = "193:5"]
    pub const SDL_SCANCODE_KP_6: SDL_Scancode = 94;
    #[c2rust::src_loc = "192:5"]
    pub const SDL_SCANCODE_KP_5: SDL_Scancode = 93;
    #[c2rust::src_loc = "191:5"]
    pub const SDL_SCANCODE_KP_4: SDL_Scancode = 92;
    #[c2rust::src_loc = "190:5"]
    pub const SDL_SCANCODE_KP_3: SDL_Scancode = 91;
    #[c2rust::src_loc = "189:5"]
    pub const SDL_SCANCODE_KP_2: SDL_Scancode = 90;
    #[c2rust::src_loc = "188:5"]
    pub const SDL_SCANCODE_KP_1: SDL_Scancode = 89;
    #[c2rust::src_loc = "187:5"]
    pub const SDL_SCANCODE_KP_ENTER: SDL_Scancode = 88;
    #[c2rust::src_loc = "186:5"]
    pub const SDL_SCANCODE_KP_PLUS: SDL_Scancode = 87;
    #[c2rust::src_loc = "185:5"]
    pub const SDL_SCANCODE_KP_MINUS: SDL_Scancode = 86;
    #[c2rust::src_loc = "184:5"]
    pub const SDL_SCANCODE_KP_MULTIPLY: SDL_Scancode = 85;
    #[c2rust::src_loc = "183:5"]
    pub const SDL_SCANCODE_KP_DIVIDE: SDL_Scancode = 84;
    #[c2rust::src_loc = "181:5"]
    pub const SDL_SCANCODE_NUMLOCKCLEAR: SDL_Scancode = 83;
    #[c2rust::src_loc = "179:5"]
    pub const SDL_SCANCODE_UP: SDL_Scancode = 82;
    #[c2rust::src_loc = "178:5"]
    pub const SDL_SCANCODE_DOWN: SDL_Scancode = 81;
    #[c2rust::src_loc = "177:5"]
    pub const SDL_SCANCODE_LEFT: SDL_Scancode = 80;
    #[c2rust::src_loc = "176:5"]
    pub const SDL_SCANCODE_RIGHT: SDL_Scancode = 79;
    #[c2rust::src_loc = "175:5"]
    pub const SDL_SCANCODE_PAGEDOWN: SDL_Scancode = 78;
    #[c2rust::src_loc = "174:5"]
    pub const SDL_SCANCODE_END: SDL_Scancode = 77;
    #[c2rust::src_loc = "173:5"]
    pub const SDL_SCANCODE_DELETE: SDL_Scancode = 76;
    #[c2rust::src_loc = "172:5"]
    pub const SDL_SCANCODE_PAGEUP: SDL_Scancode = 75;
    #[c2rust::src_loc = "171:5"]
    pub const SDL_SCANCODE_HOME: SDL_Scancode = 74;
    #[c2rust::src_loc = "169:5"]
    pub const SDL_SCANCODE_INSERT: SDL_Scancode = 73;
    #[c2rust::src_loc = "168:5"]
    pub const SDL_SCANCODE_PAUSE: SDL_Scancode = 72;
    #[c2rust::src_loc = "167:5"]
    pub const SDL_SCANCODE_SCROLLLOCK: SDL_Scancode = 71;
    #[c2rust::src_loc = "166:5"]
    pub const SDL_SCANCODE_PRINTSCREEN: SDL_Scancode = 70;
    #[c2rust::src_loc = "164:5"]
    pub const SDL_SCANCODE_F12: SDL_Scancode = 69;
    #[c2rust::src_loc = "163:5"]
    pub const SDL_SCANCODE_F11: SDL_Scancode = 68;
    #[c2rust::src_loc = "162:5"]
    pub const SDL_SCANCODE_F10: SDL_Scancode = 67;
    #[c2rust::src_loc = "161:5"]
    pub const SDL_SCANCODE_F9: SDL_Scancode = 66;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_SCANCODE_F8: SDL_Scancode = 65;
    #[c2rust::src_loc = "159:5"]
    pub const SDL_SCANCODE_F7: SDL_Scancode = 64;
    #[c2rust::src_loc = "158:5"]
    pub const SDL_SCANCODE_F6: SDL_Scancode = 63;
    #[c2rust::src_loc = "157:5"]
    pub const SDL_SCANCODE_F5: SDL_Scancode = 62;
    #[c2rust::src_loc = "156:5"]
    pub const SDL_SCANCODE_F4: SDL_Scancode = 61;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_SCANCODE_F3: SDL_Scancode = 60;
    #[c2rust::src_loc = "154:5"]
    pub const SDL_SCANCODE_F2: SDL_Scancode = 59;
    #[c2rust::src_loc = "153:5"]
    pub const SDL_SCANCODE_F1: SDL_Scancode = 58;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_SCANCODE_CAPSLOCK: SDL_Scancode = 57;
    #[c2rust::src_loc = "149:5"]
    pub const SDL_SCANCODE_SLASH: SDL_Scancode = 56;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_SCANCODE_PERIOD: SDL_Scancode = 55;
    #[c2rust::src_loc = "147:5"]
    pub const SDL_SCANCODE_COMMA: SDL_Scancode = 54;
    #[c2rust::src_loc = "130:5"]
    pub const SDL_SCANCODE_GRAVE: SDL_Scancode = 53;
    #[c2rust::src_loc = "129:5"]
    pub const SDL_SCANCODE_APOSTROPHE: SDL_Scancode = 52;
    #[c2rust::src_loc = "128:5"]
    pub const SDL_SCANCODE_SEMICOLON: SDL_Scancode = 51;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_SCANCODE_NONUSHASH: SDL_Scancode = 50;
    #[c2rust::src_loc = "102:5"]
    pub const SDL_SCANCODE_BACKSLASH: SDL_Scancode = 49;
    #[c2rust::src_loc = "101:5"]
    pub const SDL_SCANCODE_RIGHTBRACKET: SDL_Scancode = 48;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_SCANCODE_LEFTBRACKET: SDL_Scancode = 47;
    #[c2rust::src_loc = "99:5"]
    pub const SDL_SCANCODE_EQUALS: SDL_Scancode = 46;
    #[c2rust::src_loc = "98:5"]
    pub const SDL_SCANCODE_MINUS: SDL_Scancode = 45;
    #[c2rust::src_loc = "96:5"]
    pub const SDL_SCANCODE_SPACE: SDL_Scancode = 44;
    #[c2rust::src_loc = "95:5"]
    pub const SDL_SCANCODE_TAB: SDL_Scancode = 43;
    #[c2rust::src_loc = "94:5"]
    pub const SDL_SCANCODE_BACKSPACE: SDL_Scancode = 42;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_SCANCODE_ESCAPE: SDL_Scancode = 41;
    #[c2rust::src_loc = "92:5"]
    pub const SDL_SCANCODE_RETURN: SDL_Scancode = 40;
    #[c2rust::src_loc = "90:5"]
    pub const SDL_SCANCODE_0: SDL_Scancode = 39;
    #[c2rust::src_loc = "89:5"]
    pub const SDL_SCANCODE_9: SDL_Scancode = 38;
    #[c2rust::src_loc = "88:5"]
    pub const SDL_SCANCODE_8: SDL_Scancode = 37;
    #[c2rust::src_loc = "87:5"]
    pub const SDL_SCANCODE_7: SDL_Scancode = 36;
    #[c2rust::src_loc = "86:5"]
    pub const SDL_SCANCODE_6: SDL_Scancode = 35;
    #[c2rust::src_loc = "85:5"]
    pub const SDL_SCANCODE_5: SDL_Scancode = 34;
    #[c2rust::src_loc = "84:5"]
    pub const SDL_SCANCODE_4: SDL_Scancode = 33;
    #[c2rust::src_loc = "83:5"]
    pub const SDL_SCANCODE_3: SDL_Scancode = 32;
    #[c2rust::src_loc = "82:5"]
    pub const SDL_SCANCODE_2: SDL_Scancode = 31;
    #[c2rust::src_loc = "81:5"]
    pub const SDL_SCANCODE_1: SDL_Scancode = 30;
    #[c2rust::src_loc = "79:5"]
    pub const SDL_SCANCODE_Z: SDL_Scancode = 29;
    #[c2rust::src_loc = "78:5"]
    pub const SDL_SCANCODE_Y: SDL_Scancode = 28;
    #[c2rust::src_loc = "77:5"]
    pub const SDL_SCANCODE_X: SDL_Scancode = 27;
    #[c2rust::src_loc = "76:5"]
    pub const SDL_SCANCODE_W: SDL_Scancode = 26;
    #[c2rust::src_loc = "75:5"]
    pub const SDL_SCANCODE_V: SDL_Scancode = 25;
    #[c2rust::src_loc = "74:5"]
    pub const SDL_SCANCODE_U: SDL_Scancode = 24;
    #[c2rust::src_loc = "73:5"]
    pub const SDL_SCANCODE_T: SDL_Scancode = 23;
    #[c2rust::src_loc = "72:5"]
    pub const SDL_SCANCODE_S: SDL_Scancode = 22;
    #[c2rust::src_loc = "71:5"]
    pub const SDL_SCANCODE_R: SDL_Scancode = 21;
    #[c2rust::src_loc = "70:5"]
    pub const SDL_SCANCODE_Q: SDL_Scancode = 20;
    #[c2rust::src_loc = "69:5"]
    pub const SDL_SCANCODE_P: SDL_Scancode = 19;
    #[c2rust::src_loc = "68:5"]
    pub const SDL_SCANCODE_O: SDL_Scancode = 18;
    #[c2rust::src_loc = "67:5"]
    pub const SDL_SCANCODE_N: SDL_Scancode = 17;
    #[c2rust::src_loc = "66:5"]
    pub const SDL_SCANCODE_M: SDL_Scancode = 16;
    #[c2rust::src_loc = "65:5"]
    pub const SDL_SCANCODE_L: SDL_Scancode = 15;
    #[c2rust::src_loc = "64:5"]
    pub const SDL_SCANCODE_K: SDL_Scancode = 14;
    #[c2rust::src_loc = "63:5"]
    pub const SDL_SCANCODE_J: SDL_Scancode = 13;
    #[c2rust::src_loc = "62:5"]
    pub const SDL_SCANCODE_I: SDL_Scancode = 12;
    #[c2rust::src_loc = "61:5"]
    pub const SDL_SCANCODE_H: SDL_Scancode = 11;
    #[c2rust::src_loc = "60:5"]
    pub const SDL_SCANCODE_G: SDL_Scancode = 10;
    #[c2rust::src_loc = "59:5"]
    pub const SDL_SCANCODE_F: SDL_Scancode = 9;
    #[c2rust::src_loc = "58:5"]
    pub const SDL_SCANCODE_E: SDL_Scancode = 8;
    #[c2rust::src_loc = "57:5"]
    pub const SDL_SCANCODE_D: SDL_Scancode = 7;
    #[c2rust::src_loc = "56:5"]
    pub const SDL_SCANCODE_C: SDL_Scancode = 6;
    #[c2rust::src_loc = "55:5"]
    pub const SDL_SCANCODE_B: SDL_Scancode = 5;
    #[c2rust::src_loc = "54:5"]
    pub const SDL_SCANCODE_A: SDL_Scancode = 4;
    #[c2rust::src_loc = "45:5"]
    pub const SDL_SCANCODE_UNKNOWN: SDL_Scancode = 0;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_keycode.h:34"]
pub mod SDL_keycode_h {
    #[c2rust::src_loc = "45:1"]
    pub type SDL_Keycode = Sint32;
    use super::SDL_stdinc_h::Sint32;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_keyboard.h:34"]
pub mod SDL_keyboard_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct SDL_Keysym {
        pub scancode: SDL_Scancode,
        pub sym: SDL_Keycode,
        pub mod_0: Uint16,
        pub unused: Uint32,
    }
    use super::SDL_keycode_h::SDL_Keycode;
    use super::SDL_scancode_h::{SDL_Scancode, SDL_SCANCODE_UNKNOWN};
    use super::SDL_stdinc_h::{Uint16, Uint32};
    extern "C" {
        #[c2rust::src_loc = "99:1"]
        pub fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_joystick.h:34"]
pub mod SDL_joystick_h {
    #[c2rust::src_loc = "67:1"]
    pub type SDL_Joystick = _SDL_Joystick;
    #[c2rust::src_loc = "81:1"]
    pub type SDL_JoystickID = Sint32;
    use super::SDL_stdinc_h::{Sint16, Sint32, Uint8};
    extern "C" {
        #[c2rust::src_loc = "66:8"]
        pub type _SDL_Joystick;
        #[c2rust::src_loc = "126:1"]
        pub fn SDL_NumJoysticks() -> libc::c_int;
        #[c2rust::src_loc = "190:1"]
        pub fn SDL_JoystickOpen(device_index: libc::c_int) -> *mut SDL_Joystick;
        #[c2rust::src_loc = "288:1"]
        pub fn SDL_JoystickUpdate();
        #[c2rust::src_loc = "310:1"]
        pub fn SDL_JoystickGetAxis(joystick_0: *mut SDL_Joystick, axis: libc::c_int) -> Sint16;
        #[c2rust::src_loc = "374:1"]
        pub fn SDL_JoystickGetButton(joystick_0: *mut SDL_Joystick, button: libc::c_int) -> Uint8;
        #[c2rust::src_loc = "393:1"]
        pub fn SDL_JoystickClose(joystick_0: *mut SDL_Joystick);
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_gamecontroller.h:34"]
pub mod SDL_gamecontroller_h {
    #[c2rust::src_loc = "58:1"]
    pub type SDL_GameController = _SDL_GameController;
    #[c2rust::src_loc = "272:9"]
    pub type SDL_GameControllerAxis = libc::c_int;
    #[c2rust::src_loc = "281:5"]
    pub const SDL_CONTROLLER_AXIS_MAX: SDL_GameControllerAxis = 6;
    #[c2rust::src_loc = "280:5"]
    pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: SDL_GameControllerAxis = 5;
    #[c2rust::src_loc = "279:5"]
    pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: SDL_GameControllerAxis = 4;
    #[c2rust::src_loc = "278:5"]
    pub const SDL_CONTROLLER_AXIS_RIGHTY: SDL_GameControllerAxis = 3;
    #[c2rust::src_loc = "277:5"]
    pub const SDL_CONTROLLER_AXIS_RIGHTX: SDL_GameControllerAxis = 2;
    #[c2rust::src_loc = "276:5"]
    pub const SDL_CONTROLLER_AXIS_LEFTY: SDL_GameControllerAxis = 1;
    #[c2rust::src_loc = "275:5"]
    pub const SDL_CONTROLLER_AXIS_LEFTX: SDL_GameControllerAxis = 0;
    #[c2rust::src_loc = "274:5"]
    pub const SDL_CONTROLLER_AXIS_INVALID: SDL_GameControllerAxis = -1;
    #[c2rust::src_loc = "316:9"]
    pub type SDL_GameControllerButton = libc::c_int;
    #[c2rust::src_loc = "334:5"]
    pub const SDL_CONTROLLER_BUTTON_MAX: SDL_GameControllerButton = 15;
    #[c2rust::src_loc = "333:5"]
    pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: SDL_GameControllerButton = 14;
    #[c2rust::src_loc = "332:5"]
    pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: SDL_GameControllerButton = 13;
    #[c2rust::src_loc = "331:5"]
    pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: SDL_GameControllerButton = 12;
    #[c2rust::src_loc = "330:5"]
    pub const SDL_CONTROLLER_BUTTON_DPAD_UP: SDL_GameControllerButton = 11;
    #[c2rust::src_loc = "329:5"]
    pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: SDL_GameControllerButton = 10;
    #[c2rust::src_loc = "328:5"]
    pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: SDL_GameControllerButton = 9;
    #[c2rust::src_loc = "327:5"]
    pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: SDL_GameControllerButton = 8;
    #[c2rust::src_loc = "326:5"]
    pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: SDL_GameControllerButton = 7;
    #[c2rust::src_loc = "325:5"]
    pub const SDL_CONTROLLER_BUTTON_START: SDL_GameControllerButton = 6;
    #[c2rust::src_loc = "324:5"]
    pub const SDL_CONTROLLER_BUTTON_GUIDE: SDL_GameControllerButton = 5;
    #[c2rust::src_loc = "323:5"]
    pub const SDL_CONTROLLER_BUTTON_BACK: SDL_GameControllerButton = 4;
    #[c2rust::src_loc = "322:5"]
    pub const SDL_CONTROLLER_BUTTON_Y: SDL_GameControllerButton = 3;
    #[c2rust::src_loc = "321:5"]
    pub const SDL_CONTROLLER_BUTTON_X: SDL_GameControllerButton = 2;
    #[c2rust::src_loc = "320:5"]
    pub const SDL_CONTROLLER_BUTTON_B: SDL_GameControllerButton = 1;
    #[c2rust::src_loc = "319:5"]
    pub const SDL_CONTROLLER_BUTTON_A: SDL_GameControllerButton = 0;
    #[c2rust::src_loc = "318:5"]
    pub const SDL_CONTROLLER_BUTTON_INVALID: SDL_GameControllerButton = -1;
    use super::SDL_stdinc_h::{SDL_bool, Sint16, Uint8};
    extern "C" {
        #[c2rust::src_loc = "57:8"]
        pub type _SDL_GameController;
        #[c2rust::src_loc = "169:1"]
        pub fn SDL_IsGameController(joystick_index: libc::c_int) -> SDL_bool;
        #[c2rust::src_loc = "195:1"]
        pub fn SDL_GameControllerOpen(joystick_index: libc::c_int) -> *mut SDL_GameController;
        #[c2rust::src_loc = "309:1"]
        pub fn SDL_GameControllerGetAxis(
            gamecontroller: *mut SDL_GameController,
            axis: SDL_GameControllerAxis,
        ) -> Sint16;
        #[c2rust::src_loc = "360:1"]
        pub fn SDL_GameControllerGetButton(
            gamecontroller: *mut SDL_GameController,
            button: SDL_GameControllerButton,
        ) -> Uint8;
        #[c2rust::src_loc = "379:1"]
        pub fn SDL_GameControllerClose(gamecontroller: *mut SDL_GameController);
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_touch.h:34"]
pub mod SDL_touch_h {
    #[c2rust::src_loc = "41:1"]
    pub type SDL_TouchID = Sint64;
    #[c2rust::src_loc = "42:1"]
    pub type SDL_FingerID = Sint64;
    use super::SDL_stdinc_h::Sint64;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_gesture.h:34"]
pub mod SDL_gesture_h {
    #[c2rust::src_loc = "44:1"]
    pub type SDL_GestureID = Sint64;
    use super::SDL_stdinc_h::Sint64;
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_events.h:34"]
pub mod SDL_events_h {
    #[c2rust::src_loc = "55:9"]
    pub type C2RustUnnamed_3 = libc::c_uint;
    #[c2rust::src_loc = "165:5"]
    pub const SDL_LASTEVENT: C2RustUnnamed_3 = 65535;
    #[c2rust::src_loc = "160:5"]
    pub const SDL_USEREVENT: C2RustUnnamed_3 = 32768;
    #[c2rust::src_loc = "155:5"]
    pub const SDL_RENDER_DEVICE_RESET: C2RustUnnamed_3 = 8193;
    #[c2rust::src_loc = "154:5"]
    pub const SDL_RENDER_TARGETS_RESET: C2RustUnnamed_3 = 8192;
    #[c2rust::src_loc = "151:5"]
    pub const SDL_SENSORUPDATE: C2RustUnnamed_3 = 4608;
    #[c2rust::src_loc = "148:5"]
    pub const SDL_AUDIODEVICEREMOVED: C2RustUnnamed_3 = 4353;
    #[c2rust::src_loc = "147:5"]
    pub const SDL_AUDIODEVICEADDED: C2RustUnnamed_3 = 4352;
    #[c2rust::src_loc = "144:5"]
    pub const SDL_DROPCOMPLETE: C2RustUnnamed_3 = 4099;
    #[c2rust::src_loc = "143:5"]
    pub const SDL_DROPBEGIN: C2RustUnnamed_3 = 4098;
    #[c2rust::src_loc = "142:5"]
    pub const SDL_DROPTEXT: C2RustUnnamed_3 = 4097;
    #[c2rust::src_loc = "141:5"]
    pub const SDL_DROPFILE: C2RustUnnamed_3 = 4096;
    #[c2rust::src_loc = "138:5"]
    pub const SDL_CLIPBOARDUPDATE: C2RustUnnamed_3 = 2304;
    #[c2rust::src_loc = "135:5"]
    pub const SDL_MULTIGESTURE: C2RustUnnamed_3 = 2050;
    #[c2rust::src_loc = "134:5"]
    pub const SDL_DOLLARRECORD: C2RustUnnamed_3 = 2049;
    #[c2rust::src_loc = "133:5"]
    pub const SDL_DOLLARGESTURE: C2RustUnnamed_3 = 2048;
    #[c2rust::src_loc = "130:5"]
    pub const SDL_FINGERMOTION: C2RustUnnamed_3 = 1794;
    #[c2rust::src_loc = "129:5"]
    pub const SDL_FINGERUP: C2RustUnnamed_3 = 1793;
    #[c2rust::src_loc = "128:5"]
    pub const SDL_FINGERDOWN: C2RustUnnamed_3 = 1792;
    #[c2rust::src_loc = "125:5"]
    pub const SDL_CONTROLLERDEVICEREMAPPED: C2RustUnnamed_3 = 1621;
    #[c2rust::src_loc = "124:5"]
    pub const SDL_CONTROLLERDEVICEREMOVED: C2RustUnnamed_3 = 1620;
    #[c2rust::src_loc = "123:5"]
    pub const SDL_CONTROLLERDEVICEADDED: C2RustUnnamed_3 = 1619;
    #[c2rust::src_loc = "122:5"]
    pub const SDL_CONTROLLERBUTTONUP: C2RustUnnamed_3 = 1618;
    #[c2rust::src_loc = "121:5"]
    pub const SDL_CONTROLLERBUTTONDOWN: C2RustUnnamed_3 = 1617;
    #[c2rust::src_loc = "120:5"]
    pub const SDL_CONTROLLERAXISMOTION: C2RustUnnamed_3 = 1616;
    #[c2rust::src_loc = "117:5"]
    pub const SDL_JOYDEVICEREMOVED: C2RustUnnamed_3 = 1542;
    #[c2rust::src_loc = "116:5"]
    pub const SDL_JOYDEVICEADDED: C2RustUnnamed_3 = 1541;
    #[c2rust::src_loc = "115:5"]
    pub const SDL_JOYBUTTONUP: C2RustUnnamed_3 = 1540;
    #[c2rust::src_loc = "114:5"]
    pub const SDL_JOYBUTTONDOWN: C2RustUnnamed_3 = 1539;
    #[c2rust::src_loc = "113:5"]
    pub const SDL_JOYHATMOTION: C2RustUnnamed_3 = 1538;
    #[c2rust::src_loc = "112:5"]
    pub const SDL_JOYBALLMOTION: C2RustUnnamed_3 = 1537;
    #[c2rust::src_loc = "111:5"]
    pub const SDL_JOYAXISMOTION: C2RustUnnamed_3 = 1536;
    #[c2rust::src_loc = "108:5"]
    pub const SDL_MOUSEWHEEL: C2RustUnnamed_3 = 1027;
    #[c2rust::src_loc = "107:5"]
    pub const SDL_MOUSEBUTTONUP: C2RustUnnamed_3 = 1026;
    #[c2rust::src_loc = "106:5"]
    pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed_3 = 1025;
    #[c2rust::src_loc = "105:5"]
    pub const SDL_MOUSEMOTION: C2RustUnnamed_3 = 1024;
    #[c2rust::src_loc = "100:5"]
    pub const SDL_KEYMAPCHANGED: C2RustUnnamed_3 = 772;
    #[c2rust::src_loc = "99:5"]
    pub const SDL_TEXTINPUT: C2RustUnnamed_3 = 771;
    #[c2rust::src_loc = "98:5"]
    pub const SDL_TEXTEDITING: C2RustUnnamed_3 = 770;
    #[c2rust::src_loc = "97:5"]
    pub const SDL_KEYUP: C2RustUnnamed_3 = 769;
    #[c2rust::src_loc = "96:5"]
    pub const SDL_KEYDOWN: C2RustUnnamed_3 = 768;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_SYSWMEVENT: C2RustUnnamed_3 = 513;
    #[c2rust::src_loc = "92:5"]
    pub const SDL_WINDOWEVENT: C2RustUnnamed_3 = 512;
    #[c2rust::src_loc = "89:5"]
    pub const SDL_DISPLAYEVENT: C2RustUnnamed_3 = 336;
    #[c2rust::src_loc = "83:5"]
    pub const SDL_APP_DIDENTERFOREGROUND: C2RustUnnamed_3 = 262;
    #[c2rust::src_loc = "79:5"]
    pub const SDL_APP_WILLENTERFOREGROUND: C2RustUnnamed_3 = 261;
    #[c2rust::src_loc = "75:5"]
    pub const SDL_APP_DIDENTERBACKGROUND: C2RustUnnamed_3 = 260;
    #[c2rust::src_loc = "71:5"]
    pub const SDL_APP_WILLENTERBACKGROUND: C2RustUnnamed_3 = 259;
    #[c2rust::src_loc = "67:5"]
    pub const SDL_APP_LOWMEMORY: C2RustUnnamed_3 = 258;
    #[c2rust::src_loc = "63:5"]
    pub const SDL_APP_TERMINATING: C2RustUnnamed_3 = 257;
    #[c2rust::src_loc = "60:5"]
    pub const SDL_QUIT: C2RustUnnamed_3 = 256;
    #[c2rust::src_loc = "57:5"]
    pub const SDL_FIRSTEVENT: C2RustUnnamed_3 = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:16"]
    pub struct SDL_CommonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "180:16"]
    pub struct SDL_DisplayEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub display: Uint32,
        pub event: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub data1: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "195:16"]
    pub struct SDL_WindowEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub event: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub data1: Sint32,
        pub data2: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "211:16"]
    pub struct SDL_KeyboardEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub state: Uint8,
        pub repeat: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub keysym: SDL_Keysym,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "227:16"]
    pub struct SDL_TextEditingEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub text: [libc::c_char; 32],
        pub start: Sint32,
        pub length: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:16"]
    pub struct SDL_TextInputEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub text: [libc::c_char; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:16"]
    pub struct SDL_MouseMotionEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub state: Uint32,
        pub x: Sint32,
        pub y: Sint32,
        pub xrel: Sint32,
        pub yrel: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "269:16"]
    pub struct SDL_MouseButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub button: Uint8,
        pub state: Uint8,
        pub clicks: Uint8,
        pub padding1: Uint8,
        pub x: Sint32,
        pub y: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "286:16"]
    pub struct SDL_MouseWheelEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub which: Uint32,
        pub x: Sint32,
        pub y: Sint32,
        pub direction: Uint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "300:16"]
    pub struct SDL_JoyAxisEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub axis: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub value: Sint16,
        pub padding4: Uint16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "316:16"]
    pub struct SDL_JoyBallEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub ball: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub xrel: Sint16,
        pub yrel: Sint16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "332:16"]
    pub struct SDL_JoyHatEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub hat: Uint8,
        pub value: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "352:16"]
    pub struct SDL_JoyButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub button: Uint8,
        pub state: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "366:16"]
    pub struct SDL_JoyDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "377:16"]
    pub struct SDL_ControllerAxisEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub axis: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
        pub value: Sint16,
        pub padding4: Uint16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "394:16"]
    pub struct SDL_ControllerButtonEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: SDL_JoystickID,
        pub button: Uint8,
        pub state: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "409:16"]
    pub struct SDL_ControllerDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "419:16"]
    pub struct SDL_AudioDeviceEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Uint32,
        pub iscapture: Uint8,
        pub padding1: Uint8,
        pub padding2: Uint8,
        pub padding3: Uint8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "434:16"]
    pub struct SDL_TouchFingerEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub fingerId: SDL_FingerID,
        pub x: libc::c_float,
        pub y: libc::c_float,
        pub dx: libc::c_float,
        pub dy: libc::c_float,
        pub pressure: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:16"]
    pub struct SDL_MultiGestureEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub dTheta: libc::c_float,
        pub dDist: libc::c_float,
        pub x: libc::c_float,
        pub y: libc::c_float,
        pub numFingers: Uint16,
        pub padding: Uint16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "468:16"]
    pub struct SDL_DollarGestureEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub touchId: SDL_TouchID,
        pub gestureId: SDL_GestureID,
        pub numFingers: Uint32,
        pub error: libc::c_float,
        pub x: libc::c_float,
        pub y: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "486:16"]
    pub struct SDL_DropEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub file: *mut libc::c_char,
        pub windowID: Uint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "498:16"]
    pub struct SDL_SensorEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub which: Sint32,
        pub data: [libc::c_float; 6],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "509:16"]
    pub struct SDL_QuitEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "527:16"]
    pub struct SDL_UserEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub windowID: Uint32,
        pub code: Sint32,
        pub data1: *mut libc::c_void,
        pub data2: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "547:16"]
    pub struct SDL_SysWMEvent {
        pub type_0: Uint32,
        pub timestamp: Uint32,
        pub msg: *mut SDL_SysWMmsg,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "557:15"]
    pub union SDL_Event {
        pub type_0: Uint32,
        pub common: SDL_CommonEvent,
        pub display: SDL_DisplayEvent,
        pub window: SDL_WindowEvent,
        pub key: SDL_KeyboardEvent,
        pub edit: SDL_TextEditingEvent,
        pub text: SDL_TextInputEvent,
        pub motion: SDL_MouseMotionEvent,
        pub button: SDL_MouseButtonEvent,
        pub wheel: SDL_MouseWheelEvent,
        pub jaxis: SDL_JoyAxisEvent,
        pub jball: SDL_JoyBallEvent,
        pub jhat: SDL_JoyHatEvent,
        pub jbutton: SDL_JoyButtonEvent,
        pub jdevice: SDL_JoyDeviceEvent,
        pub caxis: SDL_ControllerAxisEvent,
        pub cbutton: SDL_ControllerButtonEvent,
        pub cdevice: SDL_ControllerDeviceEvent,
        pub adevice: SDL_AudioDeviceEvent,
        pub sensor: SDL_SensorEvent,
        pub quit: SDL_QuitEvent,
        pub user: SDL_UserEvent,
        pub syswm: SDL_SysWMEvent,
        pub tfinger: SDL_TouchFingerEvent,
        pub mgesture: SDL_MultiGestureEvent,
        pub dgesture: SDL_DollarGestureEvent,
        pub drop: SDL_DropEvent,
        pub padding: [Uint8; 56],
    }
    #[c2rust::src_loc = "699:1"]
    pub type SDL_EventFilter =
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> libc::c_int>;
    use super::SDL_gesture_h::SDL_GestureID;
    use super::SDL_joystick_h::SDL_JoystickID;
    use super::SDL_keyboard_h::SDL_Keysym;
    use super::SDL_stdinc_h::{Sint16, Sint32, Uint16, Uint32, Uint8};
    use super::SDL_touch_h::{SDL_FingerID, SDL_TouchID};
    extern "C" {
        #[c2rust::src_loc = "538:8"]
        pub type SDL_SysWMmsg;
        #[c2rust::src_loc = "739:1"]
        pub fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *mut libc::c_void);
        #[c2rust::src_loc = "666:1"]
        pub fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
        #[c2rust::src_loc = "610:1"]
        pub fn SDL_PumpEvents();
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_render.h:34"]
pub mod SDL_render_h {
    #[c2rust::src_loc = "91:9"]
    pub type C2RustUnnamed_4 = libc::c_uint;
    #[c2rust::src_loc = "95:5"]
    pub const SDL_TEXTUREACCESS_TARGET: C2RustUnnamed_4 = 2;
    #[c2rust::src_loc = "94:5"]
    pub const SDL_TEXTUREACCESS_STREAMING: C2RustUnnamed_4 = 1;
    #[c2rust::src_loc = "93:5"]
    pub const SDL_TEXTUREACCESS_STATIC: C2RustUnnamed_4 = 0;
    use super::SDL_rect_h::SDL_Rect;
    use super::SDL_stdinc_h::Uint32;
    use super::SDL_video_h::SDL_Window;
    extern "C" {
        #[c2rust::src_loc = "121:8"]
        pub type SDL_Renderer;
        #[c2rust::src_loc = "127:8"]
        pub type SDL_Texture;
        #[c2rust::src_loc = "1019:1"]
        pub fn SDL_DestroyRenderer(renderer_0: *mut SDL_Renderer);
        #[c2rust::src_loc = "1003:1"]
        pub fn SDL_RenderPresent(renderer_0: *mut SDL_Renderer);
        #[c2rust::src_loc = "810:1"]
        pub fn SDL_RenderCopy(
            renderer_0: *mut SDL_Renderer,
            texture: *mut SDL_Texture,
            srcrect: *const SDL_Rect,
            dstrect: *const SDL_Rect,
        ) -> libc::c_int;
        #[c2rust::src_loc = "695:1"]
        pub fn SDL_RenderClear(renderer_0: *mut SDL_Renderer) -> libc::c_int;
        #[c2rust::src_loc = "385:1"]
        pub fn SDL_UpdateTexture(
            texture: *mut SDL_Texture,
            rect: *const SDL_Rect,
            pixels: *const libc::c_void,
            pitch: libc::c_int,
        ) -> libc::c_int;
        #[c2rust::src_loc = "242:1"]
        pub fn SDL_CreateTexture(
            renderer_0: *mut SDL_Renderer,
            format: Uint32,
            access: libc::c_int,
            w: libc::c_int,
            h: libc::c_int,
        ) -> *mut SDL_Texture;
        #[c2rust::src_loc = "191:1"]
        pub fn SDL_CreateRenderer(
            window_0: *mut SDL_Window,
            index: libc::c_int,
            flags: Uint32,
        ) -> *mut SDL_Renderer;
    }
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/pcrlib.h:34"]
pub mod pcrlib_h {
    #[c2rust::src_loc = "47:9"]
    pub type soundtype = libc::c_uint;
    #[c2rust::src_loc = "47:24"]
    pub const sdlib: soundtype = 2;
    #[c2rust::src_loc = "47:19"]
    pub const spkr: soundtype = 1;
    #[c2rust::src_loc = "47:15"]
    pub const off: soundtype = 0;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "50:9"]
    pub struct spksndtype {
        pub start: word,
        pub priority: byte,
        pub samplerate: byte,
        pub name: [libc::c_char; 12],
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "55:9"]
    pub struct SPKRtable {
        pub id: [libc::c_char; 4],
        pub filelength: word,
        pub filler: [word; 5],
        pub sounds: [spksndtype; 63],
        pub freqdata: [word; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct ControlStruct {
        pub dir: dirtype,
        pub button1: boolean,
        pub button2: boolean,
    }
    #[c2rust::src_loc = "87:9"]
    pub type inputtype = libc::c_uint;
    #[c2rust::src_loc = "87:50"]
    pub const demo: inputtype = 4;
    #[c2rust::src_loc = "87:40"]
    pub const joystick2: inputtype = 3;
    #[c2rust::src_loc = "87:30"]
    pub const joystick1: inputtype = 2;
    #[c2rust::src_loc = "87:24"]
    pub const mouse: inputtype = 1;
    #[c2rust::src_loc = "87:15"]
    pub const keyboard: inputtype = 0;
    #[c2rust::src_loc = "95:1"]
    pub type demoenum = libc::c_uint;
    #[c2rust::src_loc = "95:33"]
    pub const recording: demoenum = 2;
    #[c2rust::src_loc = "95:24"]
    pub const demoplay: demoenum = 1;
    #[c2rust::src_loc = "95:16"]
    pub const notdemo: demoenum = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:9"]
    pub struct joyinfo_t {
        pub c2rust_unnamed: C2RustUnnamed_5,
        pub device: libc::c_int,
        pub isgamecontroller: boolean,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:2"]
    pub union C2RustUnnamed_5 {
        pub controller: *mut SDL_GameController,
        pub joy: *mut SDL_Joystick,
    }
    #[c2rust::src_loc = "171:9"]
    pub type grtype = libc::c_uint;
    #[c2rust::src_loc = "171:32"]
    pub const VGAgr: grtype = 3;
    #[c2rust::src_loc = "171:26"]
    pub const EGAgr: grtype = 2;
    #[c2rust::src_loc = "171:20"]
    pub const CGAgr: grtype = 1;
    #[c2rust::src_loc = "171:15"]
    pub const text: grtype = 0;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "290:8"]
    pub struct scores {
        pub score: sdword,
        pub level: sword,
        pub initials: [libc::c_char; 4],
    }
    use super::catdefs_h::{boolean, byte, dirtype, sdword, sword, word};
    use super::SDL_gamecontroller_h::SDL_GameController;
    use super::SDL_joystick_h::SDL_Joystick;
    extern "C" {
        #[c2rust::src_loc = "263:1"]
        pub fn loadgrfiles();
        #[c2rust::src_loc = "228:1"]
        pub fn drawchartile(x: libc::c_int, y: libc::c_int, tile: libc::c_int);
        #[c2rust::src_loc = "179:1"]
        pub fn WaitVBL();
        #[c2rust::src_loc = "226:1"]
        pub fn drawchar(x: libc::c_int, y: libc::c_int, charnum: libc::c_int);
        #[c2rust::src_loc = "143:1"]
        pub fn initrnd(randomize: boolean);
        #[c2rust::src_loc = "142:1"]
        pub fn initrndt(randomize: boolean);
        #[c2rust::src_loc = "70:1"]
        pub fn PlaySound(sound: libc::c_int);
        #[c2rust::src_loc = "69:1"]
        pub fn ShutdownSound();
        #[c2rust::src_loc = "68:1"]
        pub fn StartupSound();
        #[c2rust::src_loc = "64:19"]
        pub static mut SoundData: *mut SPKRtable;
        #[c2rust::src_loc = "63:18"]
        pub static mut soundmode: soundtype;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/stat.h:28"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "595:1"]
        pub fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:31"]
pub mod ctype_h {
    extern "C" {
        #[c2rust::src_loc = "125:12"]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "632:1"]
        pub fn puts(__s: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
pub mod assert_h {
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_error.h:34"]
pub mod SDL_error_h {
    extern "C" {
        #[c2rust::src_loc = "42:1"]
        pub fn SDL_GetError() -> *const libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "385:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "130:14"]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "122:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catacomb.h:34"]
pub mod catacomb_h {
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn US_CheckParm(_: *mut libc::c_char, _: *mut *mut libc::c_char) -> libc::c_int;
        #[c2rust::src_loc = "105:14"]
        pub static mut _argc: libc::c_int;
        #[c2rust::src_loc = "106:17"]
        pub static mut _argv: *mut *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:34"]
pub mod fcntl_h {
    extern "C" {
        #[c2rust::src_loc = "168:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL.h:34"]
pub mod SDL_h {
    use super::SDL_stdinc_h::Uint32;
    extern "C" {
        #[c2rust::src_loc = "125:1"]
        pub fn SDL_Quit();
        #[c2rust::src_loc = "95:1"]
        pub fn SDL_Init(flags: Uint32) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_timer.h:34"]
pub mod SDL_timer_h {
    use super::SDL_stdinc_h::Uint32;
    extern "C" {
        #[c2rust::src_loc = "71:1"]
        pub fn SDL_Delay(ms: Uint32);
    }
}
#[c2rust::header_src = "/usr/include/SDL2/SDL_mouse.h:34"]
pub mod SDL_mouse_h {
    use super::SDL_stdinc_h::{SDL_bool, Uint32, SDL_FALSE};
    use super::SDL_video_h::SDL_Window;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn SDL_GetMouseFocus() -> *mut SDL_Window;
        #[c2rust::src_loc = "121:1"]
        pub fn SDL_GetRelativeMouseState(x: *mut libc::c_int, y: *mut libc::c_int) -> Uint32;
        #[c2rust::src_loc = "162:1"]
        pub fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> libc::c_int;
    }
}
use self::assert_h::__assert_fail;
use self::catacomb_h::{US_CheckParm, _argc, _argv};
pub use self::catdefs_h::{
    boolean, byte, dirtype, dword, east, false_0, itoa, ltoa, nodir, north, northeast, northwest,
    sdword, south, southeast, southwest, sword, true_0, west, word, C2RustUnnamed,
};
use self::ctype_h::toupper;
use self::fcntl_h::open;
pub use self::pcrlib_h::{
    demo, demoenum, demoplay, drawchar, drawchartile, grtype, initrnd, initrndt, inputtype,
    joyinfo_t, joystick1, joystick2, keyboard, loadgrfiles, mouse, notdemo, off, recording, scores,
    sdlib, soundmode, soundtype, spkr, spksndtype, text, C2RustUnnamed_5, CGAgr, ControlStruct,
    EGAgr, PlaySound, SPKRtable, ShutdownSound, SoundData, StartupSound, VGAgr, WaitVBL,
};
pub use self::stat_h::stat;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use self::stdio_h::{fprintf, puts, sprintf, stderr};
use self::stdlib_h::{atexit, atoi, exit, malloc};
use self::string_h::{memset, strcat, strcpy, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_timespec_h::timespec;
use self::sys_stat_h::fstat;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __int16_t, __int32_t, __int64_t, __mode_t,
    __nlink_t, __off64_t, __off_t, __ssize_t, __syscall_slong_t, __time_t, __uid_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use self::unistd_h::{close, read, ssize_t, write};
pub use self::FILE_h::FILE;
use self::SDL_error_h::SDL_GetError;
pub use self::SDL_events_h::{
    C2RustUnnamed_3, SDL_AddEventWatch, SDL_AudioDeviceEvent, SDL_CommonEvent,
    SDL_ControllerAxisEvent, SDL_ControllerButtonEvent, SDL_ControllerDeviceEvent,
    SDL_DisplayEvent, SDL_DollarGestureEvent, SDL_DropEvent, SDL_Event, SDL_EventFilter,
    SDL_JoyAxisEvent, SDL_JoyBallEvent, SDL_JoyButtonEvent, SDL_JoyDeviceEvent, SDL_JoyHatEvent,
    SDL_KeyboardEvent, SDL_MouseButtonEvent, SDL_MouseMotionEvent, SDL_MouseWheelEvent,
    SDL_MultiGestureEvent, SDL_PollEvent, SDL_PumpEvents, SDL_QuitEvent, SDL_SensorEvent,
    SDL_SysWMEvent, SDL_SysWMmsg, SDL_TextEditingEvent, SDL_TextInputEvent, SDL_TouchFingerEvent,
    SDL_UserEvent, SDL_WindowEvent, SDL_APP_DIDENTERBACKGROUND, SDL_APP_DIDENTERFOREGROUND,
    SDL_APP_LOWMEMORY, SDL_APP_TERMINATING, SDL_APP_WILLENTERBACKGROUND,
    SDL_APP_WILLENTERFOREGROUND, SDL_AUDIODEVICEADDED, SDL_AUDIODEVICEREMOVED, SDL_CLIPBOARDUPDATE,
    SDL_CONTROLLERAXISMOTION, SDL_CONTROLLERBUTTONDOWN, SDL_CONTROLLERBUTTONUP,
    SDL_CONTROLLERDEVICEADDED, SDL_CONTROLLERDEVICEREMAPPED, SDL_CONTROLLERDEVICEREMOVED,
    SDL_DISPLAYEVENT, SDL_DOLLARGESTURE, SDL_DOLLARRECORD, SDL_DROPBEGIN, SDL_DROPCOMPLETE,
    SDL_DROPFILE, SDL_DROPTEXT, SDL_FINGERDOWN, SDL_FINGERMOTION, SDL_FINGERUP, SDL_FIRSTEVENT,
    SDL_JOYAXISMOTION, SDL_JOYBALLMOTION, SDL_JOYBUTTONDOWN, SDL_JOYBUTTONUP, SDL_JOYDEVICEADDED,
    SDL_JOYDEVICEREMOVED, SDL_JOYHATMOTION, SDL_KEYDOWN, SDL_KEYMAPCHANGED, SDL_KEYUP,
    SDL_LASTEVENT, SDL_MOUSEBUTTONDOWN, SDL_MOUSEBUTTONUP, SDL_MOUSEMOTION, SDL_MOUSEWHEEL,
    SDL_MULTIGESTURE, SDL_QUIT, SDL_RENDER_DEVICE_RESET, SDL_RENDER_TARGETS_RESET,
    SDL_SENSORUPDATE, SDL_SYSWMEVENT, SDL_TEXTEDITING, SDL_TEXTINPUT, SDL_USEREVENT,
    SDL_WINDOWEVENT,
};
pub use self::SDL_gamecontroller_h::{
    SDL_GameController, SDL_GameControllerAxis, SDL_GameControllerButton, SDL_GameControllerClose,
    SDL_GameControllerGetAxis, SDL_GameControllerGetButton, SDL_GameControllerOpen,
    SDL_IsGameController, _SDL_GameController, SDL_CONTROLLER_AXIS_INVALID,
    SDL_CONTROLLER_AXIS_LEFTX, SDL_CONTROLLER_AXIS_LEFTY, SDL_CONTROLLER_AXIS_MAX,
    SDL_CONTROLLER_AXIS_RIGHTX, SDL_CONTROLLER_AXIS_RIGHTY, SDL_CONTROLLER_AXIS_TRIGGERLEFT,
    SDL_CONTROLLER_AXIS_TRIGGERRIGHT, SDL_CONTROLLER_BUTTON_A, SDL_CONTROLLER_BUTTON_B,
    SDL_CONTROLLER_BUTTON_BACK, SDL_CONTROLLER_BUTTON_DPAD_DOWN, SDL_CONTROLLER_BUTTON_DPAD_LEFT,
    SDL_CONTROLLER_BUTTON_DPAD_RIGHT, SDL_CONTROLLER_BUTTON_DPAD_UP, SDL_CONTROLLER_BUTTON_GUIDE,
    SDL_CONTROLLER_BUTTON_INVALID, SDL_CONTROLLER_BUTTON_LEFTSHOULDER,
    SDL_CONTROLLER_BUTTON_LEFTSTICK, SDL_CONTROLLER_BUTTON_MAX,
    SDL_CONTROLLER_BUTTON_RIGHTSHOULDER, SDL_CONTROLLER_BUTTON_RIGHTSTICK,
    SDL_CONTROLLER_BUTTON_START, SDL_CONTROLLER_BUTTON_X, SDL_CONTROLLER_BUTTON_Y,
};
pub use self::SDL_gesture_h::SDL_GestureID;
use self::SDL_h::{SDL_Init, SDL_Quit};
pub use self::SDL_joystick_h::{
    SDL_Joystick, SDL_JoystickClose, SDL_JoystickGetAxis, SDL_JoystickGetButton, SDL_JoystickID,
    SDL_JoystickOpen, SDL_JoystickUpdate, SDL_NumJoysticks, _SDL_Joystick,
};
pub use self::SDL_keyboard_h::{SDL_GetKeyFromScancode, SDL_Keysym};
pub use self::SDL_keycode_h::SDL_Keycode;
use self::SDL_mouse_h::{SDL_GetMouseFocus, SDL_GetRelativeMouseState, SDL_SetRelativeMouseMode};
pub use self::SDL_pixels_h::{
    C2RustUnnamed_0, SDL_PIXELFORMAT_ABGR1555, SDL_PIXELFORMAT_ABGR32, SDL_PIXELFORMAT_ABGR4444,
    SDL_PIXELFORMAT_ABGR8888, SDL_PIXELFORMAT_ARGB1555, SDL_PIXELFORMAT_ARGB2101010,
    SDL_PIXELFORMAT_ARGB32, SDL_PIXELFORMAT_ARGB4444, SDL_PIXELFORMAT_ARGB8888,
    SDL_PIXELFORMAT_BGR24, SDL_PIXELFORMAT_BGR555, SDL_PIXELFORMAT_BGR565, SDL_PIXELFORMAT_BGR888,
    SDL_PIXELFORMAT_BGRA32, SDL_PIXELFORMAT_BGRA4444, SDL_PIXELFORMAT_BGRA5551,
    SDL_PIXELFORMAT_BGRA8888, SDL_PIXELFORMAT_BGRX8888, SDL_PIXELFORMAT_EXTERNAL_OES,
    SDL_PIXELFORMAT_INDEX1LSB, SDL_PIXELFORMAT_INDEX1MSB, SDL_PIXELFORMAT_INDEX4LSB,
    SDL_PIXELFORMAT_INDEX4MSB, SDL_PIXELFORMAT_INDEX8, SDL_PIXELFORMAT_IYUV, SDL_PIXELFORMAT_NV12,
    SDL_PIXELFORMAT_NV21, SDL_PIXELFORMAT_RGB24, SDL_PIXELFORMAT_RGB332, SDL_PIXELFORMAT_RGB444,
    SDL_PIXELFORMAT_RGB555, SDL_PIXELFORMAT_RGB565, SDL_PIXELFORMAT_RGB888, SDL_PIXELFORMAT_RGBA32,
    SDL_PIXELFORMAT_RGBA4444, SDL_PIXELFORMAT_RGBA5551, SDL_PIXELFORMAT_RGBA8888,
    SDL_PIXELFORMAT_RGBX8888, SDL_PIXELFORMAT_UNKNOWN, SDL_PIXELFORMAT_UYVY, SDL_PIXELFORMAT_YUY2,
    SDL_PIXELFORMAT_YV12, SDL_PIXELFORMAT_YVYU,
};
pub use self::SDL_rect_h::SDL_Rect;
pub use self::SDL_render_h::{
    C2RustUnnamed_4, SDL_CreateRenderer, SDL_CreateTexture, SDL_DestroyRenderer, SDL_RenderClear,
    SDL_RenderCopy, SDL_RenderPresent, SDL_Renderer, SDL_Texture, SDL_UpdateTexture,
    SDL_TEXTUREACCESS_STATIC, SDL_TEXTUREACCESS_STREAMING, SDL_TEXTUREACCESS_TARGET,
};
pub use self::SDL_scancode_h::{
    SDL_Scancode, SDL_NUM_SCANCODES, SDL_SCANCODE_0, SDL_SCANCODE_1, SDL_SCANCODE_2,
    SDL_SCANCODE_3, SDL_SCANCODE_4, SDL_SCANCODE_5, SDL_SCANCODE_6, SDL_SCANCODE_7, SDL_SCANCODE_8,
    SDL_SCANCODE_9, SDL_SCANCODE_A, SDL_SCANCODE_AC_BACK, SDL_SCANCODE_AC_BOOKMARKS,
    SDL_SCANCODE_AC_FORWARD, SDL_SCANCODE_AC_HOME, SDL_SCANCODE_AC_REFRESH, SDL_SCANCODE_AC_SEARCH,
    SDL_SCANCODE_AC_STOP, SDL_SCANCODE_AGAIN, SDL_SCANCODE_ALTERASE, SDL_SCANCODE_APOSTROPHE,
    SDL_SCANCODE_APP1, SDL_SCANCODE_APP2, SDL_SCANCODE_APPLICATION, SDL_SCANCODE_AUDIOFASTFORWARD,
    SDL_SCANCODE_AUDIOMUTE, SDL_SCANCODE_AUDIONEXT, SDL_SCANCODE_AUDIOPLAY, SDL_SCANCODE_AUDIOPREV,
    SDL_SCANCODE_AUDIOREWIND, SDL_SCANCODE_AUDIOSTOP, SDL_SCANCODE_B, SDL_SCANCODE_BACKSLASH,
    SDL_SCANCODE_BACKSPACE, SDL_SCANCODE_BRIGHTNESSDOWN, SDL_SCANCODE_BRIGHTNESSUP, SDL_SCANCODE_C,
    SDL_SCANCODE_CALCULATOR, SDL_SCANCODE_CANCEL, SDL_SCANCODE_CAPSLOCK, SDL_SCANCODE_CLEAR,
    SDL_SCANCODE_CLEARAGAIN, SDL_SCANCODE_COMMA, SDL_SCANCODE_COMPUTER, SDL_SCANCODE_COPY,
    SDL_SCANCODE_CRSEL, SDL_SCANCODE_CURRENCYSUBUNIT, SDL_SCANCODE_CURRENCYUNIT, SDL_SCANCODE_CUT,
    SDL_SCANCODE_D, SDL_SCANCODE_DECIMALSEPARATOR, SDL_SCANCODE_DELETE, SDL_SCANCODE_DISPLAYSWITCH,
    SDL_SCANCODE_DOWN, SDL_SCANCODE_E, SDL_SCANCODE_EJECT, SDL_SCANCODE_END, SDL_SCANCODE_EQUALS,
    SDL_SCANCODE_ESCAPE, SDL_SCANCODE_EXECUTE, SDL_SCANCODE_EXSEL, SDL_SCANCODE_F, SDL_SCANCODE_F1,
    SDL_SCANCODE_F10, SDL_SCANCODE_F11, SDL_SCANCODE_F12, SDL_SCANCODE_F13, SDL_SCANCODE_F14,
    SDL_SCANCODE_F15, SDL_SCANCODE_F16, SDL_SCANCODE_F17, SDL_SCANCODE_F18, SDL_SCANCODE_F19,
    SDL_SCANCODE_F2, SDL_SCANCODE_F20, SDL_SCANCODE_F21, SDL_SCANCODE_F22, SDL_SCANCODE_F23,
    SDL_SCANCODE_F24, SDL_SCANCODE_F3, SDL_SCANCODE_F4, SDL_SCANCODE_F5, SDL_SCANCODE_F6,
    SDL_SCANCODE_F7, SDL_SCANCODE_F8, SDL_SCANCODE_F9, SDL_SCANCODE_FIND, SDL_SCANCODE_G,
    SDL_SCANCODE_GRAVE, SDL_SCANCODE_H, SDL_SCANCODE_HELP, SDL_SCANCODE_HOME, SDL_SCANCODE_I,
    SDL_SCANCODE_INSERT, SDL_SCANCODE_INTERNATIONAL1, SDL_SCANCODE_INTERNATIONAL2,
    SDL_SCANCODE_INTERNATIONAL3, SDL_SCANCODE_INTERNATIONAL4, SDL_SCANCODE_INTERNATIONAL5,
    SDL_SCANCODE_INTERNATIONAL6, SDL_SCANCODE_INTERNATIONAL7, SDL_SCANCODE_INTERNATIONAL8,
    SDL_SCANCODE_INTERNATIONAL9, SDL_SCANCODE_J, SDL_SCANCODE_K, SDL_SCANCODE_KBDILLUMDOWN,
    SDL_SCANCODE_KBDILLUMTOGGLE, SDL_SCANCODE_KBDILLUMUP, SDL_SCANCODE_KP_0, SDL_SCANCODE_KP_00,
    SDL_SCANCODE_KP_000, SDL_SCANCODE_KP_1, SDL_SCANCODE_KP_2, SDL_SCANCODE_KP_3,
    SDL_SCANCODE_KP_4, SDL_SCANCODE_KP_5, SDL_SCANCODE_KP_6, SDL_SCANCODE_KP_7, SDL_SCANCODE_KP_8,
    SDL_SCANCODE_KP_9, SDL_SCANCODE_KP_A, SDL_SCANCODE_KP_AMPERSAND, SDL_SCANCODE_KP_AT,
    SDL_SCANCODE_KP_B, SDL_SCANCODE_KP_BACKSPACE, SDL_SCANCODE_KP_BINARY, SDL_SCANCODE_KP_C,
    SDL_SCANCODE_KP_CLEAR, SDL_SCANCODE_KP_CLEARENTRY, SDL_SCANCODE_KP_COLON,
    SDL_SCANCODE_KP_COMMA, SDL_SCANCODE_KP_D, SDL_SCANCODE_KP_DBLAMPERSAND,
    SDL_SCANCODE_KP_DBLVERTICALBAR, SDL_SCANCODE_KP_DECIMAL, SDL_SCANCODE_KP_DIVIDE,
    SDL_SCANCODE_KP_E, SDL_SCANCODE_KP_ENTER, SDL_SCANCODE_KP_EQUALS, SDL_SCANCODE_KP_EQUALSAS400,
    SDL_SCANCODE_KP_EXCLAM, SDL_SCANCODE_KP_F, SDL_SCANCODE_KP_GREATER, SDL_SCANCODE_KP_HASH,
    SDL_SCANCODE_KP_HEXADECIMAL, SDL_SCANCODE_KP_LEFTBRACE, SDL_SCANCODE_KP_LEFTPAREN,
    SDL_SCANCODE_KP_LESS, SDL_SCANCODE_KP_MEMADD, SDL_SCANCODE_KP_MEMCLEAR,
    SDL_SCANCODE_KP_MEMDIVIDE, SDL_SCANCODE_KP_MEMMULTIPLY, SDL_SCANCODE_KP_MEMRECALL,
    SDL_SCANCODE_KP_MEMSTORE, SDL_SCANCODE_KP_MEMSUBTRACT, SDL_SCANCODE_KP_MINUS,
    SDL_SCANCODE_KP_MULTIPLY, SDL_SCANCODE_KP_OCTAL, SDL_SCANCODE_KP_PERCENT,
    SDL_SCANCODE_KP_PERIOD, SDL_SCANCODE_KP_PLUS, SDL_SCANCODE_KP_PLUSMINUS, SDL_SCANCODE_KP_POWER,
    SDL_SCANCODE_KP_RIGHTBRACE, SDL_SCANCODE_KP_RIGHTPAREN, SDL_SCANCODE_KP_SPACE,
    SDL_SCANCODE_KP_TAB, SDL_SCANCODE_KP_VERTICALBAR, SDL_SCANCODE_KP_XOR, SDL_SCANCODE_L,
    SDL_SCANCODE_LALT, SDL_SCANCODE_LANG1, SDL_SCANCODE_LANG2, SDL_SCANCODE_LANG3,
    SDL_SCANCODE_LANG4, SDL_SCANCODE_LANG5, SDL_SCANCODE_LANG6, SDL_SCANCODE_LANG7,
    SDL_SCANCODE_LANG8, SDL_SCANCODE_LANG9, SDL_SCANCODE_LCTRL, SDL_SCANCODE_LEFT,
    SDL_SCANCODE_LEFTBRACKET, SDL_SCANCODE_LGUI, SDL_SCANCODE_LSHIFT, SDL_SCANCODE_M,
    SDL_SCANCODE_MAIL, SDL_SCANCODE_MEDIASELECT, SDL_SCANCODE_MENU, SDL_SCANCODE_MINUS,
    SDL_SCANCODE_MODE, SDL_SCANCODE_MUTE, SDL_SCANCODE_N, SDL_SCANCODE_NONUSBACKSLASH,
    SDL_SCANCODE_NONUSHASH, SDL_SCANCODE_NUMLOCKCLEAR, SDL_SCANCODE_O, SDL_SCANCODE_OPER,
    SDL_SCANCODE_OUT, SDL_SCANCODE_P, SDL_SCANCODE_PAGEDOWN, SDL_SCANCODE_PAGEUP,
    SDL_SCANCODE_PASTE, SDL_SCANCODE_PAUSE, SDL_SCANCODE_PERIOD, SDL_SCANCODE_POWER,
    SDL_SCANCODE_PRINTSCREEN, SDL_SCANCODE_PRIOR, SDL_SCANCODE_Q, SDL_SCANCODE_R,
    SDL_SCANCODE_RALT, SDL_SCANCODE_RCTRL, SDL_SCANCODE_RETURN, SDL_SCANCODE_RETURN2,
    SDL_SCANCODE_RGUI, SDL_SCANCODE_RIGHT, SDL_SCANCODE_RIGHTBRACKET, SDL_SCANCODE_RSHIFT,
    SDL_SCANCODE_S, SDL_SCANCODE_SCROLLLOCK, SDL_SCANCODE_SELECT, SDL_SCANCODE_SEMICOLON,
    SDL_SCANCODE_SEPARATOR, SDL_SCANCODE_SLASH, SDL_SCANCODE_SLEEP, SDL_SCANCODE_SPACE,
    SDL_SCANCODE_STOP, SDL_SCANCODE_SYSREQ, SDL_SCANCODE_T, SDL_SCANCODE_TAB,
    SDL_SCANCODE_THOUSANDSSEPARATOR, SDL_SCANCODE_U, SDL_SCANCODE_UNDO, SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UP, SDL_SCANCODE_V, SDL_SCANCODE_VOLUMEDOWN, SDL_SCANCODE_VOLUMEUP,
    SDL_SCANCODE_W, SDL_SCANCODE_WWW, SDL_SCANCODE_X, SDL_SCANCODE_Y, SDL_SCANCODE_Z,
};
pub use self::SDL_stdinc_h::{
    SDL_bool, Sint16, Sint32, Sint64, Uint16, Uint32, Uint8, SDL_FALSE, SDL_TRUE,
};
use self::SDL_timer_h::SDL_Delay;
pub use self::SDL_touch_h::{SDL_FingerID, SDL_TouchID};
pub use self::SDL_video_h::{
    C2RustUnnamed_1, C2RustUnnamed_2, SDL_CreateWindow, SDL_DestroyWindow, SDL_DisplayMode,
    SDL_GetCurrentDisplayMode, SDL_GetDisplayBounds, SDL_Window, SDL_WINDOWEVENT_CLOSE,
    SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_EXPOSED, SDL_WINDOWEVENT_FOCUS_GAINED,
    SDL_WINDOWEVENT_FOCUS_LOST, SDL_WINDOWEVENT_HIDDEN, SDL_WINDOWEVENT_HIT_TEST,
    SDL_WINDOWEVENT_LEAVE, SDL_WINDOWEVENT_MAXIMIZED, SDL_WINDOWEVENT_MINIMIZED,
    SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_NONE, SDL_WINDOWEVENT_RESIZED, SDL_WINDOWEVENT_RESTORED,
    SDL_WINDOWEVENT_SHOWN, SDL_WINDOWEVENT_SIZE_CHANGED, SDL_WINDOWEVENT_TAKE_FOCUS,
    SDL_WINDOW_ALLOW_HIGHDPI, SDL_WINDOW_ALWAYS_ON_TOP, SDL_WINDOW_BORDERLESS, SDL_WINDOW_FOREIGN,
    SDL_WINDOW_FULLSCREEN, SDL_WINDOW_FULLSCREEN_DESKTOP, SDL_WINDOW_HIDDEN,
    SDL_WINDOW_INPUT_FOCUS, SDL_WINDOW_INPUT_GRABBED, SDL_WINDOW_MAXIMIZED, SDL_WINDOW_MINIMIZED,
    SDL_WINDOW_MOUSE_CAPTURE, SDL_WINDOW_MOUSE_FOCUS, SDL_WINDOW_OPENGL, SDL_WINDOW_POPUP_MENU,
    SDL_WINDOW_RESIZABLE, SDL_WINDOW_SHOWN, SDL_WINDOW_SKIP_TASKBAR, SDL_WINDOW_TOOLTIP,
    SDL_WINDOW_UTILITY, SDL_WINDOW_VULKAN,
};
extern "C" {
    #[c2rust::src_loc = "1455:1"]
    pub fn SetupEmulatedVBL();
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
#[c2rust::src_loc = "1175:9"]
pub struct ctlpaneltype {
    pub grmode: word,
    pub soundmode: word,
    pub playermode: [word; 3],
    pub JoyXlow: [sword; 3],
    pub JoyYlow: [sword; 3],
    pub JoyXhigh: [sword; 3],
    pub JoyYhigh: [sword; 3],
    pub MouseSensitivity: sword,
    pub key: [byte; 8],
    pub keyB1: byte,
    pub keyB2: byte,
}
#[no_mangle]
#[c2rust::src_loc = "37:6"]
pub static mut ch: libc::c_char = 0;
#[no_mangle]
#[c2rust::src_loc = "37:9"]
pub static mut str: [libc::c_char; 80] = [0; 80];
#[no_mangle]
#[c2rust::src_loc = "41:11"]
pub static mut playermode: [inputtype; 3] = [keyboard, keyboard, joystick1];
#[no_mangle]
#[c2rust::src_loc = "43:10"]
pub static mut keydown: [boolean; 512] = [0; 512];
#[no_mangle]
#[c2rust::src_loc = "45:5"]
pub static mut JoyXlow: [libc::c_int; 3] = [0; 3];
#[no_mangle]
#[c2rust::src_loc = "45:18"]
pub static mut JoyXhigh: [libc::c_int; 3] = [0; 3];
#[no_mangle]
#[c2rust::src_loc = "45:32"]
pub static mut JoyYlow: [libc::c_int; 3] = [0; 3];
#[no_mangle]
#[c2rust::src_loc = "45:45"]
pub static mut JoyYhigh: [libc::c_int; 3] = [0; 3];
#[no_mangle]
#[c2rust::src_loc = "47:5"]
pub static mut MouseSensitivity: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "48:9"]
pub static mut mouseEvent: boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "50:5"]
pub static mut key: [libc::c_int; 8] = [0; 8];
#[no_mangle]
#[c2rust::src_loc = "50:12"]
pub static mut keyB1: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "50:18"]
pub static mut keyB2: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "52:6"]
pub static mut demobuffer: [libc::c_char; 5000] = [0; 5000];
#[no_mangle]
#[c2rust::src_loc = "53:7"]
pub static mut demoptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "54:5"]
pub static mut democount: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "55:5"]
pub static mut lastdemoval: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "56:15"]
pub static mut indemo: demoenum = notdemo;
#[c2rust::src_loc = "58:21"]
static mut lastkey: SDL_Scancode = SDL_SCANCODE_UNKNOWN;
#[no_mangle]
#[c2rust::src_loc = "60:13"]
pub static mut window: *mut SDL_Window = 0 as *const SDL_Window as *mut SDL_Window;
#[no_mangle]
#[c2rust::src_loc = "61:15"]
pub static mut renderer: *mut SDL_Renderer = 0 as *const SDL_Renderer as *mut SDL_Renderer;
#[no_mangle]
#[c2rust::src_loc = "62:14"]
pub static mut sdltexture: *mut SDL_Texture = 0 as *const SDL_Texture as *mut SDL_Texture;
#[no_mangle]
#[c2rust::src_loc = "63:10"]
pub static mut updateRect: SDL_Rect = SDL_Rect {
    x: 0,
    y: 0,
    w: 0,
    h: 0,
};
#[no_mangle]
#[c2rust::src_loc = "64:17"]
pub static mut mode: SDL_DisplayMode = SDL_DisplayMode {
    format: 0,
    w: 0,
    h: 0,
    refresh_rate: 0,
    driverdata: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
#[c2rust::src_loc = "65:11"]
pub static mut joystick: [joyinfo_t; 3] = [joyinfo_t {
    c2rust_unnamed: C2RustUnnamed_5 {
        controller: 0 as *const SDL_GameController as *mut SDL_GameController,
    },
    device: 0,
    isgamecontroller: 0,
}; 3];
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn SetupKBD() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 128 as libc::c_int as libc::c_uint {
        keydown[i as usize] = false_0 as libc::c_int as boolean;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "86:1"]
pub unsafe extern "C" fn ProcessEvents() {
    mouseEvent = false_0 as libc::c_int as boolean;
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as libc::c_int as libc::c_uint {
            keydown[event.key.keysym.scancode as usize] = true_0 as libc::c_int as boolean;
            lastkey = event.key.keysym.scancode;
        } else if event.type_0 == SDL_KEYUP as libc::c_int as libc::c_uint {
            keydown[event.key.keysym.scancode as usize] = false_0 as libc::c_int as boolean;
        } else if event.type_0 == SDL_MOUSEMOTION as libc::c_int as libc::c_uint {
            mouseEvent = true_0 as libc::c_int as boolean;
        }
    }
}
#[c2rust::src_loc = "118:16"]
static mut hasFocus: boolean = true_0 as libc::c_int as boolean;
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn WatchUIEvents(
    mut udata: *mut libc::c_void,
    mut event: *mut SDL_Event,
) -> libc::c_int {
    if (*event).type_0 == SDL_QUIT as libc::c_int as libc::c_uint {
        _quit(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else if (*event).type_0 == SDL_WINDOWEVENT as libc::c_int as libc::c_uint {
        match (*event).window.event as libc::c_int {
            13 => {
                hasFocus = false_0 as libc::c_int as boolean;
                CheckMouseMode();
            }
            12 => {
                while SDL_GetMouseFocus() != window {
                    SDL_PumpEvents();
                    SDL_Delay(10 as libc::c_int as Uint32);
                }
                hasFocus = true_0 as libc::c_int as boolean;
                CheckMouseMode();
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn ControlKBD() -> ControlStruct {
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    if keydown[key[north as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
    }
    if keydown[key[east as libc::c_int as usize] as usize] != 0 {
        xmove = 1 as libc::c_int;
    }
    if keydown[key[south as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
    }
    if keydown[key[west as libc::c_int as usize] as usize] != 0 {
        xmove = -(1 as libc::c_int);
    }
    if keydown[key[northeast as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
        xmove = 1 as libc::c_int;
    }
    if keydown[key[northwest as libc::c_int as usize] as usize] != 0 {
        ymove = -(1 as libc::c_int);
        xmove = -(1 as libc::c_int);
    }
    if keydown[key[southeast as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
        xmove = 1 as libc::c_int;
    }
    if keydown[key[southwest as libc::c_int as usize] as usize] != 0 {
        ymove = 1 as libc::c_int;
        xmove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    action.button1 = keydown[keyB1 as usize];
    action.button2 = keydown[keyB2 as usize];
    return action;
}
#[no_mangle]
#[c2rust::src_loc = "223:1"]
pub unsafe extern "C" fn ControlMouse() -> ControlStruct {
    let mut newx: libc::c_int = 0;
    let mut newy: libc::c_int = 0;
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut buttons: libc::c_int = SDL_GetRelativeMouseState(&mut newx, &mut newy) as libc::c_int;
    action.button1 =
        (buttons & (1 as libc::c_int) << 1 as libc::c_int - 1 as libc::c_int) as boolean;
    action.button2 =
        (buttons & (1 as libc::c_int) << 3 as libc::c_int - 1 as libc::c_int) as boolean;
    if mouseEvent as libc::c_int == false_0 as libc::c_int {
        action.dir = nodir;
        return action;
    }
    if newx > MouseSensitivity {
        xmove = 1 as libc::c_int;
    } else if newx < -MouseSensitivity {
        xmove = -(1 as libc::c_int);
    }
    if newy > MouseSensitivity {
        ymove = 1 as libc::c_int;
    } else if newy < -MouseSensitivity {
        ymove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    return action;
}
#[c2rust::src_loc = "284:1"]
unsafe extern "C" fn ShutdownJoysticks() {
    let mut j: libc::c_uint = 0;
    j = 1 as libc::c_int as libc::c_uint;
    while j < 3 as libc::c_int as libc::c_uint {
        if !(joystick[j as usize].device < 0 as libc::c_int) {
            if joystick[j as usize].isgamecontroller != 0 {
                SDL_GameControllerClose(joystick[j as usize].c2rust_unnamed.controller);
            } else {
                SDL_JoystickClose(joystick[j as usize].c2rust_unnamed.joy);
            }
            joystick[j as usize].device = -(1 as libc::c_int);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "313:1"]
pub unsafe extern "C" fn ProbeJoysticks() {
    let mut j: libc::c_int = 0;
    if joystick[1 as libc::c_int as usize].device > 0 as libc::c_int
        || joystick[2 as libc::c_int as usize].device > 0 as libc::c_int
    {
        ShutdownJoysticks();
    }
    j = 1 as libc::c_int;
    while j < 3 as libc::c_int {
        if j - 1 as libc::c_int >= SDL_NumJoysticks() {
            joystick[j as usize].device = -(1 as libc::c_int);
        } else {
            joystick[j as usize].device = j - 1 as libc::c_int;
            joystick[j as usize].isgamecontroller =
                SDL_IsGameController(j - 1 as libc::c_int) as boolean;
            if SDL_IsGameController(j - 1 as libc::c_int) as u64 != 0 {
                joystick[j as usize].c2rust_unnamed.controller =
                    SDL_GameControllerOpen(j - 1 as libc::c_int);
            } else {
                joystick[j as usize].c2rust_unnamed.joy = SDL_JoystickOpen(j - 1 as libc::c_int);
            }
        }
        j += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "352:1"]
pub unsafe extern "C" fn ReadJoystick(
    mut joynum: libc::c_int,
    mut xcount: *mut libc::c_int,
    mut ycount: *mut libc::c_int,
) {
    let mut a1: libc::c_int = 0;
    let mut a2: libc::c_int = 0;
    *xcount = 0 as libc::c_int;
    *ycount = 0 as libc::c_int;
    SDL_JoystickUpdate();
    if joystick[joynum as usize].isgamecontroller != 0 {
        a1 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTX,
        ) as libc::c_int;
        a2 = SDL_GameControllerGetAxis(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_AXIS_LEFTY,
        ) as libc::c_int;
    } else {
        a1 = SDL_JoystickGetAxis(
            joystick[joynum as usize].c2rust_unnamed.joy,
            0 as libc::c_int,
        ) as libc::c_int;
        a2 = SDL_JoystickGetAxis(
            joystick[joynum as usize].c2rust_unnamed.joy,
            1 as libc::c_int,
        ) as libc::c_int;
    }
    *xcount = a1;
    *ycount = a2;
}
#[no_mangle]
#[c2rust::src_loc = "385:1"]
pub unsafe extern "C" fn ControlJoystick(mut joynum: libc::c_int) -> ControlStruct {
    let mut joyx: libc::c_int = 0 as libc::c_int;
    let mut joyy: libc::c_int = 0 as libc::c_int;
    let mut xmove: libc::c_int = 0 as libc::c_int;
    let mut ymove: libc::c_int = 0 as libc::c_int;
    let mut action: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    ReadJoystick(joynum, &mut joyx, &mut joyy);
    if joystick[joynum as usize].isgamecontroller != 0 {
        action.button1 = (SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_A,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
        action.button2 = (SDL_GameControllerGetButton(
            joystick[joynum as usize].c2rust_unnamed.controller,
            SDL_CONTROLLER_BUTTON_B,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
    } else {
        action.button1 = (SDL_JoystickGetButton(
            joystick[joynum as usize].c2rust_unnamed.joy,
            0 as libc::c_int,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
        action.button2 = (SDL_JoystickGetButton(
            joystick[joynum as usize].c2rust_unnamed.joy,
            1 as libc::c_int,
        ) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as boolean;
    }
    if joyx == 0 as libc::c_int && joyy == 0 as libc::c_int {
        action.dir = nodir;
        return action;
    }
    if joyx > JoyXhigh[joynum as usize] {
        xmove = 1 as libc::c_int;
    } else if joyx < JoyXlow[joynum as usize] {
        xmove = -(1 as libc::c_int);
    }
    if joyy > JoyYhigh[joynum as usize] {
        ymove = 1 as libc::c_int;
    } else if joyy < JoyYlow[joynum as usize] {
        ymove = -(1 as libc::c_int);
    }
    match ymove * 3 as libc::c_int + xmove {
        -4 => {
            action.dir = northwest;
        }
        -3 => {
            action.dir = north;
        }
        -2 => {
            action.dir = northeast;
        }
        -1 => {
            action.dir = west;
        }
        0 => {
            action.dir = nodir;
        }
        1 => {
            action.dir = east;
        }
        2 => {
            action.dir = southwest;
        }
        3 => {
            action.dir = south;
        }
        4 => {
            action.dir = southeast;
        }
        _ => {}
    }
    return action;
}
#[no_mangle]
#[c2rust::src_loc = "449:1"]
pub unsafe extern "C" fn ControlPlayer(mut player: libc::c_int) -> ControlStruct {
    let mut ret: ControlStruct = ControlStruct {
        dir: north,
        button1: 0,
        button2: 0,
    };
    let mut val: libc::c_int = 0;
    ProcessEvents();
    if indemo as libc::c_uint == notdemo as libc::c_int as libc::c_uint
        || indemo as libc::c_uint == recording as libc::c_int as libc::c_uint
    {
        match playermode[player as usize] as libc::c_uint {
            1 => {
                ret = ControlMouse();
            }
            2 => {
                ret = ControlJoystick(1 as libc::c_int);
            }
            3 => {
                ret = ControlJoystick(2 as libc::c_int);
            }
            0 | _ => {
                ret = ControlKBD();
            }
        }
        if indemo as libc::c_uint == recording as libc::c_int as libc::c_uint {
            val = ((ret.dir as libc::c_uint) << 2 as libc::c_int
                | ((ret.button2 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                | ret.button1 as libc::c_uint) as libc::c_int;
            let fresh0 = demoptr;
            demoptr = demoptr.offset(1);
            *fresh0 = val as libc::c_char;
        }
    } else {
        let fresh1 = demoptr;
        demoptr = demoptr.offset(1);
        val = *fresh1 as libc::c_int;
        ret.button1 = (val & 1 as libc::c_int) as boolean;
        ret.button2 = ((val & 2 as libc::c_int) >> 1 as libc::c_int) as boolean;
        ret.dir = ((val
            & 4 as libc::c_int + 8 as libc::c_int + 16 as libc::c_int + 32 as libc::c_int)
            >> 2 as libc::c_int) as dirtype;
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "502:1"]
pub unsafe extern "C" fn RecordDemo() {
    demobuffer[0 as libc::c_int as usize] = level as libc::c_char;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char;
    indemo = recording;
}
#[no_mangle]
#[c2rust::src_loc = "518:1"]
pub unsafe extern "C" fn LoadDemo(mut demonum: libc::c_int) {
    let mut st2: [libc::c_char; 5] = [0; 5];
    strcpy(
        str.as_mut_ptr(),
        b"DEMO\0" as *const u8 as *const libc::c_char,
    );
    itoa(demonum, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
    strcat(str.as_mut_ptr(), _extension);
    LoadFile(str.as_mut_ptr(), demobuffer.as_mut_ptr());
    level = demobuffer[0 as libc::c_int as usize] as sword;
    demoptr = &mut *demobuffer.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char;
    indemo = demoplay;
}
#[no_mangle]
#[c2rust::src_loc = "534:1"]
pub unsafe extern "C" fn SaveDemo(mut demonum: libc::c_int) {
    let mut st2: [libc::c_char; 5] = [0; 5];
    strcpy(
        str.as_mut_ptr(),
        b"DEMO\0" as *const u8 as *const libc::c_char,
    );
    itoa(demonum, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    strcat(str.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        demobuffer.as_mut_ptr(),
        demoptr.offset_from(
            &mut *demobuffer.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char,
        ) as libc::c_long,
    );
    indemo = notdemo;
}
#[no_mangle]
#[c2rust::src_loc = "570:1"]
pub unsafe extern "C" fn clearkeys() {
    let mut i: libc::c_int = 0;
    while bioskey(1 as libc::c_int) != 0 {
        bioskey(0 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        keydown[i as usize] = 0 as libc::c_int as boolean;
        i += 1;
    }
}
#[c2rust::src_loc = "582:1"]
unsafe extern "C" fn filelength(mut fd: libc::c_int) -> libc::c_long {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &mut s) != 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return s.st_size;
}
#[no_mangle]
#[c2rust::src_loc = "601:1"]
pub unsafe extern "C" fn LoadFile(
    mut filename: *mut libc::c_char,
    mut buffer: *mut libc::c_char,
) -> libc::c_ulong {
    let mut fd: libc::c_int = 0;
    fd = open(filename, 0o400 as libc::c_int);
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_ulong;
    }
    let mut len: libc::c_long = filelength(fd);
    let mut bytesRead: ssize_t = read(fd, buffer as *mut libc::c_void, len as size_t);
    close(fd);
    return bytesRead as libc::c_ulong;
}
#[no_mangle]
#[c2rust::src_loc = "625:1"]
pub unsafe extern "C" fn SaveFile(
    mut filename: *mut libc::c_char,
    mut buffer: *mut libc::c_char,
    mut size: libc::c_long,
) {
    let mut fd: libc::c_int = 0;
    fd = open(
        filename,
        0o1 as libc::c_int | 0 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return;
    }
    write(fd, buffer as *const libc::c_void, size as size_t);
    close(fd);
}
#[no_mangle]
#[c2rust::src_loc = "648:1"]
pub unsafe extern "C" fn bloadin(mut filename: *mut libc::c_char) -> *mut libc::c_void {
    let mut handle: libc::c_int = 0;
    let mut length: libc::c_long = 0;
    let mut location: *mut libc::c_char = 0 as *mut libc::c_char;
    handle = open(filename, 0 as libc::c_int);
    if handle != -(1 as libc::c_int) {
        length = filelength(handle);
        location = malloc(length as libc::c_ulong) as *mut libc::c_char;
        close(handle);
        LoadFile(filename, location);
        return location as *mut libc::c_void;
    } else {
        return 0 as *mut libc::c_void;
    };
}
#[no_mangle]
#[c2rust::src_loc = "673:8"]
pub static mut grmode: grtype = text;
#[no_mangle]
#[c2rust::src_loc = "675:7"]
pub static mut charptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "676:7"]
pub static mut tileptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "677:7"]
pub static mut picptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "678:7"]
pub static mut spriteptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "679:7"]
pub static mut egaplaneofs: [dword; 4] = [0; 4];
#[no_mangle]
#[c2rust::src_loc = "681:5"]
pub static mut sx: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "681:8"]
pub static mut sy: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "681:11"]
pub static mut leftedge: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "683:5"]
pub static mut win_xl: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "683:12"]
pub static mut win_yl: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "683:19"]
pub static mut win_xh: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "683:26"]
pub static mut win_yh: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "685:5"]
pub static mut screencenterx: libc::c_int = 19 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "685:24"]
pub static mut screencentery: libc::c_int = 11 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "694:1"]
pub unsafe extern "C" fn drawwindow(
    mut xl: libc::c_int,
    mut yl: libc::c_int,
    mut xh: libc::c_int,
    mut yh: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    win_xl = xl;
    win_yl = yl;
    win_xh = xh;
    win_yh = yh;
    drawchar(xl, yl, 1 as libc::c_int);
    x = xl + 1 as libc::c_int;
    while x < xh {
        drawchar(x, yl, 2 as libc::c_int);
        x += 1;
    }
    drawchar(xh, yl, 3 as libc::c_int);
    y = yl + 1 as libc::c_int;
    while y < yh {
        drawchar(xl, y, 4 as libc::c_int);
        x = xl + 1 as libc::c_int;
        while x < xh {
            drawchar(x, y, ' ' as i32);
            x += 1;
        }
        drawchar(xh, y, 5 as libc::c_int);
        y += 1;
    }
    drawchar(xl, yh, 6 as libc::c_int);
    x = xl + 1 as libc::c_int;
    while x < xh {
        drawchar(x, yh, 7 as libc::c_int);
        x += 1;
    }
    drawchar(xh, yh, 8 as libc::c_int);
    leftedge = xl + 1 as libc::c_int;
    sx = leftedge;
    sy = yl + 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "729:1"]
pub unsafe extern "C" fn bar(
    mut xl: libc::c_int,
    mut yl: libc::c_int,
    mut xh: libc::c_int,
    mut yh: libc::c_int,
    mut ch_0: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = yl;
    while y <= yh {
        x = xl;
        while x <= xh {
            drawchar(x, y, ch_0);
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "739:1"]
pub unsafe extern "C" fn erasewindow() {
    bar(win_xl, win_yl, win_xh, win_yh, ' ' as i32);
}
#[no_mangle]
#[c2rust::src_loc = "751:1"]
pub unsafe extern "C" fn centerwindow(mut width: libc::c_int, mut height: libc::c_int) {
    let mut xl: libc::c_int = screencenterx - width / 2 as libc::c_int;
    let mut yl: libc::c_int = screencentery - height / 2 as libc::c_int;
    drawwindow(
        xl,
        yl,
        xl + width + 1 as libc::c_int,
        yl + height + 1 as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "765:1"]
pub unsafe extern "C" fn expwin(mut width: libc::c_int, mut height: libc::c_int) {
    if width > 2 as libc::c_int {
        if height > 2 as libc::c_int {
            expwin(width - 2 as libc::c_int, height - 2 as libc::c_int);
        } else {
            expwinh(width - 2 as libc::c_int, height);
        }
    } else if height > 2 as libc::c_int {
        expwinv(width, height - 2 as libc::c_int);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
#[c2rust::src_loc = "783:1"]
pub unsafe extern "C" fn expwinh(mut width: libc::c_int, mut height: libc::c_int) {
    if width > 2 as libc::c_int {
        expwinh(width - 2 as libc::c_int, height);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
#[c2rust::src_loc = "793:1"]
pub unsafe extern "C" fn expwinv(mut width: libc::c_int, mut height: libc::c_int) {
    if height > 2 as libc::c_int {
        expwinv(width, height - 2 as libc::c_int);
    }
    UpdateScreen();
    WaitVBL();
    centerwindow(width, height);
}
#[no_mangle]
#[c2rust::src_loc = "811:1"]
pub unsafe extern "C" fn bioskey(mut cmd: libc::c_int) -> libc::c_int {
    if lastkey as u64 != 0 {
        let mut oldkey: libc::c_int = lastkey as libc::c_int;
        if cmd != 1 as libc::c_int {
            lastkey = SDL_SCANCODE_UNKNOWN;
        }
        return oldkey;
    }
    let mut event: SDL_Event = SDL_Event { type_0: 0 };
    while SDL_PollEvent(&mut event) != 0 {
        if event.type_0 == SDL_KEYDOWN as libc::c_int as libc::c_uint {
            if cmd == 1 as libc::c_int {
                lastkey = event.key.keysym.scancode;
                return lastkey as libc::c_int;
            }
            return event.key.keysym.scancode as libc::c_int;
        }
    }
    return lastkey as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "834:1"]
pub unsafe extern "C" fn UpdateScreen() {
    static mut EGAPalette: [Uint32; 16] = [
        0 as libc::c_int as Uint32,
        0xaa as libc::c_int as Uint32,
        0xaa00 as libc::c_int as Uint32,
        0xaaaa as libc::c_int as Uint32,
        0xaa0000 as libc::c_int as Uint32,
        0xaa00aa as libc::c_int as Uint32,
        0xaa5500 as libc::c_int as Uint32,
        0xaaaaaa as libc::c_int as Uint32,
        0x555555 as libc::c_int as Uint32,
        0x5555ff as libc::c_int as Uint32,
        0x55ff55 as libc::c_int as Uint32,
        0x55ffff as libc::c_int as Uint32,
        0xff5555 as libc::c_int as Uint32,
        0xff55ff as libc::c_int as Uint32,
        0xffff55 as libc::c_int as Uint32,
        0xffffff as libc::c_int as Uint32,
    ];
    static mut CGAPalette: [Uint32; 4] = [
        0 as libc::c_int as Uint32,
        0x55ffff as libc::c_int as Uint32,
        0xff55ff as libc::c_int as Uint32,
        0xffffff as libc::c_int as Uint32,
    ];
    static mut conv: [Uint32; 64000] = [0; 64000];
    let mut i: size_t = 0 as libc::c_int as size_t;
    if grmode as libc::c_uint == EGAgr as libc::c_int as libc::c_uint {
        while i < ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong {
            conv[i as usize] = EGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if grmode as libc::c_uint == CGAgr as libc::c_int as libc::c_uint {
        while i < ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong {
            conv[i as usize] = CGAPalette[screenseg[i as usize] as usize];
            i = i.wrapping_add(1);
        }
    } else if false_0 as libc::c_int != 0
        && !(b"VGA Palette conversion not implemented.\0" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"false && \"VGA Palette conversion not implemented.\"\0" as *const u8
                as *const libc::c_char,
            b"/home/saverio/code/catacomb_ii-64k/source_project/pcrlib_c.c\0" as *const u8
                as *const libc::c_char,
            865 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"void UpdateScreen()\0"))
                .as_ptr(),
        );
    }
    SDL_UpdateTexture(
        sdltexture,
        0 as *const SDL_Rect,
        conv.as_mut_ptr() as *const libc::c_void,
        (320 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Uint32>() as libc::c_ulong) as libc::c_int,
    );
    SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, sdltexture, 0 as *const SDL_Rect, &mut updateRect);
    SDL_RenderPresent(renderer);
}
#[no_mangle]
#[c2rust::src_loc = "873:1"]
pub unsafe extern "C" fn get() -> libc::c_int {
    let mut cycle: libc::c_int = 0;
    let mut key_0: libc::c_int = 0;
    loop {
        cycle = 9 as libc::c_int;
        loop {
            key_0 = bioskey(0 as libc::c_int);
            if !(key_0 == 0 && cycle < 13 as libc::c_int) {
                break;
            }
            let fresh2 = cycle;
            cycle = cycle + 1;
            drawchar(sx, sy, fresh2);
            UpdateScreen();
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
            WaitVBL();
        }
        if !(key_0 == 0 as libc::c_int) {
            break;
        }
    }
    drawchar(sx, sy, ' ' as i32);
    UpdateScreen();
    return SDL_GetKeyFromScancode(key_0 as SDL_Scancode);
}
#[no_mangle]
#[c2rust::src_loc = "904:1"]
pub unsafe extern "C" fn print(mut str_0: *const libc::c_char) {
    let mut ch_0: libc::c_char = 0;
    loop {
        let fresh3 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh3;
        if !(ch_0 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if ch_0 as libc::c_int == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as libc::c_int == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh4 = sx;
            sx = sx + 1;
            drawchar(fresh4, sy, ch_0 as byte as libc::c_int);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "921:1"]
pub unsafe extern "C" fn printchartile(mut str_0: *const libc::c_char) {
    let mut ch_0: libc::c_char = 0;
    loop {
        let fresh5 = str_0;
        str_0 = str_0.offset(1);
        ch_0 = *fresh5;
        if !(ch_0 as libc::c_int != 0 as libc::c_int) {
            break;
        }
        if ch_0 as libc::c_int == '\n' as i32 {
            sy += 1;
            sx = leftedge;
        } else if ch_0 as libc::c_int == '\r' as i32 {
            sx = leftedge;
        } else {
            let fresh6 = sx;
            sx = sx + 1;
            drawchartile(fresh6, sy, ch_0 as byte as libc::c_int);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "945:1"]
pub unsafe extern "C" fn printint(mut val: libc::c_int) {
    itoa(val, str.as_mut_ptr(), 10 as libc::c_int);
    print(str.as_mut_ptr());
}
#[no_mangle]
#[c2rust::src_loc = "951:1"]
pub unsafe extern "C" fn printlong(mut val: libc::c_long) {
    ltoa(val as libc::c_int, str.as_mut_ptr(), 10 as libc::c_int);
    print(str.as_mut_ptr());
}
#[no_mangle]
#[c2rust::src_loc = "964:1"]
pub unsafe extern "C" fn _Verify(mut filename: *mut libc::c_char) -> libc::c_long {
    let mut handle: libc::c_int = 0;
    let mut size: libc::c_long = 0;
    handle = open(filename, 0 as libc::c_int);
    if handle == -(1 as libc::c_int) {
        return 0 as libc::c_int as libc::c_long;
    }
    size = filelength(handle);
    close(handle);
    return size;
}
#[no_mangle]
#[c2rust::src_loc = "984:1"]
pub unsafe extern "C" fn _printhexb(mut value: libc::c_uchar) {
    let mut loop_0: libc::c_int = 0;
    let mut hexstr: [libc::c_char; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [libc::c_char; 16]>(b"0123456789ABCDEF");
    let mut str_0: [libc::c_char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"\0\0");
    loop_0 = 0 as libc::c_int;
    while loop_0 < 2 as libc::c_int {
        str_0[0 as libc::c_int as usize] = hexstr[(value as libc::c_int
            >> (1 as libc::c_int - loop_0) * 4 as libc::c_int
            & 15 as libc::c_int) as usize];
        print(str_0.as_mut_ptr());
        loop_0 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1004:1"]
pub unsafe extern "C" fn _printhex(mut value: libc::c_uint) {
    print(b"$\0" as *const u8 as *const libc::c_char);
    _printhexb((value >> 8 as libc::c_int) as libc::c_uchar);
    _printhexb((value & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar);
}
#[no_mangle]
#[c2rust::src_loc = "1019:1"]
pub unsafe extern "C" fn _printbin(mut value: libc::c_uint) {
    let mut loop_0: libc::c_int = 0;
    print(b"%\0" as *const u8 as *const libc::c_char);
    loop_0 = 0 as libc::c_int;
    while loop_0 < 16 as libc::c_int {
        if value >> 15 as libc::c_int - loop_0 & 1 as libc::c_int as libc::c_uint != 0 {
            print(b"1\0" as *const u8 as *const libc::c_char);
        } else {
            print(b"0\0" as *const u8 as *const libc::c_char);
        }
        loop_0 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1036:1"]
pub unsafe extern "C" fn _printc(mut string: *mut libc::c_char) {
    sx = 1 as libc::c_int + screencenterx
        - (strlen(string)).wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    print(string);
}
#[no_mangle]
#[c2rust::src_loc = "1050:1"]
pub unsafe extern "C" fn _inputint() -> libc::c_uint {
    let mut string: [libc::c_char; 18] = *::std::mem::transmute::<&[u8; 18], &mut [libc::c_char; 18]>(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut digit: libc::c_char = 0;
    let mut hexstr: [libc::c_char; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [libc::c_char; 16]>(b"0123456789ABCDEF");
    let mut value: libc::c_uint = 0;
    let mut loop_0: libc::c_uint = 0;
    let mut loop1: libc::c_uint = 0;
    _input(string.as_mut_ptr(), 17 as libc::c_int);
    if string[0 as libc::c_int as usize] as libc::c_int == '$' as i32 {
        let mut digits: ssize_t = 0;
        digits = (strlen(string.as_mut_ptr())).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as ssize_t;
        if digits < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as libc::c_uint;
        }
        value = 0 as libc::c_int as libc::c_uint;
        loop1 = 0 as libc::c_int as libc::c_uint;
        while loop1 as libc::c_ulong <= digits as size_t {
            digit = toupper(
                string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int,
            ) as libc::c_char;
            loop_0 = 0 as libc::c_int as libc::c_uint;
            while loop_0 < 16 as libc::c_int as libc::c_uint {
                if digit as libc::c_int == hexstr[loop_0 as usize] as libc::c_int {
                    value |= loop_0
                        << (digits - loop1 as libc::c_long) * 4 as libc::c_int as libc::c_long;
                    break;
                } else {
                    loop_0 = loop_0.wrapping_add(1);
                }
            }
            loop1 = loop1.wrapping_add(1);
        }
    } else if string[0 as libc::c_int as usize] as libc::c_int == '%' as i32 {
        let mut digits_0: ssize_t = 0;
        digits_0 = (strlen(string.as_mut_ptr())).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as ssize_t;
        if digits_0 < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as libc::c_uint;
        }
        value = 0 as libc::c_int as libc::c_uint;
        loop1 = 0 as libc::c_int as libc::c_uint;
        while loop1 as libc::c_ulong <= digits_0 as size_t {
            if (string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int)
                < '0' as i32
                || string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int
                    > '1' as i32
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            value |= ((string[loop1.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                as libc::c_int
                - '0' as i32)
                << digits_0 - loop1 as libc::c_long) as libc::c_uint;
            loop1 = loop1.wrapping_add(1);
        }
    } else {
        value = atoi(string.as_mut_ptr()) as libc::c_uint;
    }
    return value;
}
#[no_mangle]
#[c2rust::src_loc = "1099:1"]
pub unsafe extern "C" fn _input(
    mut string: *mut libc::c_char,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut key_0: libc::c_char = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut loop_0: libc::c_int = 0;
    loop {
        key_0 = toupper(get() & 0xff as libc::c_int) as libc::c_char;
        if (key_0 as libc::c_int == 127 as libc::c_int || key_0 as libc::c_int == 8 as libc::c_int)
            && count > 0 as libc::c_int
        {
            count -= 1;
            drawchar(sx, sy, ' ' as i32);
            sx -= 1;
        }
        if key_0 as libc::c_int >= ' ' as i32 && key_0 as libc::c_int <= 'z' as i32 && count < max {
            let fresh7 = count;
            count = count + 1;
            *string.offset(fresh7 as isize) = key_0;
            let fresh8 = sx;
            sx = sx + 1;
            drawchar(fresh8, sy, key_0 as libc::c_int);
        }
        if !(key_0 as libc::c_int != 27 as libc::c_int && key_0 as libc::c_int != 13 as libc::c_int)
        {
            break;
        }
    }
    loop_0 = count;
    while loop_0 < max {
        *string.offset(loop_0 as isize) = 0 as libc::c_int as libc::c_char;
        loop_0 += 1;
    }
    if key_0 as libc::c_int == 13 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1133:15"]
pub static mut scoreswap: scores = scores {
    score: 0,
    level: 0,
    initials: [0; 4],
};
#[no_mangle]
#[c2rust::src_loc = "1133:26"]
pub static mut highscores: [scores; 5] = [scores {
    score: 0,
    level: 0,
    initials: [0; 4],
}; 5];
#[no_mangle]
#[c2rust::src_loc = "1135:8"]
pub static mut score: sdword = 0;
#[no_mangle]
#[c2rust::src_loc = "1136:7"]
pub static mut level: sword = 0;
#[no_mangle]
#[c2rust::src_loc = "1137:5"]
pub static mut _numlevels: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "1137:17"]
pub static mut _maxplayers: libc::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "1139:13"]
pub static mut _extension: *const libc::c_char = b"PCR\0" as *const u8 as *const libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "1140:9"]
pub static mut _cgaok: boolean = true_0 as libc::c_int as boolean;
#[no_mangle]
#[c2rust::src_loc = "1140:24"]
pub static mut _egaok: boolean = true_0 as libc::c_int as boolean;
#[no_mangle]
#[c2rust::src_loc = "1140:39"]
pub static mut _vgaok: boolean = false_0 as libc::c_int as boolean;
#[c2rust::src_loc = "1142:27"]
static mut DOSScanCodeMap: [SDL_Scancode; 128] = [
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_ESCAPE,
    SDL_SCANCODE_1,
    SDL_SCANCODE_2,
    SDL_SCANCODE_3,
    SDL_SCANCODE_4,
    SDL_SCANCODE_5,
    SDL_SCANCODE_6,
    SDL_SCANCODE_7,
    SDL_SCANCODE_8,
    SDL_SCANCODE_9,
    SDL_SCANCODE_0,
    SDL_SCANCODE_MINUS,
    SDL_SCANCODE_EQUALS,
    SDL_SCANCODE_BACKSPACE,
    SDL_SCANCODE_TAB,
    SDL_SCANCODE_Q,
    SDL_SCANCODE_W,
    SDL_SCANCODE_E,
    SDL_SCANCODE_R,
    SDL_SCANCODE_T,
    SDL_SCANCODE_Y,
    SDL_SCANCODE_U,
    SDL_SCANCODE_I,
    SDL_SCANCODE_O,
    SDL_SCANCODE_P,
    SDL_SCANCODE_LEFTBRACKET,
    SDL_SCANCODE_RIGHTBRACKET,
    SDL_SCANCODE_RETURN,
    SDL_SCANCODE_LCTRL,
    SDL_SCANCODE_A,
    SDL_SCANCODE_S,
    SDL_SCANCODE_D,
    SDL_SCANCODE_F,
    SDL_SCANCODE_G,
    SDL_SCANCODE_H,
    SDL_SCANCODE_J,
    SDL_SCANCODE_K,
    SDL_SCANCODE_L,
    SDL_SCANCODE_SEMICOLON,
    SDL_SCANCODE_APOSTROPHE,
    SDL_SCANCODE_GRAVE,
    SDL_SCANCODE_LSHIFT,
    SDL_SCANCODE_BACKSLASH,
    SDL_SCANCODE_Z,
    SDL_SCANCODE_X,
    SDL_SCANCODE_C,
    SDL_SCANCODE_V,
    SDL_SCANCODE_B,
    SDL_SCANCODE_N,
    SDL_SCANCODE_M,
    SDL_SCANCODE_COMMA,
    SDL_SCANCODE_PERIOD,
    SDL_SCANCODE_SLASH,
    SDL_SCANCODE_RSHIFT,
    SDL_SCANCODE_SYSREQ,
    SDL_SCANCODE_LALT,
    SDL_SCANCODE_SPACE,
    SDL_SCANCODE_CAPSLOCK,
    SDL_SCANCODE_F1,
    SDL_SCANCODE_F2,
    SDL_SCANCODE_F3,
    SDL_SCANCODE_F4,
    SDL_SCANCODE_F5,
    SDL_SCANCODE_F6,
    SDL_SCANCODE_F7,
    SDL_SCANCODE_F8,
    SDL_SCANCODE_F9,
    SDL_SCANCODE_F10,
    SDL_SCANCODE_NUMLOCKCLEAR,
    SDL_SCANCODE_SCROLLLOCK,
    SDL_SCANCODE_HOME,
    SDL_SCANCODE_UP,
    SDL_SCANCODE_PAGEUP,
    SDL_SCANCODE_KP_MINUS,
    SDL_SCANCODE_LEFT,
    SDL_SCANCODE_KP_5,
    SDL_SCANCODE_RIGHT,
    SDL_SCANCODE_KP_PLUS,
    SDL_SCANCODE_END,
    SDL_SCANCODE_DOWN,
    SDL_SCANCODE_PAGEDOWN,
    SDL_SCANCODE_INSERT,
    SDL_SCANCODE_DELETE,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_F11,
    SDL_SCANCODE_F12,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
    SDL_SCANCODE_UNKNOWN,
];
#[no_mangle]
#[c2rust::src_loc = "1163:1"]
pub unsafe extern "C" fn ScancodeToDOS(mut sc: SDL_Scancode) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if DOSScanCodeMap[i as usize] as libc::c_uint == sc as libc::c_uint {
            return i;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1192:1"]
pub unsafe extern "C" fn CheckMouseMode() {
    SDL_SetRelativeMouseMode(
        (hasFocus as libc::c_int != 0
            && (playermode[1 as libc::c_int as usize] as libc::c_uint
                == mouse as libc::c_int as libc::c_uint
                || playermode[2 as libc::c_int as usize] as libc::c_uint
                    == mouse as libc::c_int as libc::c_uint)) as libc::c_int as SDL_bool,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1206:1"]
pub unsafe extern "C" fn _loadctrls() {
    let mut handle: libc::c_int = 0;
    strcpy(
        str.as_mut_ptr(),
        b"CTLPANEL.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    handle = open(
        str.as_mut_ptr(),
        0 as libc::c_int | 0 as libc::c_int,
        0o200 as libc::c_int | 0o400 as libc::c_int,
    );
    if handle == -(1 as libc::c_int) {
        grmode = VGAgr;
        soundmode = spkr;
        playermode[1 as libc::c_int as usize] = keyboard;
        playermode[2 as libc::c_int as usize] = joystick1;
        JoyXlow[2 as libc::c_int as usize] = 20 as libc::c_int;
        JoyXlow[1 as libc::c_int as usize] = JoyXlow[2 as libc::c_int as usize];
        JoyXhigh[2 as libc::c_int as usize] = 60 as libc::c_int;
        JoyXhigh[1 as libc::c_int as usize] = JoyXhigh[2 as libc::c_int as usize];
        JoyYlow[2 as libc::c_int as usize] = 20 as libc::c_int;
        JoyYlow[1 as libc::c_int as usize] = JoyYlow[2 as libc::c_int as usize];
        JoyYhigh[2 as libc::c_int as usize] = 60 as libc::c_int;
        JoyYhigh[1 as libc::c_int as usize] = JoyYhigh[2 as libc::c_int as usize];
        MouseSensitivity = 5 as libc::c_int;
        key[north as libc::c_int as usize] = SDL_SCANCODE_UP as libc::c_int;
        key[northeast as libc::c_int as usize] = SDL_SCANCODE_PAGEUP as libc::c_int;
        key[east as libc::c_int as usize] = SDL_SCANCODE_RIGHT as libc::c_int;
        key[southeast as libc::c_int as usize] = SDL_SCANCODE_PAGEDOWN as libc::c_int;
        key[south as libc::c_int as usize] = SDL_SCANCODE_DOWN as libc::c_int;
        key[southwest as libc::c_int as usize] = SDL_SCANCODE_END as libc::c_int;
        key[west as libc::c_int as usize] = SDL_SCANCODE_LEFT as libc::c_int;
        key[northwest as libc::c_int as usize] = SDL_SCANCODE_HOME as libc::c_int;
        keyB1 = SDL_SCANCODE_LCTRL as libc::c_int;
        keyB2 = SDL_SCANCODE_LALT as libc::c_int;
    } else {
        let mut ctlpanel: ctlpaneltype = ctlpaneltype {
            grmode: 0,
            soundmode: 0,
            playermode: [0; 3],
            JoyXlow: [0; 3],
            JoyYlow: [0; 3],
            JoyXhigh: [0; 3],
            JoyYhigh: [0; 3],
            MouseSensitivity: 0,
            key: [0; 8],
            keyB1: 0,
            keyB2: 0,
        };
        read(
            handle,
            &mut ctlpanel as *mut ctlpaneltype as *mut libc::c_void,
            ::std::mem::size_of::<ctlpaneltype>() as libc::c_ulong,
        );
        close(handle);
        grmode = ctlpanel.grmode as grtype;
        soundmode = ctlpanel.soundmode as soundtype;
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 3 as libc::c_int as libc::c_uint {
            playermode[i as usize] = ctlpanel.playermode[i as usize] as inputtype;
            JoyXlow[i as usize] = ctlpanel.JoyXlow[i as usize] as libc::c_int;
            JoyYlow[i as usize] = ctlpanel.JoyYlow[i as usize] as libc::c_int;
            JoyXhigh[i as usize] = ctlpanel.JoyXhigh[i as usize] as libc::c_int;
            JoyYhigh[i as usize] = ctlpanel.JoyYhigh[i as usize] as libc::c_int;
            if playermode[i as usize] as libc::c_uint == mouse as libc::c_int as libc::c_uint {
                CheckMouseMode();
            }
            if playermode[i as usize] as libc::c_uint == joystick1 as libc::c_int as libc::c_uint
                || playermode[i as usize] as libc::c_uint
                    == joystick2 as libc::c_int as libc::c_uint
            {
                ProbeJoysticks();
                if playermode[i as usize] as libc::c_uint
                    == joystick1 as libc::c_int as libc::c_uint
                    && joystick[1 as libc::c_int as usize].device < 0 as libc::c_int
                    || playermode[i as usize] as libc::c_uint
                        == joystick2 as libc::c_int as libc::c_uint
                        && joystick[2 as libc::c_int as usize].device < 0 as libc::c_int
                {
                    playermode[i as usize] = keyboard;
                }
            }
            i = i.wrapping_add(1);
        }
        MouseSensitivity = ctlpanel.MouseSensitivity as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while i < 8 as libc::c_int as libc::c_uint {
            key[i as usize] = DOSScanCodeMap[ctlpanel.key[i as usize] as usize] as libc::c_int;
            i = i.wrapping_add(1);
        }
        keyB1 = DOSScanCodeMap[ctlpanel.keyB1 as usize] as libc::c_int;
        keyB2 = DOSScanCodeMap[ctlpanel.keyB2 as usize] as libc::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1275:1"]
pub unsafe extern "C" fn _savectrls() {
    let mut handle: libc::c_int = 0;
    let mut ctlpanel: ctlpaneltype = ctlpaneltype {
        grmode: 0,
        soundmode: 0,
        playermode: [0; 3],
        JoyXlow: [0; 3],
        JoyYlow: [0; 3],
        JoyXhigh: [0; 3],
        JoyYhigh: [0; 3],
        MouseSensitivity: 0,
        key: [0; 8],
        keyB1: 0,
        keyB2: 0,
    };
    strcpy(
        str.as_mut_ptr(),
        b"CTLPANEL.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    handle = open(
        str.as_mut_ptr(),
        0o1 as libc::c_int | 0 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if handle == -(1 as libc::c_int) {
        return;
    }
    ctlpanel.grmode = grmode as word;
    ctlpanel.soundmode = soundmode as word;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        ctlpanel.playermode[i as usize] = playermode[i as usize] as word;
        ctlpanel.JoyXlow[i as usize] = JoyXlow[i as usize] as sword;
        ctlpanel.JoyYlow[i as usize] = JoyYlow[i as usize] as sword;
        ctlpanel.JoyXhigh[i as usize] = JoyXhigh[i as usize] as sword;
        ctlpanel.JoyYhigh[i as usize] = JoyYhigh[i as usize] as sword;
        i = i.wrapping_add(1);
    }
    ctlpanel.MouseSensitivity = MouseSensitivity as sword;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8 as libc::c_int as libc::c_uint {
        ctlpanel.key[i as usize] = ScancodeToDOS(key[i as usize] as SDL_Scancode) as byte;
        i = i.wrapping_add(1);
    }
    ctlpanel.keyB1 = ScancodeToDOS(keyB1 as SDL_Scancode) as byte;
    ctlpanel.keyB2 = ScancodeToDOS(keyB2 as SDL_Scancode) as byte;
    write(
        handle,
        &mut ctlpanel as *mut ctlpaneltype as *const libc::c_void,
        ::std::mem::size_of::<ctlpaneltype>() as libc::c_ulong,
    );
    close(handle);
}
#[no_mangle]
#[c2rust::src_loc = "1316:1"]
pub unsafe extern "C" fn _loadhighscores() {
    let mut i: libc::c_int = 0;
    strcpy(
        str.as_mut_ptr(),
        b"SCORES.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    if LoadFile(
        str.as_mut_ptr(),
        highscores.as_mut_ptr() as *mut libc::c_char,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            highscores[i as usize].score = 100 as libc::c_int;
            highscores[i as usize].level = 1 as libc::c_int as sword;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"PCR\0" as *const u8 as *const libc::c_char,
            );
            i += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1331:1"]
pub unsafe extern "C" fn _savehighscores() {
    strcpy(
        str.as_mut_ptr(),
        b"SCORES.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    SaveFile(
        str.as_mut_ptr(),
        highscores.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[scores; 5]>() as libc::c_ulong as libc::c_long,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1345:1"]
pub unsafe extern "C" fn _showhighscores() {
    let mut i: libc::c_int = 0;
    let mut h: libc::c_long = 0;
    let mut st2: [libc::c_char; 10] = [0; 10];
    centerwindow(17 as libc::c_int, 17 as libc::c_int);
    print(b"\n   HIGH SCORES\n\n\0" as *const u8 as *const libc::c_char);
    print(b" #  SCORE LV  BY\n\0" as *const u8 as *const libc::c_char);
    print(b" - ------ -- ---\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        sx += 1;
        drawchar(sx, sy, '1' as i32 + i);
        sx += 2 as libc::c_int;
        h = highscores[i as usize].score as libc::c_long;
        if h < 100000 as libc::c_long {
            sx += 1;
        }
        if h < 10000 as libc::c_long {
            sx += 1;
        }
        if h < 1000 as libc::c_long {
            sx += 1;
        }
        if h < 100 as libc::c_long {
            sx += 1;
        }
        if h < 10 as libc::c_long {
            sx += 1;
        }
        ltoa(h as libc::c_int, str.as_mut_ptr(), 10 as libc::c_int);
        print(str.as_mut_ptr());
        sx += 1;
        if (highscores[i as usize].level as libc::c_int) < 10 as libc::c_int {
            sx += 1;
        }
        itoa(
            highscores[i as usize].level as libc::c_int,
            str.as_mut_ptr(),
            10 as libc::c_int,
        );
        print(str.as_mut_ptr());
        sx += 1;
        print((highscores[i as usize].initials).as_mut_ptr());
        print(b"\n\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    strcpy(
        str.as_mut_ptr(),
        b"SCORE:\0" as *const u8 as *const libc::c_char,
    );
    ltoa(score, st2.as_mut_ptr(), 10 as libc::c_int);
    strcat(str.as_mut_ptr(), st2.as_mut_ptr());
    _printc(str.as_mut_ptr());
}
#[no_mangle]
#[c2rust::src_loc = "1397:1"]
pub unsafe extern "C" fn _checkhighscore() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        if score > highscores[i as usize].score {
            j = 4 as libc::c_int;
            while i < j {
                k = j - 1 as libc::c_int;
                highscores[j as usize] = highscores[k as usize];
                j -= 1;
            }
            highscores[i as usize].score = score;
            highscores[i as usize].level = level;
            strcpy(
                (highscores[i as usize].initials).as_mut_ptr(),
                b"   \0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            i += 1;
        }
    }
    _showhighscores();
    UpdateScreen();
    if i < 5 as libc::c_int {
        PlaySound(16 as libc::c_int);
        clearkeys();
        sx = screencenterx - 17 as libc::c_int / 2 as libc::c_int + 14 as libc::c_int;
        sy = screencentery - 17 as libc::c_int / 2 as libc::c_int
            + 6 as libc::c_int
            + i * 2 as libc::c_int;
        j = 0 as libc::c_int;
        loop {
            k = get();
            ch = k as libc::c_char;
            if ch as libc::c_int >= ' ' as i32 && j < 3 as libc::c_int {
                drawchar(sx, sy, ch as libc::c_int);
                sx += 1;
                highscores[i as usize].initials[j as usize] = ch;
                j += 1;
            }
            if ch as libc::c_int == 8 as libc::c_int || k == 19200 as libc::c_int {
                if j > 0 as libc::c_int {
                    sx -= 1;
                    j -= 1;
                }
            }
            if !(ch as libc::c_int != 13 as libc::c_int) {
                break;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1456:6"]
pub static mut screenseg: [byte; 64000] = [0; 64000];
#[c2rust::src_loc = "1458:20"]
static mut VideoParmStrings: [*const libc::c_char; 3] = [
    b"windowed\0" as *const u8 as *const libc::c_char,
    b"screen\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
#[c2rust::src_loc = "1460:1"]
pub unsafe extern "C" fn _setupgame() {
    if SDL_Init(
        0x20 as libc::c_uint | 0x1 as libc::c_uint | 0x200 as libc::c_uint | 0x2000 as libc::c_uint,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Failed to initialize SDL: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    atexit(Some(SDL_Quit as unsafe extern "C" fn() -> ()));
    SDL_AddEventWatch(
        Some(
            WatchUIEvents as unsafe extern "C" fn(*mut libc::c_void, *mut SDL_Event) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    let mut i: libc::c_int = 0;
    let mut windowed: boolean = false_0 as libc::c_int as boolean;
    let mut winWidth: libc::c_uint = 640 as libc::c_int as libc::c_uint;
    let mut winHeight: libc::c_uint = 480 as libc::c_int as libc::c_uint;
    let mut displayindex: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < _argc {
        match US_CheckParm(
            *_argv.offset(i as isize),
            VideoParmStrings.as_mut_ptr() as *mut *mut libc::c_char,
        ) {
            0 => {
                windowed = true_0 as libc::c_int as boolean;
                i += 1;
                if i < _argc {
                    winWidth = atoi(*_argv.offset(i as isize)) as libc::c_uint;
                }
                i += 1;
                if i < _argc {
                    winHeight = atoi(*_argv.offset(i as isize)) as libc::c_uint;
                }
            }
            1 => {
                i += 1;
                if i < _argc {
                    displayindex = atoi(*_argv.offset(i as isize));
                }
            }
            _ => {}
        }
        i += 1;
    }
    let mut bounds: SDL_Rect = SDL_Rect {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
    if SDL_GetCurrentDisplayMode(displayindex, &mut mode) < -(1 as libc::c_int)
        || SDL_GetDisplayBounds(displayindex, &mut bounds) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not get display mode: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    if windowed != 0 {
        bounds.x = (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int;
        bounds.y = (0x1fff0000 as libc::c_uint | 0 as libc::c_int as libc::c_uint) as libc::c_int;
        mode.w = winWidth as libc::c_int;
        mode.h = winHeight as libc::c_int;
    }
    window = SDL_CreateWindow(
        b"The Catacomb\0" as *const u8 as *const libc::c_char,
        bounds.x,
        bounds.y,
        mode.w,
        mode.h,
        (if windowed as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            SDL_WINDOW_FULLSCREEN_DESKTOP as libc::c_int
        }) as Uint32,
    );
    if window.is_null() || {
        renderer = SDL_CreateRenderer(window, -(1 as libc::c_int), 0 as libc::c_int as Uint32);
        renderer.is_null()
    } {
        fprintf(
            stderr,
            b"Failed to create SDL window: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    sdltexture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888 as libc::c_int as Uint32,
        SDL_TEXTUREACCESS_STREAMING as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
    );
    if sdltexture.is_null() {
        fprintf(
            stderr,
            b"Could not create video buffer: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        exit(1 as libc::c_int);
    }
    if mode.w == 320 as libc::c_int && mode.h == 200 as libc::c_int
        || mode.w == 640 as libc::c_int && mode.h == 400 as libc::c_int
    {
        updateRect.w = mode.w;
        updateRect.h = mode.h;
        updateRect.y = 0 as libc::c_int;
        updateRect.x = updateRect.y;
    } else {
        updateRect.h = mode.h;
        updateRect.w = mode.h * 4 as libc::c_int / 3 as libc::c_int;
        updateRect.x = mode.w - updateRect.w >> 1 as libc::c_int;
        updateRect.y = 0 as libc::c_int;
    }
    memset(
        screenseg.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[byte; 64000]>() as libc::c_ulong,
    );
    grmode = EGAgr;
    joystick[2 as libc::c_int as usize].device = -(1 as libc::c_int);
    joystick[1 as libc::c_int as usize].device = joystick[2 as libc::c_int as usize].device;
    _loadctrls();
    if grmode as libc::c_uint == VGAgr as libc::c_int as libc::c_uint && _vgaok as libc::c_int != 0
    {
        grmode = VGAgr;
    } else if grmode as libc::c_uint >= EGAgr as libc::c_int as libc::c_uint
        && _egaok as libc::c_int != 0
    {
        grmode = EGAgr;
    } else {
        grmode = CGAgr;
    }
    strcpy(
        str.as_mut_ptr(),
        b"SOUNDS.\0" as *const u8 as *const libc::c_char,
    );
    strcat(str.as_mut_ptr(), _extension);
    SoundData = bloadin(str.as_mut_ptr()) as *mut SPKRtable;
    StartupSound();
    SetupKBD();
    initrndt(1 as libc::c_int as boolean);
    initrnd(1 as libc::c_int as boolean);
    _loadhighscores();
    loadgrfiles();
    SetupEmulatedVBL();
}
#[no_mangle]
#[c2rust::src_loc = "1584:1"]
pub unsafe extern "C" fn _quit(mut error: *mut libc::c_char) {
    if *error == 0 {
        _savehighscores();
        _savectrls();
    } else {
        puts(error);
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        puts(
            b"For techinical assistance with running this software\n\0" as *const u8
                as *const libc::c_char,
        );
        puts(
            b"    call Softdisk Publishing at 1-318-221-8311\n\0" as *const u8
                as *const libc::c_char,
        );
        puts(b"\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    ShutdownSound();
    ShutdownJoysticks();
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    renderer = 0 as *mut SDL_Renderer;
    window = 0 as *mut SDL_Window;
    exit(0 as libc::c_int);
}
