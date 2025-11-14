use std::{net::IpAddr, process::Command};

use crate::mask::Ipv4Mask;
use regex::Regex;

pub struct Network {
    masks: Vec<Ipv4Mask>,
}

impl Network {
    pub fn get() -> Self {
        let mut masks: Vec<Ipv4Mask> = vec![];
        let temp = Command::new("ip")
            .args(["address", "show"])
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8(temp.stdout).unwrap();

        let re = Regex::new(r"inet (?<inet>[^ ]*)|inet6 (?<inet6>[^ ]*)").unwrap();
        let caps = re.captures_iter(&output);
        for cap in caps {
            if let Some(mask) = &cap.name("inet") {
                masks.push(Ipv4Mask::from_str(mask.as_str()).unwrap());
                eprintln!("captured: {:?}", mask.as_str());
            } else if let Some(mask) = &cap.name("inet6") {
                eprintln!("capturedv6: {:?}", mask.as_str());
            }
        }
        return Self { masks };
    }

    pub fn is_local(self: &Self, ip: IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                for mask in &self.masks {
                    if mask.contains(ipv4) {
                        return true;
                    }
                }
            }
            _ => todo!(),
        }
        return false;
    }
}
