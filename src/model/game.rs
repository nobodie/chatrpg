use super::{
    node::{Connection, Node, NodeId, NodeStatus},
    old::{Aura, Effect, Item},
};
use crate::{
    error::{RpgError, RpgResult},
    PlayerChoice,
};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

pub struct Game {
    nodes: Vec<Node>,
    connections: HashSet<Connection>,

    current_node: NodeId,
    _inventory: Vec<Item>,
    _money: u16,
    _auras: Vec<Aura>,

    _effects: Vec<Effect>,
}

impl Game {
    pub fn new() -> Self {
        let mut spawn = Node::new();
        spawn.status = NodeStatus::Visited;

        let mut game = Self {
            nodes: vec![spawn],
            connections: HashSet::new(),
            current_node: NodeId(0),
            _inventory: Vec::new(),
            _money: 0,
            _auras: Vec::new(),
            _effects: Vec::new(),
        };

        game.generate_surroundings(3, 0);

        game
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> RpgResult<&mut Node> {
        self.nodes.get_mut(id.0).ok_or(RpgError::UnknownNodeId(id))
    }

    pub fn get_node(&self, id: NodeId) -> RpgResult<&Node> {
        self.nodes.get(id.0).ok_or(RpgError::UnknownNodeId(id))
    }

    pub fn get_current_node(&self) -> (NodeId, &Node) {
        (
            self.current_node.clone(),
            self.get_node(self.current_node.clone()).expect("cant fail"),
        )
    }

    pub fn get_node_ids_with_status(&self, status: NodeStatus) -> Vec<NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(id, n)| (n.status == status).then_some(NodeId(id)))
            .collect()
    }

    pub fn _get_nodes_with_status(&self, status: NodeStatus) -> Vec<(NodeId, &Node)> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(id, n)| (n.status == status).then_some((NodeId(id), n)))
            .collect()
    }

    pub fn _get_nodes_with_status_mut(&mut self, status: NodeStatus) -> Vec<(NodeId, &mut Node)> {
        self.nodes
            .iter_mut()
            .enumerate()
            .filter_map(|(id, n)| (n.status == status).then_some((NodeId(id), n)))
            .collect()
    }

    pub fn move_to(&mut self, node_id: NodeId) -> RpgResult<()> {
        if !self
            .connections
            .iter()
            .any(|c| c.contains(self.current_node.clone()) && c.contains(node_id.clone()))
        {
            return Err(RpgError::NodeNotConnected(
                self.current_node.clone(),
                node_id,
            ));
        }

        let node = self.get_node_mut(node_id.clone())?;

        node.status = NodeStatus::Visited;
        self.current_node = node_id;

        Ok(())
    }

    fn generate_surroundings(
        &mut self,
        connections_with_new_nodes: u8,
        connections_to_unvisited_nodes: u8,
    ) {
        for _ in 0..connections_with_new_nodes {
            let new_node = Node::new();

            let new_node_id = NodeId(self.nodes.len());

            self.nodes.push(new_node);
            self.connections
                .insert(Connection((self.current_node.clone(), new_node_id)));
        }

        let mut unvisited_nodes = self.get_node_ids_with_status(NodeStatus::Unvisited);
        unvisited_nodes.shuffle(&mut thread_rng());

        for _ in 0..connections_to_unvisited_nodes {
            if let Some(id) = unvisited_nodes.pop() {
                self.connections
                    .insert(Connection((self.current_node.clone(), id)));
            }
        }
    }

    pub fn generate_choices(&self) -> Vec<PlayerChoice> {
        let mut choices = Vec::new();

        for (node_id, surrounding_node) in self.get_surrounding_nodes() {
            match surrounding_node.status {
                NodeStatus::Visited => choices.push(PlayerChoice::VisitNode(node_id)),
                NodeStatus::Unvisited => choices.push(PlayerChoice::DiscoverNode(node_id)),
            }
        }
        choices.push(PlayerChoice::Quit);

        choices
    }

    pub fn get_surrounding_nodes(&self) -> Vec<(NodeId, &Node)> {
        self.get_surrounding_node_ids()
            .iter()
            .map(|id| (id.clone(), self.get_node(id.clone()).expect("can't fail")))
            .collect()
    }

    pub fn get_surrounding_node_ids(&self) -> Vec<NodeId> {
        self.connections
            .iter()
            .filter(|c| c.contains(self.current_node.clone()))
            .map(|c| c.other_end(self.current_node.clone()))
            .collect()
    }

    pub fn handle_choice(&mut self, choice: &PlayerChoice) -> RpgResult<bool> {
        match choice {
            PlayerChoice::Quit => return Ok(true),
            PlayerChoice::DiscoverNode(node_id) => {
                self.move_to(node_id.clone())?;
                self.generate_surroundings(2, 1);
            }
            PlayerChoice::VisitNode(node_id) => {
                self.move_to(node_id.clone())?;
            }
        }

        Ok(false)
    }
}
