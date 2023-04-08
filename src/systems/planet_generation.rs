use crate::{
    components::game_world::planet::Planet,
    systems::random_generation::generate_random_value_from_range_u8,
};

pub fn generate_planets(seed: u64) -> Vec<Planet> {
    let quantity = generate_random_value_from_range_u8(seed, 1, 10);
    let mut index = 0;
    let mut planets = Vec::new();

    while quantity > index {
        planets.push(Planet::default());
        index += 1;
    }

    planets
}

#[cfg(test)]
mod planet_generation_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 8)]
    #[case(4545, 5)]
    #[case(7000, 9)]
    fn be_able_to_generate_planets(#[case] seed: u64, #[case] size: usize) {
        // When
        let planets = generate_planets(seed);

        // Then
        assert_eq!(size, planets.len())
    }
}
