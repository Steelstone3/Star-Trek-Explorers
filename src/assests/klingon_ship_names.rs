use crate::controllers::random_number::get_random_number_from_range;
use rand_derive2::RandGen;
use std::fmt::{Display, Formatter};

const KLINGON_FACTION_PREFIX: &str = "IKS";
const KLINGON_LOWEST_SERIAL_NUMBER: u64 = 100;
const KLINGON_HIGHEST_SERIAL_NUMBER: u64 = 9999;

#[derive(RandGen)]
pub enum KlingonShipName {
    Chang,
    Amak,
    BaHwil,
}

impl Display for KlingonShipName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KlingonShipName::Chang => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(KLINGON_FACTION_PREFIX, "Chang")
                )
            }

            KlingonShipName::Amak => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(KLINGON_FACTION_PREFIX, "Amak")
                )
            }

            KlingonShipName::BaHwil => {
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
        get_random_number_from_range(KLINGON_LOWEST_SERIAL_NUMBER, KLINGON_HIGHEST_SERIAL_NUMBER)
    )
}
