use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum FederationShipClass {
    Galaxy,
    Intrepid,
    Defiant,
    Sovereign,
    Oberth,
    Nova,
    Saber,
    Miranda,
    Constellation,
    Cheyenne,
    Dakota,
    Prometheus,
    Nebula,
    Luna,
    Akira,
    Excelsior,
    Ambassador,
    Odyssey,
}

impl Display for FederationShipClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationShipClass::Galaxy => {
                write!(f, "Galaxy")
            }
            FederationShipClass::Intrepid => {
                write!(f, "Intrepid")
            }
            FederationShipClass::Defiant => {
                write!(f, "Defiant")
            }
            FederationShipClass::Sovereign => {
                write!(f, "Sovereign")
            }
            FederationShipClass::Oberth => {
                write!(f, "Oberth")
            }
            FederationShipClass::Nova => {
                write!(f, "Nova")
            }
            FederationShipClass::Saber => {
                write!(f, "Saber")
            }
            FederationShipClass::Miranda => {
                write!(f, "Miranda")
            }
            FederationShipClass::Constellation => {
                write!(f, "Constellation")
            }
            FederationShipClass::Cheyenne => {
                write!(f, "Constellation")
            }
            FederationShipClass::Dakota => {
                write!(f, "Dakota")
            }
            FederationShipClass::Prometheus => {
                write!(f, "Prometheus")
            }
            FederationShipClass::Nebula => {
                write!(f, "Nebula")
            }
            FederationShipClass::Luna => {
                write!(f, "Luna")
            }
            FederationShipClass::Akira => {
                write!(f, "Akira")
            }
            FederationShipClass::Excelsior => {
                write!(f, "Excelsior")
            }
            FederationShipClass::Ambassador => {
                write!(f, "Ambassador")
            }
            FederationShipClass::Odyssey => {
                write!(f, "Odyssey")
            }
        }
    }
}
