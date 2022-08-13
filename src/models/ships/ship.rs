const SHIELD_STRENGTH: u32 = 100;
const HULL_INTEGRITY: u32 = 100;
const PHASER_MAX_DAMAGE: u32 = 100;
const PHASER_MIN_DAMAGE: u32 = 100;
const TORPEDO_MAX_DAMAGE: u32 = 100;
const TORPEDO_MIN_DAMAGE: u32 = 100;

pub struct Ship {
    pub display_symbol: char,
    pub name: String,
    pub faction: String,
    pub class: String,
    pub shield_strength: u32,
    pub hull_integrity: u32,
    pub phaser_max_damage: u32,
    pub phaser_min_damage: u32,
    pub torpedo_max_damage: u32,
    pub torpedo_min_damage: u32,
}

impl Ship {
    pub fn create_ship(display_symbol: char, name: String, faction: String, class: String) -> Ship {
        Ship {
            display_symbol,
            name,
            faction,
            class,
            shield_strength: SHIELD_STRENGTH,
            hull_integrity: HULL_INTEGRITY,
            phaser_max_damage: PHASER_MAX_DAMAGE,
            phaser_min_damage: PHASER_MIN_DAMAGE,
            torpedo_max_damage: TORPEDO_MAX_DAMAGE,
            torpedo_min_damage: TORPEDO_MIN_DAMAGE,
        }
    }

    pub fn scan_ship(ship: &Ship) -> String {
        let scanned_ship = format!(
            "Scanning ship...\n Ship ({}):\nName: {}\nFaction: {}\nClass: {}",
            ship.display_symbol, ship.name, ship.faction, ship.class
        );

        scanned_ship
    }
}

#[cfg(test)]
mod ship_model_should {
    use super::*;

    #[test]
    fn create_ship() {
        let starship = Ship::create_ship(
            'S',
            "USS Enterprise NCC-474661".to_string(),
            "Federation".to_string(),
            "Defiant Class".to_string(),
        );

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-474661", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Defiant Class", starship.class);
        assert_eq!(SHIELD_STRENGTH, starship.shield_strength);
        assert_eq!(HULL_INTEGRITY, starship.hull_integrity);
    }

    #[test]
    fn scan_ship() {
        let starship = Ship {
            display_symbol: 'K',
            name: "IKS-5176 Amak".to_string(),
            faction: "Klingon Empire".to_string(),
            class: "D7 Battle Cruiser".to_string(),
            shield_strength: SHIELD_STRENGTH,
            hull_integrity: HULL_INTEGRITY,
            phaser_max_damage: PHASER_MAX_DAMAGE,
            phaser_min_damage: PHASER_MIN_DAMAGE,
            torpedo_max_damage: TORPEDO_MAX_DAMAGE,
            torpedo_min_damage: TORPEDO_MIN_DAMAGE,
        };

        let scanned_ship = Ship::scan_ship(&starship);

        assert_eq!("Scanning ship...\n Ship (K):\nName: IKS-5176 Amak\nFaction: Klingon Empire\nClass: D7 Battle Cruiser", scanned_ship);
    }
}
