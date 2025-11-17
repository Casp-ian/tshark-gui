use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{
    io::{self, BufRead},
    net::IpAddr,
    process::Command,
    thread,
};

use crate::{mask::Ipv4Mask, parse};
use regex::Regex;

pub struct Network {
    masks: Vec<Ipv4Mask>,
    ips: Vec<IpAddr>,
    reader_channel: Receiver<String>,
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
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
        return Self {
            masks,
            ips: vec![],
            reader_channel: spawn_stdin_channel(),
        };
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

    pub fn update(self: &mut Self) {
        loop {
            let line = match self.reader_channel.try_recv() {
                Ok(x) => x,
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {
                    panic!("reader thread died")
                }
            };

            let packet = parse::parse(line).unwrap();
            if let Some(data) = packet {
                if !self.ips.contains(&data.from) {
                    println!(
                        "new ip: {}, it is local {}",
                        data.from,
                        self.is_local(data.from),
                    );
                    self.ips.push(data.from);
                }
                if !self.ips.contains(&data.dest) {
                    println!(
                        "new ip: {}, it is local {}",
                        data.dest,
                        self.is_local(data.dest),
                    );
                    self.ips.push(data.dest);
                }
            }
        }
    }
}
