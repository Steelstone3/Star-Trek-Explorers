use super::{galaxy_exploration::GalaxyExploration, game::Game};
use crate::controllers::combat::fight;
use crate::states::planet_exploration::PlanetExploration;

pub struct Combat;

impl From<Game<GalaxyExploration>> for Game<Combat> {
    fn from(state: Game<GalaxyExploration>) -> Game<Combat> {
        let mut game = Game {
            state: Combat,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        };

        fight(&game.player_ship, &game.ally_ships, &mut game.hostile_ships);

        game
    }
}

impl From<Game<PlanetExploration>> for Game<Combat> {
    fn from(state: Game<PlanetExploration>) -> Game<Combat> {
        let mut game = Game {
            state: Combat,
            player_ship: state.player_ship,
            galaxy: state.galaxy,
            ally_ships: state.ally_ships,
            neutral_ships: state.neutral_ships,
            hostile_ships: state.hostile_ships,
            game_progress: state.game_progress,
        };

        fight(&game.player_ship, &game.ally_ships, &mut game.hostile_ships);

        game
    }
}