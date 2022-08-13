use ::libc;
extern "C" {
    static mut objdef: [objdeftype; 23];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type byte = uint8_t;
pub type word = uint16_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const gunthinks: C2RustUnnamed_0 = 10;
pub const gunthinke: C2RustUnnamed_0 = 9;
pub const explode: C2RustUnnamed_0 = 8;
pub const fade: C2RustUnnamed_0 = 7;
pub const idle: C2RustUnnamed_0 = 6;
pub const straight: C2RustUnnamed_0 = 5;
pub const ramdiag: C2RustUnnamed_0 = 4;
pub const ramstraight: C2RustUnnamed_0 = 3;
pub const dragoncmd: C2RustUnnamed_0 = 2;
pub const gargcmd: C2RustUnnamed_0 = 1;
pub const playercmd: C2RustUnnamed_0 = 0;
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
#[no_mangle]
pub unsafe extern "C" fn initobjects() {
    objdef[player as libc::c_int as usize].think = playercmd as libc::c_int as byte;
    objdef[player as libc::c_int as usize].contact = benign as libc::c_int as byte;
    objdef[player as libc::c_int as usize].solid = true as byte;
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
    objdef[goblin as libc::c_int as usize].solid = true as byte;
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
    objdef[skeleton as libc::c_int as usize].solid = true as byte;
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
    objdef[ogre as libc::c_int as usize].solid = true as byte;
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
    objdef[gargoyle as libc::c_int as usize].solid = true as byte;
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
    objdef[dragon as libc::c_int as usize].solid = true as byte;
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
    objdef[turbogre as libc::c_int as usize].solid = true as byte;
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
    objdef[wallhit as libc::c_int as usize].solid = true as byte;
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
    objdef[dead1 as libc::c_int as usize].solid = false as byte;
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
    objdef[dead2 as libc::c_int as usize].solid = false as byte;
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
    objdef[dead3 as libc::c_int as usize].solid = false as byte;
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
    objdef[dead4 as libc::c_int as usize].solid = false as byte;
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
    objdef[dead5 as libc::c_int as usize].solid = false as byte;
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
    objdef[shot as libc::c_int as usize].solid = false as byte;
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
    objdef[guns as libc::c_int as usize].solid = true as byte;
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
    objdef[gune as libc::c_int as usize].solid = true as byte;
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
    objdef[rock as libc::c_int as usize].solid = false as byte;
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
    objdef[bigshot as libc::c_int as usize].solid = false as byte;
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
    objdef[teleporter as libc::c_int as usize].solid = false as byte;
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
    objdef[secretgate as libc::c_int as usize].solid = false as byte;
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
