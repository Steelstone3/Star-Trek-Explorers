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

impl Game {
    pub fn print_all_ships_in_world(&self) {
        self.player_ship.ship_identification.display_ship_name_and_faction();
        
        // for ship in self.federation_ships {

        // }
    }
}

#[cfg(test)]
mod game_should {
    #[test]
    #[ignore = "hard to test"]
    fn create_a_default_game() {}
}
