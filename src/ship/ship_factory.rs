use crate::names::factions::federation_starship_classification::{
    FACTION_FEDERATION, FEDERATION_STARSHIP_CLASS, FEDERATION_STARSHIP_NAMES,
};
use crate::names::factions::klingon_starship_classification::{
    FACTION_KLINGON_EMPIRE, KLINGON_STARSHIP_CLASS, KLINGON_STARSHIP_NAMES,
};
use crate::game_randomiser::random_controller::{get_seeded_random_name, get_seeded_random_number};

const FEDERATION_SERIAL_NUMBER_LOWER_BOUND: u64 = 1000;
const FEDERATION_SERIAL_NUMBER_UPPER_BOUND: u64 = 999999;
const KLINGON_EMPIRE_SERIAL_NUMBER_LOWER_BOUND: u64 = 100;
const KLINGON_EMPIRE_SERIAL_NUMBER_UPPER_BOUND: u64 = 9999;

pub struct Ship {
    pub display_symbol: char,
    pub name: String,
    pub faction: String,
    pub class: String,
}

impl Ship {
    pub fn create_player_federation_ship(name: &str, seed: u64) -> Ship {
        return Ship {
            display_symbol: 'S',
            name: format!(
                "USS {} NCC-{}",
                name,
                get_seeded_random_number(
                    seed,
                    FEDERATION_SERIAL_NUMBER_LOWER_BOUND,
                    FEDERATION_SERIAL_NUMBER_UPPER_BOUND
                )
            ),
            faction: String::from(FACTION_FEDERATION),
            class: get_seeded_random_name(&FEDERATION_STARSHIP_CLASS, seed)
        };
    }

    pub fn create_federation_ship(seed: u64) -> Ship {
        return Ship {
            display_symbol: 'F',
            name: format!(
                "USS {} NCC-{}",
                get_seeded_random_name(&FEDERATION_STARSHIP_NAMES, seed),
                get_seeded_random_number(
                    seed,
                    FEDERATION_SERIAL_NUMBER_LOWER_BOUND,
                    FEDERATION_SERIAL_NUMBER_UPPER_BOUND
                )
            ),
            faction: String::from(FACTION_FEDERATION),
            class: get_seeded_random_name(&FEDERATION_STARSHIP_CLASS, seed),
        };
    }

    pub fn create_klingon_ship(seed: u64) -> Ship {
        return Ship {
            display_symbol: 'K',
            name: format!(
                "IKS-{} {}",
                get_seeded_random_number(
                    seed,
                    KLINGON_EMPIRE_SERIAL_NUMBER_LOWER_BOUND,
                    KLINGON_EMPIRE_SERIAL_NUMBER_UPPER_BOUND
                ),
                get_seeded_random_name(&KLINGON_STARSHIP_NAMES, seed)
            ),
            faction: String::from(FACTION_KLINGON_EMPIRE),
            class: get_seeded_random_name(&KLINGON_STARSHIP_CLASS, seed),
        };
    }

    pub fn scan_ship(ship: Ship) -> String {
        let scanned_ship = format!(
            "Scanning ship...\n Ship ({}):\nName: {}\nFaction: {}\nClass: {}",
            ship.display_symbol, ship.name, ship.faction, ship.class
        );

        return scanned_ship;
    }
}

#[cfg(test)]
mod ship_model_should {
    use super::*;

    #[test]
    fn create_player_federation_ship() {
        let starship = Ship::create_player_federation_ship("Enterprise", 1701);

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-474661", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Defiant Class", starship.class);
    }

    #[test]
    fn create_federation_ship() {
        let starship = Ship::create_federation_ship(6969);

        assert_eq!('F', starship.display_symbol);
        assert_eq!("USS Excelsior NCC-605599", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }

    #[test]
    fn create_klingon_ship() {
        let starship = Ship::create_klingon_ship(6969);

        assert_eq!('K', starship.display_symbol);
        assert_eq!("IKS-5176 Amak", starship.name);
        assert_eq!("Klingon Empire", starship.faction);
        assert_eq!("D7 Battle Cruiser", starship.class);
    }

    #[test]
    fn scan_ship() {
        let starship = Ship {
            display_symbol: 'K',
            name: "IKS-5176 Amak".to_string(),
            faction: "Klingon Empire".to_string(),
            class: "D7 Battle Cruiser".to_string()
        };

        let scanned_ship = Ship::scan_ship(starship);

        assert_eq!("Scanning ship...\n Ship (K):\nName: IKS-5176 Amak\nFaction: Klingon Empire\nClass: D7 Battle Cruiser", scanned_ship);
    }
}
