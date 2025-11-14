// #![feature(ip)]
use std::{
    io::{self},
    net::IpAddr,
};

use crate::network::Network;

mod mask;
mod network;
mod parse;

fn main() -> io::Result<()> {
    let mut ips: Vec<IpAddr> = vec![];

    let network: Network = Network::get();
    // let mut tos: Vec<IpAddr> = vec![];

    for line in io::stdin().lines() {
        let packet = parse::parse(line.unwrap()).unwrap();
        // if let Some(data) = packet {
        //     println!();
        //     println!("{} → {}", data.from, data.dest);
        //     println!("{} → {}", data.from.is_global(), data.dest.is_global());
        // }

        if let Some(data) = packet {
            if !ips.contains(&data.from) {
                println!(
                    "new ip: {}, it is local {}",
                    data.from,
                    network.is_local(data.from),
                );
                ips.push(data.from);
            }
            if !ips.contains(&data.dest) {
                println!(
                    "new ip: {}, it is local {}",
                    data.dest,
                    network.is_local(data.dest),
                );
                ips.push(data.dest);
            }
        }
        // println!("sources: {:?}", froms);
        // println!("destinations: {:?}", tos);
    }

    return Ok(());
}
