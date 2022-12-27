use crate::components::{hull::Hull, shield::Shield, sprite::Sprite};

pub struct Starship {
    pub sprite: Sprite,
    shield: Shield,
    hull: Hull,
}

impl Default for Starship {
    fn default() -> Self {
        Self {
            sprite: Sprite {
                path: "starship_enterprise_e.png".to_owned(),
            },
            shield: Shield {},
            hull: Hull {},
        }
    }
}
