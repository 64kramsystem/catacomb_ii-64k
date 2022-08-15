use crate::{gr_type::grtype, input_type::inputtype, sound_type::soundtype};

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
        }
    }
}
