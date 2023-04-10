use super::{ship::Ship, world::World};

pub struct Game {
    pub player_ship: Ship,
    pub federation_ships: Vec<Ship>,
    pub klingon_ships: Vec<Ship>,
    pub world: World,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_ship: Ship::new_federation_ship(),
            federation_ships: Default::default(),
            klingon_ships: Default::default(),
            world: Default::default(),
        }
    }
}

#[cfg(test)]
mod game_should {
    #[test]
    fn create_a_default_game() {}
}
