use std::io::Write;

pub trait Serialize {
    fn serialize<W: Write>(&self, w: W);
}
