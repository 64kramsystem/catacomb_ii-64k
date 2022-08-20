use ::libc;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h:19"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    use super::types_h::{__uint16_t, __uint8_t};
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catdefs.h:19"]
pub mod catdefs_h {
    #[c2rust::src_loc = "34:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "34:13"]
    pub const true_0: C2RustUnnamed = 1;
    #[c2rust::src_loc = "34:7"]
    pub const false_0: C2RustUnnamed = 0;
    #[c2rust::src_loc = "37:1"]
    pub type byte = uint8_t;
    #[c2rust::src_loc = "39:1"]
    pub type word = uint16_t;
    #[c2rust::src_loc = "83:9"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "84:28"]
    pub const gunthinks: C2RustUnnamed_0 = 10;
    #[c2rust::src_loc = "84:18"]
    pub const gunthinke: C2RustUnnamed_0 = 9;
    #[c2rust::src_loc = "84:10"]
    pub const explode: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "84:5"]
    pub const fade: C2RustUnnamed_0 = 7;
    #[c2rust::src_loc = "83:72"]
    pub const idle: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "83:63"]
    pub const straight: C2RustUnnamed_0 = 5;
    #[c2rust::src_loc = "83:55"]
    pub const ramdiag: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "83:43"]
    pub const ramstraight: C2RustUnnamed_0 = 3;
    #[c2rust::src_loc = "83:33"]
    pub const dragoncmd: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "83:25"]
    pub const gargcmd: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "83:15"]
    pub const playercmd: C2RustUnnamed_0 = 0;
    #[c2rust::src_loc = "86:9"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "86:42"]
    pub const nukeshot: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "86:36"]
    pub const mshot: C2RustUnnamed_1 = 3;
    #[c2rust::src_loc = "86:30"]
    pub const pshot: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "86:22"]
    pub const monster: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "86:15"]
    pub const benign: C2RustUnnamed_1 = 0;
    #[c2rust::src_loc = "88:9"]
    pub type C2RustUnnamed_2 = libc::c_uint;
    #[c2rust::src_loc = "90:32"]
    pub const lastclass: C2RustUnnamed_2 = 23;
    #[c2rust::src_loc = "90:27"]
    pub const guns: C2RustUnnamed_2 = 22;
    #[c2rust::src_loc = "90:22"]
    pub const gune: C2RustUnnamed_2 = 21;
    #[c2rust::src_loc = "90:11"]
    pub const secretgate: C2RustUnnamed_2 = 20;
    #[c2rust::src_loc = "90:5"]
    pub const torch: C2RustUnnamed_2 = 19;
    #[c2rust::src_loc = "89:67"]
    pub const teleporter: C2RustUnnamed_2 = 18;
    #[c2rust::src_loc = "89:61"]
    pub const dead6: C2RustUnnamed_2 = 17;
    #[c2rust::src_loc = "89:55"]
    pub const dead5: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "89:49"]
    pub const dead4: C2RustUnnamed_2 = 15;
    #[c2rust::src_loc = "89:43"]
    pub const dead3: C2RustUnnamed_2 = 14;
    #[c2rust::src_loc = "89:37"]
    pub const dead2: C2RustUnnamed_2 = 13;
    #[c2rust::src_loc = "89:31"]
    pub const dead1: C2RustUnnamed_2 = 12;
    #[c2rust::src_loc = "89:26"]
    pub const rock: C2RustUnnamed_2 = 11;
    #[c2rust::src_loc = "89:18"]
    pub const bigshot: C2RustUnnamed_2 = 10;
    #[c2rust::src_loc = "89:13"]
    pub const shot: C2RustUnnamed_2 = 9;
    #[c2rust::src_loc = "89:5"]
    pub const wallhit: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "88:67"]
    pub const turbogre: C2RustUnnamed_2 = 7;
    #[c2rust::src_loc = "88:60"]
    pub const dragon: C2RustUnnamed_2 = 6;
    #[c2rust::src_loc = "88:51"]
    pub const gargoyle: C2RustUnnamed_2 = 5;
    #[c2rust::src_loc = "88:46"]
    pub const ogre: C2RustUnnamed_2 = 4;
    #[c2rust::src_loc = "88:37"]
    pub const skeleton: C2RustUnnamed_2 = 3;
    #[c2rust::src_loc = "88:30"]
    pub const goblin: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "88:23"]
    pub const player: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "88:15"]
    pub const nothing: C2RustUnnamed_2 = 0;
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "109:9"]
    pub struct objdeftype {
        pub think: byte,
        pub contact: byte,
        pub solid: byte,
        pub firstchar: word,
        pub size: byte,
        pub stages: byte,
        pub dirmask: byte,
        pub speed: word,
        pub hitpoints: byte,
        pub damage: byte,
        pub points: word,
        pub filler: [byte; 2],
    }
    use super::stdint_uintn_h::{uint16_t, uint8_t};
}
#[c2rust::header_src = "/home/saverio/code/catacomb_ii-64k/source_project/catacomb.h:20"]
pub mod catacomb_h {
    use super::catdefs_h::{byte, objdeftype, word};
    extern "C" {
        #[c2rust::src_loc = "76:21"]
        pub static mut objdef: [objdeftype; 23];
    }
}
use self::catacomb_h::objdef;
pub use self::catdefs_h::{
    benign, bigshot, byte, dead1, dead2, dead3, dead4, dead5, dead6, dragon, dragoncmd, explode,
    fade, false_0, gargcmd, gargoyle, goblin, gune, guns, gunthinke, gunthinks, idle, lastclass,
    monster, mshot, nothing, nukeshot, objdeftype, ogre, player, playercmd, pshot, ramdiag,
    ramstraight, rock, secretgate, shot, skeleton, straight, teleporter, torch, true_0, turbogre,
    wallhit, word, C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2,
};
pub use self::stdint_uintn_h::{uint16_t, uint8_t};
pub use self::types_h::{__uint16_t, __uint8_t};
#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn initobjects() {
    objdef[player as libc::c_int as usize].think = playercmd as libc::c_int as byte;
    objdef[player as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[player as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].firstchar = 256 as libc::c_int as word;
    objdef[player as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].speed = 256 as libc::c_int as word;
    objdef[player as libc::c_int as usize].hitpoints = 12 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[player as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[goblin as libc::c_int as usize].think = ramstraight as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 64 as libc::c_int) as word;
    objdef[goblin as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].speed = 75 as libc::c_int as word;
    objdef[goblin as libc::c_int as usize].hitpoints = 1 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].damage = 1 as libc::c_int as byte;
    objdef[goblin as libc::c_int as usize].points = 50 as libc::c_int as word;
    objdef[skeleton as libc::c_int as usize].think = ramdiag as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 128 as libc::c_int) as word;
    objdef[skeleton as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].speed = 130 as libc::c_int as word;
    objdef[skeleton as libc::c_int as usize].hitpoints = 1 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].damage = 1 as libc::c_int as byte;
    objdef[skeleton as libc::c_int as usize].points = 150 as libc::c_int as word;
    objdef[ogre as libc::c_int as usize].think = ramstraight as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 67 as libc::c_int * 4 as libc::c_int) as word;
    objdef[ogre as libc::c_int as usize].size = 3 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].speed = 75 as libc::c_int as word;
    objdef[ogre as libc::c_int as usize].hitpoints = 5 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].damage = 2 as libc::c_int as byte;
    objdef[ogre as libc::c_int as usize].points = 250 as libc::c_int as word;
    objdef[gargoyle as libc::c_int as usize].think = gargcmd as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 35 as libc::c_int * 9 as libc::c_int)
        as word;
    objdef[gargoyle as libc::c_int as usize].size = 4 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].speed = 175 as libc::c_int as word;
    objdef[gargoyle as libc::c_int as usize].hitpoints = 10 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].damage = 3 as libc::c_int as byte;
    objdef[gargoyle as libc::c_int as usize].points = 500 as libc::c_int as word;
    objdef[dragon as libc::c_int as usize].think = dragoncmd as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 35 as libc::c_int * 9 as libc::c_int
        + 19 as libc::c_int * 16 as libc::c_int)
        as word;
    objdef[dragon as libc::c_int as usize].size = 5 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].speed = 100 as libc::c_int as word;
    objdef[dragon as libc::c_int as usize].hitpoints = 75 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].damage = 5 as libc::c_int as byte;
    objdef[dragon as libc::c_int as usize].points = 1000 as libc::c_int as word;
    objdef[turbogre as libc::c_int as usize].think = ramstraight as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].contact = monster as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 19 as libc::c_int * 9 as libc::c_int)
        as word;
    objdef[turbogre as libc::c_int as usize].size = 3 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].stages = 4 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].speed = 255 as libc::c_int as word;
    objdef[turbogre as libc::c_int as usize].hitpoints = 5 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].damage = 2 as libc::c_int as byte;
    objdef[turbogre as libc::c_int as usize].points = 500 as libc::c_int as word;
    objdef[wallhit as libc::c_int as usize].think = fade as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].firstchar = 26 as libc::c_int as word;
    objdef[wallhit as libc::c_int as usize].size = 1 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[wallhit as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[wallhit as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[dead1 as libc::c_int as usize].think = explode as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].firstchar = 29 as libc::c_int as word;
    objdef[dead1 as libc::c_int as usize].size = 1 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[dead1 as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[dead1 as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[dead2 as libc::c_int as usize].think = fade as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 224 as libc::c_int) as word;
    objdef[dead2 as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[dead2 as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[dead2 as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[dead3 as libc::c_int as usize].think = fade as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 9 as libc::c_int * 16 as libc::c_int)
        as word;
    objdef[dead3 as libc::c_int as usize].size = 3 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[dead3 as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[dead3 as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[dead4 as libc::c_int as usize].think = fade as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 35 as libc::c_int * 9 as libc::c_int
        + 16 as libc::c_int * 16 as libc::c_int)
        as word;
    objdef[dead4 as libc::c_int as usize].size = 4 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[dead4 as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[dead4 as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[dead5 as libc::c_int as usize].think = fade as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].firstchar = (256 as libc::c_int
        + 67 as libc::c_int * 4 as libc::c_int
        + 35 as libc::c_int * 9 as libc::c_int
        + 19 as libc::c_int * 16 as libc::c_int
        + 25 as libc::c_int * 16 as libc::c_int)
        as word;
    objdef[dead5 as libc::c_int as usize].size = 5 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].stages = 3 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].speed = 80 as libc::c_int as word;
    objdef[dead5 as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[dead5 as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[shot as libc::c_int as usize].think = straight as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].contact = pshot as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].firstchar = 154 as libc::c_int as word;
    objdef[shot as libc::c_int as usize].size = 1 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].stages = 2 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].speed = 256 as libc::c_int as word;
    objdef[shot as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].damage = 1 as libc::c_int as byte;
    objdef[shot as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[guns as libc::c_int as usize].think = gunthinks as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 67 as libc::c_int * 4 as libc::c_int - 8 as libc::c_int) as word;
    objdef[guns as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].stages = 1 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].speed = 10 as libc::c_int as word;
    objdef[guns as libc::c_int as usize].hitpoints = 255 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[guns as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[gune as libc::c_int as usize].think = gunthinke as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].solid = true_0 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 67 as libc::c_int * 4 as libc::c_int - 4 as libc::c_int) as word;
    objdef[gune as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].stages = 1 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].speed = 10 as libc::c_int as word;
    objdef[gune as libc::c_int as usize].hitpoints = 255 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[gune as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[rock as libc::c_int as usize].think = straight as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].contact = mshot as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].firstchar = 153 as libc::c_int as word;
    objdef[rock as libc::c_int as usize].size = 1 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].stages = 2 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].speed = 256 as libc::c_int as word;
    objdef[rock as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].damage = 1 as libc::c_int as byte;
    objdef[rock as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[bigshot as libc::c_int as usize].think = straight as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].contact = nukeshot as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 192 as libc::c_int) as word;
    objdef[bigshot as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].stages = 2 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].dirmask = 3 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].speed = 256 as libc::c_int as word;
    objdef[bigshot as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].damage = 1 as libc::c_int as byte;
    objdef[bigshot as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[teleporter as libc::c_int as usize].think = idle as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 236 as libc::c_int) as word;
    objdef[teleporter as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].stages = 5 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].speed = 200 as libc::c_int as word;
    objdef[teleporter as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[teleporter as libc::c_int as usize].points = 0 as libc::c_int as word;
    objdef[secretgate as libc::c_int as usize].think = idle as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].solid = false_0 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].firstchar =
        (256 as libc::c_int + 67 as libc::c_int * 4 as libc::c_int - 12 as libc::c_int) as word;
    objdef[secretgate as libc::c_int as usize].size = 2 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].stages = 1 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].dirmask = 0 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].speed = 200 as libc::c_int as word;
    objdef[secretgate as libc::c_int as usize].hitpoints = 0 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].damage = 0 as libc::c_int as byte;
    objdef[secretgate as libc::c_int as usize].points = 0 as libc::c_int as word;
}
