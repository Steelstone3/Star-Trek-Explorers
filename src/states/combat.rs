use crate::controllers::{encounters::{federation_encounter::generate_federation_ships, klingon_encounter::generate_klingon_ships}, random_number::get_random_number_from_range};

use super::{
    galaxy_exploration::GalaxyExploration, game::Game, planet_exploration::PlanetExploration,
};

pub struct Combat;

impl From<Game<GalaxyExploration>> for Game<Combat> {
    fn from(state: Game<GalaxyExploration>) -> Game<Combat> {
        let mut game =Game {
            state: Combat,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        };

        start_combat(&mut game);

        game
    }
}

impl From<Game<PlanetExploration>> for Game<Combat> {
    fn from(state: Game<PlanetExploration>) -> Game<Combat> {
        let mut game = Game {
            state: Combat,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        };

        start_combat(&mut game);

        game
    }
}

fn start_combat(game: &mut Game<Combat>) {
    game.federation_ships = generate_federation_ships(get_random_number_from_range(0, 5) as u32);
    game.klingon_ships = generate_klingon_ships(get_random_number_from_range(1, 10) as u32);

    if !game.klingon_ships.is_empty() {
        // enter_combat(
        //     &mut game.player_ship,
        //     &mut game.federation_ships,
        //     &mut game.klingon_ships,
        // );
    }
}

// fn enter_combat(player_ship: &mut FederationShip, federation_ships: &[FederationShip], klingon_ships: &[KlingonShip]) -> _ {
//     todo!()
// }