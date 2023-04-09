use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, Debug, RandGen)]
pub enum FederationShipName {
    Akira,
    Archon,
    Atlantis,
    Avenger,
    Aventine,
    Bozeman,
    Centaur,
    Challenger,
    Chekov,
    Chimera,
    Columbia,
    Cortez,
    Curry,
    Dauntless,
    Defiant,
    Discovery,
    Endeavour,
    Enterprise,
    Excalibur,
    Excelsio,
    Excelsior,
    Farragut,
    Galaxy,
    Grissom,
    Hood,
    Horizon,
    Intrepid,
    Lakota,
    Lexington,
    Majestic,
    Melbourne,
    Merrimack,
    Monitor,
    Nebula,
    Newton,
    Nova,
    Odyssey,
    Pasteur,
    Potemkin,
    Prometheus,
    Proxima,
    Relativity,
    Saratoga,
    Stargazer,
    Sutherland,
    Thunderchild,
    Titan,
    Valiant,
    Voyager,
    Yorktown,
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
            FederationShipName::Voyager => {
                write!(f, "Voyager")
            }
            FederationShipName::Discovery => {
                write!(f, "Discovery")
            }
            FederationShipName::Excelsio => {
                write!(f, "Excelsio")
            }
            FederationShipName::Prometheus => {
                write!(f, "Prometheus")
            }
            FederationShipName::Titan => {
                write!(f, "Titan")
            }
            FederationShipName::Stargazer => {
                write!(f, "Stargazer")
            }
            FederationShipName::Hood => {
                write!(f, "Hood")
            }
            FederationShipName::Atlantis => {
                write!(f, "Atlantis")
            }
            FederationShipName::Aventine => {
                write!(f, "Aventine")
            }
            FederationShipName::Chimera => {
                write!(f, "Chimera")
            }
            FederationShipName::Columbia => {
                write!(f, "Columbia")
            }
            FederationShipName::Endeavour => {
                write!(f, "Endeavour")
            }
            FederationShipName::Excalibur => {
                write!(f, "Excalibur")
            }
            FederationShipName::Excelsior => {
                write!(f, "Excelsior")
            }
            FederationShipName::Galaxy => {
                write!(f, "Galaxy")
            }
            FederationShipName::Intrepid => {
                write!(f, "Intrepid")
            }
            FederationShipName::Lexington => {
                write!(f, "Lexington")
            }
            FederationShipName::Melbourne => {
                write!(f, "Melbourne")
            }
            FederationShipName::Nova => {
                write!(f, "Nova")
            }
            FederationShipName::Odyssey => {
                write!(f, "Odyssey")
            }
            FederationShipName::Sutherland => {
                write!(f, "Sutherland")
            }
            FederationShipName::Thunderchild => {
                write!(f, "Thunderchild")
            }
            FederationShipName::Valiant => {
                write!(f, "Valiant")
            }
            FederationShipName::Yorktown => {
                write!(f, "Yorktown")
            }
            FederationShipName::Akira => {
                write!(f, "Akira")
            }
            FederationShipName::Archon => {
                write!(f, "Archon")
            }
            FederationShipName::Avenger => {
                write!(f, "Avenger")
            }
            FederationShipName::Bozeman => {
                write!(f, "Bozeman")
            }
            FederationShipName::Centaur => {
                write!(f, "Centaur")
            }
            FederationShipName::Chekov => {
                write!(f, "Chekov")
            }
            FederationShipName::Cortez => {
                write!(f, "Cortez")
            }
            FederationShipName::Curry => {
                write!(f, "Curry")
            }
            FederationShipName::Dauntless => {
                write!(f, "Dauntless")
            }
            FederationShipName::Farragut => {
                write!(f, "Farragut")
            }
            FederationShipName::Grissom => {
                write!(f, "Grissom")
            }
            FederationShipName::Horizon => {
                write!(f, "Horizon")
            }
            FederationShipName::Lakota => {
                write!(f, "Lakota")
            }
            FederationShipName::Majestic => {
                write!(f, "Majestic")
            }
            FederationShipName::Merrimack => {
                write!(f, "Merrimack")
            }
            FederationShipName::Monitor => {
                write!(f, "Monitor")
            }
            FederationShipName::Nebula => {
                write!(f, "Nebula")
            }
            FederationShipName::Newton => {
                write!(f, "Newton")
            }
            FederationShipName::Pasteur => {
                write!(f, "Pasteur")
            }
            FederationShipName::Potemkin => {
                write!(f, "Potemkin")
            }
            FederationShipName::Proxima => {
                write!(f, "Proxima")
            }
            FederationShipName::Relativity => {
                write!(f, "Relativity")
            }
            FederationShipName::Saratoga => {
                write!(f, "Saratoga")
            }
        }
    }
}
