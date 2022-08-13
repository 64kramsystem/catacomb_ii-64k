use ::libc;

use crate::extra_constants::PC_BASE_TIMER;
extern "C" {
    pub type SDL_mutex;
    pub type SDL_semaphore;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn SDL_memset(dst: *mut libc::c_void, c: libc::c_int, len: size_t) -> *mut libc::c_void;
    fn SDL_GetError() -> *const libc::c_char;
    fn SDL_CreateMutex() -> *mut SDL_mutex;
    fn SDL_LockMutex(mutex: *mut SDL_mutex) -> libc::c_int;
    fn SDL_UnlockMutex(mutex: *mut SDL_mutex) -> libc::c_int;
    fn SDL_CreateSemaphore(initial_value: Uint32) -> *mut SDL_sem;
    fn SDL_DestroySemaphore(sem: *mut SDL_sem);
    fn SDL_SemWait(sem: *mut SDL_sem) -> libc::c_int;
    fn SDL_SemPost(sem: *mut SDL_sem) -> libc::c_int;
    fn SDL_SemValue(sem: *mut SDL_sem) -> Uint32;
    fn SDL_OpenAudioDevice(
        device: *const libc::c_char,
        iscapture: libc::c_int,
        desired: *const SDL_AudioSpec,
        obtained: *mut SDL_AudioSpec,
        allowed_changes: libc::c_int,
    ) -> SDL_AudioDeviceID;
    fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: libc::c_int);
    fn SDL_CloseAudio();
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn SDL_InitSubSystem(flags: Uint32) -> libc::c_int;
    fn SDL_RemoveTimer(id: SDL_TimerID) -> SDL_bool;
    fn SDL_AddTimer(
        interval: Uint32,
        callback: SDL_TimerCallback,
        param: *mut libc::c_void,
    ) -> SDL_TimerID;
    static mut screenseg: [byte; 64000];
    static mut egaplaneofs: [dword; 4];
    static mut picptr: *mut libc::c_void;
    static mut charptr: *mut libc::c_void;
    static mut pictable: [pictype; 64];
    fn UpdateScreen();
    static mut grmode: grtype;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
