use crate::states::{game::Game, galaxy_exploration::{GalaxyExploration}, combat::Combat, game_over::GameOver};

pub fn start_state() {
    let new_game = Game::new();
    let galaxy_exploration = Game::<GalaxyExploration>::from(new_game);
    let combat = Game::<Combat>::from(galaxy_exploration);
    let _game_over = Game::<GameOver>::from(combat);
}