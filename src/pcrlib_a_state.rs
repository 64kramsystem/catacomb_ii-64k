use crate::{
    extra_types::boolean,
    pcrlib_a::{SDL_AudioSpec, SavedSoundStruct},
    safe_sdl::{SDL_mutex, SDL_semaphore},
    sound_type::soundtype,
    sound_type::soundtype::*,
    spkr_table::SPKRtable,
};
use std::ptr;

// Globals previously belonging to pcrlib_a.rs.
//
#[rustfmt::skip]
pub struct PcrlibAState {
    // //////////////////////////////////////////////////////////
    // Rust port: shared
    // //////////////////////////////////////////////////////////

    pub SoundData: SPKRtable,
    pub soundmode: soundtype,

    // //////////////////////////////////////////////////////////
    // Rust port: private to pcrlib_a.rs
    // //////////////////////////////////////////////////////////

    pub SndPriority: u8,
    pub _dontplay: i32,
    pub AudioMutex: *mut SDL_mutex,
    pub AudioSpec: SDL_AudioSpec,
    pub AudioDev: u32,
    pub pcVolume: libc::c_short,
    pub pcPhaseTick: u32,
    pub pcPhaseLength: u32,
    pub pcActive: boolean,
    pub pcSamplesPerTick: u32,
    pub pcNumReadySamples: u32,
    pub pcLastSample: u16,
    pub pcLengthLeft: u32,
    // Rust port: Pointer to SoundData.freqdata
    pub pcSound: Option<usize>,
    pub SavedSound: SavedSoundStruct,
    pub rndindex: u16,
    pub indexi: u16,
    pub indexj: u16,
    pub LastRnd: u16,
    pub RndArray: [u16; 17],
    pub vblsem: *mut SDL_semaphore,
    pub vbltimer: i32,

    // //////////////////////////////////////////////////////////
    // Rust port: private to cpanel.rs
    // //////////////////////////////////////////////////////////

    pub xormask: i32,
}
impl PcrlibAState {
    pub fn new(
        SndPriority: u8,
        _dontplay: i32,
        AudioMutex: *mut SDL_mutex,
        AudioSpec: SDL_AudioSpec,
        AudioDev: u32,
        pcVolume: libc::c_short,
        pcPhaseTick: u32,
        pcPhaseLength: u32,
        pcActive: boolean,
        pcSamplesPerTick: u32,
        pcNumReadySamples: u32,
        pcLastSample: u16,
        pcLengthLeft: u32,
        pcSound: Option<usize>,
        SavedSound: SavedSoundStruct,
        rndindex: u16,
        indexi: u16,
        indexj: u16,
        LastRnd: u16,
        RndArray: [u16; 17],
        vblsem: *mut SDL_semaphore,
        vbltimer: i32,
        SoundData: SPKRtable,
        soundmode: soundtype,
        xormask: i32,
    ) -> Self {
        Self {
            SndPriority,
            _dontplay,
            AudioMutex,
            AudioSpec,
            AudioDev,
            pcVolume,
            pcPhaseTick,
            pcPhaseLength,
            pcActive,
            pcSamplesPerTick,
            pcNumReadySamples,
            pcLastSample,
            pcLengthLeft,
            pcSound,
            SavedSound,
            rndindex,
            indexi,
            indexj,
            LastRnd,
            RndArray,
            vblsem,
            vbltimer,
            SoundData,
            soundmode,
            xormask,
        }
    }
}

impl Default for PcrlibAState {
    fn default() -> Self {
        Self::new(
            0,
            0,
            0 as *mut SDL_mutex,
            SDL_AudioSpec {
                freq: 0,
                format: 0,
                channels: 0,
                silence: 0,
                samples: 0,
                padding: 0,
                size: 0,
                callback: None,
                userdata: ptr::null_mut(),
            },
            0,
            5000,
            0,
            0,
            false as boolean,
            0,
            0,
            0,
            0,
            None,
            SavedSoundStruct {
                SndPriority: 0,
                pcSamplesPerTick: 0,
                pcLengthLeft: 0,
                pcSound: None,
            },
            0,
            0,
            0,
            0,
            [0; 17],
            0 as *mut SDL_semaphore,
            0,
            SPKRtable::default(),
            spkr,
            0,
        )
    }
}
