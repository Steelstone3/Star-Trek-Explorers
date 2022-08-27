use crate::{
    presenters::presenter::{confirmation, menu_of},
    states::{
        combat::Combat, galaxy_exploration::GalaxyExploration, game::Game, game_over::GameOver,
    },
};

const NEW_GAME: &str = "New Game";
const LOAD_GAME: &str = "Load Game";

pub fn start_state() {
    if main_menu() == NEW_GAME {
        while confirmation("\nStart New Game:") {
            let new_game = Game::new();
            let galaxy_exploration = Game::<GalaxyExploration>::from(new_game);
            let combat = Game::<Combat>::from(galaxy_exploration);
            let _game_over = Game::<GameOver>::from(combat);
        }
    }
}

fn main_menu() -> String {
    menu_of(
        "Star Trek Explorers:",
        vec![NEW_GAME.to_string(), LOAD_GAME.to_string()],
    )
}
