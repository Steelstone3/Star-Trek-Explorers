use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub enum StarName {
    Aldebaran,
    Algol,
    AlphaAquilae,
    AlphaCentauriA,
    AlphaCentauriB,
    Altair,
    Antares,
    Arcturus,
    BarnardsStar,
    Bellatrix,
    BetaPictoris,
    Betelgeuse,
    Canopus,
    Capella,
    Castor,
    DeltaPavonis,
    Deneb,
    EpsilonEridani,
    Fomalhaut,
    GammaCrucis,
    Pollux,
    ProximaCentauri,
    Regulus,
    Rigel,
    Scheat,
    SiriusA,
    Sol,
    Spica,
    Vega,
}

impl Display for StarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarName::Aldebaran => {
                write!(f, "Aldebaran")
            }
            StarName::Algol => {
                write!(f, "Algol")
            }
            StarName::AlphaAquilae => {
                write!(f, "Alpha Aquilae")
            }
            StarName::AlphaCentauriA => {
                write!(f, "Alpha Centauri A")
            }
            StarName::AlphaCentauriB => {
                write!(f, "Alpha Centauri B")
            }
            StarName::Altair => {
                write!(f, "Altair")
            }
            StarName::Antares => {
                write!(f, "Antares")
            }
            StarName::Arcturus => {
                write!(f, "Arcturus")
            }
            StarName::BarnardsStar => {
                write!(f, "Barnard's Star")
            }
            StarName::Bellatrix => {
                write!(f, "Bellatrix")
            }
            StarName::BetaPictoris => {
                write!(f, "Beta Pictoris")
            }
            StarName::Betelgeuse => {
                write!(f, "Betelgeuse")
            }
            StarName::Canopus => {
                write!(f, "Canopus")
            }
            StarName::Capella => {
                write!(f, "Capella")
            }
            StarName::Castor => {
                write!(f, "Castor")
            }
            StarName::DeltaPavonis => {
                write!(f, "Delta Pavonis")
            }
            StarName::Deneb => {
                write!(f, "Deneb")
            }
            StarName::EpsilonEridani => {
                write!(f, "Epsilon Eridani")
            }
            StarName::Fomalhaut => {
                write!(f, "Fomalhaut")
            }
            StarName::GammaCrucis => {
                write!(f, "Gamma Crucis")
            }
            StarName::Pollux => {
                write!(f, "Pollux")
            }
            StarName::ProximaCentauri => {
                write!(f, "Proxima Centauri")
            }
            StarName::Regulus => {
                write!(f, "Regulus")
            }
            StarName::Rigel => {
                write!(f, "Rigel")
            }
            StarName::Scheat => {
                write!(f, "Scheat")
            }
            StarName::SiriusA => {
                write!(f, "Sirius A")
            }
            StarName::Sol => {
                write!(f, "Sol")
            }
            StarName::Spica => {
                write!(f, "Spica")
            }
            StarName::Vega => {
                write!(f, "Vega")
            }
        }
    }
}
