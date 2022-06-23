use crate::names::federation_starship_names::FEDERATION_STARSHIP_NAMES;
use crate::names::random::get_seeded_random_number;
use crate::names::random::get_seeded_random_name;
use crate::ship::ship_model::Ship;

const SERIAL_NUMBER_LOWER_BOUND: u64 = 1000;
const SERIAL_NUMBER_UPPER_BOUND: u64 = 999999;

pub fn create_player(name: &str, seed: u64) -> Ship {
    return Ship {
        display_symbol: 'S',
        name: format!(
            "USS {} NCC-{}",
            name,
            get_seeded_random_number(seed, SERIAL_NUMBER_LOWER_BOUND, SERIAL_NUMBER_UPPER_BOUND)
        ),
        faction: "Federation".to_string(),
        class: "Sovereign Class".to_string(),
    };
}

pub fn create_npc(seed: u64) -> Ship {
    return Ship {
        display_symbol: 'F',
        name: format!(
            "USS {} NCC-{}",
            get_seeded_random_name(FEDERATION_STARSHIP_NAMES.to_vec(), seed),
            get_seeded_random_number(seed, SERIAL_NUMBER_LOWER_BOUND, SERIAL_NUMBER_UPPER_BOUND)
        ),
        faction: "Federation".to_string(),
        class: "Sovereign Class".to_string(),
    };
}

#[cfg(test)]
mod sovereign_class_factory_should {
    use super::*;

    #[test]
    fn create_an_npc_galaxy_class_starship() {
        let starship = create_npc(6969);

        assert_eq!('F', starship.display_symbol);
        assert_eq!("USS Excelsior NCC-605599", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }

    #[test]
    fn create_a_player_sovereign_class_starship() {
        let starship = create_player("Enterprise", 45);

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-309758", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }
}
