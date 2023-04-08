use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Torpedo {}

impl Default for Torpedo {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod torpedo_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
