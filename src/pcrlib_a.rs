use ::libc;

use crate::{
    cpanel::pictable,
    extra_constants::PC_BASE_TIMER,
    extra_types::boolean,
    global_state::GlobalState,
    pcrlib_c::{charptr, egaplaneofs, grmode, picptr, UpdateScreen},
    safe_sdl::*,
    spkr_table::SPKRtable,
};
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
type __time_t = i64;
type time_t = __time_t;
type SDL_bool = u32;
const SDL_TRUE: SDL_bool = 1;
const SDL_FALSE: SDL_bool = 0;
pub type SDL_sem = SDL_semaphore;
type SDL_AudioFormat = u16;
type SDL_AudioCallback = Option<unsafe extern "C" fn(*mut libc::c_void, *mut u8, i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_AudioSpec {
    pub freq: i32,
    pub format: SDL_AudioFormat,
    pub channels: u8,
    pub silence: u8,
    pub samples: u16,
    pub padding: u16,
    pub size: u32,
    pub callback: SDL_AudioCallback,
    pub userdata: *mut libc::c_void,
}
pub type SDL_AudioDeviceID = u32;
pub type SDL_TimerCallback = Option<unsafe extern "C" fn(u32, *mut libc::c_void) -> u32>;
pub type SDL_TimerID = i32;

type soundtype = u32;
const sdlib: soundtype = 2;
const spkr: soundtype = 1;
const off: soundtype = 0;

#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_0 {
    pub SndPriority: u8,
    pub pcSamplesPerTick: u32,
    pub pcLengthLeft: u32,
    pub pcSound: *mut u16,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct pictype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub name: [i8; 8],
}
type C2RustUnnamed_1 = u32;
const screenpitch: C2RustUnnamed_1 = 320;
type C2RustUnnamed_2 = u32;
const VBL_TIME: C2RustUnnamed_2 = 14;

#[inline]
unsafe fn EGA(mut chan: *const u8, mut ofs: u8) -> u8 {
    return ((*chan.offset(3) as i32 >> ofs as i32 & 1) << 3
        | (*chan.offset(2) as i32 >> ofs as i32 & 1) << 2
        | (*chan.offset(1) as i32 >> ofs as i32 & 1) << 1
        | *chan.offset(0) as i32 >> ofs as i32 & 1) as u8;
}

pub static mut SoundData: *mut SPKRtable = 0 as *const SPKRtable as *mut SPKRtable;
pub static mut soundmode: soundtype = spkr;
static mut SndPriority: u8 = 0;
pub static mut xormask: i32 = 0;
static mut _dontplay: i32 = 0;
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
static mut pcPhaseTick: u32 = 0;
static mut pcPhaseLength: u32 = 0;
static mut pcActive: boolean = false as boolean;
static mut pcSamplesPerTick: u32 = 0;
static mut pcNumReadySamples: u32 = 0;
static mut pcLastSample: u16 = 0;
static mut pcLengthLeft: u32 = 0;
static mut pcSound: *mut u16 = 0 as *const u16 as *mut u16;

#[inline]
unsafe fn _SDL_turnOnPCSpeaker(mut pcSample: u16) {
    // There is a bug in the SDL port; the data types used don't cover the range of values.
    // See [here](https://github.com/Blzut3/CatacombSDL/issues/4).
    //
    pcPhaseLength = pcSample as u32 * AudioSpec.freq as u32 / (2 * PC_BASE_TIMER);
    pcActive = true as boolean;
}

#[inline]
unsafe fn _SDL_turnOffPCSpeaker() {
    pcActive = false as boolean;
    pcPhaseTick = 0;
}

#[inline]
unsafe fn _SDL_PCService() {
    if !pcSound.is_null() {
        if *pcSound as i32 != pcLastSample as i32 {
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
            pcSound = 0 as *mut u16;
            SndPriority = 0;
            _SDL_turnOffPCSpeaker();
        }
    }
}

