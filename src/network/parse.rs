use crate::network::{Packet, Protocol};

mod ek;
mod tabs;

pub fn parse(string: String) -> Result<Option<Packet>, String> {
    // return ek::parse(string);
    return tabs::parse(string);
}
