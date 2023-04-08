use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Phaser {}

impl Default for Phaser {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod phaser_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
