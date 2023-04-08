use crate::components::ship::name::faction_name::FactionName;

use super::random_generation::generate_random_value_from_range_u16;

pub fn generate_random_identifier(seed: u64, faction_name: FactionName) -> String {
    let identifier = generate_random_value_from_range_u16(seed, 1000, u16::MAX);

    match faction_name {
        FactionName::Federation => {
            format!("USS-{identifier}")
        }
        FactionName::KlingonEmpire => {
            format!("IKS-{identifier}")
        }
    }
}

#[cfg(test)]
mod ship_identifier_generation_should {
    use super::*;

    #[test]
    fn be_able_to_generate_a_random_identifier_for_federation_faction() {
        // Given
        let seed = 0;
        let faction = FactionName::Federation;

        // When
        let identifier = generate_random_identifier(seed, faction);

        // Then
        assert_eq!("USS-52722".to_string(), identifier);
    }

    #[test]
    fn be_able_to_generate_a_random_identifier_for_klingon_faction() {
        // Given
        let seed = 0;
        let faction = FactionName::KlingonEmpire;

        // When
        let identifier = generate_random_identifier(seed, faction);

        // Then
        assert_eq!("IKS-52722".to_string(), identifier);
    }
}
