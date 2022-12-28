pub struct Shield {
    max_shield: u8,
    current_shield: u8,
    regeneration_rate: u8,
}

impl Default for Shield {
    fn default() -> Self {
        Self {
            max_shield: 100,
            current_shield: 100,
            regeneration_rate: 25,
        }
    }
}
