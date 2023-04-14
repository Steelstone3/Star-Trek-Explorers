use crate::systems::random_generation::{generate_random_value_from_range_u8, generate_seed};
use rand_derive2::RandGen;
use std::fmt::Display;

const FEDERATION_SHIP_NAMES: [ShipName; 50] = [
    ShipName::Akira,
    ShipName::Archon,
    ShipName::Atlantis,
    ShipName::Avenger,
    ShipName::Aventine,
    ShipName::Bozeman,
    ShipName::Centaur,
    ShipName::Challenger,
    ShipName::Chekov,
    ShipName::Chimera,
    ShipName::Columbia,
    ShipName::Cortez,
    ShipName::Curry,
    ShipName::Dauntless,
    ShipName::Defiant,
    ShipName::Discovery,
    ShipName::Endeavour,
    ShipName::Enterprise,
    ShipName::Excalibur,
    ShipName::Excelsio,
    ShipName::Excelsior,
    ShipName::Farragut,
    ShipName::Galaxy,
    ShipName::Grissom,
    ShipName::Hood,
    ShipName::Horizon,
    ShipName::Intrepid,
    ShipName::Lakota,
    ShipName::Lexington,
    ShipName::Majestic,
    ShipName::Melbourne,
    ShipName::Merrimack,
    ShipName::Monitor,
    ShipName::Nebula,
    ShipName::Newton,
    ShipName::Nova,
    ShipName::Odyssey,
    ShipName::Pasteur,
    ShipName::Potemkin,
    ShipName::Prometheus,
    ShipName::Proxima,
    ShipName::Relativity,
    ShipName::Saratoga,
    ShipName::Stargazer,
    ShipName::Sutherland,
    ShipName::Thunderchild,
    ShipName::Titan,
    ShipName::Valiant,
    ShipName::Voyager,
    ShipName::Yorktown,
];

const KLINGON_SHIP_NAMES: [ShipName; 26] = [
    ShipName::Amar,
    ShipName::BMoth,
    ShipName::Brel,
    ShipName::Buruk,
    ShipName::ChTang,
    ShipName::Groth,
    ShipName::Heghta,
    ShipName::KVada,
    ShipName::KVort,
    ShipName::Ktinga,
    ShipName::Kitang,
    ShipName::Koraga,
    ShipName::MChar,
    ShipName::NeghVar,
    ShipName::Ningtao,
    ShipName::Orantho,
    ShipName::Pagh,
    ShipName::Rotarran,
    ShipName::Slivin,
    ShipName::Somraw,
    ShipName::TAcog,
    ShipName::Tagana,
    ShipName::TohKaht,
    ShipName::Voodieh,
    ShipName::Vorcha,
    ShipName::Vorn,
];

#[derive(PartialEq, Debug, Copy, Clone, RandGen)]
pub enum ShipName {
    // Federation Ship Names
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
    // Klingon Ship Names
    Amar,
    BMoth,
    Brel,
    Buruk,
    ChTang,
    Groth,
    Heghta,
    KVada,
    KVort,
    Ktinga,
    Kitang,
    Koraga,
    MChar,
    NeghVar,
    Ningtao,
    Orantho,
    Pagh,
    Rotarran,
    Slivin,
    Somraw,
    TAcog,
    Tagana,
    TohKaht,
    Voodieh,
    Vorcha,
    Vorn,
}

