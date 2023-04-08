use crate::components::game_world::star::Star;

use super::random_generation::generate_random_value_from_range_u16;

pub fn generate_stars(seed: u64) -> Vec<Star> {
    let quantity = generate_random_value_from_range_u16(seed, 500, u16::MAX);
    let mut index = 0;
    let mut planets = Vec::new();

    while quantity > index {
        planets.push(Star::default());
        index += 1;
    }

    planets
}

#[cfg(test)]
mod star_generation_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 52622)]
    #[case(4545, 33853)]
    #[case(7000, 59820)]
    fn be_able_to_generate_star(#[case] seed: u64, #[case] size: usize) {
        // When
        let stars = generate_stars(seed);

        // Then
        assert_eq!(size, stars.len())
    }
}
