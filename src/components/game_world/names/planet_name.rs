use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum PlanetName {
    Earth,
    Mars,
}

impl Display for PlanetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanetName::Earth => {
                write!(f, "Earth")
            }
            PlanetName::Mars => {
                write!(f, "Mars")
            }
        }
    }
}
