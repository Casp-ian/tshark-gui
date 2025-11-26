use std::{net::IpAddr, process::Command};

use regex::Regex;

use crate::network::mask::Ipv6Mask;

use super::mask::Ipv4Mask;

#[derive(Debug)]
pub enum Group {
    Me,
    Local,
    Global,
}

pub struct Grouper {
    me: Vec<IpAddr>,
    ipv4masks: Vec<Ipv4Mask>,
    ipv6masks: Vec<Ipv6Mask>,
}

impl Grouper {
    pub fn new() -> Grouper {
        let mut ipv4masks: Vec<Ipv4Mask> = vec![];
        let mut ipv6masks: Vec<Ipv6Mask> = vec![];
        let temp = Command::new("ip")
            .args(["address", "show"])
            .output()
            .expect("failed to start stdin reading");

        let output = String::from_utf8(temp.stdout).unwrap();

        let re = Regex::new(r"inet (?<inet>[^ ]*)|inet6 (?<inet6>[^ ]*)").unwrap();
        let caps = re.captures_iter(&output);
        for cap in caps {
            if let Some(mask) = &cap.name("inet") {
                ipv4masks.push(Ipv4Mask::from_str(mask.as_str()).unwrap());
            } else if let Some(mask) = &cap.name("inet6") {
                ipv6masks.push(Ipv6Mask::from_str(mask.as_str()).unwrap());
            }
        }

        let mut me: Vec<IpAddr> = vec![];
        for mask in &ipv6masks {
            me.push(IpAddr::V6(mask.ip));
        }
        for mask in &ipv4masks {
            me.push(IpAddr::V4(mask.ip));
        }
        return Grouper {
            me,
            ipv4masks,
            ipv6masks,
        };
    }

    pub fn group(&self, ip: &IpAddr) -> Group {
        if self.is_me(ip) {
            return Group::Me;
        }
        if self.is_local(ip) {
            return Group::Local;
        }
        return Group::Global;
    }

    pub fn is_me(&self, ip: &IpAddr) -> bool {
        for me in &self.me {
            if ip == me {
                return true;
            }
        }
        return false;
    }

    pub fn is_local(&self, ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                for mask in &self.ipv4masks {
                    if mask.contains(ipv4) {
                        return true;
                    }
                }
            }
            IpAddr::V6(ipv6) => {
                for mask in &self.ipv6masks {
                    if mask.contains(ipv6) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
