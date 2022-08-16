use super::{galaxy_exploration::GalaxyExploration, game::Game};
use crate::{
    controllers::{combat::combat_controller::enter_combat, encounters::{klingon_encounter::generate_klingon_ships, federation_encounter::generate_federation_ships}, game::game_randomiser::get_random_number},
    states::planet_exploration::PlanetExploration,
};

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

        game.ally_ships = generate_federation_ships(get_random_number(0, 5) as u32);
        game.hostile_ships = generate_klingon_ships(get_random_number(1, 10) as u32);
        start_combat(&mut game);

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

        game.ally_ships = generate_federation_ships(get_random_number(0, 5) as u32);
        game.hostile_ships = generate_klingon_ships(get_random_number(1, 10) as u32);
        start_combat(&mut game);

        game
    }
}

fn start_combat(game: &mut Game<Combat>) {
    if !game.hostile_ships.is_empty() {
        enter_combat(
            &mut game.player_ship,
            &mut game.ally_ships,
            &mut game.hostile_ships,
        );
    }
}
