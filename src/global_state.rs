use crate::{exit_type::exittype, obj_def_type::objdeftype, vec2::Vec2};

/// Includes most of the former globals.
pub struct GlobalState {
    pub gamexit: exittype, /*determines what to do after playloop*/

    pub oldtiles: [i32; 576],        /*tile displayed last refresh*/
    pub background: [[i32; 86]; 87], /*base map*/
    pub view: [[i32; 86]; 87],       /*base map with objects drawn in*/
    //     int originx, originy;       /*current world location of ul corn*/
    pub priority: [u8; 2048], /*tile draw overlap priorities*/

    //     sword items[6], saveitems[6];
    pub items: [i16; 6],
    //     int shotpower; /*0-13 characters in power meter*/
    pub side: i32, /*which side shots come from*/
    //     int boltsleft; /*number of shots left in a bolt*/

    //     activeobj o[maxobj + 1], saveo[1]; /*everything that moves is here*/
    //     objtype obj, altobj;               /*total info about objecton and alt*/
    //     int altnum;                        /*o[#] of altobj*/
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
        objdef: [objdeftype; 23],
        side: i32,
        view: [[i32; 86]; 87],
        screencenter: Vec2,
        gamexit: exittype,
        oldtiles: [i32; 576],
        screenseg: [u8; 64000],
        background: [[i32; 86]; 87],
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
            screenseg,
            background,
        }
    }
}
