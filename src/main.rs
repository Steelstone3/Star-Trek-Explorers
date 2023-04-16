use entities::game::Game;
use systems::random_generation::{generate_random_value_from_range_u8, generate_seed};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.print_player_ship();
    game.print_all_ai_ships();
    game.print_universe();
    game.start_combat_system(generate_random_value_from_range_u8(
        generate_seed(),
        0,
        (game.klingon_ships.len() - 1) as u8,
    ) as usize);
}
