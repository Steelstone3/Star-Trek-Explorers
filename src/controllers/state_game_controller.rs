use crate::states::{game::Game, planet_exploration::PlanetExploration, galaxy_exploration::GalaxyExploration};

pub fn start_state() {
    let new_game = Game::new();
    let galaxy_exploration = Game::<GalaxyExploration>::from(new_game);
    let _planet_exploration = Game::<PlanetExploration>::from(galaxy_exploration);
}