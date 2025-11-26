use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};

pub struct Ipv4Mask {
    pub ip: Ipv4Addr,
    pub bits: u8,
}

impl Ipv4Mask {
    pub fn from_str(from: &str) -> Result<Self, AddrParseError> {
        let mut split = from.split('/');

        let ip: Ipv4Addr = split.next().unwrap().parse()?;
        let bits: u8 = split.next().unwrap().parse::<u8>().unwrap();

        return Ok(Self { ip, bits });
    }

    pub fn contains(self: &Self, ip: &Ipv4Addr) -> bool {
        let mask;
        if self.bits == 32 {
            mask = u32::max_value();
        } else {
            mask = !((1u32 << self.bits) - 1);
        }
        let left = ip.to_bits();
        let right = self.ip.to_bits();
        return (left & mask) == (right & mask);
    }
}

pub struct Ipv6Mask {
    pub ip: Ipv6Addr,
    pub bits: u16,
}

impl Ipv6Mask {
    pub fn from_str(from: &str) -> Result<Self, AddrParseError> {
        let mut split = from.split('/');

        let ip: Ipv6Addr = split.next().unwrap().parse()?;
        let bits: u16 = split.next().unwrap().parse::<u16>().unwrap();

        return Ok(Self { ip, bits });
    }

    pub fn contains(self: &Self, ip: &Ipv6Addr) -> bool {
        let mask;
        if self.bits == 128 {
            mask = u128::max_value();
        } else {
            mask = !((1u128 << self.bits) - 1);
        }
        let left = ip.to_bits();
        let right = self.ip.to_bits();
        return (left & mask) == (right & mask);
    }
}

// struct Ipv6Mask {}
