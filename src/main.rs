use components::ship::names::faction_name::FactionName;
use entities::game::Game;
use systems::{random_generation::generate_seed, ship_generation::generate_ships};

use crate::{entities::ship::Ship, systems::combat::combat_turn};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.player_ship
        .ship_identification
        .display_ship_name_and_faction();

    generate_ships(&mut game, FactionName::Federation, generate_seed());
    generate_ships(&mut game, FactionName::KlingonEmpire, generate_seed());

    for ship in game.federation_ships {
        ship.ship_identification.display_ship_name_and_faction();
    }

    for ship in game.klingon_ships {
        ship.ship_identification.display_ship_name_and_faction();
    }

    println!("{}", game.world.universe);

    let mut hostile = Ship::new_klingon_ship();

    combat_turn(
        generate_seed(),
        game.player_ship.ship_systems.select_ship_weapon_type(),
        &game.player_ship,
        &mut hostile,
    );
}
