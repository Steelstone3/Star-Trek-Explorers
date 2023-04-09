use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum StarName {
    Sol,
    AlphaCentauri,
}

impl Display for StarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarName::Sol => {
                write!(f, "Sol")
            }
            StarName::AlphaCentauri => {
                write!(f, "Alpha Centauri")
            }
        }
    }
}
