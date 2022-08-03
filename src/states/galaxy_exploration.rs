use super::{new_game::NewGame, game::Game};

pub struct GalaxyExploration;

impl From<Game<NewGame>> for Game<GalaxyExploration> {
    fn from(state: Game<NewGame>) -> Game<GalaxyExploration> {
        Game {
            state: GalaxyExploration,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        }
    }
}