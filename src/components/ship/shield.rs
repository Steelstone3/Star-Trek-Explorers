use rand_derive2::RandGen;

use super::damage::DamageTaker;

#[derive(PartialEq, Debug, RandGen, Default)]
pub struct Shield {}



impl DamageTaker for Shield {
    fn take_damage(&mut self) {}
}

#[cfg(test)]
mod shield_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
