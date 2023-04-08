use rand_derive2::RandGen;

use super::damage::DamageTaker;

#[derive(PartialEq, Debug, RandGen, Default)]
pub struct Hull {}



impl DamageTaker for Hull {
    fn take_damage(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod hull_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
