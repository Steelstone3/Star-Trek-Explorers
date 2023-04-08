use rand_derive2::RandGen;

use super::damage::DamageDealer;

#[derive(PartialEq, Debug, RandGen, Default)]
pub struct Torpedo {}



impl DamageDealer for Torpedo {
    fn calculate_damage(&self) -> u8 {
        todo!()
    }
}

#[cfg(test)]
mod torpedo_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
