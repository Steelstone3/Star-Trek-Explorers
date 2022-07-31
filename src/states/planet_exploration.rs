use crate::states::galaxy_exploration::GalaxyExploration;
use crate::controllers::state_controller::GameStateController;

use crate::Game;

pub struct PlanetExploration {
    game: Game,
}

impl GameStateController for PlanetExploration {
    fn start_state(game: Game) {
        let planet_exploration = PlanetExploration { game };

        let _star_system = planet_exploration.game.galaxy.star_systems.get(planet_exploration.game.game_progress).expect("Index out of bounds");
        
        // display_next_scanned_star_system(star_system);

        // Self::next_state(galaxy_exploration.game);
    }

    fn next_state(game: Game) {
        GalaxyExploration::start_state(game);
    }
}
