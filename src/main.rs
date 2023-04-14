use crate::{entities::ship::Ship, systems::combat::combat_turn};
use entities::game::Game;
use systems::random_generation::generate_seed;

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.generate_games_ships();
    game.print_player_ship();
    game.print_all_ai_ships();
    game.print_universe();

    let mut hostile = Ship::new_klingon_ship();

    combat_turn(
        generate_seed(),
        game.player_ship.ship_systems.select_ship_weapon_type(),
        &game.player_ship,
        &mut hostile,
    );
}
