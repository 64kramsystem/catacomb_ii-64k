#![allow(clippy::all)]
#![deny(clippy::correctness)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;
pub mod cat_play;
pub mod catasm;
pub mod cpanel;
pub mod objects;
pub mod pcrlib_a;
pub mod pcrlib_c;
pub mod rleasm;
