use bevy::{sprite::SpriteBundle, prelude::{Camera2dBundle, Commands, Res, AssetServer, default}};

pub fn setup_game_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("starship_enterprise_e.png"),
        ..default()
    });
}