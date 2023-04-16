use super::random_generation::generate_random_value_from_range_u16;

pub fn generate_random_identifier(seed: u64) -> u16 {
    generate_random_value_from_range_u16(seed, 1000, u16::MAX)
}

#[cfg(test)]
mod ship_identifier_generation_should {
    use super::*;

    #[test]
    fn be_able_to_generate_a_random_identifier_for_faction() {
        // Given
        let seed = 0;

        // When
        let identifier = generate_random_identifier(seed);

        // Then
        assert_eq!(52722, identifier);
    }
}
