pub struct SystemPower {
    current_weapons_power: u8,
    current_shields_power: u8,
    current_engines_power: u8,
    current_deflector_power: u8,
    current_auxiliary_power: u8,
    max_system_power: u8,
    power_regeneration_rate: u8,
}

impl SystemPower {
    pub(crate) fn create(current_weapons_power: u8,
                         current_shields_power: u8,
                         current_engines_power: u8,
                         current_deflector_power: u8,
                         current_auxiliary_power: u8,
                         max_system_power: u8,
                         power_regeneration_rate: u8) -> SystemPower {
        return SystemPower {
            current_weapons_power,
            current_shields_power,
            current_engines_power,
            current_deflector_power,
            current_auxiliary_power,
            max_system_power,
            power_regeneration_rate,
        };
    }
}

#[cfg(test)]
mod model_meta_data_should {
    use super::*;

    #[test]
    fn create_starship_system_power() {
        let system_power = given_system_power();

        assert_eq!(system_power.max_system_power, 100);
        assert_eq!(system_power.current_shields_power, 100);
        assert_eq!(system_power.current_auxiliary_power, 100);
        assert_eq!(system_power.current_engines_power, 100);
        assert_eq!(system_power.current_deflector_power, 100);
        assert_eq!(system_power.current_weapons_power, 100);
        assert_eq!(system_power.power_regeneration_rate, 10);
    }

    fn given_system_power() -> SystemPower {
        SystemPower::create(
            100,
            100,
            100,
            100,
            100,
            100,
            10
        )
    }
}
