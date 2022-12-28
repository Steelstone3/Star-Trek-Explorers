use bevy::prelude::{Input, KeyCode, Res};

pub fn movement(keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::W) {
        // W is being held down
    }
    else if keys.pressed(KeyCode::S) {
        
    }
    else if keys.pressed(KeyCode::A) {

    }
    else if keys.pressed(KeyCode::D) {
        
    }
}