use p_m_serde::Deserialize;

#[derive(Clone, Copy)]
pub struct pictype {
    pub width: i16,
    pub height: i16,
    pub shapeptr: u32,
    pub name: [i8; 8],
}

impl Deserialize for pictype {
    fn deserialize<R: std::io::Read>(mut r: R) -> Self {
        let width = Deserialize::deserialize(&mut r);
        let height = Deserialize::deserialize(&mut r);
        let shapeptr = Deserialize::deserialize(&mut r);
        let name = Deserialize::deserialize(&mut r);

        Self {
            width,
            height,
            shapeptr,
            name,
        }
    }
}
