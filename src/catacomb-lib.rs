#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(extern_types)]
#![warn(
    clippy::assign_op_pattern,
    clippy::correctness,
    clippy::precedence,
    clippy::unnecessary_mut_passed,
    dead_code,
    unused_mut
)]
#![allow(
    clippy::identity_op,
    clippy::int_plus_one,
    clippy::missing_safety_doc,
    clippy::needless_return,
    clippy::nonminimal_bool,
    clippy::unnecessary_cast,
    clippy::wildcard_in_or_patterns,
    clippy::zero_ptr
)]

mod active_obj;
mod cat_play;
pub mod catacomb;
mod catasm;
mod class_type;
mod control_struct;
mod cpanel;
mod cpanel_state;
mod ctl_panel_type;
mod demo_enum;
mod dir_type;
mod exit_type;
mod extra_constants;
mod extra_macros;
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
mod pic_file_type;
mod pic_type;
mod rleasm;
mod safe_sdl;
mod scan_codes;
mod scores;
mod sound_type;
mod spkr_table;
mod spksndtype;
mod sprite_type;
mod state_type;
mod tag_type;
mod think_type;
mod vec2;
