use crate::components::game_world::universe::Universe;

pub struct World {
    pub universe: Universe,
}

impl Default for World {
    fn default() -> Self {
        Self {
            universe: Default::default(),
        }
    }
}