unsafe fn _SDL_PCPlaySound(mut sound: i32) {
    safe_SDL_LockMutex(AudioMutex);
    pcPhaseTick = 0;
    pcLastSample = 0;
    pcLengthLeft = ((*SoundData).sounds[sound as usize].start as i32
        - (*SoundData).sounds[(sound - 1) as usize].start as i32
        >> 1) as u32;
    pcSound = (SoundData as *mut u8)
        .offset((*SoundData).sounds[(sound - 1) as usize].start as i32 as isize)
        as *mut u16;
    SndPriority = (*SoundData).sounds[(sound - 1) as usize].priority;
    pcSamplesPerTick = (AudioSpec.freq
        / (1193181 * (*SoundData).sounds[(sound - 1) as usize].samplerate as i32 >> 16))
        as u32;
    safe_SDL_UnlockMutex(AudioMutex);
}

unsafe fn _SDL_PCStopSound() {
    safe_SDL_LockMutex(AudioMutex);
    pcSound = 0 as *mut u16;
    _SDL_turnOffPCSpeaker();
    safe_SDL_UnlockMutex(AudioMutex);
}

unsafe fn _SDL_ShutPC() {
    _SDL_PCStopSound();
}

unsafe extern "C" fn UpdateSPKR(
    mut _userdata: *mut libc::c_void,
    mut stream: *mut u8,
    mut len: i32,
) {
    if soundmode as u32 != spkr as i32 as u32 {
        memset(stream as *mut libc::c_void, 0, len as u64);
        return;
    }
    let mut sampleslen: i32 = len >> 1;
    let mut stream16: *mut i16 = stream as *mut libc::c_void as *mut i16;
    safe_SDL_LockMutex(AudioMutex);
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
                        pcVolume = -(pcVolume as i32) as libc::c_short;
                        pcPhaseTick = 0;
                    }
                }
            } else {
                while pcNumReadySamples != 0 && sampleslen != 0 {
                    pcNumReadySamples = pcNumReadySamples.wrapping_sub(1);
                    sampleslen -= 1;
                    let fresh2 = stream16;
                    stream16 = stream16.offset(1);
                    *fresh2 = 0 as i16;
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
    safe_SDL_UnlockMutex(AudioMutex);
}

pub unsafe fn StartupSound() {
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
    safe_SDL_memset(
        &mut desired as *mut SDL_AudioSpec as *mut libc::c_void,
        0,
        ::std::mem::size_of::<SDL_AudioSpec>() as u64,
    );
    desired.freq = 48000;
    desired.format = 0x8010 as i32 as SDL_AudioFormat;
    desired.channels = 1 as u8;
    desired.samples = 4096 as u16;
    desired.callback =
        Some(UpdateSPKR as unsafe extern "C" fn(*mut libc::c_void, *mut u8, i32) -> ());
    AudioMutex = safe_SDL_CreateMutex();
    if AudioMutex.is_null() || safe_SDL_InitSubSystem(0x10 as u32) < 0 || {
        AudioDev = safe_SDL_OpenAudioDevice(0 as *const i8, 0, &mut desired, &mut AudioSpec, 0);
        AudioDev == 0
    } {
        println!("Audio initialization failed: {:?}", safe_SDL_GetError());
        soundmode = off;
        _dontplay = 1;
        return;
    }
    pcSamplesPerTick = (AudioSpec.freq / 145) as u32;
    soundmode = spkr;
    safe_SDL_PauseAudioDevice(AudioDev, 0);
}

pub unsafe fn ShutdownSound() {
    if _dontplay != 0 {
        return;
    }
    _SDL_ShutPC();
    safe_SDL_CloseAudio();
}

pub unsafe fn PlaySound(mut sound: i32) {
    if _dontplay != 0 {
        return;
    }
    if (*SoundData).sounds[(sound - 1) as usize].priority as i32 >= SndPriority as i32 {
        _SDL_PCPlaySound(sound);
    }
}

unsafe fn StopSound() {
    if _dontplay != 0 {
        return;
    }
    _SDL_PCStopSound();
}
static mut SavedSound: C2RustUnnamed_0 = C2RustUnnamed_0 {
    SndPriority: 0,
    pcSamplesPerTick: 0,
    pcLengthLeft: 0,
    pcSound: 0 as *const u16 as *mut u16,
};

pub unsafe fn PauseSound() {
    if _dontplay != 0 {
        return;
    }
    safe_SDL_LockMutex(AudioMutex);
    SavedSound.SndPriority = SndPriority;
    SavedSound.pcSamplesPerTick = pcSamplesPerTick;
    SavedSound.pcLengthLeft = pcLengthLeft;
    SavedSound.pcSound = pcSound;
    SndPriority = 0;
    pcLengthLeft = 0;
    pcSound = 0 as *mut u16;
    _SDL_turnOffPCSpeaker();
    safe_SDL_UnlockMutex(AudioMutex);
}

pub unsafe fn ContinueSound() {
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

pub unsafe fn WaitEndSound(gs: &mut GlobalState) {
    if _dontplay != 0 {
        return;
    }
    UpdateScreen(gs);
    while !pcSound.is_null() {
        WaitVBL();
    }
}
static mut rndindex: u16 = 0;
static mut rndtable: [u8; 256] = [
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

static mut indexi: u16 = 0;
static mut indexj: u16 = 0;
static mut LastRnd: u16 = 0;
static mut RndArray: [u16; 17] = [0; 17];
static mut baseRndArray: [u16; 17] = [
    1, 1, 2, 3, 5, 8, 13, 21, 54, 75, 129, 204, 323, 527, 850, 1377, 2227,
];

pub unsafe fn initrnd(mut randomize: boolean) {
    memcpy(
        RndArray.as_mut_ptr() as *mut libc::c_void,
        baseRndArray.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[u16; 17]>() as u64,
    );
    LastRnd = 0;
    indexi = 17;
    indexj = 5;
    if randomize != 0 {
        let mut now: time_t = time(0 as *mut time_t);
        RndArray[16] = (now & 0xffff as i32 as i64) as u16;
        RndArray[4] = (now & 0xffff as i32 as i64 ^ now >> 16 & 0xffff as i32 as i64) as u16;
    }
    rnd(0xffff as i32 as u16);
}

pub unsafe fn rnd(mut maxval: u16) -> i32 {
    let mut mask: u16 = 0;
    let mut shift: u16 = 0;
    let mut val: i32 = 0;
    if maxval as i32 == 0 {
        return 0;
    }
    mask = 0xffff as i32 as u16;
    shift = maxval;
    while shift as i32 & 0x8000 as i32 == 0 {
        shift = ((shift as i32) << 1) as u16;
        mask = (mask as i32 >> 1) as u16;
    }
    val = RndArray[(indexi as i32 - 1) as usize] as i32
        + RndArray[(indexj as i32 - 1) as usize] as i32
        + 1;
    RndArray[(indexi as i32 - 1) as usize] = val as u16;
    val += LastRnd as i32;
    LastRnd = val as u16;
    indexi = indexi.wrapping_sub(1);
    if indexi as i32 == 0 {
        indexi = 17;
    }
    indexj = indexj.wrapping_sub(1);
    if indexj as i32 == 0 {
        indexj = 17;
    }
    val &= mask as i32;
    if val > maxval as i32 {
        val >>= 1;
    }
    return val;
}

pub unsafe fn initrndt(mut randomize: boolean) {
    rndindex = (if randomize as i32 != 0 {
        time(0 as *mut time_t) & 0xff as i32 as i64
    } else {
        0
    }) as u16;
}

pub unsafe fn rndt() -> i32 {
    rndindex = (rndindex as i32 + 1 & 0xff as i32) as u16;
    return rndtable[rndindex as usize] as i32;
}
#[no_mangle]
static mut vblsem: *mut SDL_sem = 0 as *const SDL_sem as *mut SDL_sem;
#[no_mangle]
static mut vbltimer: SDL_TimerID = 0;

unsafe extern "C" fn VBLCallback(mut _interval: u32, mut _param: *mut libc::c_void) -> u32 {
    safe_SDL_SemPost(vblsem);
    return VBL_TIME as i32 as u32;
}

pub unsafe extern "C" fn ShutdownEmulatedVBL() {
    safe_SDL_RemoveTimer(vbltimer);
    safe_SDL_DestroySemaphore(vblsem);
}

pub unsafe fn SetupEmulatedVBL() {
    vblsem = safe_SDL_CreateSemaphore(0 as u32);
    vbltimer = safe_SDL_AddTimer(
        VBL_TIME as i32 as u32,
        Some(VBLCallback as unsafe extern "C" fn(u32, *mut libc::c_void) -> u32),
        0 as *mut libc::c_void,
    );
    safe_register_shutdown_vbl_on_exit();
}

pub unsafe fn WaitVBL() {
    loop {
        safe_SDL_SemWait(vblsem);
        if !(safe_SDL_SemValue(vblsem) != 0) {
            break;
        }
    }
}

pub unsafe fn drawchar(mut x: i32, mut y: i32, mut charnum: i32, gs: &mut GlobalState) {
    let mut vbuf: *mut u8 = gs
        .screenseg
        .as_mut_ptr()
        .offset(((y << 3) * screenpitch as i32) as isize)
        .offset((x << 3) as isize);
    let mut src: *mut u8 = 0 as *mut u8;
    let mut i: u32 = 0;
    match grmode as u32 {
        1 => {
            src = (charptr as *mut u8).offset((charnum * 16) as isize);
            i = 0;
            while i < 8 {
                let fresh10 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh10 = (*src.offset(0) as i32 >> 6 & 3) as u8;
                let fresh11 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh11 = (*src.offset(0) as i32 >> 4 & 3) as u8;
                let fresh12 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh12 = (*src.offset(0) as i32 >> 2 & 3) as u8;
                let fresh13 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh13 = (*src.offset(0) as i32 >> 0 & 3) as u8;
                let fresh14 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh14 = (*src.offset(1) as i32 >> 6 & 3) as u8;
                let fresh15 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh15 = (*src.offset(1) as i32 >> 4 & 3) as u8;
                let fresh16 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh16 = (*src.offset(1) as i32 >> 2 & 3) as u8;
                *vbuf = (*src.offset(1) as i32 >> 0 & 3) as u8;
                vbuf = vbuf.offset((screenpitch as i32 - 7) as isize);
                i = i.wrapping_add(1);
                src = src.offset(2);
            }
        }
        3 => {
            src = (charptr as *mut u8).offset((charnum * 64) as isize);
            i = 0;
            while i < 8 {
                *(vbuf as *mut u64) = *(src as *mut u64);
                i = i.wrapping_add(1);
                src = src.offset(8);
                vbuf = vbuf.offset((screenpitch as i32 - 7) as isize);
            }
        }
        2 | _ => {
            src = (charptr as *mut u8).offset((charnum * 8) as isize);
            i = 0;
            while i < 8 {
                let chan: [u8; 4] = [
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
                vbuf = vbuf.offset((screenpitch as i32 - 7) as isize);
                i = i.wrapping_add(1);
                src = src.offset(1);
            }
        }
    };
}

pub unsafe fn drawpic(mut x: i32, mut y: i32, mut picnum: i32, gs: &mut GlobalState) {
    let mut vbuf: *mut u8 = gs
        .screenseg
        .as_mut_ptr()
        .offset((y * screenpitch as i32) as isize)
        .offset(x as isize);
    let mut src: *mut u8 = 0 as *mut u8;
    let mut i: u32 = 0;
    let mut picwidth: u32 = pictable[picnum as usize].width as u32;
    let mut picheight: u32 = pictable[picnum as usize].height as u32;
    src = (picptr as *mut u8).offset(pictable[picnum as usize].shapeptr as isize);
    match grmode as u32 {
        1 => loop {
            i = picwidth;
            loop {
                let fresh25 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh25 = (*src.offset(0) as i32 >> 6 & 3) as u8;
                let fresh26 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh26 = (*src.offset(0) as i32 >> 4 & 3) as u8;
                let fresh27 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh27 = (*src.offset(0) as i32 >> 2 & 3) as u8;
                let fresh28 = vbuf;
                vbuf = vbuf.offset(1);
                *fresh28 = (*src.offset(0) as i32 >> 0 & 3) as u8;
                src = src.offset(1);
                i = i.wrapping_sub(1);
                if !(i != 0) {
                    break;
                }
            }
            vbuf = vbuf.offset((screenpitch as i32 as u32).wrapping_sub(picwidth << 2) as isize);
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
            vbuf = vbuf.offset((screenpitch as i32 as u32).wrapping_sub(picwidth) as isize);
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
        2 | _ => loop {
            i = picwidth;
            loop {
                let chan: [u8; 4] = [
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
            vbuf = vbuf.offset((screenpitch as i32 as u32).wrapping_sub(picwidth << 3) as isize);
            picheight = picheight.wrapping_sub(1);
            if !(picheight != 0) {
                break;
            }
        },
    };
}
