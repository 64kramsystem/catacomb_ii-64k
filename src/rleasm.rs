use ::libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type byte = uint8_t;
#[no_mangle]
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
            let mut len: byte = ((val as libc::c_int & 0x7f as libc::c_int) + 1) as byte;
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                len as libc::c_ulong,
            );
            source = source.offset(len as libc::c_int as isize);
            dest = dest.offset(len as libc::c_int as isize);
        } else {
            let mut len_0: byte = (val as libc::c_int + 3) as byte;
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
