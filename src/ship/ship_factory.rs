pub struct Ship {
    pub display_symbol: char,
    pub name: String,
    pub faction: String,
    pub class: String,
}

impl Ship {
    pub fn create_ship(display_symbol: char, name: String, faction: String, class: String) -> Ship {
        return Ship {
            display_symbol,
            name,
            faction,
            class,
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
    }

    #[test]
    fn scan_ship() {
        let starship = Ship {
            display_symbol: 'K',
            name: "IKS-5176 Amak".to_string(),
            faction: "Klingon Empire".to_string(),
            class: "D7 Battle Cruiser".to_string(),
        };

        let scanned_ship = Ship::scan_ship(starship);

        assert_eq!("Scanning ship...\n Ship (K):\nName: IKS-5176 Amak\nFaction: Klingon Empire\nClass: D7 Battle Cruiser", scanned_ship);
    }
}
