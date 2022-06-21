use crate::world::star_system::StarSystem;

pub struct Galaxy {
    star_systems: Vec<StarSystem>,
}

impl Galaxy {
    pub(crate) fn generate(seed: u64) -> Galaxy {
        let mut star_systems = vec![];

        // for i in 0..25 {
            // let mut used_star_names: Vec<&str> = vec![];
            // let mut used_planet_names: Vec<&str> = vec![];
            // let star_system = StarSystem::generate(used_star_names, used_planet_names, seed);
            // star_systems.push(star_system);

            // for star_system in star_systems {
            //     used_star_names.push(star_system.name.as_str());
            // }

            // for p in star_system.planets {
            // used_planet_names.push(p.name.as_str());
            // }
        // }

        return Galaxy { star_systems };
    }
}

#[cfg(test)]
mod galaxy_should {
    use super::*;

    #[test]
    #[ignore]
    fn create_a_galaxy() {
        let galaxy = Galaxy::generate(45);
        assert_eq!("Kronos", galaxy.star_systems[0].name);
        assert_eq!("Mars", galaxy.star_systems[1].name);
    }
}
