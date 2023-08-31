pub enum Effect {
    _LocationSpawnChance {
        chance: f32,
        lore: String,
        effects: Vec<LocationEffect>,
    },
}

pub enum LocationEffect {
    _SpawnItem { lore: String },
    _SpawnNpc { lore: String },
}

pub struct Item {}

pub struct Npc {
    _name: String,
    _informations: Vec<Effect>,
}

pub enum Aura {
    _Malediction,
    _Benediction,
}
