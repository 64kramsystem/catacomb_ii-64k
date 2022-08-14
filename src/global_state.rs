use crate::{exit_type::exittype, obj_def_type::objdeftype, vec2::Vec2};

/// Includes most of the former globals.
pub struct GlobalState {
    pub priority: [u8; 2048],
    pub items: [i16; 6],
    pub objdef: [objdeftype; 23],
    pub side: i32,
    pub view: [[i32; 86]; 87],
    pub screencenter: Vec2,
    pub gamexit: exittype,
    pub oldtiles: [i32; 576],
}

impl GlobalState {
    pub fn new(
        priority: [u8; 2048],
        items: [i16; 6],
        objdef: [objdeftype; 23],
        side: i32,
        view: [[i32; 86]; 87],
        screencenter: Vec2,
        gamexit: exittype,
        oldtiles: [i32; 576],
    ) -> Self {
        Self {
            priority,
            items,
            objdef,
            side,
            view,
            screencenter,
            gamexit,
            oldtiles,
        }
    }
}
