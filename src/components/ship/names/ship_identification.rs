use super::{
    faction_name::FactionName, ship_class::ShipClass, ship_name::ShipName, serial_number::SerialNumber,
};
use crate::{
    components::ship::names::{ship_class::get_random_class, ship_name::get_random_name},
};
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, Clone, Copy, RandGen)]
pub struct ShipIdentification {
    pub name: ShipName,
    pub class: ShipClass,
    pub serial_number: SerialNumber,
    pub faction: FactionName,
}

impl ShipIdentification {
    pub fn new(seed: u64, faction: FactionName) -> Self {
        match faction {
            FactionName::Federation => Self {
                serial_number: SerialNumber::new(seed, &faction),
                faction,
                name: get_random_name(seed, FactionName::Federation),
                class: get_random_class(seed, FactionName::Federation),
            },
            FactionName::KlingonEmpire => Self {
                serial_number: SerialNumber::new(seed, &faction),
                faction,
                name: get_random_name(seed, FactionName::KlingonEmpire),
                class: get_random_class(seed, FactionName::KlingonEmpire),
            },
        }
    }

    pub fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Class: {} |",
            self.serial_number, self.name, self.class
        )
    }

    pub fn display_ship_name_and_faction(&self) {
        println!(
            "| Name: {} {} | Class: {} | Faction {} |",
            self.serial_number, self.name, self.class, self.faction
        )
    }
}

#[cfg(test)]
mod ship_identification_should {
    use super::*;
    use crate::components::ship::names::faction_name::FactionName;

    #[test]
    fn create_new_federation() {
        // Given
        let ship_identification = ShipIdentification::new(0, FactionName::Federation);

        // Then
        assert_eq!(ShipName::Prometheus, ship_identification.name);
        assert_eq!(ShipClass::Luna, ship_identification.class);
        assert_eq!(FactionName::Federation, ship_identification.faction);
        assert_ne!(
            String::default(),
            ship_identification.serial_number.to_string()
        );
        assert_eq!("USS-52722", ship_identification.serial_number.to_string());
    }

    #[test]
    fn create_new_klingon() {
        // Given
        let ship_identification = ShipIdentification::new(0, FactionName::KlingonEmpire);

        // Then
        assert_eq!(ShipName::TAcog, ship_identification.name);
        assert_eq!(ShipClass::Pogach, ship_identification.class);
        assert_eq!(FactionName::KlingonEmpire, ship_identification.faction);
        assert_ne!(
            String::default(),
            ship_identification.serial_number.to_string()
        );
        assert_eq!("IKS-52722", ship_identification.serial_number.to_string());
    }
}
