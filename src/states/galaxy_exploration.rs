use super::{game::Game, new_game::NewGame};
use crate::states::combat::Combat;

pub struct GalaxyExploration;

impl From<Game<NewGame>> for Game<GalaxyExploration> {
    fn from(state: Game<NewGame>) -> Game<GalaxyExploration> {
        Game {
            state: GalaxyExploration,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
    }
}

impl From<Game<Combat>> for Game<GalaxyExploration> {
    fn from(state: Game<Combat>) -> Game<GalaxyExploration> {
        Game {
            state: GalaxyExploration,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
    }
}
