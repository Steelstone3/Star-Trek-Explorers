use crate::components::game_world::star::Star;

pub fn generate_stars() -> [Star; 100] {
    let stars: [Star; 100] = [
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default_visible_star(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
        Star::default(),
    ];

    stars
}

#[cfg(test)]
mod star_generation_should {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn be_able_to_generate_star() {
        // Given
        let stop_watch = Instant::now();

        // When
        let stars = generate_stars();

        // Then
        assert!(Duration::from_millis(50) > stop_watch.elapsed());
        assert_eq!(100, stars.len())
    }

    #[test]
    fn are_stars_visible() {
        // Given
        let mut visible_stars: Vec<bool> = Vec::new();
        let stars = generate_stars();

        // When
        for star in stars {
            if star.is_visible {
                visible_stars.push(true);
            }
        }

        // Then
        assert!(24 < visible_stars.len())
    }
}
