use rand_derive2::RandGen;
use std::fmt::Display;

use crate::systems::random_generation::{generate_random_value_from_range_u8, generate_seed};

const FEDERATION_SHIP_CLASSES: [ShipClass; 18] = [
    ShipClass::Galaxy,
    ShipClass::Intrepid,
    ShipClass::Defiant,
    ShipClass::Sovereign,
    ShipClass::Oberth,
    ShipClass::Nova,
    ShipClass::Saber,
    ShipClass::Miranda,
    ShipClass::Constellation,
    ShipClass::Cheyenne,
    ShipClass::Dakota,
    ShipClass::Prometheus,
    ShipClass::Nebula,
    ShipClass::Luna,
    ShipClass::Akira,
    ShipClass::Excelsior,
    ShipClass::Ambassador,
    ShipClass::Odyssey,
];

const KLINGON_SHIP_CLASSES: [ShipClass; 40] = [
    ShipClass::NeghVar,
    ShipClass::Geljoq,
    ShipClass::Brel,
    ShipClass::Felgra,
    ShipClass::Kmpec,
    ShipClass::KVort,
    ShipClass::Qethla,
    ShipClass::Torath,
    ShipClass::Vorcha,
    ShipClass::Denat,
    ShipClass::DughHegh,
    ShipClass::Felketh,
    ShipClass::Goralis,
    ShipClass::Jenthar,
    ShipClass::Ktinga,
    ShipClass::Lotleh,
    ShipClass::Ngapej,
    ShipClass::Pachag,
    ShipClass::QaDlej,
    ShipClass::Vodleq,
    ShipClass::Tormag,
    ShipClass::Roqul,
    ShipClass::BaHreth,
    ShipClass::HajHal,
    ShipClass::Kelvar,
    ShipClass::Qacheng,
    ShipClass::Savar,
    ShipClass::Tobeq,
    ShipClass::Yotwl,
    ShipClass::Bachchund,
    ShipClass::DeSjoh,
    ShipClass::Pogach,
    ShipClass::Sompek,
    ShipClass::TroQa,
    ShipClass::Blakoth,
    ShipClass::DorHub,
    ShipClass::Drenok,
    ShipClass::Qljtagh,
    ShipClass::Veltas,
    ShipClass::Vergrah,
];

#[derive(PartialEq, Debug, Copy, Clone, RandGen)]
pub enum ShipClass {
    // Federation Ship Classes
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
    // Klingon Ship Classes
    NeghVar,
    Geljoq,
    Brel,
    Felgra,
    Kmpec,
    KVort,
    Qethla,
    Torath,
    Vorcha,
    Denat,
    DughHegh,
    Felketh,
    Goralis,
    Jenthar,
    Ktinga,
    Lotleh,
    Ngapej,
    Pachag,
    QaDlej,
    Vodleq,
    Tormag,
    Roqul,
    BaHreth,
    HajHal,
    Kelvar,
    Qacheng,
    Savar,
    Tobeq,
    Yotwl,
    Bachchund,
    DeSjoh,
    Pogach,
    Sompek,
    TroQa,
    Blakoth,
    DorHub,
    Drenok,
    Qljtagh,
    Veltas,
    Vergrah,
}

