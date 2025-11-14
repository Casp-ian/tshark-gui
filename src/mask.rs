use std::net::{AddrParseError, Ipv4Addr};

pub struct Ipv4Mask {
    ip: Ipv4Addr,
    bits: u8,
}

impl Ipv4Mask {
    pub fn from_str(from: &str) -> Result<Self, AddrParseError> {
        let mut split = from.split('/');

        let ip: Ipv4Addr = split.next().unwrap().parse()?;
        let bits: u8 = split.next().unwrap().parse::<u8>().unwrap();

        return Ok(Self { ip, bits });
    }

    pub fn contains(self: &Self, ip: Ipv4Addr) -> bool {
        let mask = !((1u32 << self.bits) - 1);
        let left = ip.to_bits();
        let right = self.ip.to_bits();
        return (left & mask) == (right & mask);
    }
}

// struct Ipv6Mask {}
