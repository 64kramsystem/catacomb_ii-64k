use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use sdl2::{
    mouse::MouseUtil, AudioSubsystem, EventPump, EventSubsystem, GameControllerSubsystem,
    JoystickSubsystem, Sdl, TimerSubsystem, VideoSubsystem,
};

/// Rust port: A type that wraps the Sdl instance in a Rc, and implements a few subsystem methods,
/// in order to de-clutter its usage.
#[derive(Clone)]
pub struct RcSdl {
    /// The Rc is necessary in order to be used in Sdl events.
    sdl: Rc<Sdl>,
    // The following need to stay in scope (at least currently).
    _audio: AudioSubsystem,
    joystick: JoystickSubsystem,
    game_controller: GameControllerSubsystem,
    // This also needs to stay in scope due to lifetimes
    timer: TimerSubsystem,
    // This needs to stay in scope because there can be only one.
    event_pump: Rc<RefCell<EventPump>>,
}

impl RcSdl {
    pub fn init_sdl() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");

        let audio = sdl.audio().unwrap();
        let joystick = sdl.joystick().unwrap();
        let game_controller = sdl.game_controller().unwrap();
        let timer = sdl.timer().unwrap();
        let event_pump = Rc::new(RefCell::new(sdl.event_pump().unwrap()));

        Self {
            sdl: Rc::new(sdl),
            _audio: audio,
            joystick,
            game_controller,
            timer,
            event_pump,
        }
    }
}

impl RcSdl {
    pub fn video(&self) -> VideoSubsystem {
        self.sdl.video().unwrap()
    }

    pub fn timer(&self) -> &TimerSubsystem {
        &self.timer
    }

    pub fn joystick(&self) -> &JoystickSubsystem {
        &self.joystick
    }

    pub fn game_controller(&self) -> &GameControllerSubsystem {
        &self.game_controller
    }

    pub fn event(&self) -> EventSubsystem {
        self.sdl.event().unwrap()
    }

    pub fn mouse(&self) -> MouseUtil {
        self.sdl.mouse()
    }

    pub fn event_pump(&self) -> RefMut<EventPump> {
        (*self.event_pump).borrow_mut()
    }
}
