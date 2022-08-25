use crate::controllers::random_number::get_random_number_from_range;
use rand_derive2::RandGen;
use std::fmt::Display;
use std::fmt::Formatter;

const FEDERATION_FACTION_PREFIX: &str = "USS";
const FEDERATION_LOWEST_SERIAL_NUMBER: u64 = 1000;
const FEDERATION_HIGHEST_SERIAL_NUMBER: u64 = 999999;

#[derive(RandGen)]
pub enum FederationShipName {
    Ambassador,
    Akira,
    Excelsior,
    KobayashiMaru,
    Enterprise,
    Defiant,
    Voyager,
}

impl Display for FederationShipName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationShipName::Ambassador => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Ambassador")
                )
            }

            FederationShipName::Akira => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Akira")
                )
            }

            FederationShipName::Excelsior => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Excelsior")
                )
            }

            FederationShipName::KobayashiMaru => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Kobayashi Maru")
                )
            }

            FederationShipName::Enterprise => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Kobayashi Maru")
                )
            }

            FederationShipName::Defiant => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Kobayashi Maru")
                )
            }

            FederationShipName::Voyager => {
                write!(
                    formatter,
                    "{}",
                    display_ship_name(FEDERATION_FACTION_PREFIX, "Kobayashi Maru")
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
        get_random_number_from_range(
            FEDERATION_LOWEST_SERIAL_NUMBER,
            FEDERATION_HIGHEST_SERIAL_NUMBER
        )
    )
}