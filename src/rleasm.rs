// Rust Port: Not intended to be fast; the substance is pretty much the same, the difference being
// that indices are used, rather than pointers.
// C2Rust introduces some very confusing `fresh*` variables, which are redundant.
pub fn RLEExpand(source: &[u8], dest: &mut [u8]) {
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
