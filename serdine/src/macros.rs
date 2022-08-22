macro_rules! impl_for_numeric {
    ( Deserialize, $( $type:ty ),+ ) => {
        $(
            impl crate::Deserialize for $type {
                fn deserialize<R: std::io::Read>(mut r: R) -> Self {
                    let mut buffer = [0; std::mem::size_of::<$type>()];
                    r.read_exact(&mut buffer).unwrap();
                    <$type>::from_le_bytes(buffer)
                }
            }
        )+
    };
    ( Serialize, $( $type:ty ),+ ) => {
        $(
            impl crate::Serialize for $type {
                fn serialize<W: std::io::Write>(&self, mut w: W) {
                    let encoded = self.to_le_bytes();
                    w.write_all(&encoded).unwrap();
                }
            }
        )+
    };
}

pub(crate) use impl_for_numeric;
