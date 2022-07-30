use crate::presenters::galaxy_presenter::display_galaxy;
use crate::states::new_game::Game;

pub struct GalaxyExploration {
    game: Game,
}

pub trait GalaxyExplorationStateController {
    fn start_state(game: Game);
    fn next_state(game: Game);
}

impl GalaxyExplorationStateController for GalaxyExploration {
    fn start_state(game:Game) {
        let galaxy_exploration = GalaxyExploration {
            game
        };

        display_galaxy(galaxy_exploration.game.galaxy);

        // Self::next_state(galaxy_exploration.game);
    }

    fn next_state(game: Game) {}
}
