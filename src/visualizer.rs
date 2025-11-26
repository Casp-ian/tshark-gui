use std::collections::BTreeMap;
use std::net::IpAddr;

use crate::network::{IpInfo, Network};

#[derive(Debug)]
pub struct Node {
    pub info: IpInfo,
    pub pos: Pos,
}

#[derive(Debug)]
pub struct Edge<T> {
    pub from: T,
    pub dest: T,
    pub brightness: f32,
    // pub protocol: Protocol,
}

pub struct Visualizer {
    network: Network,
    nodes: BTreeMap<IpAddr, Node>,
    edges: Vec<Edge<IpAddr>>,
}

impl Visualizer {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            nodes: BTreeMap::new(),
            edges: vec![],
        }
    }

    pub fn update(self: &mut Self) {
        let news = self.network.get_new_packets();
        for new in news {
            self.edges.push(Edge {
                from: new.from,
                dest: new.dest,
                brightness: 1.0,
            });

            if !self.nodes.contains_key(&new.from) {
                self.add_node(&new.from, self.network.discover(&new.from));
            }
            if !self.nodes.contains_key(&new.dest) {
                self.add_node(&new.dest, self.network.discover(&new.dest));
            }
        }
    }

    fn add_node(&mut self, ip: &IpAddr, info: IpInfo) -> &Node {
        let node = Node {
            info: info,
            pos: Pos(rand::random_range(0.1..=0.9), rand::random_range(0.1..=0.9)),
        };
        self.nodes.insert(*ip, node);

        return &self.nodes.get(ip).unwrap();
    }

    pub fn solve(self: &mut Self, strength: f32) {
        self.edges
            .iter_mut()
            .for_each(|edge| edge.brightness -= strength);
        self.edges.retain(|edge| edge.brightness > 0.0);

        let mut results: Vec<Pos> = vec![];
        results.reserve(self.nodes.len());

        let mut repels: Vec<Pos> = vec![];

        for (i, node1) in self.nodes.values().enumerate() {
            let mut force = Pos(0.0, 0.0);
            for (j, node2) in self.nodes.values().enumerate() {
                if i == j {
                    continue;
                }
                force = force + node1.pos.repel(node2.pos).mul(0.0001 * strength);
            }
            repels.push(force);
        }

        for (i, node) in self.nodes.values_mut().enumerate() {
            node.pos = node.pos + repels[i];
        }

        for (i, node) in self.nodes.values_mut().enumerate() {
            node.pos = node.pos + node.pos.center_force().mul(5.0 * strength);
        }
        return;
    }

    pub fn get_nodes(&self) -> Vec<(&IpAddr, &Node)> {
        return self.nodes.iter().collect();
    }
    pub fn get_edges(&self) -> Vec<Edge<Pos>> {
        return self
            .edges
            .iter()
            .map(|line| Edge {
                from: self.nodes.get(&line.from).unwrap().pos,
                dest: self.nodes.get(&line.dest).unwrap().pos,
                brightness: line.brightness,
            })
            .collect();
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
    fn center_force(self) -> Self {
        let diff = self - Pos(0.5, 0.5);
        let mag = (diff.0 * diff.0) + (diff.1 * diff.1);
        let mag = mag * mag;
        return diff.mul(-mag);
    }
    fn grav(self, rhs: Self) -> Self {
        let diff = self - rhs;
        let mag = f32::max((diff.0 * diff.0) + (diff.1 * diff.1), 0.1);
        Pos(diff.0 / -mag, diff.1 / -mag)
    }
    fn repel(self, rhs: Self) -> Self {
        let diff = self - rhs;
        let mag = (diff.0 * diff.0) + (diff.1 * diff.1);
        if mag > 0.1 {
            return Pos(0.0, 0.0);
        }
        Pos(diff.0 / mag, diff.1 / mag)
    }

    pub fn mul(self, rhs: f32) -> Self {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}