pub type SDL_sem = SDL_semaphore;
pub type SDL_AudioFormat = Uint16;
pub type SDL_AudioCallback =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut Uint8, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioSpec {
    pub freq: libc::c_int,
    pub format: SDL_AudioFormat,
    pub channels: Uint8,
    pub silence: Uint8,
    pub samples: Uint16,
    pub padding: Uint16,
    pub size: Uint32,
    pub callback: SDL_AudioCallback,
    pub userdata: *mut libc::c_void,
}
pub type SDL_AudioDeviceID = Uint32;
pub type SDL_TimerCallback = Option<unsafe extern "C" fn(Uint32, *mut libc::c_void) -> Uint32>;
pub type SDL_TimerID = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type boolean = uint16_t;
pub type byte = uint8_t;
pub type word = uint16_t;
pub type sword = int16_t;
pub type dword = uint32_t;
pub type qword = uint64_t;
pub type soundtype = libc::c_uint;
pub const sdlib: soundtype = 2;
pub const spkr: soundtype = 1;
pub const off: soundtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct spksndtype {
    pub start: word,
    pub priority: byte,
    pub samplerate: byte,
    pub name: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SPKRtable {
    pub id: [libc::c_char; 4],
    pub filelength: word,
    pub filler: [word; 5],
    pub sounds: [spksndtype; 63],
    pub freqdata: [word; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub SndPriority: byte,
    pub pcSamplesPerTick: libc::c_uint,
    pub pcLengthLeft: libc::c_uint,
    pub pcSound: *mut word,
}
pub type grtype = libc::c_uint;
pub const VGAgr: grtype = 3;
pub const EGAgr: grtype = 2;
pub const CGAgr: grtype = 1;
pub const text: grtype = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct pictype {
    pub width: sword,
    pub height: sword,
    pub shapeptr: dword,
    pub name: [libc::c_char; 8],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const screenpitch: C2RustUnnamed_1 = 320;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const VBL_TIME: C2RustUnnamed_2 = 14;
#[inline]
unsafe extern "C" fn EGA(mut chan: *const byte, mut ofs: byte) -> byte {
    return ((*chan.offset(3 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
        & 1 as libc::c_int)
        << 3 as libc::c_int
        | (*chan.offset(2 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int)
            << 2 as libc::c_int
        | (*chan.offset(1 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int)
            << 1 as libc::c_int
        | *chan.offset(0 as libc::c_int as isize) as libc::c_int >> ofs as libc::c_int
            & 1 as libc::c_int) as byte;
}
#[no_mangle]
pub static mut SoundData: *mut SPKRtable = 0 as *const SPKRtable as *mut SPKRtable;
#[no_mangle]
pub static mut soundmode: soundtype = spkr;
static mut SndPriority: byte = 0;
#[no_mangle]
pub static mut xormask: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut _dontplay: libc::c_int = 0 as libc::c_int;
static mut AudioMutex: *mut SDL_mutex = 0 as *const SDL_mutex as *mut SDL_mutex;
static mut AudioSpec: SDL_AudioSpec = SDL_AudioSpec {
    freq: 0,
    format: 0,
    channels: 0,
    silence: 0,
    samples: 0,
    padding: 0,
    size: 0,
    callback: None,
    userdata: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut AudioDev: SDL_AudioDeviceID = 0;
static mut pcVolume: libc::c_short = 5000 as libc::c_int as libc::c_short;
static mut pcPhaseTick: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut pcPhaseLength: libc::c_uint = 0;
static mut pcActive: boolean = false_0 as libc::c_int as boolean;
static mut pcSamplesPerTick: libc::c_uint = 0;
static mut pcNumReadySamples: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut pcLastSample: word = 0 as libc::c_int as word;
static mut pcLengthLeft: libc::c_uint = 0;
static mut pcSound: *mut word = 0 as *const word as *mut word;
#[inline]
unsafe extern "C" fn _SDL_turnOnPCSpeaker(mut pcSample: word) {
    // The transpiler break the correctness here; in the source project, `AudioSpec.freq` is a long
    // int (__syscall_slong_t), but it's translated as c_int, which doesn't accommodate the multiplication
    // result range (u32).
    //
    pcPhaseLength = pcSample as libc::c_uint * AudioSpec.freq as libc::c_uint / (2 * PC_BASE_TIMER);
    pcActive = true_0 as libc::c_int as boolean;
}
#[inline]
unsafe extern "C" fn _SDL_turnOffPCSpeaker() {
    pcActive = false_0 as libc::c_int as boolean;
    pcPhaseTick = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn _SDL_PCService() {
    if !pcSound.is_null() {
        if *pcSound as libc::c_int != pcLastSample as libc::c_int {
            pcLastSample = *pcSound;
            if pcLastSample != 0 {
                _SDL_turnOnPCSpeaker(pcLastSample);
            } else {
                _SDL_turnOffPCSpeaker();
            }
        }
        pcSound = pcSound.offset(1);
        pcLengthLeft = pcLengthLeft.wrapping_sub(1);
        if pcLengthLeft == 0 {
            pcSound = 0 as *mut word;
            SndPriority = 0 as libc::c_int as byte;
            _SDL_turnOffPCSpeaker();
        }
    }
}
unsafe extern "C" fn _SDL_PCPlaySound(mut sound: libc::c_int) {
    SDL_LockMutex(AudioMutex);
    pcPhaseTick = 0 as libc::c_int as libc::c_uint;
    pcLastSample = 0 as libc::c_int as word;
    pcLengthLeft = ((*SoundData).sounds[sound as usize].start as libc::c_int
        - (*SoundData).sounds[(sound - 1 as libc::c_int) as usize].start as libc::c_int
        >> 1 as libc::c_int) as libc::c_uint;
    pcSound = (SoundData as *mut byte).offset(
        (*SoundData).sounds[(sound - 1 as libc::c_int) as usize].start as libc::c_int as isize,
    ) as *mut word;
    SndPriority = (*SoundData).sounds[(sound - 1 as libc::c_int) as usize].priority;
    pcSamplesPerTick = (AudioSpec.freq
        / (1193181 as libc::c_int
            * (*SoundData).sounds[(sound - 1 as libc::c_int) as usize].samplerate as libc::c_int
            >> 16 as libc::c_int)) as libc::c_uint;
    SDL_UnlockMutex(AudioMutex);
}
unsafe extern "C" fn _SDL_PCStopSound() {
    SDL_LockMutex(AudioMutex);
    pcSound = 0 as *mut word;
    _SDL_turnOffPCSpeaker();
    SDL_UnlockMutex(AudioMutex);
}
unsafe extern "C" fn _SDL_ShutPC() {
    _SDL_PCStopSound();
}
unsafe extern "C" fn UpdateSPKR(
    mut _userdata: *mut libc::c_void,
    mut stream: *mut Uint8,
    mut len: libc::c_int,
) {
    if soundmode as libc::c_uint != spkr as libc::c_int as libc::c_uint {
        memset(
            stream as *mut libc::c_void,
            0 as libc::c_int,
            len as libc::c_ulong,
        );
        return;
    }
    let mut sampleslen: libc::c_int = len >> 1 as libc::c_int;
    let mut stream16: *mut Sint16 = stream as *mut libc::c_void as *mut Sint16;
    SDL_LockMutex(AudioMutex);
    loop {
        if pcNumReadySamples != 0 {
            if pcActive != 0 {
                while pcNumReadySamples != 0 && sampleslen != 0 {
                    pcNumReadySamples = pcNumReadySamples.wrapping_sub(1);
                    sampleslen -= 1;
                    let fresh0 = stream16;
                    stream16 = stream16.offset(1);
                    *fresh0 = pcVolume;
                    let fresh1 = pcPhaseTick;
                    pcPhaseTick = pcPhaseTick.wrapping_add(1);
                    if fresh1 >= pcPhaseLength {
                        pcVolume = -(pcVolume as libc::c_int) as libc::c_short;
                        pcPhaseTick = 0 as libc::c_int as libc::c_uint;
                    }
                }
            } else {
                while pcNumReadySamples != 0 && sampleslen != 0 {
                    pcNumReadySamples = pcNumReadySamples.wrapping_sub(1);
                    sampleslen -= 1;
                    let fresh2 = stream16;
                    stream16 = stream16.offset(1);
                    *fresh2 = 0 as libc::c_int as Sint16;
                }
            }
            if sampleslen == 0 {
                break;
            }
        }
        _SDL_PCService();
        pcNumReadySamples = pcSamplesPerTick;
        if !(pcNumReadySamples != 0) {
            break;
        }
    }
    SDL_UnlockMutex(AudioMutex);
}
#[no_mangle]
pub unsafe extern "C" fn StartupSound() {
    let mut desired: SDL_AudioSpec = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: 0 as *const libc::c_void as *mut libc::c_void,
    };
    SDL_memset(
        &mut desired as *mut SDL_AudioSpec as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong,
    );
    desired.freq = 48000 as libc::c_int;
    desired.format = 0x8010 as libc::c_int as SDL_AudioFormat;
    desired.channels = 1 as libc::c_int as Uint8;
    desired.samples = 4096 as libc::c_int as Uint16;
    desired.callback =
        Some(UpdateSPKR as unsafe extern "C" fn(*mut libc::c_void, *mut Uint8, libc::c_int) -> ());
    AudioMutex = SDL_CreateMutex();
    if AudioMutex.is_null() || SDL_InitSubSystem(0x10 as libc::c_uint) < 0 as libc::c_int || {
        AudioDev = SDL_OpenAudioDevice(
            0 as *const libc::c_char,
            0 as libc::c_int,
            &mut desired,
            &mut AudioSpec,
            0 as libc::c_int,
        );
        AudioDev == 0 as libc::c_int as libc::c_uint
    } {
        printf(
            b"Audio initialization failed: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        soundmode = off;
        _dontplay = 1 as libc::c_int;
        return;
    }
    pcSamplesPerTick = (AudioSpec.freq / 145 as libc::c_int) as libc::c_uint;
    soundmode = spkr;
    SDL_PauseAudioDevice(AudioDev, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ShutdownSound() {
    if _dontplay != 0 {
        return;
    }
    _SDL_ShutPC();
    SDL_CloseAudio();
}
#[no_mangle]
pub unsafe extern "C" fn PlaySound(mut sound: libc::c_int) {
    if _dontplay != 0 {
        return;
    }
    if (*SoundData).sounds[(sound - 1 as libc::c_int) as usize].priority as libc::c_int
        >= SndPriority as libc::c_int
    {
        _SDL_PCPlaySound(sound);
    }
}
#[no_mangle]
pub unsafe extern "C" fn StopSound() {
    if _dontplay != 0 {
        return;
    }
    _SDL_PCStopSound();
}
static mut SavedSound: C2RustUnnamed_0 = C2RustUnnamed_0 {
    SndPriority: 0,
    pcSamplesPerTick: 0,
    pcLengthLeft: 0,
    pcSound: 0 as *const word as *mut word,
};
#[no_mangle]
pub unsafe extern "C" fn PauseSound() {
    if _dontplay != 0 {
        return;
    }
    SDL_LockMutex(AudioMutex);
    SavedSound.SndPriority = SndPriority;
    SavedSound.pcSamplesPerTick = pcSamplesPerTick;
    SavedSound.pcLengthLeft = pcLengthLeft;
    SavedSound.pcSound = pcSound;
    SndPriority = 0 as libc::c_int as byte;
    pcLengthLeft = 0 as libc::c_int as libc::c_uint;
    pcSound = 0 as *mut word;
    _SDL_turnOffPCSpeaker();
    SDL_UnlockMutex(AudioMutex);
}
#[no_mangle]
pub unsafe extern "C" fn ContinueSound() {
    if _dontplay != 0 {
        return;
    }
    pcPhaseTick = 0 as libc::c_int as libc::c_uint;
    pcLastSample = 0 as libc::c_int as word;
    SndPriority = SavedSound.SndPriority;
    pcSamplesPerTick = SavedSound.pcSamplesPerTick;
    pcLengthLeft = SavedSound.pcLengthLeft;
    pcSound = SavedSound.pcSound;
}
#[no_mangle]
pub unsafe extern "C" fn WaitEndSound() {
    if _dontplay != 0 {
        return;
    }
    UpdateScreen();
    while !pcSound.is_null() {
        WaitVBL();
    }
}
static mut rndindex: word = 0;
static mut rndtable: [byte; 256] = [
    0 as libc::c_int as byte,
    8 as libc::c_int as byte,
    109 as libc::c_int as byte,
    220 as libc::c_int as byte,
    222 as libc::c_int as byte,
    241 as libc::c_int as byte,
    149 as libc::c_int as byte,
    107 as libc::c_int as byte,
    75 as libc::c_int as byte,
    248 as libc::c_int as byte,
    254 as libc::c_int as byte,
    140 as libc::c_int as byte,
    16 as libc::c_int as byte,
    66 as libc::c_int as byte,
    74 as libc::c_int as byte,
    21 as libc::c_int as byte,
    211 as libc::c_int as byte,
    47 as libc::c_int as byte,
    80 as libc::c_int as byte,
    242 as libc::c_int as byte,
    154 as libc::c_int as byte,
    27 as libc::c_int as byte,
    205 as libc::c_int as byte,
    128 as libc::c_int as byte,
    161 as libc::c_int as byte,
    89 as libc::c_int as byte,
    77 as libc::c_int as byte,
    36 as libc::c_int as byte,
    95 as libc::c_int as byte,
    110 as libc::c_int as byte,
    85 as libc::c_int as byte,
    48 as libc::c_int as byte,
    212 as libc::c_int as byte,
    140 as libc::c_int as byte,
    211 as libc::c_int as byte,
    249 as libc::c_int as byte,
    22 as libc::c_int as byte,
    79 as libc::c_int as byte,
    200 as libc::c_int as byte,
    50 as libc::c_int as byte,
    28 as libc::c_int as byte,
    188 as libc::c_int as byte,
    52 as libc::c_int as byte,
    140 as libc::c_int as byte,
    202 as libc::c_int as byte,
    120 as libc::c_int as byte,
    68 as libc::c_int as byte,
    145 as libc::c_int as byte,
    62 as libc::c_int as byte,
    70 as libc::c_int as byte,
    184 as libc::c_int as byte,
    190 as libc::c_int as byte,
    91 as libc::c_int as byte,
    197 as libc::c_int as byte,
    152 as libc::c_int as byte,
    224 as libc::c_int as byte,
    149 as libc::c_int as byte,
    104 as libc::c_int as byte,
    25 as libc::c_int as byte,
    178 as libc::c_int as byte,
    252 as libc::c_int as byte,
    182 as libc::c_int as byte,
    202 as libc::c_int as byte,
    182 as libc::c_int as byte,
    141 as libc::c_int as byte,
    197 as libc::c_int as byte,
    4 as libc::c_int as byte,
    81 as libc::c_int as byte,
    181 as libc::c_int as byte,
    242 as libc::c_int as byte,
    145 as libc::c_int as byte,
    42 as libc::c_int as byte,
    39 as libc::c_int as byte,
    227 as libc::c_int as byte,
    156 as libc::c_int as byte,
    198 as libc::c_int as byte,
    225 as libc::c_int as byte,
    193 as libc::c_int as byte,
    219 as libc::c_int as byte,
    93 as libc::c_int as byte,
    122 as libc::c_int as byte,
    175 as libc::c_int as byte,
    249 as libc::c_int as byte,
    0 as libc::c_int as byte,
    175 as libc::c_int as byte,
    143 as libc::c_int as byte,
    70 as libc::c_int as byte,
    239 as libc::c_int as byte,
    46 as libc::c_int as byte,
    246 as libc::c_int as byte,
    163 as libc::c_int as byte,
    53 as libc::c_int as byte,
    163 as libc::c_int as byte,
    109 as libc::c_int as byte,
    168 as libc::c_int as byte,
    135 as libc::c_int as byte,
    2 as libc::c_int as byte,
    235 as libc::c_int as byte,
    25 as libc::c_int as byte,
    92 as libc::c_int as byte,
    20 as libc::c_int as byte,
    145 as libc::c_int as byte,
    138 as libc::c_int as byte,
    77 as libc::c_int as byte,
    69 as libc::c_int as byte,
    166 as libc::c_int as byte,
    78 as libc::c_int as byte,
    176 as libc::c_int as byte,
    173 as libc::c_int as byte,
    212 as libc::c_int as byte,
    166 as libc::c_int as byte,
    113 as libc::c_int as byte,
    94 as libc::c_int as byte,
    161 as libc::c_int as byte,
    41 as libc::c_int as byte,
    50 as libc::c_int as byte,
    239 as libc::c_int as byte,
    49 as libc::c_int as byte,
    111 as libc::c_int as byte,
    164 as libc::c_int as byte,
    70 as libc::c_int as byte,
    60 as libc::c_int as byte,
    2 as libc::c_int as byte,
    37 as libc::c_int as byte,
    171 as libc::c_int as byte,
    75 as libc::c_int as byte,
    136 as libc::c_int as byte,
    156 as libc::c_int as byte,
    11 as libc::c_int as byte,
    56 as libc::c_int as byte,
    42 as libc::c_int as byte,
    146 as libc::c_int as byte,
    138 as libc::c_int as byte,
    229 as libc::c_int as byte,
    73 as libc::c_int as byte,
    146 as libc::c_int as byte,
    77 as libc::c_int as byte,
    61 as libc::c_int as byte,
    98 as libc::c_int as byte,
    196 as libc::c_int as byte,
    135 as libc::c_int as byte,
    106 as libc::c_int as byte,
    63 as libc::c_int as byte,
    197 as libc::c_int as byte,
    195 as libc::c_int as byte,
    86 as libc::c_int as byte,
    96 as libc::c_int as byte,
    203 as libc::c_int as byte,
    113 as libc::c_int as byte,
    101 as libc::c_int as byte,
    170 as libc::c_int as byte,
    247 as libc::c_int as byte,
    181 as libc::c_int as byte,
    113 as libc::c_int as byte,
    80 as libc::c_int as byte,
    250 as libc::c_int as byte,
    108 as libc::c_int as byte,
    7 as libc::c_int as byte,
    255 as libc::c_int as byte,
    237 as libc::c_int as byte,
    129 as libc::c_int as byte,
    226 as libc::c_int as byte,
    79 as libc::c_int as byte,
    107 as libc::c_int as byte,
    112 as libc::c_int as byte,
    166 as libc::c_int as byte,
    103 as libc::c_int as byte,
    241 as libc::c_int as byte,
    24 as libc::c_int as byte,
    223 as libc::c_int as byte,
    239 as libc::c_int as byte,
    120 as libc::c_int as byte,
    198 as libc::c_int as byte,
    58 as libc::c_int as byte,
    60 as libc::c_int as byte,
    82 as libc::c_int as byte,
    128 as libc::c_int as byte,
    3 as libc::c_int as byte,
    184 as libc::c_int as byte,
    66 as libc::c_int as byte,
    143 as libc::c_int as byte,
    224 as libc::c_int as byte,
    145 as libc::c_int as byte,
    224 as libc::c_int as byte,
    81 as libc::c_int as byte,
    206 as libc::c_int as byte,
    163 as libc::c_int as byte,
    45 as libc::c_int as byte,
    63 as libc::c_int as byte,
    90 as libc::c_int as byte,
    168 as libc::c_int as byte,
    114 as libc::c_int as byte,
    59 as libc::c_int as byte,
    33 as libc::c_int as byte,
    159 as libc::c_int as byte,
    95 as libc::c_int as byte,
    28 as libc::c_int as byte,
    139 as libc::c_int as byte,
    123 as libc::c_int as byte,
    98 as libc::c_int as byte,
    125 as libc::c_int as byte,
    196 as libc::c_int as byte,
    15 as libc::c_int as byte,
    70 as libc::c_int as byte,
    194 as libc::c_int as byte,
    253 as libc::c_int as byte,
    54 as libc::c_int as byte,
    14 as libc::c_int as byte,
    109 as libc::c_int as byte,
    226 as libc::c_int as byte,
    71 as libc::c_int as byte,
    17 as libc::c_int as byte,
    161 as libc::c_int as byte,
    93 as libc::c_int as byte,
    186 as libc::c_int as byte,
    87 as libc::c_int as byte,
    244 as libc::c_int as byte,
    138 as libc::c_int as byte,
    20 as libc::c_int as byte,
    52 as libc::c_int as byte,
    123 as libc::c_int as byte,
    251 as libc::c_int as byte,
    26 as libc::c_int as byte,
    36 as libc::c_int as byte,
    17 as libc::c_int as byte,
    46 as libc::c_int as byte,
    52 as libc::c_int as byte,
    231 as libc::c_int as byte,
    232 as libc::c_int as byte,
    76 as libc::c_int as byte,
    31 as libc::c_int as byte,
    221 as libc::c_int as byte,
    84 as libc::c_int as byte,
    37 as libc::c_int as byte,
    216 as libc::c_int as byte,
    165 as libc::c_int as byte,
    212 as libc::c_int as byte,
    106 as libc::c_int as byte,
    197 as libc::c_int as byte,
    242 as libc::c_int as byte,
    98 as libc::c_int as byte,
    43 as libc::c_int as byte,
    39 as libc::c_int as byte,
    175 as libc::c_int as byte,
    254 as libc::c_int as byte,
    145 as libc::c_int as byte,
    190 as libc::c_int as byte,
    84 as libc::c_int as byte,
    118 as libc::c_int as byte,
    222 as libc::c_int as byte,
    187 as libc::c_int as byte,
    136 as libc::c_int as byte,
    120 as libc::c_int as byte,
    163 as libc::c_int as byte,
    236 as libc::c_int as byte,
    249 as libc::c_int as byte,
];
static mut indexi: word = 0;
static mut indexj: word = 0;
static mut LastRnd: word = 0;
static mut RndArray: [word; 17] = [0; 17];
static mut baseRndArray: [word; 17] = [
    1 as libc::c_int as word,
    1 as libc::c_int as word,
    2 as libc::c_int as word,
    3 as libc::c_int as word,
    5 as libc::c_int as word,
    8 as libc::c_int as word,
    13 as libc::c_int as word,
    21 as libc::c_int as word,
    54 as libc::c_int as word,
    75 as libc::c_int as word,
    129 as libc::c_int as word,
    204 as libc::c_int as word,
    323 as libc::c_int as word,
    527 as libc::c_int as word,
    850 as libc::c_int as word,
    1377 as libc::c_int as word,
    2227 as libc::c_int as word,
];
#[no_mangle]
pub unsafe extern "C" fn initrnd(mut randomize: boolean) {
    memcpy(
        RndArray.as_mut_ptr() as *mut libc::c_void,
        baseRndArray.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[word; 17]>() as libc::c_ulong,
    );
    LastRnd = 0 as libc::c_int as word;
    indexi = 17 as libc::c_int as word;
    indexj = 5 as libc::c_int as word;
    if randomize != 0 {
        let mut now: time_t = time(0 as *mut time_t);
        RndArray[16 as libc::c_int as usize] =
            (now & 0xffff as libc::c_int as libc::c_long) as word;
        RndArray[4 as libc::c_int as usize] = (now & 0xffff as libc::c_int as libc::c_long
            ^ now >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_long)
            as word;
    }
    rnd(0xffff as libc::c_int as word);
}
#[no_mangle]
pub unsafe extern "C" fn rnd(mut maxval: word) -> libc::c_int {
    let mut mask: word = 0;
    let mut shift: word = 0;
    let mut val: libc::c_int = 0;
    if maxval as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    mask = 0xffff as libc::c_int as word;
    shift = maxval;
    while shift as libc::c_int & 0x8000 as libc::c_int == 0 {
        shift = ((shift as libc::c_int) << 1 as libc::c_int) as word;
        mask = (mask as libc::c_int >> 1 as libc::c_int) as word;
    }
    val = RndArray[(indexi as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
        + RndArray[(indexj as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int
        + 1 as libc::c_int;
    RndArray[(indexi as libc::c_int - 1 as libc::c_int) as usize] = val as word;
    val += LastRnd as libc::c_int;
    LastRnd = val as word;
    indexi = indexi.wrapping_sub(1);
    if indexi as libc::c_int == 0 as libc::c_int {
        indexi = 17 as libc::c_int as word;
    }
    indexj = indexj.wrapping_sub(1);
    if indexj as libc::c_int == 0 as libc::c_int {
        indexj = 17 as libc::c_int as word;
    }
    val &= mask as libc::c_int;
    if val > maxval as libc::c_int {
        val >>= 1 as libc::c_int;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn initrndt(mut randomize: boolean) {
    rndindex = (if randomize as libc::c_int != 0 {
        time(0 as *mut time_t) & 0xff as libc::c_int as libc::c_long
    } else {
        0 as libc::c_int as libc::c_long
    }) as word;
}
#[no_mangle]
pub unsafe extern "C" fn rndt() -> libc::c_int {
    rndindex = (rndindex as libc::c_int + 1 as libc::c_int & 0xff as libc::c_int) as word;
    return rndtable[rndindex as usize] as libc::c_int;
}
#[no_mangle]
pub static mut vblsem: *mut SDL_sem = 0 as *const SDL_sem as *mut SDL_sem;
#[no_mangle]
pub static mut vbltimer: SDL_TimerID = 0;
unsafe extern "C" fn VBLCallback(mut _interval: Uint32, mut _param: *mut libc::c_void) -> Uint32 {
    SDL_SemPost(vblsem);
    return VBL_TIME as libc::c_int as Uint32;
}
unsafe extern "C" fn ShutdownEmulatedVBL() {
    SDL_RemoveTimer(vbltimer);
    SDL_DestroySemaphore(vblsem);
}
#[no_mangle]
pub unsafe extern "C" fn SetupEmulatedVBL() {
    vblsem = SDL_CreateSemaphore(0 as libc::c_int as Uint32);
    vbltimer = SDL_AddTimer(
        VBL_TIME as libc::c_int as Uint32,
        Some(VBLCallback as unsafe extern "C" fn(Uint32, *mut libc::c_void) -> Uint32),
        0 as *mut libc::c_void,
    );
    atexit(Some(ShutdownEmulatedVBL as unsafe extern "C" fn() -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn WaitVBL() {
    loop {
        SDL_SemWait(vblsem);
        if !(SDL_SemValue(vblsem) != 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn drawchar(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut charnum: libc::c_int,
) {
    let mut vbuf: *mut byte = screenseg
        .as_mut_ptr()
        .offset(((y << 3 as libc::c_int) * screenpitch as libc::c_int) as isize)
        .offset((x << 3 as libc::c_int) as isize);
    let mut src: *mut byte = 0 as *mut byte;
    let mut i: libc::c_uint = 0;
    match grmode as libc::c_uint {
        1 => {
            src = (charptr as *mut byte).offset((charnum * 16 as libc::c_int) as isize);
            i = 0 as libc::c_int as libc::c_uint;
            while i < 8 as libc::c_int as libc::c_uint {
                let fresh10 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh10 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 6 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh11 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh11 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 4 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh12 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh12 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh13 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh13 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 0 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh14 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh14 = (*src.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 6 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh15 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh15 = (*src.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 4 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh16 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh16 = (*src.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int
                    & 3 as libc::c_int) as byte;
                *vbuf = (*src.offset(1 as libc::c_int as isize) as libc::c_int >> 0 as libc::c_int
                    & 3 as libc::c_int) as byte;
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7 as libc::c_int) as isize);
                i = i.wrapping_add(1);
                src = src.offset(2 as libc::c_int as isize);
            }
        }
        3 => {
            src = (charptr as *mut byte).offset((charnum * 64 as libc::c_int) as isize);
            i = 0 as libc::c_int as libc::c_uint;
            while i < 8 as libc::c_int as libc::c_uint {
                *(vbuf as *mut qword) = *(src as *mut qword);
                i = i.wrapping_add(1);
                src = src.offset(8 as libc::c_int as isize);
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7 as libc::c_int) as isize);
            }
        }
        2 | _ => {
            src = (charptr as *mut byte).offset((charnum * 8 as libc::c_int) as isize);
            i = 0 as libc::c_int as libc::c_uint;
            while i < 8 as libc::c_int as libc::c_uint {
                let chan: [byte; 4] = [
                    *src.offset(egaplaneofs[0 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[1 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[2 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[3 as libc::c_int as usize] as isize),
                ];
                let fresh3 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh3 = EGA(chan.as_ptr(), 7 as libc::c_int as byte);
                let fresh4 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh4 = EGA(chan.as_ptr(), 6 as libc::c_int as byte);
                let fresh5 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh5 = EGA(chan.as_ptr(), 5 as libc::c_int as byte);
                let fresh6 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh6 = EGA(chan.as_ptr(), 4 as libc::c_int as byte);
                let fresh7 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh7 = EGA(chan.as_ptr(), 3 as libc::c_int as byte);
                let fresh8 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh8 = EGA(chan.as_ptr(), 2 as libc::c_int as byte);
                let fresh9 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh9 = EGA(chan.as_ptr(), 1 as libc::c_int as byte);
                *vbuf = EGA(chan.as_ptr(), 0 as libc::c_int as byte);
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7 as libc::c_int) as isize);
                i = i.wrapping_add(1);
                src = src.offset(1);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn drawpic(mut x: libc::c_int, mut y: libc::c_int, mut picnum: libc::c_int) {
    let mut vbuf: *mut byte = screenseg
        .as_mut_ptr()
        .offset((y * screenpitch as libc::c_int) as isize)
        .offset(x as isize);
    let mut src: *mut byte = 0 as *mut byte;
    let mut i: libc::c_uint = 0;
    let mut picwidth: libc::c_uint = pictable[picnum as usize].width as libc::c_uint;
    let mut picheight: libc::c_uint = pictable[picnum as usize].height as libc::c_uint;
    src = (picptr as *mut byte).offset(pictable[picnum as usize].shapeptr as isize);
    match grmode as libc::c_uint {
        1 => loop {
            i = picwidth;
            loop {
                let fresh25 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh25 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 6 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh26 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh26 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 4 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh27 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh27 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int
                    & 3 as libc::c_int) as byte;
                let fresh28 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh28 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 0 as libc::c_int
                    & 3 as libc::c_int) as byte;
                src = src.offset(1);
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset(
                (screenpitch as libc::c_int as libc::c_uint)
                    .wrapping_sub(picwidth << 2 as libc::c_int) as isize,
            );
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
        3 => loop {
            i = picwidth;
            loop {
                let fresh29 = src;
                src = src.offset(1);
                let fresh30 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh30 = *fresh29;
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset(
                (screenpitch as libc::c_int as libc::c_uint).wrapping_sub(picwidth) as isize,
            );
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
        2 | _ => loop {
            i = picwidth;
            loop {
                let chan: [byte; 4] = [
                    *src.offset(egaplaneofs[0 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[1 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[2 as libc::c_int as usize] as isize),
                    *src.offset(egaplaneofs[3 as libc::c_int as usize] as isize),
                ];
                src = src.offset(1);
                let fresh17 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh17 = EGA(chan.as_ptr(), 7 as libc::c_int as byte);
                let fresh18 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh18 = EGA(chan.as_ptr(), 6 as libc::c_int as byte);
                let fresh19 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh19 = EGA(chan.as_ptr(), 5 as libc::c_int as byte);
                let fresh20 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh20 = EGA(chan.as_ptr(), 4 as libc::c_int as byte);
                let fresh21 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh21 = EGA(chan.as_ptr(), 3 as libc::c_int as byte);
                let fresh22 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh22 = EGA(chan.as_ptr(), 2 as libc::c_int as byte);
                let fresh23 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh23 = EGA(chan.as_ptr(), 1 as libc::c_int as byte);
                let fresh24 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh24 = EGA(chan.as_ptr(), 0 as libc::c_int as byte);
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset(
                (screenpitch as libc::c_int as libc::c_uint)
                    .wrapping_sub(picwidth << 3 as libc::c_int) as isize,
            );
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
    };
}
