use crate::models::ship::meta_data::MetaData;
use crate::models::ship::system_power::SystemPower;
use crate::models::ship::systems::Systems;
use crate::Starship;

pub fn create_klingon_bird_of_prey() -> Starship {
    return Starship {
        meta_data: MetaData::create(
            "IKS".to_string(),
            "Chang".to_string(),
            "of House Targ".to_string(),
            "IKS-66345".to_string(),
            2000,
        ),
        systems: Systems::create(
            7,
            800,
            4,
            10,
            10,
            50,
            100,
            10,
            100,
        ),
        system_power: SystemPower::create(
            100,
            100,
            100,
            100,
            100,
            100,
            10,
        ),
    };
}

#[cfg(test)]
mod factory_bird_of_prey_should {
    use super::*;

    #[test]
    fn create_starship_with_fixed_defaults() {
        let starship = create_klingon_bird_of_prey();

        assert_eq!(starship.meta_data.details(), "IKS Chang of House Targ IKS-66345");
    }
}