use crate::ship::ship_model::Ship;

pub fn create(name: &str) -> Ship {
    return Ship {
        display_symbol: 'S',
        name: format!("USS {} NCC-{}", name, 62711),
        faction: "Federation".to_string(),
        class: "Sovereign Class".to_string(),
    };
}

#[cfg(test)]
mod sovereign_class_factory_should {
    use super::*;

    #[test]
    fn create_a_galaxy_class_starship_with_fixed_defaults() {
         let starship = create("Enterprise");

        assert_eq!( 'S', starship.display_symbol);
        assert_eq!("USS Enterprise NCC-62711", starship.name);
        assert_eq!("Federation", starship.faction);
        assert_eq!("Sovereign Class", starship.class);
    }
}