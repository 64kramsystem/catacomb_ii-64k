use sdl2::timer::Timer;

use crate::{
    pcrlib_a::{SDL_AudioSpec, SavedSoundStruct},
    safe_sdl::SDL_semaphore,
    sound_type::soundtype,
    sound_type::soundtype::*,
    spkr_table::SPKRtable,
};
use std::ptr;

// Globals previously belonging to pcrlib_a.rs.
//
#[rustfmt::skip]
pub struct PcrlibAState<'a> {
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
    // Rust port: The audio mutex has been moved to be in the `pcrlib_a` module scope, in order to
    // avoid borrowing contention on the PcrlibAState instance.
    pub AudioSpec: SDL_AudioSpec,
    pub AudioDev: u32,
    pub pcVolume: libc::c_short,
    pub pcPhaseTick: u32,
    pub pcPhaseLength: u32,
    pub pcActive: bool,
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
    pub vbltimer: Option<Timer<'a, 'a>>,

    // //////////////////////////////////////////////////////////
    // Rust port: private to cpanel.rs
    // //////////////////////////////////////////////////////////

    pub xormask: i32,
}
impl<'a> PcrlibAState<'a> {
    pub fn new(// SndPriority: u8,
        // _dontplay: i32,
        // AudioSpec: SDL_AudioSpec,
        // AudioDev: u32,
        // pcVolume: libc::c_short,
        // pcPhaseTick: u32,
        // pcPhaseLength: u32,
        // pcActive: bool,
        // pcSamplesPerTick: u32,
        // pcNumReadySamples: u32,
        // pcLastSample: u16,
        // pcLengthLeft: u32,
        // pcSound: Option<usize>,
        // SavedSound: SavedSoundStruct,
        // rndindex: u16,
        // indexi: u16,
        // indexj: u16,
        // LastRnd: u16,
        // RndArray: [u16; 17],
        // vblsem: *mut SDL_semaphore,
        // vbltimer: i32,
        // SoundData: SPKRtable,
        // soundmode: soundtype,
        // xormask: i32,
    ) -> Self {
        Self {
            SndPriority: 0,
            _dontplay: 0,
            AudioSpec: SDL_AudioSpec {
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
            AudioDev: 0,
            pcVolume: 5000,
            pcPhaseTick: 0,
            pcPhaseLength: 0,
            pcActive: false,
            pcSamplesPerTick: 0,
            pcNumReadySamples: 0,
            pcLastSample: 0,
            pcLengthLeft: 0,
            pcSound: None,
            SavedSound: SavedSoundStruct {
                SndPriority: 0,
                pcSamplesPerTick: 0,
                pcLengthLeft: 0,
                pcSound: None,
            },
            rndindex: 0,
            indexi: 0,
            indexj: 0,
            LastRnd: 0,
            RndArray: [0; 17],
            vblsem: 0 as *mut SDL_semaphore,
            vbltimer: None,
            SoundData: SPKRtable::default(),
            soundmode: spkr,
            xormask: 0,
        }
    }
}
