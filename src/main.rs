use components::ship::names::faction_name::FactionName;
use entities::game::Game;
use systems::{random_generation::generate_seed, ship_generation::generate_ships};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.player_ship
        .ship_identification
        .display_ship_name_and_faction(
            game.player_ship.name.to_string(),
            game.player_ship.class.to_string(),
        );

    generate_ships(&mut game, FactionName::Federation, generate_seed());
    generate_ships(&mut game, FactionName::KlingonEmpire, generate_seed());

    for ship in game.federation_ships {
        ship.ship_identification
            .display_ship_name_and_faction(ship.name.to_string(), ship.class.to_string());
    }

    for ship in game.klingon_ships {
        ship.ship_identification
            .display_ship_name_and_faction(ship.name.to_string(), ship.class.to_string());
    }

    println!("{}", game.world.universe);
}
