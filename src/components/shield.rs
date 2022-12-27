pub struct Shield {
    max_shield: u8,
    min_shield: u8,
    regeneration_rate: u8,
}

impl Default for Shield {
    fn default() -> Self {
        Self {
            max_shield: 100,
            min_shield: 0,
            regeneration_rate: 25,
        }
    }
}
