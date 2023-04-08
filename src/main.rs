use components::ship::name::faction_name::FactionName;
use entities::{
    game::Game,
    ships::ship::Ship,
};
use systems::{random_generation::generate_seed, ship_generation::generate_ships};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.player_ship.display_ship_name();

    generate_ships(&mut game, FactionName::Federation, generate_seed());
    generate_ships(&mut game, FactionName::KlingonEmpire, generate_seed());

    for ship in game.federation_ships {
        ship.display_ship_name();
    }

    for ship in game.klingon_ships {
        ship.display_ship_name();
    }
}
