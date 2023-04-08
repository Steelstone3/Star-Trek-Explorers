use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct Hull {}

impl Default for Hull {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod hull_should {
    #[test]
    #[ignore = "not implemented"]
    fn do_something() {}
}