impl Display for ShipClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Federation Ship Classes
            ShipClass::Galaxy => {
                write!(f, "Galaxy")
            }
            ShipClass::Intrepid => {
                write!(f, "Intrepid")
            }
            ShipClass::Defiant => {
                write!(f, "Defiant")
            }
            ShipClass::Sovereign => {
                write!(f, "Sovereign")
            }
            ShipClass::Oberth => {
                write!(f, "Oberth")
            }
            ShipClass::Nova => {
                write!(f, "Nova")
            }
            ShipClass::Saber => {
                write!(f, "Saber")
            }
            ShipClass::Miranda => {
                write!(f, "Miranda")
            }
            ShipClass::Constellation => {
                write!(f, "Constellation")
            }
            ShipClass::Cheyenne => {
                write!(f, "Constellation")
            }
            ShipClass::Dakota => {
                write!(f, "Dakota")
            }
            ShipClass::Prometheus => {
                write!(f, "Prometheus")
            }
            ShipClass::Nebula => {
                write!(f, "Nebula")
            }
            ShipClass::Luna => {
                write!(f, "Luna")
            }
            ShipClass::Akira => {
                write!(f, "Akira")
            }
            ShipClass::Excelsior => {
                write!(f, "Excelsior")
            }
            ShipClass::Ambassador => {
                write!(f, "Ambassador")
            }
            ShipClass::Odyssey => {
                write!(f, "Odyssey")
            }
            // Klingon Ship Classes
            ShipClass::NeghVar => {
                write!(f, "Negh'Var")
            }
            ShipClass::Geljoq => {
                write!(f, "Gel'joQ")
            }
            ShipClass::Brel => {
                write!(f, "B'rel")
            }
            ShipClass::Felgra => {
                write!(f, "Felg'ra")
            }
            ShipClass::Kmpec => {
                write!(f, "K'mpec")
            }
            ShipClass::KVort => {
                write!(f, "K'Vort")
            }
            ShipClass::Qethla => {
                write!(f, "Qethla'")
            }
            ShipClass::Torath => {
                write!(f, "Torath")
            }
            ShipClass::Vorcha => {
                write!(f, "Vor'cha")
            }
            ShipClass::Denat => {
                write!(f, "De'nat")
            }
            ShipClass::DughHegh => {
                write!(f, "DughHegh")
            }
            ShipClass::Felketh => {
                write!(f, "Fel'keth")
            }
            ShipClass::Goralis => {
                write!(f, "Goralis")
            }
            ShipClass::Jenthar => {
                write!(f, "Jen'thar")
            }
            ShipClass::Ktinga => {
                write!(f, "K't'inga")
            }
            ShipClass::Lotleh => {
                write!(f, "Lotl'eh")
            }
            ShipClass::Ngapej => {
                write!(f, "Ngapej")
            }
            ShipClass::Pachag => {
                write!(f, "Pa'chag")
            }
            ShipClass::QaDlej => {
                write!(f, "QaDlej")
            }
            ShipClass::Roqul => {
                write!(f, "Ro'qul")
            }
            ShipClass::Tormag => {
                write!(f, "Tormag")
            }
            ShipClass::Vodleq => {
                write!(f, "VodleQ")
            }
            ShipClass::BaHreth => {
                write!(f, "BaH'reth")
            }
            ShipClass::HajHal => {
                write!(f, "HajHal")
            }
            ShipClass::Kelvar => {
                write!(f, "Kel'var")
            }
            ShipClass::Qacheng => {
                write!(f, "Qa'cheng")
            }
            ShipClass::Savar => {
                write!(f, "Sa'var")
            }
            ShipClass::Tobeq => {
                write!(f, "To'beq")
            }
            ShipClass::Yotwl => {
                write!(f, "Yotwl")
            }
            ShipClass::Bachchund => {
                write!(f, "Bach'chunD")
            }
            ShipClass::DeSjoh => {
                write!(f, "DeSjoH")
            }
            ShipClass::Pogach => {
                write!(f, "Po'gach")
            }
            ShipClass::Sompek => {
                write!(f, "Sompek")
            }
            ShipClass::TroQa => {
                write!(f, "Tro'Qa")
            }
            ShipClass::Blakoth => {
                write!(f, "Bla'koth")
            }
            ShipClass::DorHub => {
                write!(f, "DorHub")
            }
            ShipClass::Drenok => {
                write!(f, "Drenok")
            }
            ShipClass::Qljtagh => {
                write!(f, "Qlj'tagh")
            }
            ShipClass::Veltas => {
                write!(f, "Vel'taS")
            }
            ShipClass::Vergrah => {
                write!(f, "Ver'graH")
            }
        }
    }
}

pub fn get_random_federation_class() -> ShipClass {
    FEDERATION_SHIP_CLASSES[get_index((FEDERATION_SHIP_CLASSES.len() - 1) as u8)]
}

pub fn get_random_klingon_class() -> ShipClass {
    KLINGON_SHIP_CLASSES[get_index((KLINGON_SHIP_CLASSES.len() - 1) as u8)]
}

fn get_index(name_length: u8) -> usize {
    generate_random_value_from_range_u8(generate_seed(), 0, name_length) as usize
}