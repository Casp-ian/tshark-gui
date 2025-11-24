use crate::network::{Packet, Protocol};

mod ek;
mod tabs;

pub fn parse(string: String) -> Result<Option<Packet>, String> {
    // return tabs::parse(string);
    return ek::parse(string);
}
