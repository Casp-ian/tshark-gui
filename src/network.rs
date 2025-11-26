use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{
    io::{self},
    net::IpAddr,
    thread,
};

pub mod grouper;
pub mod mask;
mod parse;

#[derive(Debug)]
pub enum Protocol {
    Other,
    QUIC,
    DNS { name: String },
}

#[derive(Debug)]
pub struct IpInfo {
    pub group: grouper::Group,
}

#[derive(Debug)]
pub struct Packet {
    pub timestamp: String,
    pub from: IpAddr,
    pub dest: IpAddr,
    // pub protocol: Protocol,
}

pub struct Network {
    grouper: grouper::Grouper,
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
        return Self {
            grouper: grouper::Grouper::new(),
            reader_channel: spawn_stdin_channel(),
        };
    }

    pub fn discover(self: &Self, ip: &IpAddr) -> IpInfo {
        return IpInfo {
            group: self.grouper.group(ip),
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
