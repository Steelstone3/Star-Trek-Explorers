pub struct Starship {
    meta_data: MetaData,
    systems: Systems,
    system_power: SystemPower,
}

impl Starship {
    pub fn details(self) -> String {
        return [self.meta_data.prefix.as_str(),
            self.meta_data.name.as_str(),
            self.meta_data.suffix.as_str(),
            self.meta_data.serial_number.as_str()].join(" ");
    }

    pub fn create_default(name: String,
                          suffix: String,
                          serial_number: String) -> Starship {
        return Starship {
            meta_data: MetaData::create_meta_data("USS".to_string(), name, suffix, serial_number, 2000),
            systems: Systems::create_systems(),
            system_power: SystemPower::create_system_power(),
        };
    }

    pub fn create(prefix: String,
                  name: String,
                  suffix: String,
                  serial_number: String,
                  crew_compliment: u16) -> Starship {
        return Starship {
            meta_data: MetaData::create_meta_data(prefix, name, suffix, serial_number, crew_compliment),
            systems: Systems::create_systems(),
            system_power: SystemPower::create_system_power(),
        };
    }
}

struct MetaData {
    prefix: String,
    name: String,
    suffix: String,
    serial_number: String,
    crew_compliment: u16,
}

impl MetaData {
    fn create_meta_data(
        prefix: String, name: String,
        suffix: String,
        serial_number: String,
        crew_compliment: u16) -> MetaData {
        return MetaData {
            prefix,
            name,
            suffix,
            serial_number,
            crew_compliment,
        };
    }
}

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
    fn create_systems() -> Systems {
        return Systems {
            max_warp_factor: 9,
            max_impulse: 500,
            number_of_phaser: 15,
            number_of_torpedoes: 40,
            max_base_phaser_damage: 10,
            max_base_torpedo_damage: 50,
            shield_strength: 100,
            shield_regeneration_rate: 10,
            hull_integrity_field: 100,
        };
    }
}

struct SystemPower {
    current_weapons_power: u8,
    current_shields_power: u8,
    current_engines_power: u8,
    current_deflector_power: u8,
    current_auxiliary_power: u8,
    max_system_power: u8,
    power_regeneration_rate: u8,
}

impl SystemPower {
    fn create_system_power() -> SystemPower {
        return SystemPower {
            current_weapons_power: 100,
            current_shields_power: 100,
            current_engines_power: 100,
            current_deflector_power: 100,
            current_auxiliary_power: 100,
            max_system_power: 100,
            power_regeneration_rate: 10,
        };
    }
}

#[cfg(test)]
mod model_starship_should {
    use super::*;

    #[test]
    fn create_a_player_starship() {
        let starship = Starship::create_default("Enterprise".to_string(), "E".to_string(), "NCC-1701".to_string());

        assert_eq!(starship.meta_data.prefix, "USS");
        assert_eq!(starship.meta_data.name, "Enterprise");
        assert_eq!(starship.meta_data.suffix, "E");
        assert_eq!(starship.meta_data.serial_number, "NCC-1701");
        assert_eq!(starship.meta_data.crew_compliment, 2000);

        assert_eq!(starship.systems.max_warp_factor, 9);
        assert_eq!(starship.systems.max_impulse, 500);
        assert_eq!(starship.systems.number_of_phaser, 15);
        assert_eq!(starship.systems.number_of_torpedoes, 40);
        assert_eq!(starship.systems.max_base_phaser_damage, 10);
        assert_eq!(starship.systems.max_base_torpedo_damage, 50);
        assert_eq!(starship.systems.shield_strength, 100);
        assert_eq!(starship.systems.shield_regeneration_rate, 10);
        assert_eq!(starship.systems.hull_integrity_field, 100);

        assert_eq!(starship.system_power.current_weapons_power, 100);
        assert_eq!(starship.system_power.current_weapons_power, 100);
        assert_eq!(starship.system_power.current_shields_power, 100);
        assert_eq!(starship.system_power.current_engines_power, 100);
        assert_eq!(starship.system_power.current_deflector_power, 100);
        assert_eq!(starship.system_power.current_auxiliary_power, 100);
        assert_eq!(starship.system_power.max_system_power, 100);
        assert_eq!(starship.system_power.power_regeneration_rate, 10);
    }

    //TODO do we want to consider factories for different ships
    #[test]
    fn create_a_npc_starship() {
        let starship = Starship::create("IKS".to_string(),
                                        "Chang".to_string(),
                                        "of House Targ".to_string(),
                                        "IKS-657842".to_string(),
                                        200);

        assert_eq!(starship.meta_data.prefix, "IKS");
        assert_eq!(starship.meta_data.name, "Chang");
        assert_eq!(starship.meta_data.suffix, "of House Targ");
        assert_eq!(starship.meta_data.serial_number, "IKS-657842");
        assert_eq!(starship.meta_data.crew_compliment, 200);

        assert_eq!(starship.systems.max_warp_factor, 9);
        assert_eq!(starship.systems.max_impulse, 500);
        assert_eq!(starship.systems.number_of_phaser, 15);
        assert_eq!(starship.systems.number_of_torpedoes, 40);
        assert_eq!(starship.systems.max_base_phaser_damage, 10);
        assert_eq!(starship.systems.max_base_torpedo_damage, 50);
        assert_eq!(starship.systems.shield_strength, 100);
        assert_eq!(starship.systems.shield_regeneration_rate, 10);
        assert_eq!(starship.systems.hull_integrity_field, 100);

        assert_eq!(starship.system_power.current_weapons_power, 100);
        assert_eq!(starship.system_power.current_weapons_power, 100);
        assert_eq!(starship.system_power.current_shields_power, 100);
        assert_eq!(starship.system_power.current_engines_power, 100);
        assert_eq!(starship.system_power.current_deflector_power, 100);
        assert_eq!(starship.system_power.current_auxiliary_power, 100);
        assert_eq!(starship.system_power.max_system_power, 100);
        assert_eq!(starship.system_power.power_regeneration_rate, 10);
    }

    #[test]
    fn get_starship_details() {
        let starship = Starship::create_default("Enterprise".to_string(),
                                        "E".to_string(),
                                        "NCC-1701".to_string());

        assert_eq!(starship.details(), "USS Enterprise E NCC-1701");
    }
}