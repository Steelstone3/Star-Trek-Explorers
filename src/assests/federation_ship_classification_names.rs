use rand_derive2::RandGen;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(RandGen)]
pub enum FederationShipClassification {
    Defiant,
    Intrepid,
    Consitution,
    Ambassador,
    Galaxy,
    Sovereign,
}

impl Display for FederationShipClassification {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationShipClassification::Defiant => {
                write!(formatter, "{}", display_ship_class("Defiant"))
            }

            FederationShipClassification::Intrepid => {
                write!(formatter, "{}", display_ship_class("Intrepid"))
            }

            FederationShipClassification::Consitution => {
                write!(formatter, "{}", display_ship_class("Consitution"))
            }

            FederationShipClassification::Ambassador => {
                write!(formatter, "{}", display_ship_class("Ambassador"))
            }

            FederationShipClassification::Galaxy => {
                write!(formatter, "{}", display_ship_class("Galaxy"))
            }

            FederationShipClassification::Sovereign => {
                write!(formatter, "{}", display_ship_class("Sovereign"))
            }
        }
    }
}

fn display_ship_class(class: &str) -> String {
    format!("{} Class", class)
}
