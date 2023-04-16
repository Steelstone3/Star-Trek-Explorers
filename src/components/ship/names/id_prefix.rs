use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(Debug, PartialEq, Clone, Copy, RandGen)]
pub enum IdPrefix {
    FederationIdPrefix,
    KlingonIdPrefix,
}

impl Display for IdPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdPrefix::FederationIdPrefix => {
                write!(f, "USS-")
            }
            IdPrefix::KlingonIdPrefix => {
                write!(f, "IKS-")
            }
        }
    }
}