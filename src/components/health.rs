pub struct Health {
    max_health: u8,
    current_health: u8,
    regeneration_rate: u8,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max_health: 100,
            current_health: 100,
            regeneration_rate: 25,
        }
    }
}
