use rand_derive2::RandGen;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(RandGen, Clone, Copy)]
#[derive(PartialEq, Debug)]
pub enum Faction {
    FederationOfPlanets,
    FerengiAlliance,
    KlingonEmpire,
    RomulanEmpire,
    CardassianEmpire,
    BorgCollective,
}

impl Display for Faction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Faction::FederationOfPlanets => {
                write!(formatter, "United Federation Of Planets",)
            }

            Faction::FerengiAlliance => {
                write!(formatter, "Ferengi Alliance",)
            }

            Faction::KlingonEmpire => {
                write!(formatter, "Klingon Empire",)
            }

            Faction::RomulanEmpire => {
                write!(formatter, "Romulan Empire",)
            }

            Faction::CardassianEmpire => {
                write!(formatter, "Cardassian Empire",)
            }

            Faction::BorgCollective => {
                write!(formatter, "Borg Collective",)
            }
        }
    }
}
