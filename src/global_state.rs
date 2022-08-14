use crate::{
    active_obj::activeobj, exit_type::exittype, obj_def_type::objdeftype, obj_type::objtype,
    vec2::Vec2,
};

/// Includes most of the former globals.
pub struct GlobalState {
    pub gamexit: exittype, /*determines what to do after playloop*/

    pub oldtiles: [i32; 576],        /*tile displayed last refresh*/
    pub background: [[i32; 86]; 87], /*base map*/
    pub view: [[i32; 86]; 87],       /*base map with objects drawn in*/
    pub origin: Vec2,                /*current world location of ul corn*/
    pub priority: [u8; 2048],        /*tile draw overlap priorities*/

    pub items: [i16; 6],
    pub saveitems: [i16; 6],
    pub shotpower: i32, /*0-13 characters in power meter*/
    pub side: i32,      /*which side shots come from*/
    pub boltsleft: i32, /*number of shots left in a bolt*/

    pub o: [activeobj; 201],   /*everything that moves is here*/
    pub saveo: [activeobj; 1], // ^^
    pub obj: objtype,          /*total info about objecton and alt*/
    pub altobj: objtype,       // ^^
    pub altnum: i32,           /*o[#] of altobj*/
    //     int numobj, objecton;              /*number of objects in o now*/

    //
    pub objdef: [objdeftype; 23],

    //     boolean playdone, leveldone;
    //
    //     boolean tempb;
    //     char *tempp;
    //
    //     int chkx, chky, chkspot; /*spot being checked by walk*/
    //
    //     word frameon;
    //     char *grmem;
    //     classtype clvar;
    //
    //     int VGAPAL; // just to make pcrlib happy
    //
    //     boolean exitdemo, resetgame;
    //     statetype gamestate;
    //
    //     ControlStruct ctrl;
    //
    //     char *pics, *picsexact;
    //
    //     unsigned EGADATASTART;
    //
    //     sdword savescore;
    //
    //     // NOLAN ADDED
    //     boolean GODMODE = false;

    // loaded into ES in the draw routines
    // should be adjusted after grmode
    // switches, page flipping, and scrolls
    pub screenseg: [u8; 64000],

    pub screencenter: Vec2,
}

impl GlobalState {
    pub fn new(
        priority: [u8; 2048],
        items: [i16; 6],
        saveitems: [i16; 6],
        shotpower: i32,
        o: [activeobj; 201],
        saveo: [activeobj; 1],
        obj: objtype,
        altobj: objtype,
        altnum: i32,
        objdef: [objdeftype; 23],
        side: i32,
        boltsleft: i32,
        view: [[i32; 86]; 87],
        screencenter: Vec2,
        gamexit: exittype,
        oldtiles: [i32; 576],
        screenseg: [u8; 64000],
        background: [[i32; 86]; 87],
        origin: Vec2,
    ) -> Self {
        Self {
            priority,
            items,
            saveitems,
            shotpower,
            o,
            saveo,
            obj,
            altobj,
            altnum,
            objdef,
            side,
            boltsleft,
            view,
            screencenter,
            gamexit,
            oldtiles,
            screenseg,
            background,
            origin,
        }
    }
}
