use crate::states::{game::Game, planet_exploration::PlanetExploration, galaxy_exploration::GalaxyExploration, combat::Combat, game_over::GameOver};

pub fn start_state() {
    let new_game = Game::new();
    
    let galaxy_exploration_1 = Game::<GalaxyExploration>::from(new_game);
    let galaxy_combat = Game::<Combat>::from(galaxy_exploration_1);
    let galaxy_exploration_2 = Game::<GalaxyExploration>::from(galaxy_combat);
    
    let planet_exploration_1 = Game::<PlanetExploration>::from(galaxy_exploration_2);
    let planet_combat = Game::<Combat>::from(planet_exploration_1);
    
    let _game_over = Game::<GameOver>::from(planet_combat);
}