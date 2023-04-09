use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, Debug, RandGen)]
pub enum KlingonShipClass {
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

impl Display for KlingonShipClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KlingonShipClass::NeghVar => {
                write!(f, "Negh'Var")
            }
            KlingonShipClass::Geljoq => {
                write!(f, "Gel'joQ")
            }
            KlingonShipClass::Brel => {
                write!(f, "B'rel")
            }
            KlingonShipClass::Felgra => {
                write!(f, "Felg'ra")
            }
            KlingonShipClass::Kmpec => {
                write!(f, "K'mpec")
            }
            KlingonShipClass::KVort => {
                write!(f, "K'Vort")
            }
            KlingonShipClass::Qethla => {
                write!(f, "Qethla'")
            }
            KlingonShipClass::Torath => {
                write!(f, "Torath")
            }
            KlingonShipClass::Vorcha => {
                write!(f, "Vor'cha")
            }
            KlingonShipClass::Denat => {
                write!(f, "De'nat")
            }
            KlingonShipClass::DughHegh => {
                write!(f, "DughHegh")
            }
            KlingonShipClass::Felketh => {
                write!(f, "Fel'keth")
            }
            KlingonShipClass::Goralis => {
                write!(f, "Goralis")
            }
            KlingonShipClass::Jenthar => {
                write!(f, "Jen'thar")
            }
            KlingonShipClass::Ktinga => {
                write!(f, "K't'inga")
            }
            KlingonShipClass::Lotleh => {
                write!(f, "Lotl'eh")
            }
            KlingonShipClass::Ngapej => {
                write!(f, "Ngapej")
            }
            KlingonShipClass::Pachag => {
                write!(f, "Pa'chag")
            }
            KlingonShipClass::QaDlej => {
                write!(f, "QaDlej")
            }
            KlingonShipClass::Roqul => {
                write!(f, "Ro'qul")
            }
            KlingonShipClass::Tormag => {
                write!(f, "Tormag")
            }
            KlingonShipClass::Vodleq => {
                write!(f, "VodleQ")
            }
            KlingonShipClass::BaHreth => {
                write!(f, "BaH'reth")
            }
            KlingonShipClass::HajHal => {
                write!(f, "HajHal")
            }
            KlingonShipClass::Kelvar => {
                write!(f, "Kel'var")
            }
            KlingonShipClass::Qacheng => {
                write!(f, "Qa'cheng")
            }
            KlingonShipClass::Savar => {
                write!(f, "Sa'var")
            }
            KlingonShipClass::Tobeq => {
                write!(f, "To'beq")
            }
            KlingonShipClass::Yotwl => {
                write!(f, "Yotwl")
            }
            KlingonShipClass::Bachchund => {
                write!(f, "Bach'chunD")
            }
            KlingonShipClass::DeSjoh => {
                write!(f, "DeSjoH")
            }
            KlingonShipClass::Pogach => {
                write!(f, "Po'gach")
            }
            KlingonShipClass::Sompek => {
                write!(f, "Sompek")
            }
            KlingonShipClass::TroQa => {
                write!(f, "Tro'Qa")
            }
            KlingonShipClass::Blakoth => {
                write!(f, "Bla'koth")
            }
            KlingonShipClass::DorHub => {
                write!(f, "DorHub")
            }
            KlingonShipClass::Drenok => {
                write!(f, "Drenok")
            }
            KlingonShipClass::Qljtagh => {
                write!(f, "Qlj'tagh")
            }
            KlingonShipClass::Veltas => {
                write!(f, "Vel'taS")
            }
            KlingonShipClass::Vergrah => {
                write!(f, "Ver'graH")
            }
        }
    }
}
