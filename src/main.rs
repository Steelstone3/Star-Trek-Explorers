mod components;
mod entities;
mod systems;

use bevy::prelude::*;
use systems::game_setup::setup_game_sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_game_sprites)
        .run();
}