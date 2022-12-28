use bevy::prelude::{KeyCode, Res, Input};

pub fn fire(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        // Fire weapon
    }
}