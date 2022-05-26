pub struct Systems {
    max_warp_factor: u8,
    max_impulse: u16,
    number_of_phaser: u8,
    number_of_torpedoes: u8,
    max_base_phaser_damage: u8,
    max_base_torpedo_damage: u8,
    shield_strength: u8,
    shield_regeneration_rate: u8,
    hull_integrity_field: u8,
}

impl Systems {
    pub(crate) fn create(max_warp_factor: u8,
                         max_impulse: u16,
                         number_of_phaser: u8,
                         number_of_torpedoes: u8,
                         max_base_phaser_damage: u8,
                         max_base_torpedo_damage: u8,
                         shield_strength: u8,
                         shield_regeneration_rate: u8,
                         hull_integrity_field: u8) -> Systems {
        return Systems {
            max_warp_factor,
            max_impulse,
            number_of_phaser,
            number_of_torpedoes,
            max_base_phaser_damage,
            max_base_torpedo_damage,
            shield_strength,
            shield_regeneration_rate,
            hull_integrity_field,
        };
    }
}


#[cfg(test)]
mod model_meta_data_should {
    use super::*;

    #[test]
    fn create_starship_meta_data_details() {
        let systems = given_systems();

        assert_eq!(systems.max_warp_factor, 9);
        assert_eq!(systems.max_impulse, 500);
        assert_eq!(systems.number_of_phaser, 15);
        assert_eq!(systems.number_of_torpedoes, 40);
        assert_eq!(systems.max_base_phaser_damage, 10);
        assert_eq!(systems.max_base_torpedo_damage, 50);
        assert_eq!(systems.shield_strength, 100);
        assert_eq!(systems.shield_regeneration_rate, 10);
        assert_eq!(systems.hull_integrity_field, 100);
    }

    fn given_systems() -> Systems {
        Systems::create(
            9,
            500,
            15,
            40,
            10,
            50,
            100,
            10,
            100
        )
    }
}
