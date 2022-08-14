use ::libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn RLEExpand(mut source: *mut i8, mut dest: *mut i8, mut origlen: i64) {
    let end: *const i8 = dest.offset(origlen as isize);
    while dest < end as *mut i8 {
        let fresh0 = source;
        source = source.offset(1);
        let mut val: u8 = *(fresh0 as *mut u8);
        if val as i32 & 0x80 as i32 != 0 {
            let mut len: u8 = ((val as i32 & 0x7f as i32) + 1) as u8;
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                len as u64,
            );
            source = source.offset(len as i32 as isize);
            dest = dest.offset(len as i32 as isize);
        } else {
            let mut len_0: u8 = (val as i32 + 3) as u8;
            let fresh1 = source;
            source = source.offset(1);
            memset(
                dest as *mut libc::c_void,
                *(fresh1 as *mut u8) as i32,
                len_0 as u64,
            );
            dest = dest.offset(len_0 as i32 as isize);
        }
    }
}
