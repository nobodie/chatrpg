use enum_derived::Rand;

#[derive(Debug)]
pub enum LocationEffect {
    _SpawnItem { lore: String },
    _SpawnNpc { lore: String },
}

pub struct Item {}

#[derive(Rand)]
pub struct Npc {}

pub enum Aura {
    _Malediction,
    _Benediction,
}
