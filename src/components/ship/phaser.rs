use rand_derive2::RandGen;

use super::damage::DamageDealer;

#[derive(PartialEq, Debug, RandGen, Default)]
pub struct Phaser {}



impl DamageDealer for Phaser {
    fn calculate_damage(&self) -> u8 {
        todo!()
    }
}

#[cfg(test)]
mod phaser_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
