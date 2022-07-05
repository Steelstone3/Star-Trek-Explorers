use crate::names::factions::klingon_starship_classification::KLINGON_STARSHIP_CLASS;
use crate::names::factions::klingon_starship_classification::FACTION_KLINGON_EMPIRE;
use crate::names::factions::klingon_starship_classification::KLINGON_STARSHIP_NAMES;
use crate::game_randomiser::random_controller::get_seeded_random_name;
use crate::game_randomiser::random_controller::get_seeded_random_number;
use crate::Ship;

const KLINGON_EMPIRE_SERIAL_NUMBER_LOWER_BOUND: u64 = 100;
const KLINGON_EMPIRE_SERIAL_NUMBER_UPPER_BOUND: u64 = 9999;

pub fn create_klingon_ship(seed: u64) -> Ship {
    return Ship::create_ship(
        'K',
        format!(
            "IKS-{} {}",
            get_seeded_random_number(
                seed,
                KLINGON_EMPIRE_SERIAL_NUMBER_LOWER_BOUND,
                KLINGON_EMPIRE_SERIAL_NUMBER_UPPER_BOUND
            ),
            get_seeded_random_name(&KLINGON_STARSHIP_NAMES, seed)
        ),
        String::from(FACTION_KLINGON_EMPIRE),
        get_seeded_random_name(&KLINGON_STARSHIP_CLASS, seed),
    );
}

#[cfg(test)]
mod klingon_ship_factory_should {
    use super::*;

    #[test]
    fn create_a_klingon_ship() {
        let starship = create_klingon_ship(6969);

        assert_eq!('K', starship.display_symbol);
        assert_eq!("IKS-5176 Amak", starship.name);
        assert_eq!("Klingon Empire", starship.faction);
        assert_eq!("D7 Battle Cruiser", starship.class);
    }
}
