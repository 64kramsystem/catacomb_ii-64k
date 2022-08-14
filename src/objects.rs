use crate::{class_type::classtype::*, tag_type::tagtype::*, think_type::thinktype::*};

extern "C" {
    static mut objdef: [objdeftype; 23];
}

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

pub unsafe fn initobjects() {
    objdef[player as usize].think = playercmd as u8;
    objdef[player as usize].contact = benign as u8;
    objdef[player as usize].solid = true as u8;
    objdef[player as usize].firstchar = 256;
    objdef[player as usize].size = 2;
    objdef[player as usize].stages = 4;
    objdef[player as usize].dirmask = 3;
    objdef[player as usize].speed = 256;
    objdef[player as usize].hitpoints = 12;
    objdef[player as usize].damage = 0;
    objdef[player as usize].points = 0;
    objdef[goblin as usize].think = ramstraight as u8;
    objdef[goblin as usize].contact = monster as u8;
    objdef[goblin as usize].solid = true as u8;
    objdef[goblin as usize].firstchar = (256 + 64) as u16;
    objdef[goblin as usize].size = 2;
    objdef[goblin as usize].stages = 4;
    objdef[goblin as usize].dirmask = 3;
    objdef[goblin as usize].speed = 75;
    objdef[goblin as usize].hitpoints = 1;
    objdef[goblin as usize].damage = 1;
    objdef[goblin as usize].points = 50;
    objdef[skeleton as usize].think = ramdiag as u8;
    objdef[skeleton as usize].contact = monster as u8;
    objdef[skeleton as usize].solid = true as u8;
    objdef[skeleton as usize].firstchar = (256 + 128) as u16;
    objdef[skeleton as usize].size = 2;
    objdef[skeleton as usize].stages = 4;
    objdef[skeleton as usize].dirmask = 3;
    objdef[skeleton as usize].speed = 130;
    objdef[skeleton as usize].hitpoints = 1;
    objdef[skeleton as usize].damage = 1;
    objdef[skeleton as usize].points = 150;
    objdef[ogre as usize].think = ramstraight as u8;
    objdef[ogre as usize].contact = monster as u8;
    objdef[ogre as usize].solid = true as u8;
    objdef[ogre as usize].firstchar = (256 + 67 * 4) as u16;
    objdef[ogre as usize].size = 3;
    objdef[ogre as usize].stages = 4;
    objdef[ogre as usize].dirmask = 3;
    objdef[ogre as usize].speed = 75;
    objdef[ogre as usize].hitpoints = 5;
    objdef[ogre as usize].damage = 2;
    objdef[ogre as usize].points = 250;
    objdef[gargoyle as usize].think = gargcmd as u8;
    objdef[gargoyle as usize].contact = monster as u8;
    objdef[gargoyle as usize].solid = true as u8;
    objdef[gargoyle as usize].firstchar = (256 + 67 * 4 + 35 * 9) as u16;
    objdef[gargoyle as usize].size = 4;
    objdef[gargoyle as usize].stages = 4;
    objdef[gargoyle as usize].dirmask = 3;
    objdef[gargoyle as usize].speed = 175;
    objdef[gargoyle as usize].hitpoints = 10;
    objdef[gargoyle as usize].damage = 3;
    objdef[gargoyle as usize].points = 500;
    objdef[dragon as usize].think = dragoncmd as u8;
    objdef[dragon as usize].contact = monster as u8;
    objdef[dragon as usize].solid = true as u8;
    objdef[dragon as usize].firstchar = (256 + 67 * 4 + 35 * 9 + 19 * 16) as u16;
    objdef[dragon as usize].size = 5;
    objdef[dragon as usize].stages = 4;
    objdef[dragon as usize].dirmask = 3;
    objdef[dragon as usize].speed = 100;
    objdef[dragon as usize].hitpoints = 75;
    objdef[dragon as usize].damage = 5;
    objdef[dragon as usize].points = 1000;
    objdef[turbogre as usize].think = ramstraight as u8;
    objdef[turbogre as usize].contact = monster as u8;
    objdef[turbogre as usize].solid = true as u8;
    objdef[turbogre as usize].firstchar = (256 + 67 * 4 + 19 * 9) as u16;
    objdef[turbogre as usize].size = 3;
    objdef[turbogre as usize].stages = 4;
    objdef[turbogre as usize].dirmask = 3;
    objdef[turbogre as usize].speed = 255;
    objdef[turbogre as usize].hitpoints = 5;
    objdef[turbogre as usize].damage = 2;
    objdef[turbogre as usize].points = 500;
    objdef[wallhit as usize].think = fade as u8;
    objdef[wallhit as usize].contact = benign as u8;
    objdef[wallhit as usize].solid = true as u8;
    objdef[wallhit as usize].firstchar = 26;
    objdef[wallhit as usize].size = 1;
    objdef[wallhit as usize].stages = 3;
    objdef[wallhit as usize].dirmask = 0;
    objdef[wallhit as usize].speed = 80;
    objdef[wallhit as usize].hitpoints = 0;
    objdef[wallhit as usize].damage = 0;
    objdef[wallhit as usize].points = 0;
    objdef[dead1 as usize].think = explode as u8;
    objdef[dead1 as usize].contact = benign as u8;
    objdef[dead1 as usize].solid = false as u8;
    objdef[dead1 as usize].firstchar = 29;
    objdef[dead1 as usize].size = 1;
    objdef[dead1 as usize].stages = 3;
    objdef[dead1 as usize].dirmask = 0;
    objdef[dead1 as usize].speed = 80;
    objdef[dead1 as usize].hitpoints = 0;
    objdef[dead1 as usize].damage = 0;
    objdef[dead1 as usize].points = 0;
    objdef[dead2 as usize].think = fade as u8;
    objdef[dead2 as usize].contact = benign as u8;
    objdef[dead2 as usize].solid = false as u8;
    objdef[dead2 as usize].firstchar = (256 + 224) as u16;
    objdef[dead2 as usize].size = 2;
    objdef[dead2 as usize].stages = 3;
    objdef[dead2 as usize].dirmask = 0;
    objdef[dead2 as usize].speed = 80;
    objdef[dead2 as usize].hitpoints = 0;
    objdef[dead2 as usize].damage = 0;
    objdef[dead2 as usize].points = 0;
    objdef[dead3 as usize].think = fade as u8;
    objdef[dead3 as usize].contact = benign as u8;
    objdef[dead3 as usize].solid = false as u8;
    objdef[dead3 as usize].firstchar = (256 + 67 * 4 + 9 * 16) as u16;
    objdef[dead3 as usize].size = 3;
    objdef[dead3 as usize].stages = 3;
    objdef[dead3 as usize].dirmask = 0;
    objdef[dead3 as usize].speed = 80;
    objdef[dead3 as usize].hitpoints = 0;
    objdef[dead3 as usize].damage = 0;
    objdef[dead3 as usize].points = 0;
    objdef[dead4 as usize].think = fade as u8;
    objdef[dead4 as usize].contact = benign as u8;
    objdef[dead4 as usize].solid = false as u8;
    objdef[dead4 as usize].firstchar = (256 + 67 * 4 + 35 * 9 + 16 * 16) as u16;
    objdef[dead4 as usize].size = 4;
    objdef[dead4 as usize].stages = 3;
    objdef[dead4 as usize].dirmask = 0;
    objdef[dead4 as usize].speed = 80;
    objdef[dead4 as usize].hitpoints = 0;
    objdef[dead4 as usize].damage = 0;
    objdef[dead4 as usize].points = 0;
    objdef[dead5 as usize].think = fade as u8;
    objdef[dead5 as usize].contact = benign as u8;
    objdef[dead5 as usize].solid = false as u8;
    objdef[dead5 as usize].firstchar = (256 + 67 * 4 + 35 * 9 + 19 * 16 + 25 * 16) as u16;
    objdef[dead5 as usize].size = 5;
    objdef[dead5 as usize].stages = 3;
    objdef[dead5 as usize].dirmask = 0;
    objdef[dead5 as usize].speed = 80;
    objdef[dead5 as usize].hitpoints = 0;
    objdef[dead5 as usize].damage = 0;
    objdef[dead5 as usize].points = 0;
    objdef[shot as usize].think = straight as u8;
    objdef[shot as usize].contact = pshot as u8;
    objdef[shot as usize].solid = false as u8;
    objdef[shot as usize].firstchar = 154;
    objdef[shot as usize].size = 1;
    objdef[shot as usize].stages = 2;
    objdef[shot as usize].dirmask = 3;
    objdef[shot as usize].speed = 256;
    objdef[shot as usize].hitpoints = 0;
    objdef[shot as usize].damage = 1;
    objdef[shot as usize].points = 0;
    objdef[guns as usize].think = gunthinks as u8;
    objdef[guns as usize].contact = benign as u8;
    objdef[guns as usize].solid = true as u8;
    objdef[guns as usize].firstchar = (256 + 67 * 4 - 8) as u16;
    objdef[guns as usize].size = 2;
    objdef[guns as usize].stages = 1;
    objdef[guns as usize].dirmask = 0;
    objdef[guns as usize].speed = 10;
    objdef[guns as usize].hitpoints = 255;
    objdef[guns as usize].damage = 0;
    objdef[guns as usize].points = 0;
    objdef[gune as usize].think = gunthinke as u8;
    objdef[gune as usize].contact = benign as u8;
    objdef[gune as usize].solid = true as u8;
    objdef[gune as usize].firstchar = (256 + 67 * 4 - 4) as u16;
    objdef[gune as usize].size = 2;
    objdef[gune as usize].stages = 1;
    objdef[gune as usize].dirmask = 0;
    objdef[gune as usize].speed = 10;
    objdef[gune as usize].hitpoints = 255;
    objdef[gune as usize].damage = 0;
    objdef[gune as usize].points = 0;
    objdef[rock as usize].think = straight as u8;
    objdef[rock as usize].contact = mshot as u8;
    objdef[rock as usize].solid = false as u8;
    objdef[rock as usize].firstchar = 153;
    objdef[rock as usize].size = 1;
    objdef[rock as usize].stages = 2;
    objdef[rock as usize].dirmask = 0;
    objdef[rock as usize].speed = 256;
    objdef[rock as usize].hitpoints = 0;
    objdef[rock as usize].damage = 1;
    objdef[rock as usize].points = 0;
    objdef[bigshot as usize].think = straight as u8;
    objdef[bigshot as usize].contact = nukeshot as u8;
    objdef[bigshot as usize].solid = false as u8;
    objdef[bigshot as usize].firstchar = (256 + 192) as u16;
    objdef[bigshot as usize].size = 2;
    objdef[bigshot as usize].stages = 2;
    objdef[bigshot as usize].dirmask = 3;
    objdef[bigshot as usize].speed = 256;
    objdef[bigshot as usize].hitpoints = 0;
    objdef[bigshot as usize].damage = 1;
    objdef[bigshot as usize].points = 0;
    objdef[teleporter as usize].think = idle as u8;
    objdef[teleporter as usize].contact = benign as u8;
    objdef[teleporter as usize].solid = false as u8;
    objdef[teleporter as usize].firstchar = (256 + 236) as u16;
    objdef[teleporter as usize].size = 2;
    objdef[teleporter as usize].stages = 5;
    objdef[teleporter as usize].dirmask = 0;
    objdef[teleporter as usize].speed = 200;
    objdef[teleporter as usize].hitpoints = 0;
    objdef[teleporter as usize].damage = 0;
    objdef[teleporter as usize].points = 0;
    objdef[secretgate as usize].think = idle as u8;
    objdef[secretgate as usize].contact = benign as u8;
    objdef[secretgate as usize].solid = false as u8;
    objdef[secretgate as usize].firstchar = (256 + 67 * 4 - 12) as u16;
    objdef[secretgate as usize].size = 2;
    objdef[secretgate as usize].stages = 1;
    objdef[secretgate as usize].dirmask = 0;
    objdef[secretgate as usize].speed = 200;
    objdef[secretgate as usize].hitpoints = 0;
    objdef[secretgate as usize].damage = 0;
    objdef[secretgate as usize].points = 0;
}
