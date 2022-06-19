// let mut rng: Pcg64 = Seeder::from("stripy zebra").make_rng();
// println!("{}", rng.gen::<char>());
use super::ship_model;

pub fn create(name: &str) -> Ship {
    return Ship {

    }
}

#[cfg(test)]
mod sovereign_class_factory_should {
    use super::*;

    #[test]
    fn create_a_galaxy_class_starship_with_fixed_defaults() {
        let starship = create("Enterprise");

        assert_eq!(starship.display_symbol, 'S');
        assert_eq!(starship.name, "USS Enterprise-E NCC-6743523");
        assert_eq!(starship.faction, "Federation");
        assert_eq!(starship.class, "Sovereign Class");
    }
}