use crate::names::random::get_random_name;
use crate::names::moon_names::MOON_NAMES;
use crate::names::moon_descriptions::MOON_DESCRIPTIONS;
use crate::names::random::get_unique_name;

pub struct Moon {
    name: String,
    description: String,
}

impl Moon {
    pub(crate) fn generate(used_names: &[&str], seed: u64) -> Moon {
        return Moon {
            name: get_unique_name(&MOON_NAMES, used_names, seed),
            description: get_random_name(&MOON_DESCRIPTIONS, seed)
        };
    }
}

#[cfg(test)]
mod moon_should {
    use crate::world::moon::Moon;

    #[test]
    fn create_a_moon() {
        let moon = Moon::generate(&["Luna"], 45);

        assert_eq!("Titan", moon.name);
        assert_eq!("A small rocky mass.", moon.description);
    }
}