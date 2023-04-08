use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn generate_random_value_from_range_u8(seed: u64, min: u8, max: u8) -> u8 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(min..max)
}

pub fn generate_random_value_from_range_u16(seed: u64, min: u16, max: u16) -> u16 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(min..max)
}

pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(u64::MIN..u64::MAX)
}

#[cfg(test)]
mod random_generation_should {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 204)]
    #[case(4545, 130)]
    #[case(3000, 133)]
    fn be_able_to_generate_random_u8_value_from_range(#[case] seed: u64, #[case] value: u8) {
        // When
        let result = generate_random_value_from_range_u8(seed, u8::MIN, u8::MAX);

        // Then
        assert_eq!(value, result);
    }

    #[rstest]
    #[case(0, 52523)]
    #[case(4545, 33610)]
    #[case(3000, 34354)]
    fn be_able_to_generate_random_u16_value_from_range(#[case] seed: u64, #[case] value: u16) {
        // When
        let result = generate_random_value_from_range_u16(seed, u16::MIN, u16::MAX);

        // Then
        assert_eq!(value, result);
    }
}
