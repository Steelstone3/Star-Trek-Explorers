pub struct Ship {
    pub display_symbol: char,
    pub name: String,
    pub faction: String,
    pub class: String,
}

impl Ship {
    pub fn create(display_symbol: char, name: &str, faction: &str, class: &str) -> Ship {
        return Ship {
            display_symbol,
            name: String::from(name),
            faction: String::from(faction),
            class: String::from(class),
        };
    }

    pub fn scan_ship(ship: Ship){
        println!("Scanning ship...");
        println!("Ship ({}):\nName: {}\nFaction: {}\nClass: {}",ship.display_symbol, ship.name, ship.faction, ship.class)
    }
}

#[cfg(test)]
mod ship_model_should {
    use super::*;

    #[test]
    fn create_a_starship() {
        let starship = Ship::create(
            'S',
            "USS Enterprise-E NCC-1701",
            "Federation",
            "Sovereign Class",
        );

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise-E NCC-1701", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }
}