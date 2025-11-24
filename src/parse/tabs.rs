use std::net::IpAddr;

use crate::network::{Packet, Protocol};

pub fn parse(string: String) -> Result<Packet, String> {
    let mut split = string.split("\t");
    let _id = split.next();
    let time = split.next().unwrap().to_owned().replace(' ', "");
    let from = split.next().unwrap().replace(' ', "").parse().unwrap();
    let _arrow = split.next();
    let dest = split.next().unwrap().replace(' ', "").parse().unwrap();
    // let protocol = split.next();

    return Ok(Packet {
        timestamp: time,
        from: from,
        dest: dest,
    });
}
