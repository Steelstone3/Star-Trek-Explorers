use crate::controllers::random_number::get_random_number_from_range;
use rand_derive2::RandGen;
use std::fmt::Display;
use std::fmt::Formatter;

pub const FEDERATION_SHIP_NAMES: [ShipName; 7] = [
    ShipName::Ambassador,
    ShipName::Akira,
    ShipName::Excelsior,
    ShipName::KobayashiMaru,
    ShipName::Enterprise,
    ShipName::Defiant,
    ShipName::Voyager,
];
const FEDERATION_FACTION_PREFIX: &str = "USS";

pub const KLINGON_SHIP_NAMES: [ShipName; 3] = [ShipName::Amak, ShipName::Chang, ShipName::BaHwil];
const KLINGON_FACTION_PREFIX: &str = "IKS";

const LOWEST_SERIAL_NUMBER: u64 = 1000;
const HIGHEST_SERIAL_NUMBER: u64 = 999999;

#[derive(RandGen, PartialEq, Clone, Copy)]
pub enum ShipName {
    //Federation Ship Names
    Ambassador,
    Akira,
    Excelsior,
    KobayashiMaru,
    Enterprise,
    Defiant,
    Voyager,
    //Klingon Ship Names
    Chang,
    Amak,
    BaHwil,
}

impl Display for ShipName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipName::Ambassador => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Ambassador")
                )
            }

            ShipName::Akira => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Akira")
                )
            }

            ShipName::Excelsior => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Excelsior")
                )
            }

            ShipName::KobayashiMaru => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Kobayashi Maru")
                )
            }

            ShipName::Enterprise => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Enterprise")
                )
            }

            ShipName::Defiant => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Defiant")
                )
            }

            ShipName::Voyager => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Voyager")
                )
            }

            ShipName::Chang => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(KLINGON_FACTION_PREFIX, "Chang")
                )
            }

            ShipName::Amak => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(KLINGON_FACTION_PREFIX, "Amak")
                )
            }

            ShipName::BaHwil => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(KLINGON_FACTION_PREFIX, "BaHwil")
                )
            }
        }
    }
}

fn display_ship_name(prefix: &str, name: &str) -> String {
    format!(
        "{} {} {}",
        prefix,
        name,
        get_random_number_from_range(LOWEST_SERIAL_NUMBER, HIGHEST_SERIAL_NUMBER)
    )
}
