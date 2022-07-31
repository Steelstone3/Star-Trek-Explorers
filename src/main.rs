use crate::models::game::Game;
use crate::controllers::state_controller::NewGameStateController;

mod assests;
mod controllers;
mod models;
mod presenters;
mod states;

fn main() {
    Game::start_state();
}