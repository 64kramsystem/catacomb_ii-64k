use std::{
    io::{Read, Write},
    mem,
};

use serdine::{
    derive::{Deserialize, Serialize},
    Serialize,
};

use crate::{gr_type::grtype, sound_type::soundtype};

#[derive(Copy, Clone, Default, Deserialize, Serialize)]
pub struct ctlpaneltype {
    #[deserialize = "deserialize_grmode"]
    #[serialize = "serialize_grmode"]
    pub grmode: grtype,
    #[deserialize = "deserialize_soundmode"]
    #[serialize = "serialize_soundmode"]
    pub soundmode: soundtype,
    pub playermode: [u16; 3],
    pub JoyXlow: [i16; 3],
    pub JoyYlow: [i16; 3],
    pub JoyXhigh: [i16; 3],
    pub JoyYhigh: [i16; 3],
    pub MouseSensitivity: i16,
    pub key: [u8; 8],
    pub keyB1: u8,
    pub keyB2: u8,
}

fn deserialize_grmode<R: Read>(mut r: R) -> grtype {
    let mut buffer = [0; mem::size_of::<grtype>()];
    r.read_exact(&mut buffer).unwrap();
    u16::from_le_bytes(buffer).into()
}

fn serialize_grmode<W: Write>(instance: &grtype, w: W) {
    <grtype as Into<u16>>::into(*instance).serialize(w);
}

fn deserialize_soundmode<R: Read>(mut r: R) -> soundtype {
    let mut buffer = [0; mem::size_of::<soundtype>()];
    r.read_exact(&mut buffer).unwrap();
    u16::from_le_bytes(buffer).into()
}

fn serialize_soundmode<W: Write>(instance: &soundtype, w: W) {
    <soundtype as Into<u16>>::into(*instance).serialize(w);
}
