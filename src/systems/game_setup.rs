use bevy::{
    prelude::{default, AssetServer, Camera2dBundle, Commands, Res},
    sprite::SpriteBundle,
};

use crate::entities::starship::Starship;

pub fn setup_game_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load(spawn_starship()),
        ..default()
    });
}

fn spawn_starship() -> String {
    Starship::default().sprite.path
}
