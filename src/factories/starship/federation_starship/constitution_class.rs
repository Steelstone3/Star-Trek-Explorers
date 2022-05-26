use crate::models::ship::meta_data::MetaData;
use crate::models::ship::system_power::SystemPower;
use crate::models::ship::systems::Systems;
use crate::Starship;

pub fn create_federation_constitution_class(name: String, suffix: String, serial_number:String) -> Starship {
    return Starship { meta_data: MetaData::create(
            "USS".to_string(),
            name,
            suffix,
            serial_number,
            2000
        ), systems: Systems::create(
            9,
            500,
            15,
            40,
            10,
            50,
            100,
            10,
            100
        ), system_power: SystemPower::create(
        100,
        100,
        100,
        100,
        100,
        100,
        10
    ) }
}

#[cfg(test)]
mod factory_constitution_class_should {
    use super::*;

    #[test]
    fn create_starship_with_fixed_defaults() {
        let starship = create_federation_constitution_class(
            "Enterprise".to_string(),
            "E".to_string(),
            "NCC-1701".to_string()
        );

        assert_eq!(starship.meta_data.details(), "USS Enterprise E NCC-1701");
    }
}