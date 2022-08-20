use ::libc;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:20"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:20"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:20"]
pub mod catdefs_h {
    #[c2rust::src_loc = "37:1"]
    pub type byte = uint8_t;
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/usr/include/string.h:20"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
pub use self::catdefs_h::byte;
pub use self::stdint_uintn_h::uint8_t;
use self::string_h::{memcpy, memset};
pub use self::types_h::__uint8_t;
#[no_mangle]
#[c2rust::src_loc = "22:1"]
pub unsafe extern "C" fn RLEExpand(
    mut source: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut origlen: libc::c_long,
) {
    let end: *const libc::c_char = dest.offset(origlen as isize);
    while dest < end as *mut libc::c_char {
        let fresh0 = source;
        source = source.offset(1);
        let mut val: byte = *(fresh0 as *mut byte);
        if val as libc::c_int & 0x80 as libc::c_int != 0 {
            let mut len: byte =
                ((val as libc::c_int & 0x7f as libc::c_int) + 1 as libc::c_int) as byte;
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                len as libc::c_ulong,
            );
            source = source.offset(len as libc::c_int as isize);
            dest = dest.offset(len as libc::c_int as isize);
        } else {
            let mut len_0: byte = (val as libc::c_int + 3 as libc::c_int) as byte;
            let fresh1 = source;
            source = source.offset(1);
            memset(
                dest as *mut libc::c_void,
                *(fresh1 as *mut byte) as libc::c_int,
                len_0 as libc::c_ulong,
            );
            dest = dest.offset(len_0 as libc::c_int as isize);
        }
    }
}
