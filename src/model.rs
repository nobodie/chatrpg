pub struct Inventory {}

pub enum Effect {
    LocationSpawnChance {
        chance: f32,
        lore: String,
        effects: Vec<LocationEffect>,
    },
}

pub enum LocationEffect {
    SpawnItem { lore: String },
    SpawnNpc { lore: String },
}

pub struct Item {}

pub struct NodeId(usize);

pub enum NodeStatus {
    Visited,
    Unvisited,
}

pub struct Npc {
    name: String,
    informations: Vec<Effect>,
}

pub struct Node {
    name: String,
    description: String,
    connections: Vec<NodeId>,
    status: NodeStatus,
    informations: Vec<Effect>,
    npcs: Vec<Npc>,
}

pub enum Aura {
    Malediction,
    Benediction,
}

pub struct game {
    nodes: Vec<Node>,

    current_node: NodeId,
    inventory: Vec<Item>,
    money: u16,
    auras: Vec<Aura>,

    effects: Vec<Effect>,
}
