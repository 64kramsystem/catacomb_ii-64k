use crate::{
    extra_types::boolean,
    pcrlib_a::{SDL_AudioDeviceID, SDL_AudioSpec, SDL_TimerID, SDL_sem, SavedSoundStruct},
    safe_sdl::SDL_mutex,
    sound_type::soundtype,
    spkr_table::SPKRtable,
};

// Globals previously belonging to pcrlib_a.rs.
//
pub struct PcrlibAState {
    /*
    Private
    */
    pub SndPriority: u8,
    pub _dontplay: i32,
    pub AudioMutex: *mut SDL_mutex,
    pub AudioSpec: SDL_AudioSpec,
    pub AudioDev: SDL_AudioDeviceID,
    pub pcVolume: libc::c_short,
    pub pcPhaseTick: u32,
    pub pcPhaseLength: u32,
    pub pcActive: boolean,
    pub pcSamplesPerTick: u32,
    pub pcNumReadySamples: u32,
    pub pcLastSample: u16,
    pub pcLengthLeft: u32,
    pub pcSound: *mut u16,
    pub SavedSound: SavedSoundStruct,
    pub rndindex: u16,
    pub indexi: u16,
    pub indexj: u16,
    pub LastRnd: u16,
    pub RndArray: [u16; 17],
    pub vblsem: *mut SDL_sem,
    pub vbltimer: SDL_TimerID,
    /*
    Public
     */
    pub SoundData: *mut SPKRtable,
    pub soundmode: soundtype,
    pub xormask: i32,
}
impl PcrlibAState {
    pub fn new(
        SndPriority: u8,
        _dontplay: i32,
        AudioMutex: *mut SDL_mutex,
        AudioSpec: SDL_AudioSpec,
        AudioDev: SDL_AudioDeviceID,
        pcVolume: libc::c_short,
        pcPhaseTick: u32,
        pcPhaseLength: u32,
        pcActive: boolean,
        pcSamplesPerTick: u32,
        pcNumReadySamples: u32,
        pcLastSample: u16,
        pcLengthLeft: u32,
        pcSound: *mut u16,
        SavedSound: SavedSoundStruct,
        rndindex: u16,
        indexi: u16,
        indexj: u16,
        LastRnd: u16,
        RndArray: [u16; 17],
        vblsem: *mut SDL_sem,
        vbltimer: SDL_TimerID,
        SoundData: *mut SPKRtable,
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
