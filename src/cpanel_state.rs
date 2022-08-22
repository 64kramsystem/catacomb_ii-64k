use crate::{
    gr_type::grtype::{self, *},
    input_type::inputtype::{self, *},
    pic_type::pictype,
    sound_type::soundtype::{self, *},
    sprite_type::spritetype,
};

// Globals previously belonging to cpanel.rs.
//
pub struct CpanelState {
    /*
    Private
    */
    pub spotok: [[i32; 5]; 4],
    pub row: i32,
    pub collumn: i32,
    pub oldgrmode: grtype,
    pub newgrmode: grtype,
    pub oldsoundmode: soundtype,
    pub newsoundmode: soundtype,
    pub oldplayermode: [inputtype; 3],
    pub newplayermode: [inputtype; 3],
    pub joy1ok: i32,
    pub joy2ok: i32,
    pub mouseok: i32,

    pub egaplane: [u32; 4],
    pub image: spritetype,
    pub spritetable: [spritetype; 10],
    // Rust port: There wasn't any obvious purpose for this varaible; when `pics` was loaded, this
    // was deallocated (if valid), and replaced with a copy of the `pics` pointer.
    // pub lastgrpic: *mut libc::c_void,
    pub numchars: i32,
    pub numtiles: i32,
    pub numpics: i32,
    pub numsprites: i32,

    /*
    Public
     */
    pub pictable: [pictype; 64],
}
impl CpanelState {
    pub fn new(
        spotok: [[i32; 5]; 4],
        row: i32,
        collumn: i32,
        oldgrmode: grtype,
        newgrmode: grtype,
        oldsoundmode: soundtype,
        newsoundmode: soundtype,
        oldplayermode: [inputtype; 3],
        newplayermode: [inputtype; 3],
        joy1ok: i32,
        joy2ok: i32,
        mouseok: i32,
        egaplane: [u32; 4],
        image: spritetype,
        spritetable: [spritetype; 10],
        numchars: i32,
        numtiles: i32,
        numpics: i32,
        numsprites: i32,
        pictable: [pictype; 64],
    ) -> Self {
        Self {
            spotok,
            row,
            collumn,
            oldgrmode,
            newgrmode,
            oldsoundmode,
            newsoundmode,
            oldplayermode,
            newplayermode,
            joy1ok,
            joy2ok,
            mouseok,
            egaplane,
            image,
            spritetable,
            numchars,
            numtiles,
            numpics,
            numsprites,
            pictable,
        }
    }
}

impl Default for CpanelState {
    fn default() -> Self {
        Self::new(
            [[0; 5]; 4],
            0,
            0,
            text,
            text,
            off,
            off,
            [keyboard; 3],
            [keyboard; 3],
            0,
            0,
            0,
            [0; 4],
            spritetype {
                width: 0,
                height: 0,
                shapeptr: 0,
                maskptr: 0,
                xl: 0,
                yl: 0,
                xh: 0,
                yh: 0,
                name: [0; 12],
            },
            [spritetype {
                width: 0,
                height: 0,
                shapeptr: 0,
                maskptr: 0,
                xl: 0,
                yl: 0,
                xh: 0,
                yh: 0,
                name: [0; 12],
            }; 10],
            0,
            0,
            0,
            0,
            [pictype {
                width: 0,
                height: 0,
                shapeptr: 0,
                name: [0; 8],
            }; 64],
        )
    }
}
