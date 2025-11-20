// #![feature(ip)]
use std::io::{self};

use crate::{network::Network, visualizer::Visualizer};

mod gui;
mod mask;
mod network;
mod parse;
mod visualizer;

fn main() -> io::Result<()> {
    let network: Network = Network::get();
    let visualizer: Visualizer = Visualizer::new(network);

    gui::open_window(visualizer).unwrap();

    std::process::exit(0);
}
