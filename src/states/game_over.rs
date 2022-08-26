use crate::states::{combat::Combat, game::Game};

pub struct GameOver;

impl From<Game<Combat>> for Game<GameOver> {
    fn from(state: Game<Combat>) -> Game<GameOver> {
        println!("Game over!");
        Game {
            state: GameOver,
            player_ship: state.player_ship,
            federation_ships: state.federation_ships,
            klingon_ships: state.klingon_ships,
        }
    }
}