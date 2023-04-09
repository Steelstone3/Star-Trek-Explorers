use crate::systems::ship_identifier_generation::generate_random_identifier;

use super::faction_name::FactionName;
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct ShipIdentification {
    pub serial_number: String,
    pub faction: FactionName,
}

impl ShipIdentification {
    pub fn new(seed: u64, faction: FactionName) -> Self {
        Self {
            serial_number: generate_random_identifier(seed, &faction),
            faction,
        }
    }

    pub fn display_ship_name(&self, ship_name: String, ship_class: String) {
        println!(
            "| Name: {} {} | Class: {} |",
            self.serial_number, ship_name, ship_class
        )
    }

    pub fn display_ship_name_and_faction(&self, ship_name: String, ship_class: String) {
        println!(
            "| Name: {} {} | Class: {} | Faction {} |",
            self.serial_number, ship_name, ship_class, self.faction
        )
    }
}

#[cfg(test)]
mod ship_identification_should {
    use super::*;
    use crate::components::ship::names::faction_name::FactionName;

    #[test]
    fn create_new() {
        // Given
        let ship_identification = ShipIdentification::new(0, FactionName::Federation);

        // Then
        assert_eq!(FactionName::Federation, ship_identification.faction);
        assert_eq!("USS-52722", ship_identification.serial_number);
    }
}
