pub struct Hull {
    max_hull: u8,
    min_hull: u8,
    repair_rate: u8,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            max_hull: 100,
            min_hull: 0,
            repair_rate: 5,
        }
    }
}