impl Display for ShipName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Federation Ship Names
            ShipName::Enterprise => {
                write!(f, "Enterprise")
            }
            ShipName::Defiant => {
                write!(f, "Defiant")
            }
            ShipName::Challenger => {
                write!(f, "Challenger")
            }
            ShipName::Voyager => {
                write!(f, "Voyager")
            }
            ShipName::Discovery => {
                write!(f, "Discovery")
            }
            ShipName::Excelsio => {
                write!(f, "Excelsio")
            }
            ShipName::Prometheus => {
                write!(f, "Prometheus")
            }
            ShipName::Titan => {
                write!(f, "Titan")
            }
            ShipName::Stargazer => {
                write!(f, "Stargazer")
            }
            ShipName::Hood => {
                write!(f, "Hood")
            }
            ShipName::Atlantis => {
                write!(f, "Atlantis")
            }
            ShipName::Aventine => {
                write!(f, "Aventine")
            }
            ShipName::Chimera => {
                write!(f, "Chimera")
            }
            ShipName::Columbia => {
                write!(f, "Columbia")
            }
            ShipName::Endeavour => {
                write!(f, "Endeavour")
            }
            ShipName::Excalibur => {
                write!(f, "Excalibur")
            }
            ShipName::Excelsior => {
                write!(f, "Excelsior")
            }
            ShipName::Galaxy => {
                write!(f, "Galaxy")
            }
            ShipName::Intrepid => {
                write!(f, "Intrepid")
            }
            ShipName::Lexington => {
                write!(f, "Lexington")
            }
            ShipName::Melbourne => {
                write!(f, "Melbourne")
            }
            ShipName::Nova => {
                write!(f, "Nova")
            }
            ShipName::Odyssey => {
                write!(f, "Odyssey")
            }
            ShipName::Sutherland => {
                write!(f, "Sutherland")
            }
            ShipName::Thunderchild => {
                write!(f, "Thunderchild")
            }
            ShipName::Valiant => {
                write!(f, "Valiant")
            }
            ShipName::Yorktown => {
                write!(f, "Yorktown")
            }
            ShipName::Akira => {
                write!(f, "Akira")
            }
            ShipName::Archon => {
                write!(f, "Archon")
            }
            ShipName::Avenger => {
                write!(f, "Avenger")
            }
            ShipName::Bozeman => {
                write!(f, "Bozeman")
            }
            ShipName::Centaur => {
                write!(f, "Centaur")
            }
            ShipName::Chekov => {
                write!(f, "Chekov")
            }
            ShipName::Cortez => {
                write!(f, "Cortez")
            }
            ShipName::Curry => {
                write!(f, "Curry")
            }
            ShipName::Dauntless => {
                write!(f, "Dauntless")
            }
            ShipName::Farragut => {
                write!(f, "Farragut")
            }
            ShipName::Grissom => {
                write!(f, "Grissom")
            }
            ShipName::Horizon => {
                write!(f, "Horizon")
            }
            ShipName::Lakota => {
                write!(f, "Lakota")
            }
            ShipName::Majestic => {
                write!(f, "Majestic")
            }
            ShipName::Merrimack => {
                write!(f, "Merrimack")
            }
            ShipName::Monitor => {
                write!(f, "Monitor")
            }
            ShipName::Nebula => {
                write!(f, "Nebula")
            }
            ShipName::Newton => {
                write!(f, "Newton")
            }
            ShipName::Pasteur => {
                write!(f, "Pasteur")
            }
            ShipName::Potemkin => {
                write!(f, "Potemkin")
            }
            ShipName::Proxima => {
                write!(f, "Proxima")
            }
            ShipName::Relativity => {
                write!(f, "Relativity")
            }
            ShipName::Saratoga => {
                write!(f, "Saratoga")
            }
            // Klingon Ship Names
            ShipName::Amar => {
                write!(f, "Amar")
            }
            ShipName::BMoth => {
                write!(f, "B'Moth")
            }
            ShipName::Brel => {
                write!(f, "B'rel")
            }
            ShipName::Buruk => {
                write!(f, "B'rel")
            }
            ShipName::ChTang => {
                write!(f, "Ch'Tang")
            }
            ShipName::Groth => {
                write!(f, "Groth")
            }
            ShipName::Heghta => {
                write!(f, "Heghta")
            }
            ShipName::KVada => {
                write!(f, "K'Vada")
            }
            ShipName::KVort => {
                write!(f, "K'Vort")
            }
            ShipName::Ktinga => {
                write!(f, "K't'inga")
            }
            ShipName::Kitang => {
                write!(f, "Ki'tang")
            }
            ShipName::Koraga => {
                write!(f, "Koraga")
            }
            ShipName::MChar => {
                write!(f, "M'Char")
            }
            ShipName::NeghVar => {
                write!(f, "NeghVar")
            }
            ShipName::Ningtao => {
                write!(f, "Ning'tao")
            }
            ShipName::Orantho => {
                write!(f, "Orantho")
            }
            ShipName::Pagh => {
                write!(f, "Pagh")
            }
            ShipName::Rotarran => {
                write!(f, "Rotarran")
            }
            ShipName::Slivin => {
                write!(f, "Slivin")
            }
            ShipName::Somraw => {
                write!(f, "Somraw")
            }
            ShipName::TAcog => {
                write!(f, "T'Acog")
            }
            ShipName::Tagana => {
                write!(f, "Tagana")
            }
            ShipName::TohKaht => {
                write!(f, "Toh'Kaht")
            }
            ShipName::Voodieh => {
                write!(f, "Voodieh")
            }
            ShipName::Vorcha => {
                write!(f, "Vor'cha")
            }
            ShipName::Vorn => {
                write!(f, "Vorn")
            }
        }
    }
}

pub fn get_random_federation_name() -> ShipName {
    FEDERATION_SHIP_NAMES[get_index((FEDERATION_SHIP_NAMES.len() - 1) as u8)]
}

pub fn get_random_klingon_name() -> ShipName {
    KLINGON_SHIP_NAMES[get_index((KLINGON_SHIP_NAMES.len() - 1) as u8)]
}

fn get_index(name_length: u8) -> usize {
    generate_random_value_from_range_u8(generate_seed(), 0, name_length) as usize
}

#[cfg(test)]
mod ship_name_should {
    use super::*;

    #[test]
    fn be_able_to_get_random_federation_name() {
        // When
        let class = get_random_federation_name();

        // Then
        assert!(FEDERATION_SHIP_NAMES.contains(&class));
        assert!(!KLINGON_SHIP_NAMES.contains(&class));
    }

    #[test]
    fn be_able_to_get_random_klingon_name() {
        // When
        let class = get_random_klingon_name();

        // Then
        assert!(KLINGON_SHIP_NAMES.contains(&class));
        assert!(!FEDERATION_SHIP_NAMES.contains(&class));
    }
}
