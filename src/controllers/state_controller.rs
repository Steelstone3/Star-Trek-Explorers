use crate::Game;

pub trait NewGameStateController {
    fn start_state();
    fn next_state(game: Game);
}

pub trait GameStateController {
    fn start_state(game: Game);
    fn next_state(game: Game);
}