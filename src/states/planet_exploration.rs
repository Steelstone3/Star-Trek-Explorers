use super::game::Game;
use crate::states::combat::Combat;
use crate::states::galaxy_exploration::GalaxyExploration;

pub struct PlanetExploration;

impl From<Game<GalaxyExploration>> for Game<PlanetExploration> {
    fn from(state: Game<GalaxyExploration>) -> Game<PlanetExploration> {
        Game {
            state: PlanetExploration,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
    }
}

impl From<Game<Combat>> for Game<PlanetExploration> {
    fn from(state: Game<Combat>) -> Game<PlanetExploration> {
        Game {
            state: PlanetExploration,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
    }
}
