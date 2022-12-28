pub struct Hull {
    max_hull: u8,
    current_hull: u8,
    repair_rate: u8,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            max_hull: 100,
            current_hull: 100,
            repair_rate: 5,
        }
    }
}
