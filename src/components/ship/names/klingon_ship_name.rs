use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(PartialEq, Debug, RandGen)]
pub enum KlingonShipName {
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

impl Display for KlingonShipName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KlingonShipName::Amar => {
                write!(f, "Amar")
            }
            KlingonShipName::BMoth => {
                write!(f, "B'Moth")
            }
            KlingonShipName::Brel => {
                write!(f, "B'rel")
            }
            KlingonShipName::Buruk => {
                write!(f, "B'rel")
            }
            KlingonShipName::ChTang => {
                write!(f, "Ch'Tang")
            }
            KlingonShipName::Groth => {
                write!(f, "Groth")
            }
            KlingonShipName::Heghta => {
                write!(f, "Heghta")
            }
            KlingonShipName::KVada => {
                write!(f, "K'Vada")
            }
            KlingonShipName::KVort => {
                write!(f, "K'Vort")
            }
            KlingonShipName::Ktinga => {
                write!(f, "K't'inga")
            }
            KlingonShipName::Kitang => {
                write!(f, "Ki'tang")
            }
            KlingonShipName::Koraga => {
                write!(f, "Koraga")
            }
            KlingonShipName::MChar => {
                write!(f, "M'Char")
            }
            KlingonShipName::NeghVar => {
                write!(f, "NeghVar")
            }
            KlingonShipName::Ningtao => {
                write!(f, "Ning'tao")
            }
            KlingonShipName::Orantho => {
                write!(f, "Orantho")
            }
            KlingonShipName::Pagh => {
                write!(f, "Pagh")
            }
            KlingonShipName::Rotarran => {
                write!(f, "Rotarran")
            }
            KlingonShipName::Slivin => {
                write!(f, "Slivin")
            }
            KlingonShipName::Somraw => {
                write!(f, "Somraw")
            }
            KlingonShipName::TAcog => {
                write!(f, "T'Acog")
            }
            KlingonShipName::Tagana => {
                write!(f, "Tagana")
            }
            KlingonShipName::TohKaht => {
                write!(f, "Toh'Kaht")
            }
            KlingonShipName::Voodieh => {
                write!(f, "Voodieh")
            }
            KlingonShipName::Vorcha => {
                write!(f, "Vor'cha")
            }
            KlingonShipName::Vorn => {
                write!(f, "Vorn")
            }
        }
    }
}
