use entities::game::Game;

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let mut game = Game::default();

    game.print_player_ship();
    game.print_all_ai_ships();
    game.print_universe();
    game.start_combat();
}
