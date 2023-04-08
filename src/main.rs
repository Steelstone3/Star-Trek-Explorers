use components::ship::name::faction_name::FactionName;
use entities::{ships::{federation_ship::FederationShip, ship::Ship}, game::Game};
use systems::ship_generation::{self, generate_ships};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.player_ship = FederationShip::default();

    game.player_ship.display_ship_name();
    
    generate_ships(&mut game, FactionName::Federation, u8::MAX);
    generate_ships(&mut game, FactionName::KlingonEmpire, u8::MAX);

    for ship in game.federation_ships {
        ship.display_ship_name();
    }

    for ship in game.klingon_ships {
        ship.display_ship_name();
    }
}
