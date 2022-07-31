use crate::states::planet_exploration::PlanetExploration;
use crate::controllers::state_controller::GameStateController;
use crate::models::game::Game;
use crate::presenters::galaxy_presenter::display_galaxy;

pub struct GalaxyExploration {
    game: Game,
}

impl GameStateController for GalaxyExploration {
    fn start_state(game: Game) {
        let galaxy_exploration = GalaxyExploration { game };

        display_galaxy(galaxy_exploration.game.galaxy);

        // Self::next_state(galaxy_exploration.game);
    }

    fn next_state(game: Game) {
        PlanetExploration::start_state(game);
    }
}
