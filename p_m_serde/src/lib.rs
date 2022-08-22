// Poor Man's Serde. Trivial pair of interfaces for de/serialization.

mod builtin_types;
mod deserialize;
mod macros;
mod serialize;

pub use deserialize::Deserialize;
pub use serialize::Serialize;
