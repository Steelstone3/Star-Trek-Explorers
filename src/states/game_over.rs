use crate::states::{combat::Combat, game::Game};

pub struct GameOver;

impl From<Game<Combat>> for Game<GameOver> {
    fn from(state: Game<Combat>) -> Game<GameOver> {
        println!("\nGame over!");
        Game {
            state: GameOver,
            player_ship: state.player_ship,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships
        }
    }
}