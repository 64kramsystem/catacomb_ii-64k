use ::libc;

use crate::think_type::thinktype::*;

extern "C" {
    static mut objdef: [objdeftype; 23];
}

pub type C2RustUnnamed_1 = libc::c_uint;
pub const nukeshot: C2RustUnnamed_1 = 4;
pub const mshot: C2RustUnnamed_1 = 3;
pub const pshot: C2RustUnnamed_1 = 2;
pub const monster: C2RustUnnamed_1 = 1;
pub const benign: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const lastclass: C2RustUnnamed_2 = 23;
pub const guns: C2RustUnnamed_2 = 22;
pub const gune: C2RustUnnamed_2 = 21;
pub const secretgate: C2RustUnnamed_2 = 20;
pub const torch: C2RustUnnamed_2 = 19;
pub const teleporter: C2RustUnnamed_2 = 18;
pub const dead6: C2RustUnnamed_2 = 17;
pub const dead5: C2RustUnnamed_2 = 16;
pub const dead4: C2RustUnnamed_2 = 15;
pub const dead3: C2RustUnnamed_2 = 14;
pub const dead2: C2RustUnnamed_2 = 13;
pub const dead1: C2RustUnnamed_2 = 12;
pub const rock: C2RustUnnamed_2 = 11;
pub const bigshot: C2RustUnnamed_2 = 10;
pub const shot: C2RustUnnamed_2 = 9;
pub const wallhit: C2RustUnnamed_2 = 8;
pub const turbogre: C2RustUnnamed_2 = 7;
pub const dragon: C2RustUnnamed_2 = 6;
pub const gargoyle: C2RustUnnamed_2 = 5;
pub const ogre: C2RustUnnamed_2 = 4;
pub const skeleton: C2RustUnnamed_2 = 3;
pub const goblin: C2RustUnnamed_2 = 2;
pub const player: C2RustUnnamed_2 = 1;
pub const nothing: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct objdeftype {
    pub think: u8,
    pub contact: u8,
    pub solid: u8,
    pub firstchar: u16,
    pub size: u8,
    pub stages: u8,
    pub dirmask: u8,
    pub speed: u16,
    pub hitpoints: u8,
    pub damage: u8,
    pub points: u16,
    pub filler: [u8; 2],
}
#[no_mangle]
pub unsafe extern "C" fn initobjects() {
    objdef[player as libc::c_int as usize].think = playercmd as libc::c_int as u8;
    objdef[player as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[player as libc::c_int as usize].solid = true as u8;
    objdef[player as libc::c_int as usize].firstchar = 256;
    objdef[player as libc::c_int as usize].size = 2;
    objdef[player as libc::c_int as usize].stages = 4;
    objdef[player as libc::c_int as usize].dirmask = 3;
    objdef[player as libc::c_int as usize].speed = 256;
    objdef[player as libc::c_int as usize].hitpoints = 12;
    objdef[player as libc::c_int as usize].damage = 0;
    objdef[player as libc::c_int as usize].points = 0;
    objdef[goblin as libc::c_int as usize].think = ramstraight as libc::c_int as u8;
    objdef[goblin as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[goblin as libc::c_int as usize].solid = true as u8;
    objdef[goblin as libc::c_int as usize].firstchar = (256 + 64) as u16;
    objdef[goblin as libc::c_int as usize].size = 2;
    objdef[goblin as libc::c_int as usize].stages = 4;
    objdef[goblin as libc::c_int as usize].dirmask = 3;
    objdef[goblin as libc::c_int as usize].speed = 75;
    objdef[goblin as libc::c_int as usize].hitpoints = 1;
    objdef[goblin as libc::c_int as usize].damage = 1;
    objdef[goblin as libc::c_int as usize].points = 50;
    objdef[skeleton as libc::c_int as usize].think = ramdiag as libc::c_int as u8;
    objdef[skeleton as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[skeleton as libc::c_int as usize].solid = true as u8;
    objdef[skeleton as libc::c_int as usize].firstchar = (256 + 128) as u16;
    objdef[skeleton as libc::c_int as usize].size = 2;
    objdef[skeleton as libc::c_int as usize].stages = 4;
    objdef[skeleton as libc::c_int as usize].dirmask = 3;
    objdef[skeleton as libc::c_int as usize].speed = 130;
    objdef[skeleton as libc::c_int as usize].hitpoints = 1;
    objdef[skeleton as libc::c_int as usize].damage = 1;
    objdef[skeleton as libc::c_int as usize].points = 150;
    objdef[ogre as libc::c_int as usize].think = ramstraight as libc::c_int as u8;
    objdef[ogre as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[ogre as libc::c_int as usize].solid = true as u8;
    objdef[ogre as libc::c_int as usize].firstchar = (256 + 67 * 4) as u16;
    objdef[ogre as libc::c_int as usize].size = 3;
    objdef[ogre as libc::c_int as usize].stages = 4;
    objdef[ogre as libc::c_int as usize].dirmask = 3;
    objdef[ogre as libc::c_int as usize].speed = 75;
    objdef[ogre as libc::c_int as usize].hitpoints = 5;
    objdef[ogre as libc::c_int as usize].damage = 2;
    objdef[ogre as libc::c_int as usize].points = 250;
    objdef[gargoyle as libc::c_int as usize].think = gargcmd as libc::c_int as u8;
    objdef[gargoyle as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[gargoyle as libc::c_int as usize].solid = true as u8;
    objdef[gargoyle as libc::c_int as usize].firstchar = (256 + 67 * 4 + 35 * 9) as u16;
    objdef[gargoyle as libc::c_int as usize].size = 4;
    objdef[gargoyle as libc::c_int as usize].stages = 4;
    objdef[gargoyle as libc::c_int as usize].dirmask = 3;
    objdef[gargoyle as libc::c_int as usize].speed = 175;
    objdef[gargoyle as libc::c_int as usize].hitpoints = 10;
    objdef[gargoyle as libc::c_int as usize].damage = 3;
    objdef[gargoyle as libc::c_int as usize].points = 500;
    objdef[dragon as libc::c_int as usize].think = dragoncmd as libc::c_int as u8;
    objdef[dragon as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[dragon as libc::c_int as usize].solid = true as u8;
    objdef[dragon as libc::c_int as usize].firstchar = (256 + 67 * 4 + 35 * 9 + 19 * 16) as u16;
    objdef[dragon as libc::c_int as usize].size = 5;
    objdef[dragon as libc::c_int as usize].stages = 4;
    objdef[dragon as libc::c_int as usize].dirmask = 3;
    objdef[dragon as libc::c_int as usize].speed = 100;
    objdef[dragon as libc::c_int as usize].hitpoints = 75;
    objdef[dragon as libc::c_int as usize].damage = 5;
    objdef[dragon as libc::c_int as usize].points = 1000;
    objdef[turbogre as libc::c_int as usize].think = ramstraight as libc::c_int as u8;
    objdef[turbogre as libc::c_int as usize].contact = monster as libc::c_int as u8;
    objdef[turbogre as libc::c_int as usize].solid = true as u8;
    objdef[turbogre as libc::c_int as usize].firstchar = (256 + 67 * 4 + 19 * 9) as u16;
    objdef[turbogre as libc::c_int as usize].size = 3;
    objdef[turbogre as libc::c_int as usize].stages = 4;
    objdef[turbogre as libc::c_int as usize].dirmask = 3;
    objdef[turbogre as libc::c_int as usize].speed = 255;
    objdef[turbogre as libc::c_int as usize].hitpoints = 5;
    objdef[turbogre as libc::c_int as usize].damage = 2;
    objdef[turbogre as libc::c_int as usize].points = 500;
    objdef[wallhit as libc::c_int as usize].think = fade as libc::c_int as u8;
    objdef[wallhit as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[wallhit as libc::c_int as usize].solid = true as u8;
    objdef[wallhit as libc::c_int as usize].firstchar = 26;
    objdef[wallhit as libc::c_int as usize].size = 1;
    objdef[wallhit as libc::c_int as usize].stages = 3;
    objdef[wallhit as libc::c_int as usize].dirmask = 0;
    objdef[wallhit as libc::c_int as usize].speed = 80;
    objdef[wallhit as libc::c_int as usize].hitpoints = 0;
    objdef[wallhit as libc::c_int as usize].damage = 0;
    objdef[wallhit as libc::c_int as usize].points = 0;
    objdef[dead1 as libc::c_int as usize].think = explode as libc::c_int as u8;
    objdef[dead1 as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[dead1 as libc::c_int as usize].solid = false as u8;
    objdef[dead1 as libc::c_int as usize].firstchar = 29;
    objdef[dead1 as libc::c_int as usize].size = 1;
    objdef[dead1 as libc::c_int as usize].stages = 3;
    objdef[dead1 as libc::c_int as usize].dirmask = 0;
    objdef[dead1 as libc::c_int as usize].speed = 80;
    objdef[dead1 as libc::c_int as usize].hitpoints = 0;
    objdef[dead1 as libc::c_int as usize].damage = 0;
    objdef[dead1 as libc::c_int as usize].points = 0;
    objdef[dead2 as libc::c_int as usize].think = fade as libc::c_int as u8;
    objdef[dead2 as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[dead2 as libc::c_int as usize].solid = false as u8;
    objdef[dead2 as libc::c_int as usize].firstchar = (256 + 224) as u16;
    objdef[dead2 as libc::c_int as usize].size = 2;
    objdef[dead2 as libc::c_int as usize].stages = 3;
    objdef[dead2 as libc::c_int as usize].dirmask = 0;
    objdef[dead2 as libc::c_int as usize].speed = 80;
    objdef[dead2 as libc::c_int as usize].hitpoints = 0;
    objdef[dead2 as libc::c_int as usize].damage = 0;
    objdef[dead2 as libc::c_int as usize].points = 0;
    objdef[dead3 as libc::c_int as usize].think = fade as libc::c_int as u8;
    objdef[dead3 as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[dead3 as libc::c_int as usize].solid = false as u8;
    objdef[dead3 as libc::c_int as usize].firstchar = (256 + 67 * 4 + 9 * 16) as u16;
    objdef[dead3 as libc::c_int as usize].size = 3;
    objdef[dead3 as libc::c_int as usize].stages = 3;
    objdef[dead3 as libc::c_int as usize].dirmask = 0;
    objdef[dead3 as libc::c_int as usize].speed = 80;
    objdef[dead3 as libc::c_int as usize].hitpoints = 0;
    objdef[dead3 as libc::c_int as usize].damage = 0;
    objdef[dead3 as libc::c_int as usize].points = 0;
    objdef[dead4 as libc::c_int as usize].think = fade as libc::c_int as u8;
    objdef[dead4 as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[dead4 as libc::c_int as usize].solid = false as u8;
    objdef[dead4 as libc::c_int as usize].firstchar = (256 + 67 * 4 + 35 * 9 + 16 * 16) as u16;
    objdef[dead4 as libc::c_int as usize].size = 4;
    objdef[dead4 as libc::c_int as usize].stages = 3;
    objdef[dead4 as libc::c_int as usize].dirmask = 0;
    objdef[dead4 as libc::c_int as usize].speed = 80;
    objdef[dead4 as libc::c_int as usize].hitpoints = 0;
    objdef[dead4 as libc::c_int as usize].damage = 0;
    objdef[dead4 as libc::c_int as usize].points = 0;
    objdef[dead5 as libc::c_int as usize].think = fade as libc::c_int as u8;
    objdef[dead5 as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[dead5 as libc::c_int as usize].solid = false as u8;
    objdef[dead5 as libc::c_int as usize].firstchar =
        (256 + 67 * 4 + 35 * 9 + 19 * 16 + 25 * 16) as u16;
    objdef[dead5 as libc::c_int as usize].size = 5;
    objdef[dead5 as libc::c_int as usize].stages = 3;
    objdef[dead5 as libc::c_int as usize].dirmask = 0;
    objdef[dead5 as libc::c_int as usize].speed = 80;
    objdef[dead5 as libc::c_int as usize].hitpoints = 0;
    objdef[dead5 as libc::c_int as usize].damage = 0;
    objdef[dead5 as libc::c_int as usize].points = 0;
    objdef[shot as libc::c_int as usize].think = straight as libc::c_int as u8;
    objdef[shot as libc::c_int as usize].contact = pshot as libc::c_int as u8;
    objdef[shot as libc::c_int as usize].solid = false as u8;
    objdef[shot as libc::c_int as usize].firstchar = 154;
    objdef[shot as libc::c_int as usize].size = 1;
    objdef[shot as libc::c_int as usize].stages = 2;
    objdef[shot as libc::c_int as usize].dirmask = 3;
    objdef[shot as libc::c_int as usize].speed = 256;
    objdef[shot as libc::c_int as usize].hitpoints = 0;
    objdef[shot as libc::c_int as usize].damage = 1;
    objdef[shot as libc::c_int as usize].points = 0;
    objdef[guns as libc::c_int as usize].think = gunthinks as libc::c_int as u8;
    objdef[guns as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[guns as libc::c_int as usize].solid = true as u8;
    objdef[guns as libc::c_int as usize].firstchar = (256 + 67 * 4 - 8) as u16;
    objdef[guns as libc::c_int as usize].size = 2;
    objdef[guns as libc::c_int as usize].stages = 1;
    objdef[guns as libc::c_int as usize].dirmask = 0;
    objdef[guns as libc::c_int as usize].speed = 10;
    objdef[guns as libc::c_int as usize].hitpoints = 255;
    objdef[guns as libc::c_int as usize].damage = 0;
    objdef[guns as libc::c_int as usize].points = 0;
    objdef[gune as libc::c_int as usize].think = gunthinke as libc::c_int as u8;
    objdef[gune as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[gune as libc::c_int as usize].solid = true as u8;
    objdef[gune as libc::c_int as usize].firstchar = (256 + 67 * 4 - 4) as u16;
    objdef[gune as libc::c_int as usize].size = 2;
    objdef[gune as libc::c_int as usize].stages = 1;
    objdef[gune as libc::c_int as usize].dirmask = 0;
    objdef[gune as libc::c_int as usize].speed = 10;
    objdef[gune as libc::c_int as usize].hitpoints = 255;
    objdef[gune as libc::c_int as usize].damage = 0;
    objdef[gune as libc::c_int as usize].points = 0;
    objdef[rock as libc::c_int as usize].think = straight as libc::c_int as u8;
    objdef[rock as libc::c_int as usize].contact = mshot as libc::c_int as u8;
    objdef[rock as libc::c_int as usize].solid = false as u8;
    objdef[rock as libc::c_int as usize].firstchar = 153;
    objdef[rock as libc::c_int as usize].size = 1;
    objdef[rock as libc::c_int as usize].stages = 2;
    objdef[rock as libc::c_int as usize].dirmask = 0;
    objdef[rock as libc::c_int as usize].speed = 256;
    objdef[rock as libc::c_int as usize].hitpoints = 0;
    objdef[rock as libc::c_int as usize].damage = 1;
    objdef[rock as libc::c_int as usize].points = 0;
    objdef[bigshot as libc::c_int as usize].think = straight as libc::c_int as u8;
    objdef[bigshot as libc::c_int as usize].contact = nukeshot as libc::c_int as u8;
    objdef[bigshot as libc::c_int as usize].solid = false as u8;
    objdef[bigshot as libc::c_int as usize].firstchar = (256 + 192) as u16;
    objdef[bigshot as libc::c_int as usize].size = 2;
    objdef[bigshot as libc::c_int as usize].stages = 2;
    objdef[bigshot as libc::c_int as usize].dirmask = 3;
    objdef[bigshot as libc::c_int as usize].speed = 256;
    objdef[bigshot as libc::c_int as usize].hitpoints = 0;
    objdef[bigshot as libc::c_int as usize].damage = 1;
    objdef[bigshot as libc::c_int as usize].points = 0;
    objdef[teleporter as libc::c_int as usize].think = idle as libc::c_int as u8;
    objdef[teleporter as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[teleporter as libc::c_int as usize].solid = false as u8;
    objdef[teleporter as libc::c_int as usize].firstchar = (256 + 236) as u16;
    objdef[teleporter as libc::c_int as usize].size = 2;
    objdef[teleporter as libc::c_int as usize].stages = 5;
    objdef[teleporter as libc::c_int as usize].dirmask = 0;
    objdef[teleporter as libc::c_int as usize].speed = 200;
    objdef[teleporter as libc::c_int as usize].hitpoints = 0;
    objdef[teleporter as libc::c_int as usize].damage = 0;
    objdef[teleporter as libc::c_int as usize].points = 0;
    objdef[secretgate as libc::c_int as usize].think = idle as libc::c_int as u8;
    objdef[secretgate as libc::c_int as usize].contact = benign as libc::c_int as u8;
    objdef[secretgate as libc::c_int as usize].solid = false as u8;
    objdef[secretgate as libc::c_int as usize].firstchar = (256 + 67 * 4 - 12) as u16;
    objdef[secretgate as libc::c_int as usize].size = 2;
    objdef[secretgate as libc::c_int as usize].stages = 1;
    objdef[secretgate as libc::c_int as usize].dirmask = 0;
    objdef[secretgate as libc::c_int as usize].speed = 200;
    objdef[secretgate as libc::c_int as usize].hitpoints = 0;
    objdef[secretgate as libc::c_int as usize].damage = 0;
    objdef[secretgate as libc::c_int as usize].points = 0;
}
