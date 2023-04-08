use super::star::Star;

#[allow(dead_code)]
pub struct Universe {
    stars: Vec<Star>,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            stars: vec![Star::default()],
        }
    }
}

#[cfg(test)]
mod universe_should {}
