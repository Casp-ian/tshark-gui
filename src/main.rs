// #![feature(ip)]
use std::io::{self};

use crate::network::Network;

mod gui;
mod mask;
mod network;
mod parse;
mod visualizer;

fn main() -> io::Result<()> {
    let network: Network = Network::get();

    gui::open_window(network).unwrap();

    return Ok(());
}
