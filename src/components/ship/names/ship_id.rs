use std::fmt::Display;
use rand_derive2::RandGen;
use crate::{
    components::ship::names::faction_name::FactionName,
    systems::{
        random_generation::generate_seed, ship_identifier_generation::generate_random_identifier,
    },
};

#[derive(PartialEq, Debug, Copy, Clone, RandGen)]
pub enum SerialNumber {
    FederationId,
    KlingonId,
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialNumber::FederationId => {
                write!(
                    f,
                    "{}",
                    generate_random_identifier(generate_seed(), &FactionName::Federation)
                )
            }
            SerialNumber::KlingonId => {
                write!(
                    f,
                    "{}",
                    generate_random_identifier(generate_seed(), &FactionName::KlingonEmpire)
                )
            }
        }
    }
}
