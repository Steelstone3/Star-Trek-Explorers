use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, Debug, RandGen)]
pub enum KlingonShipName {
    Amar,
    BMoth,
}

impl Display for KlingonShipName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KlingonShipName::Amar => {
                write!(f, "Amar")
            }
            KlingonShipName::BMoth => {
                write!(f, "B'Moth")
            }
        }
    }
}
