use crate::models::universe::star_system::StarSystem;

pub struct Galaxy {
    pub star_systems: Vec<StarSystem>,
}

impl Galaxy {
    pub fn create_galaxy(quantity_of_star_systems: usize) -> Galaxy {
        Galaxy {
            star_systems: StarSystem::create_random_star_systems(quantity_of_star_systems)
        }
    }
}

#[cfg(test)]
mod galaxy_should {
    use super::*;

    #[test]
    fn create_galaxy() {
        const QUANTITY: usize = 100;

        let galaxy = Galaxy::create_galaxy(QUANTITY);

        assert_eq!(QUANTITY, galaxy.star_systems.len());
    }
}
