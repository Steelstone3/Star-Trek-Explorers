use crate::states::{combat::Combat, game::Game};

pub struct GameOver;

impl From<Game<Combat>> for Game<GameOver> {
    fn from(state: Game<Combat>) -> Game<GameOver> {
        println!("Game over!");
        Game {
            state: GameOver,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        }
    }
}