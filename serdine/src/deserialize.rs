use std::io::Read;

pub trait Deserialize {
    fn deserialize<R: Read>(r: R) -> Self;
}
