use crate::components::game_world::universe::Universe;

#[allow(dead_code)]
pub struct World {
    universe: Universe,
}

impl Default for World {
    fn default() -> Self {
        Self { universe: Universe::default() }
    }
}

#[cfg(test)]
mod world_should {}
