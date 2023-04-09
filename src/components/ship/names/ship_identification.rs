use super::faction_name::FactionName;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct ShipIdentification {
    pub serial_number: String,
    pub faction: FactionName,
}
