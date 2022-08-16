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
mod active_obj;
mod cat_play;
pub mod catacomb;
mod catasm;
mod class_type;
mod control_struct;
mod cpanel;
mod cpanel_state;
mod demo_enum;
mod dir_type;
mod exit_type;
mod extra_constants;
mod extra_macros;
mod extra_types;
mod global_state;
mod gr_type;
mod input_type;
mod obj_def_type;
mod obj_type;
mod objects;
mod pcrlib_a;
mod pcrlib_a_state;
mod pcrlib_c;
mod pcrlib_c_state;
mod rleasm;
mod safe_sdl;
mod scan_codes;
mod scores;
mod sound_type;
mod spkr_table;
mod spksndtype;
mod state_type;
mod tag_type;
mod think_type;
mod vec2;
