use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{
    io::{self},
    net::IpAddr,
    process::Command,
    thread,
};

use crate::{mask::Ipv4Mask, parse};
use regex::Regex;

#[derive(Debug)]
pub enum Protocol {
    Other,
    QUIC,
    DNS { name: String },
}

#[derive(Debug)]
pub struct Packet {
    pub timestamp: String,
    pub from: IpAddr,
    pub dest: IpAddr,
    // pub protocol: Protocol,
}

#[derive(Debug)]
pub struct IpInfo {
    pub local: bool,
}

pub struct Network {
    masks: Vec<Ipv4Mask>,
    reader_channel: Receiver<Packet>,
}

fn spawn_stdin_channel() -> Receiver<Packet> {
    let (tx, rx) = mpsc::channel::<Packet>();
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if let Ok(packet) = parse::parse(buffer) {
                if let Some(packet) = packet {
                    tx.send(packet).unwrap();
                }
            } else {
                eprintln!("parsing error");
            }
        }
    });
    rx
}

impl Network {
    pub fn get() -> Self {
        let mut masks: Vec<Ipv4Mask> = vec![];
        let temp = Command::new("ip")
            .args(["address", "show"])
            .output()
            .expect("failed to start stdin reading");

        let output = String::from_utf8(temp.stdout).unwrap();

        let re = Regex::new(r"inet (?<inet>[^ ]*)|inet6 (?<inet6>[^ ]*)").unwrap();
        let caps = re.captures_iter(&output);
        for cap in caps {
            if let Some(mask) = &cap.name("inet") {
                masks.push(Ipv4Mask::from_str(mask.as_str()).unwrap());
                // eprintln!("captured: {:?}", mask.as_str());
            } else if let Some(mask) = &cap.name("inet6") {
                // TODO actually store mask
                // eprintln!("capturedv6: {:?}", mask.as_str());
            }
        }
        return Self {
            masks,
            reader_channel: spawn_stdin_channel(),
        };
    }

    fn is_local(&self, ip: &IpAddr) -> bool {
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

    pub fn discover(self: &Self, ip: &IpAddr) -> IpInfo {
        return IpInfo {
            local: self.is_local(ip),
        };
    }

    pub fn get_new_packets(self: &mut Self) -> Vec<Packet> {
        let mut connections: Vec<Packet> = vec![];
        loop {
            let packet = match self.reader_channel.try_recv() {
                Ok(x) => x,
                Err(TryRecvError::Empty) => return connections,
                Err(TryRecvError::Disconnected) => {
                    panic!("reader thread died")
                }
            };
            connections.push(packet);
        }
    }
}
