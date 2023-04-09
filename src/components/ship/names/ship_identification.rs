use rand_derive2::RandGen;
use super::faction_name::FactionName;

#[derive(PartialEq, Debug, RandGen)]
pub struct ShipIdentification {
    pub serial_number: String,
    pub faction: FactionName,
}