use super::planet::Planet;

#[allow(dead_code)]
pub struct Star {
    planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            planets: vec![Planet::default()],
        }
    }
}
