use super::star::Star;

pub struct Universe {
    stars: Vec<Star>,
}

impl Default for Universe {
    fn default() -> Self {
        Self { stars: todo!() }
    }
}

#[cfg(test)]
mod universe_should {}