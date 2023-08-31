use super::old::{Effect, Npc};
use std::{fmt::Display, hash::Hash};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct NodeId(pub usize);

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

#[derive(Eq)]
pub struct Connection(pub (NodeId, NodeId));

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) || (self.0 .0 == other.0 .1 && self.0 .1 == other.0 .0)
    }
}

impl Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.0 .0 < self.0 .1 {
            self.0 .0.hash(state);
            self.0 .1.hash(state);
        } else {
            self.0 .1.hash(state);
            self.0 .0.hash(state);
        }
    }
}

impl Connection {
    pub fn contains(&self, node: NodeId) -> bool {
        self.0 .0 == node || self.0 .1 == node
    }

    pub fn other_end(&self, node: NodeId) -> NodeId {
        if self.0 .0 == node {
            self.0 .1.clone()
        } else {
            self.0 .0.clone()
        }
    }
}

#[derive(PartialEq)]
pub enum NodeStatus {
    Visited,
    Unvisited,
}

pub struct Node {
    pub status: NodeStatus,
    _informations: Vec<Effect>,
    _npcs: Vec<Npc>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            status: NodeStatus::Unvisited,
            _informations: Vec::new(),
            _npcs: Vec::new(),
        }
    }
}
