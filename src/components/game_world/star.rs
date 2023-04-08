use super::planet::Planet;

pub struct Star {
    planets: Vec<Planet>,
}

impl Default for Star {
    fn default() -> Self {
        Self { planets: todo!() }
    }
}

#[cfg(test)]
mod star_should {}