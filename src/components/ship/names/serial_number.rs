use std::fmt::Display;

use rand_derive2::RandGen;

use crate::systems::ship_identifier_generation::generate_random_identifier;

use super::{faction_name::FactionName, id_prefix::IdPrefix};

#[derive(Debug, PartialEq,Clone, Copy, RandGen)]
pub struct SerialNumber {
    pub id_prefix: IdPrefix,
    pub id_number: u16,
}

impl SerialNumber {
    pub fn new(seed: u64, faction: &FactionName) -> Self {
        match faction {
            FactionName::Federation => Self {
                id_prefix: IdPrefix::FederationIdPrefix,
                id_number: generate_random_identifier(seed),
            },
            FactionName::KlingonEmpire => Self {
                id_prefix: IdPrefix::KlingonIdPrefix,
                id_number: generate_random_identifier(seed),
            },
        }
    }
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.id_prefix, self.id_number)
    }
}

#[cfg(test)]
mod serial_number_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, FactionName::Federation, IdPrefix::FederationIdPrefix, 52722)]
    #[case(1111, FactionName::Federation, IdPrefix::FederationIdPrefix, 60010)]
    #[case(1111, FactionName::KlingonEmpire, IdPrefix::KlingonIdPrefix, 60010)]
    fn create_new(
        #[case] seed: u64,
        #[case] faction: FactionName,
        #[case] expected_prefix: IdPrefix,
        #[case] expected_id_number: u16,
    ) {
        // Given
        let serial_number = SerialNumber::new(seed, &faction);

        // Then
        assert_eq!(expected_prefix, serial_number.id_prefix);
        assert_eq!(expected_id_number, serial_number.id_number);
    }

    #[rstest]
    #[case(0, FactionName::Federation, "USS-52722")]
    #[case(0, FactionName::KlingonEmpire, "IKS-52722")]
    #[case(1111, FactionName::Federation, "USS-60010")]
    fn display_should_return_full_ship_identification(
        #[case] seed: u64,
        #[case] faction: FactionName,
        #[case] expected_display: String,
    ) {
        // Given
        let serial_number = SerialNumber::new(seed, &faction);

        // When
        let id = serial_number.to_string();

        // Then
        assert_eq!(expected_display, id);
    }
}
