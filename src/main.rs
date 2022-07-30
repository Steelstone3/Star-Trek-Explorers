use crate::states::new_game::GameStateController;
use crate::states::new_game::Game;

mod assests;
mod controllers;
mod models;
mod presenters;
mod states;

fn main() {
    Game::start_state();
}