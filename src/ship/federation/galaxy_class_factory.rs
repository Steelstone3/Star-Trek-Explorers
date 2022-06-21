use crate::names::random::get_seeded_random_number;
use crate::names::random::get_unique_name;
use crate::names::federation_starship_names::FEDERATION_STARSHIP_NAMES;
use crate::ship::ship_model::Ship;

pub fn create_player(name: &str, seed:u64) -> Ship {
    return Ship {
        display_symbol: 'S',
        name: format!("USS {} NCC-{}", name, get_seeded_random_number(seed)),
        faction: "Federation".to_string(),
        class: "Galaxy Class".to_string(),
    };
}

pub fn create_npc(used_names: &[&str], seed: u64) -> Ship {
    return Ship {
        display_symbol: 'F',
        name: format!("USS {} NCC-{}", get_unique_name(&FEDERATION_STARSHIP_NAMES, &used_names), get_seeded_random_number(seed)),
        faction: "Federation".to_string(),
        class: "Galaxy Class".to_string(),
    };
}

#[cfg(test)]
mod galaxy_class_factory_should {
use super::*;

    #[test]
    fn create_an_npc_galaxy_class_starship() {
        let starship = create_npc(&FEDERATION_STARSHIP_NAMES.to_vec(), 32);

        assert_eq!('F', starship.display_symbol);
        assert_eq!("USS Jerald NCC-279249", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Galaxy Class", starship.class);
    }

    #[test]
    fn create_a_player_galaxy_class_starship() {
        let starship = create_player("Enterprise", 21);

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-75197", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Galaxy Class", starship.class);
    }
}
