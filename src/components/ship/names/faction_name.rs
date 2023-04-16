use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, Clone, Copy, RandGen)]
pub enum FactionName {
    Federation,
    KlingonEmpire,
}

impl Display for FactionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactionName::Federation => {
                write!(f, "Federation Of Planets")
            }
            FactionName::KlingonEmpire => {
                write!(f, "Klingon Empire")
            }
        }
    }
}
