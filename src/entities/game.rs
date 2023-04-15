use crate::{
    components::ship::names::faction_name::FactionName,
    systems::{
        combat::combat_turn, random_generation::generate_seed, ship_generation::generate_ships,
    },
};

use super::{ship::Ship, world::World};

pub struct Game {
    pub player_ship: Ship,
    pub federation_ships: [Ship; 10],
    pub klingon_ships: [Ship; 10],
    pub world: World,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_ship: Ship::new_federation_ship(),
            federation_ships: generate_ships(FactionName::Federation),
            klingon_ships: generate_ships(FactionName::KlingonEmpire),
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

    pub fn start_combat(&mut self) {
        combat_turn(
            generate_seed(),
            self.player_ship.ship_systems.select_ship_weapon_type(),
            &self.player_ship,
            &mut self.klingon_ships[0],
        );

        for ship in &self.klingon_ships {
            combat_turn(
                generate_seed(),
                ship.ship_systems.select_ship_weapon_type_ai(),
                ship,
                &mut self.player_ship,
            );
        }
    }
}

#[cfg(test)]
mod game_should {
    #[test]
    #[ignore = "hard to test"]
    fn create_a_default_game() {}
}
