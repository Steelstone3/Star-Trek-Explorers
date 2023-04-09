use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum PlanetClass {
    ClassA,
    ClassM,
}

impl Display for PlanetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetClass::ClassA => {
                write!(f, "Class A")
            }
            PlanetClass::ClassM => {
                write!(f, "Class M")
            }
        }
    }
}
