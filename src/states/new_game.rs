use crate::controllers::state_controller::GameStateController;
use crate::controllers::state_controller::NewGameStateController;
use crate::models::universe::galaxy::Galaxy;
use crate::presenters::presenter::read_numeric_u32;
use crate::presenters::ship_presenter::player_create_character;
use crate::presenters::ship_presenter::scan_ship;
use crate::states::galaxy_exploration::GalaxyExploration;
use crate::Game;

impl NewGameStateController for Game {
    fn start_state() {
        let game = Game {
            player_ship: player_create_character(),
            galaxy: Galaxy::create_galaxy(read_numeric_u32("Enter game size", 0, 1000) as usize),
            ally_ships: vec![],
            neutral_ships: vec![],
            hostile_ships: vec![],
            game_progress: 0,
        };

        scan_ship(&game.player_ship);

        Self::next_state(game);
    }

    fn next_state(game: Game) {
        GalaxyExploration::start_state(game);
    }
}
