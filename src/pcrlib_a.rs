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
    return ((*chan.offset(3) as libc::c_int >> ofs as libc::c_int & 1) << 3
        | (*chan.offset(2) as libc::c_int >> ofs as libc::c_int & 1) << 2
        | (*chan.offset(1) as libc::c_int >> ofs as libc::c_int & 1) << 1
        | *chan.offset(0) as libc::c_int >> ofs as libc::c_int & 1) as byte;
}
#[no_mangle]
pub static mut SoundData: *mut SPKRtable = 0 as *const SPKRtable as *mut SPKRtable;
#[no_mangle]
pub static mut soundmode: soundtype = spkr;
static mut SndPriority: byte = 0;
#[no_mangle]
pub static mut xormask: libc::c_int = 0;
#[no_mangle]
pub static mut _dontplay: libc::c_int = 0;
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
static mut pcVolume: libc::c_short = 5000;
static mut pcPhaseTick: libc::c_uint = 0;
static mut pcPhaseLength: libc::c_uint = 0;
static mut pcActive: boolean = false as boolean;
static mut pcSamplesPerTick: libc::c_uint = 0;
static mut pcNumReadySamples: libc::c_uint = 0;
static mut pcLastSample: word = 0;
static mut pcLengthLeft: libc::c_uint = 0;
static mut pcSound: *mut word = 0 as *const word as *mut word;
#[inline]
unsafe extern "C" fn _SDL_turnOnPCSpeaker(mut pcSample: word) {
    // The transpiler break the correctness here; in the sdl port project, `AudioSpec.freq` is a long
    // int (__syscall_slong_t), but it's translated as c_int, which doesn't accommodate the multiplication
    // result range (u32).
    //
    pcPhaseLength = pcSample as libc::c_uint * AudioSpec.freq as libc::c_uint / (2 * PC_BASE_TIMER);
    pcActive = true as boolean;
}
#[inline]
unsafe extern "C" fn _SDL_turnOffPCSpeaker() {
    pcActive = false as boolean;
    pcPhaseTick = 0;
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
            SndPriority = 0;
            _SDL_turnOffPCSpeaker();
        }
    }
}
unsafe extern "C" fn _SDL_PCPlaySound(mut sound: libc::c_int) {
    SDL_LockMutex(AudioMutex);
    pcPhaseTick = 0;
    pcLastSample = 0;
    pcLengthLeft = ((*SoundData).sounds[sound as usize].start as libc::c_int
        - (*SoundData).sounds[(sound - 1) as usize].start as libc::c_int
        >> 1) as libc::c_uint;
    pcSound = (SoundData as *mut byte)
        .offset((*SoundData).sounds[(sound - 1) as usize].start as libc::c_int as isize)
        as *mut word;
    SndPriority = (*SoundData).sounds[(sound - 1) as usize].priority;
    pcSamplesPerTick = (AudioSpec.freq
        / (1193181 * (*SoundData).sounds[(sound - 1) as usize].samplerate as libc::c_int >> 16))
        as libc::c_uint;
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
        memset(stream as *mut libc::c_void, 0, len as libc::c_ulong);
        return;
    }
    let mut sampleslen: libc::c_int = len >> 1;
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
                        pcPhaseTick = 0;
                    }
                }
            } else {
                while pcNumReadySamples != 0 && sampleslen != 0 {
                    pcNumReadySamples = pcNumReadySamples.wrapping_sub(1);
                    sampleslen -= 1;
                    let fresh2 = stream16;
                    stream16 = stream16.offset(1);
                    *fresh2 = 0 as Sint16;
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
        0,
        ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong,
    );
    desired.freq = 48000;
    desired.format = 0x8010 as libc::c_int as SDL_AudioFormat;
    desired.channels = 1 as Uint8;
    desired.samples = 4096 as Uint16;
    desired.callback =
        Some(UpdateSPKR as unsafe extern "C" fn(*mut libc::c_void, *mut Uint8, libc::c_int) -> ());
    AudioMutex = SDL_CreateMutex();
    if AudioMutex.is_null() || SDL_InitSubSystem(0x10 as libc::c_uint) < 0 || {
        AudioDev =
            SDL_OpenAudioDevice(0 as *const libc::c_char, 0, &mut desired, &mut AudioSpec, 0);
        AudioDev == 0
    } {
        printf(
            b"Audio initialization failed: %s\n\0" as *const u8 as *const libc::c_char,
            SDL_GetError(),
        );
        soundmode = off;
        _dontplay = 1;
        return;
    }
    pcSamplesPerTick = (AudioSpec.freq / 145) as libc::c_uint;
    soundmode = spkr;
    SDL_PauseAudioDevice(AudioDev, 0);
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
    if (*SoundData).sounds[(sound - 1) as usize].priority as libc::c_int
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
    SndPriority = 0;
    pcLengthLeft = 0;
    pcSound = 0 as *mut word;
    _SDL_turnOffPCSpeaker();
    SDL_UnlockMutex(AudioMutex);
}
#[no_mangle]
pub unsafe extern "C" fn ContinueSound() {
    if _dontplay != 0 {
        return;
    }
    pcPhaseTick = 0;
    pcLastSample = 0;
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
    0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242, 154,
    27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50, 28, 188,
    52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25, 178, 252, 182,
    202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193, 219, 93, 122, 175,
    249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235, 25, 92, 20, 145, 138,
    77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49, 111, 164, 70, 60, 2, 37,
    171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98, 196, 135, 106, 63, 197, 195,
    86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7, 255, 237, 129, 226, 79, 107, 112,
    166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128, 3, 184, 66, 143, 224, 145, 224, 81,
    206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28, 139, 123, 98, 125, 196, 15, 70, 194, 253,
    54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244, 138, 20, 52, 123, 251, 26, 36, 17, 46, 52,
    231, 232, 76, 31, 221, 84, 37, 216, 165, 212, 106, 197, 242, 98, 43, 39, 175, 254, 145, 190,
    84, 118, 222, 187, 136, 120, 163, 236, 249,
];
static mut indexi: word = 0;
static mut indexj: word = 0;
static mut LastRnd: word = 0;
static mut RndArray: [word; 17] = [0; 17];
static mut baseRndArray: [word; 17] = [
    1, 1, 2, 3, 5, 8, 13, 21, 54, 75, 129, 204, 323, 527, 850, 1377, 2227,
];
#[no_mangle]
pub unsafe extern "C" fn initrnd(mut randomize: boolean) {
    memcpy(
        RndArray.as_mut_ptr() as *mut libc::c_void,
        baseRndArray.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[word; 17]>() as libc::c_ulong,
    );
    LastRnd = 0;
    indexi = 17;
    indexj = 5;
    if randomize != 0 {
        let mut now: time_t = time(0 as *mut time_t);
        RndArray[16] = (now & 0xffff as libc::c_int as libc::c_long) as word;
        RndArray[4] = (now & 0xffff as libc::c_int as libc::c_long
            ^ now >> 16 & 0xffff as libc::c_int as libc::c_long) as word;
    }
    rnd(0xffff as libc::c_int as word);
}
#[no_mangle]
pub unsafe extern "C" fn rnd(mut maxval: word) -> libc::c_int {
    let mut mask: word = 0;
    let mut shift: word = 0;
    let mut val: libc::c_int = 0;
    if maxval as libc::c_int == 0 {
        return 0;
    }
    mask = 0xffff as libc::c_int as word;
    shift = maxval;
    while shift as libc::c_int & 0x8000 as libc::c_int == 0 {
        shift = ((shift as libc::c_int) << 1) as word;
        mask = (mask as libc::c_int >> 1) as word;
    }
    val = RndArray[(indexi as libc::c_int - 1) as usize] as libc::c_int
        + RndArray[(indexj as libc::c_int - 1) as usize] as libc::c_int
        + 1;
    RndArray[(indexi as libc::c_int - 1) as usize] = val as word;
    val += LastRnd as libc::c_int;
    LastRnd = val as word;
    indexi = indexi.wrapping_sub(1);
    if indexi as libc::c_int == 0 {
        indexi = 17;
    }
    indexj = indexj.wrapping_sub(1);
    if indexj as libc::c_int == 0 {
        indexj = 17;
    }
    val &= mask as libc::c_int;
    if val > maxval as libc::c_int {
        val >>= 1;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn initrndt(mut randomize: boolean) {
    rndindex = (if randomize as libc::c_int != 0 {
        time(0 as *mut time_t) & 0xff as libc::c_int as libc::c_long
    } else {
        0
    }) as word;
}
#[no_mangle]
pub unsafe extern "C" fn rndt() -> libc::c_int {
    rndindex = (rndindex as libc::c_int + 1 & 0xff as libc::c_int) as word;
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
    vblsem = SDL_CreateSemaphore(0 as Uint32);
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
        .offset(((y << 3) * screenpitch as libc::c_int) as isize)
        .offset((x << 3) as isize);
    let mut src: *mut byte = 0 as *mut byte;
    let mut i: libc::c_uint = 0;
    match grmode as libc::c_uint {
        1 => {
            src = (charptr as *mut byte).offset((charnum * 16) as isize);
            i = 0;
            while i < 8 {
                let fresh10 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh10 = (*src.offset(0) as libc::c_int >> 6 & 3) as byte;
                let fresh11 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh11 = (*src.offset(0) as libc::c_int >> 4 & 3) as byte;
                let fresh12 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh12 = (*src.offset(0) as libc::c_int >> 2 & 3) as byte;
                let fresh13 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh13 = (*src.offset(0) as libc::c_int >> 0 & 3) as byte;
                let fresh14 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh14 = (*src.offset(1) as libc::c_int >> 6 & 3) as byte;
                let fresh15 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh15 = (*src.offset(1) as libc::c_int >> 4 & 3) as byte;
                let fresh16 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh16 = (*src.offset(1) as libc::c_int >> 2 & 3) as byte;
                *vbuf = (*src.offset(1) as libc::c_int >> 0 & 3) as byte;
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7) as isize);
                i = i.wrapping_add(1);
                src = src.offset(2);
            }
        }
        3 => {
            src = (charptr as *mut byte).offset((charnum * 64) as isize);
            i = 0;
            while i < 8 {
                *(vbuf as *mut qword) = *(src as *mut qword);
                i = i.wrapping_add(1);
                src = src.offset(8);
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7) as isize);
            }
        }
        2 | _ => {
            src = (charptr as *mut byte).offset((charnum * 8) as isize);
            i = 0;
            while i < 8 {
                let chan: [byte; 4] = [
                    *src.offset(egaplaneofs[0] as isize),
                    *src.offset(egaplaneofs[1] as isize),
                    *src.offset(egaplaneofs[2] as isize),
                    *src.offset(egaplaneofs[3] as isize),
                ];
                let fresh3 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh3 = EGA(chan.as_ptr(), 7);
                let fresh4 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh4 = EGA(chan.as_ptr(), 6);
                let fresh5 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh5 = EGA(chan.as_ptr(), 5);
                let fresh6 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh6 = EGA(chan.as_ptr(), 4);
                let fresh7 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh7 = EGA(chan.as_ptr(), 3);
                let fresh8 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh8 = EGA(chan.as_ptr(), 2);
                let fresh9 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh9 = EGA(chan.as_ptr(), 1);
                *vbuf = EGA(chan.as_ptr(), 0);
                vbuf = vbuf.offset((screenpitch as libc::c_int - 7) as isize);
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
                *fresh25 = (*src.offset(0) as libc::c_int >> 6 & 3) as byte;
                let fresh26 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh26 = (*src.offset(0) as libc::c_int >> 4 & 3) as byte;
                let fresh27 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh27 = (*src.offset(0) as libc::c_int >> 2 & 3) as byte;
                let fresh28 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh28 = (*src.offset(0) as libc::c_int >> 0 & 3) as byte;
                src = src.offset(1);
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset(
                (screenpitch as libc::c_int as libc::c_uint).wrapping_sub(picwidth << 2) as isize,
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
                    *src.offset(egaplaneofs[0] as isize),
                    *src.offset(egaplaneofs[1] as isize),
                    *src.offset(egaplaneofs[2] as isize),
                    *src.offset(egaplaneofs[3] as isize),
                ];
                src = src.offset(1);
                let fresh17 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh17 = EGA(chan.as_ptr(), 7);
                let fresh18 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh18 = EGA(chan.as_ptr(), 6);
                let fresh19 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh19 = EGA(chan.as_ptr(), 5);
                let fresh20 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh20 = EGA(chan.as_ptr(), 4);
                let fresh21 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh21 = EGA(chan.as_ptr(), 3);
                let fresh22 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh22 = EGA(chan.as_ptr(), 2);
                let fresh23 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh23 = EGA(chan.as_ptr(), 1);
                let fresh24 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh24 = EGA(chan.as_ptr(), 0);
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset(
                (screenpitch as libc::c_int as libc::c_uint).wrapping_sub(picwidth << 3) as isize,
            );
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
    };
}
