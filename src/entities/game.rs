use crate::{
    components::ship::names::faction_name::FactionName,
    systems::{
        combat::combat_turn, random_generation::generate_seed, ship_generation::generate_ships,
    },
};

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
    pub fn print_player_ship(&self) {
        self.player_ship
            .ship_identification
            .display_ship_name_and_faction();
    }

    pub fn print_all_ai_ships(&self) {
        for ship in &self.federation_ships {
            ship.ship_identification.display_ship_name_and_faction();
        }

        for ship in &self.klingon_ships {
            ship.ship_identification.display_ship_name_and_faction();
        }
    }

    pub fn print_universe(&self) {
        println!("{}", self.world.universe);
    }

    pub fn generate_games_ships(&mut self) {
        generate_ships(self, FactionName::Federation, generate_seed());
        generate_ships(self, FactionName::KlingonEmpire, generate_seed());
    }

    pub fn start_combat(&mut self) {
        combat_turn(
            generate_seed(),
            self.player_ship.ship_systems.select_ship_weapon_type(),
            &self.player_ship,
            &mut self.klingon_ships[0],
        );

        combat_turn(
            generate_seed(),
            self.klingon_ships[0].ship_systems.select_ship_weapon_type_ai(),
            &self.klingon_ships[0],
            &mut self.player_ship,
        );
    }
}

#[cfg(test)]
mod game_should {
    #[test]
    #[ignore = "hard to test"]
    fn create_a_default_game() {}
}
