use super::{
    galaxy_exploration::GalaxyExploration, game::Game, planet_exploration::PlanetExploration,
};

pub struct Combat;

impl From<Game<GalaxyExploration>> for Game<Combat> {
    fn from(state: Game<GalaxyExploration>) -> Game<Combat> {
        //let game =
        Game {
            state: Combat,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
        // game
    }
}

impl From<Game<PlanetExploration>> for Game<Combat> {
    fn from(state: Game<PlanetExploration>) -> Game<Combat> {
        // let game = 
        Game {
            state: Combat,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }

        // game
    }
}
