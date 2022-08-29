use std::rc::Rc;

use sdl2::{
    AudioSubsystem, EventSubsystem, GameControllerSubsystem, JoystickSubsystem, Sdl,
    TimerSubsystem, VideoSubsystem,
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
}

impl RcSdl {
    pub fn init_sdl() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");

        let audio = sdl.audio().unwrap();
        let joystick = sdl.joystick().unwrap();
        let game_controller = sdl.game_controller().unwrap();

        Self {
            sdl: Rc::new(sdl),
            _audio: audio,
            joystick,
            game_controller,
        }
    }
}

impl RcSdl {
    pub fn video(&self) -> VideoSubsystem {
        self.sdl.video().unwrap()
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
}
