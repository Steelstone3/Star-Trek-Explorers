use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum FederationShipName {
    Enterprise,
    Defiant,
    Challenger,
}

impl Display for FederationShipName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationShipName::Enterprise => {
                write!(f, "Enterprise")
            }
            FederationShipName::Defiant => {
                write!(f, "Defiant")
            }
            FederationShipName::Challenger => {
                write!(f, "Challenger")
            }
        }
    }
}
