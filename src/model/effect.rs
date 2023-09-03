use enum_derived::Rand;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

#[derive(Debug, Rand)]
pub enum Effect {
    /*_LocationSpawnChance {
        chance: f32,
        lore: String,
        effects: Vec<LocationEffect>,
    },*/
    #[custom_rand(generate_surroundings_effect)]
    GenerateSurroundings {
        connections_with_new_nodes: u8,
        connections_to_unvisited_nodes: u8,
    },

    #[custom_rand(spawn_npc_effect)]
    SpawnNpc { spawn_chance: f64 },
}

pub fn spawn_npc_effect() -> Effect {
    Effect::SpawnNpc { spawn_chance: 1.0 }
}

pub fn generate_surroundings_effect() -> Effect {
    let mut rng = thread_rng();

    Effect::GenerateSurroundings {
        connections_with_new_nodes: Uniform::from(2..4).sample(&mut rng),
        connections_to_unvisited_nodes: Uniform::from(0..3).sample(&mut rng),
    }
}
