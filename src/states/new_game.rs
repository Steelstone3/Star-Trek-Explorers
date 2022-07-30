use crate::presenters::ship_presenter::scan_ship;
use crate::models::ships::ship::Ship;
use crate::models::universe::galaxy::Galaxy;
use crate::presenters::ship_presenter::player_create_character;
use crate::states::galaxy_exploration::GalaxyExploration;
use crate::states::galaxy_exploration::GalaxyExplorationStateController;

pub struct Game {
    player_ship: Ship,
    pub galaxy: Galaxy,
    ally_ships: Vec<Ship>,
    neutral_ships: Vec<Ship>,
    hostile_ships: Vec<Ship>,
}

pub trait GameStateController {
    fn start_state();
    fn next_state(game: Game);
}

impl GameStateController for Game {
    fn start_state() {
        let game = Game {
            player_ship: player_create_character(),
            galaxy: Galaxy::create_galaxy(1),
            ally_ships: vec![],
            neutral_ships: vec![],
            hostile_ships: vec![],
        };

        // scan_ship(game.player_ship);

        Self::next_state(game);
    }

    fn next_state(game: Game) {
        GalaxyExploration::start_state(game);
    }
}
