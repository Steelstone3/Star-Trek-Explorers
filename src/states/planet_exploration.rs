use crate::states::combat::Combat;
use crate::states::galaxy_exploration::GalaxyExploration;

use super::game::Game;

pub struct PlanetExploration;

impl From<Game<GalaxyExploration>> for Game<PlanetExploration> {
    fn from(state: Game<GalaxyExploration>) -> Game<PlanetExploration> {
        Game {
            state: PlanetExploration,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        }
    }
}

impl From<Game<Combat>> for Game<PlanetExploration> {
    fn from(state: Game<Combat>) -> Game<PlanetExploration> {
        Game {
            state: PlanetExploration,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        }
    }
}