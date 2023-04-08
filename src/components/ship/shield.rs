use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Shield {}

impl Default for Shield {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod shield_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
