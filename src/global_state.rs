use crate::{exit_type::exittype, obj_def_type::objdeftype};

/// Includes most of the former globals.
pub struct GlobalState {
    pub priority: [u8; 2048],
    pub items: [i16; 6],
    pub objdef: [objdeftype; 23],
    pub side: i32,
    pub view: [[i32; 86]; 87],
    pub screencenterx: i32,
    pub screencentery: i32,
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
        screencenterx: i32,
        screencentery: i32,
        gamexit: exittype,
        oldtiles: [i32; 576],
    ) -> Self {
        Self {
            priority,
            items,
            objdef,
            side,
            view,
            screencenterx,
            screencentery,
            gamexit,
            oldtiles,
        }
    }
}
