pub struct Moon {
    name: String,
    description: String,
}

impl Moon {
    pub(crate) fn generate() -> Moon {
        return Moon {
            name: String::from("Luna"),
            description: String::from("A small rocky mass.")
        };
    }
}

#[cfg(test)]
mod moon_should {
    use crate::world::moon::Moon;

    #[test]
    fn create_a_moon() {
        let moon = Moon::generate();

        assert_eq!("Luna", moon.name);
        assert_eq!("A small rocky mass.", moon.description);
    }
}