use std::net::IpAddr;

use crate::network::{Computer, Network};

pub struct Visualizer {
    network: Network,
    pub nodes: Vec<Node>,
    pub lines: Vec<Line>,
}

impl Visualizer {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            nodes: vec![],
            lines: vec![],
        }
    }

    pub fn update(self: &mut Self) {
        let news = self.network.get_new_packets();
        self.lines = vec![];
        for new in news {
            let from = match self.get(new.from) {
                Some(node) => node.pos,
                None => {
                    let computer = self.network.group(new.from);
                    let node = self.add_computer(computer);
                    node.pos
                }
            };
            let dest = match self.get(new.dest) {
                Some(node) => node.pos,
                None => {
                    let computer = self.network.group(new.dest);
                    let node = self.add_computer(computer);
                    node.pos
                }
            };

            self.lines.push(Line {
                from: from,
                to: dest,
            });
        }
    }

    fn add_computer(&mut self, computer: Computer) -> &Node {
        let node = Node {
            computer,
            pos: Pos(rand::random_range(0.4..=0.6), rand::random_range(0.4..=0.6)),
        };
        self.nodes.push(node);

        // TODO insane
        return &self.nodes[self.nodes.len() - 1];
    }

    fn get(&self, ip: IpAddr) -> Option<&Node> {
        for node in &self.nodes {
            if node.computer.ip == ip {
                return Some(&node);
            }
        }
        return None;
    }

    pub fn solve(self: &mut Self, strength: f32) {
        // TODO

        for i in 0..self.nodes.len() {
            let old = &self.nodes[i];
            let mut force = Pos(0.0, 0.0);
            force = force + old.pos.grav(Pos(0.5, 0.5));
            for j in 0..self.nodes.len() {
                if i != j {
                    let other = &self.nodes[j];
                    force = force + old.pos.repel(other.pos);
                }
            }

            self.nodes[i].pos = self.nodes[i].pos + force.mul(strength);
        }
        return;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pos(pub f32, pub f32);

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Pos {
    fn grav(self, rhs: Self) -> Self {
        let diff = self - rhs;
        let mag = f32::max((diff.0 * diff.0) + (diff.1 * diff.1), 0.1);
        Pos(diff.0 / -mag, diff.1 / -mag)
    }
    fn repel(self, rhs: Self) -> Self {
        let diff = self - rhs;
        let mag = (diff.0 * diff.0) + (diff.1 * diff.1);
        Pos(diff.0 / mag, diff.1 / mag)
    }

    pub fn mul(self, rhs: f32) -> Self {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug)]
pub struct Node {
    pub computer: Computer,
    pub pos: Pos,
}

#[derive(Debug)]
pub struct Line {
    pub from: Pos,
    pub to: Pos,
}
