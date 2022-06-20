use crate::ship::ship_model::Ship;

pub fn create_player(name: &str) -> Ship {
    return Ship {
        display_symbol: 'S',
        name: format!("USS {} NCC-{}", name, 62711),
        faction: "Federation".to_string(),
        class: "Galaxy Class".to_string(),
    };
}

pub fn create_npc() -> Ship {
    return Ship {
        display_symbol: 'F',
        name: format!("USS {} NCC-{}", "Enterprise", 62711),
        faction: "Federation".to_string(),
        class: "Galaxy Class".to_string(),
    };
}

#[cfg(test)]
mod galaxy_class_factory_should {
    use super::*;

    #[test]
    fn create_an_npc_galaxy_class_starship() {
        let starship = create_npc();

        assert_eq!('F', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-62711", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Galaxy Class", starship.class);
    }

    #[test]
    fn create_a_player_galaxy_class_starship() {
        let starship = create_player("Enterprise");

        assert_eq!('S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-62711", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Galaxy Class", starship.class);
    }
}
