use ::libc;

extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}

pub unsafe fn RLEExpand(mut source: *mut i8, mut dest: *const i8, origlen: i64) {
    let end: *const i8 = dest.offset(origlen as isize);
    while dest < end as *mut i8 {
        let fresh0 = source;
        source = source.offset(1);
        let val: u8 = *(fresh0 as *mut u8);
        if val as i32 & 0x80 as i32 != 0 {
            let len: u8 = ((val as i32 & 0x7f as i32) + 1) as u8;
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                len as u64,
            );
            source = source.offset(len as i32 as isize);
            dest = dest.offset(len as i32 as isize);
        } else {
            let len_0: u8 = (val as i32 + 3) as u8;
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

// Rust Port: Not intended to be fast; the substance is pretty much the same, the difference being
// that indices are used, rather than pointers.
// C2Rust introduces some very confusing `fresh*` variables, which are redundant.
pub fn port_temp_RLEExpand(source: &[u8], dest: &mut [u8]) {
    let mut source_i = 0;
    let mut dest_i = 0;

    while dest_i < dest.len() {
        let val: u8 = source[source_i];
        source_i += 1;

        if val & 0x80 != 0 {
            let len = ((val & 0x7f) + 1) as usize;
            dest[dest_i..dest_i + len].copy_from_slice(&source[source_i..source_i + len]);
            source_i += len;
            dest_i += len;
        } else {
            let len = (val + 3) as usize;
            dest[dest_i..dest_i + len].fill(source[source_i]);
            source_i += 1;
            dest_i += len;
        }
    }
}
