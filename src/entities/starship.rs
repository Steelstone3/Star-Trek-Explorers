use crate::components::{hull::Hull, shield::Shield, sprite::Sprite};

pub struct Starship {
    pub sprite: Sprite,
    shield: Shield,
    hull: Hull,
}

impl Starship {
    pub fn new(sprite_name: String) -> Self {
        Self {
            sprite: Sprite {
                path: sprite_name + ".png",
            },
            shield: Shield::default(),
            hull: Hull::default(),
        }
    }
}
