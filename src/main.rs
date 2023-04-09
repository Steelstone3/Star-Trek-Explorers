use components::ship::names::faction_name::FactionName;
use entities::{game::Game, ships::ship::Ship};
use systems::{random_generation::generate_seed, ship_generation::generate_ships};

use crate::{entities::ships::klingon_ship::KlingonShip, systems::combat::start_combat};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.player_ship.display_ship_name_and_faction();

    generate_ships(&mut game, FactionName::Federation, generate_seed());
    generate_ships(&mut game, FactionName::KlingonEmpire, generate_seed());

    for ship in game.federation_ships {
        ship.display_ship_name_and_faction();
    }

    for ship in game.klingon_ships {
        ship.display_ship_name_and_faction();
    }

    println!("{}", game.world.universe);

    let mut hostile = KlingonShip::default();
    start_combat(&mut game.player_ship, &mut hostile);
}
