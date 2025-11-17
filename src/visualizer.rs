use std::net::IpAddr;

pub struct Visualizer {
    pub nodes: Vec<Node>,
    pub lines: Vec<Line>,
}

impl Visualizer {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            lines: vec![],
        }
    }
    pub fn solve(self: &mut Self, strength: f32) {
        todo!();
    }

    pub fn addNode(self: &mut Self, node: Node) {
        todo!();
    }
}

pub struct Node {
    pub ip: IpAddr,
    pub group: String,
    pub pos: (f32, f32),
}

pub struct Line {
    pub from: (f32, f32),
    pub to: (f32, f32),
}
