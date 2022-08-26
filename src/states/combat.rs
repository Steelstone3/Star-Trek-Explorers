use super::{
    galaxy_exploration::GalaxyExploration, game::Game, planet_exploration::PlanetExploration,
};
use crate::controllers::combat::enter_combat;

pub struct Combat;

impl From<Game<GalaxyExploration>> for Game<Combat> {
    fn from(state: Game<GalaxyExploration>) -> Game<Combat> {
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
    if !game.klingon_ships.is_empty() {
        enter_combat(&mut game.player_ship);
    }
}

// fn enter_combat(player_ship: &mut FederationShip, federation_ships: &[FederationShip], klingon_ships: &[KlingonShip]) -> _ {
//     todo!()
// }
