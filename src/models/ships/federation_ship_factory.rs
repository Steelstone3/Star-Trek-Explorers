use crate::models::ships::ship::Ship;
use crate::assests::names::factions::federation_starship_classification::FEDERATION_STARSHIP_NAMES;
use crate::assests::names::factions::federation_starship_classification::FEDERATION_STARSHIP_CLASS;
use crate::controllers::game_randomiser::get_seeded_random_name;
use crate::assests::names::factions::federation_starship_classification::FACTION_FEDERATION;
use crate::controllers::game_randomiser::get_seeded_random_number;

const FEDERATION_SERIAL_NUMBER_LOWER_BOUND: u64 = 1000;
const FEDERATION_SERIAL_NUMBER_UPPER_BOUND: u64 = 999999;

pub fn create_player_federation_ship(name: &str, seed: u64) -> Ship {
    return Ship::create_ship(
        'S',
        format!(
            "USS {} NCC-{}",
            name,
            get_seeded_random_number(
                seed,
                FEDERATION_SERIAL_NUMBER_LOWER_BOUND,
                FEDERATION_SERIAL_NUMBER_UPPER_BOUND
            )
        ),
        String::from(FACTION_FEDERATION),
        get_seeded_random_name(&FEDERATION_STARSHIP_CLASS, seed),
    );
}

#[allow(dead_code)]
pub fn create_federation_ship(seed: u64) -> Ship {
    return Ship::create_ship(
        'F',
        format!(
            "USS {} NCC-{}",
            get_seeded_random_name(&FEDERATION_STARSHIP_NAMES, seed),
            get_seeded_random_number(
                seed,
                FEDERATION_SERIAL_NUMBER_LOWER_BOUND,
                FEDERATION_SERIAL_NUMBER_UPPER_BOUND
            )
        ),
        String::from(FACTION_FEDERATION),
        get_seeded_random_name(&FEDERATION_STARSHIP_CLASS, seed),
    );
}

#[cfg(test)]
mod federation_ship_factory_should {
    use super::*;

    #[test]
    fn create_a_player_federation_ship() {
        let starship = create_player_federation_ship("Enterprise", 1701);

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-474661", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Defiant Class", starship.class);
    }

    #[test]
    fn create_a_federation_ship() {
        let starship = create_federation_ship(6969);

        assert_eq!('F', starship.display_symbol);
        assert_eq!("USS Excelsior NCC-605599", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }
}
