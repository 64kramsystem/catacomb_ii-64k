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

use demo_enum::demoenum;

extern crate libc;
mod active_obj;
mod cat_play;
pub mod catacomb;
mod catasm;
mod class_type;
mod control_struct;
mod cpanel;
mod demo_enum;
mod dir_type;
mod exit_type;
mod extra_constants;
mod extra_macros;
mod extra_types;
mod global_state;
mod gr_type;
mod obj_def_type;
mod obj_type;
mod objects;
mod pcrlib_a;
mod pcrlib_c;
mod rleasm;
mod scores;
mod sdl_scan_codes;
mod state_type;
mod tag_type;
mod think_type;

static mut indemo: demoenum = demoenum::notdemo;
