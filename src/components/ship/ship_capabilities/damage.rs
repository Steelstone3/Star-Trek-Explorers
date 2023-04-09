pub trait DamageTaker {
    fn take_damage(&mut self, damage: u8);
}

pub trait DamageDealer {
    fn calculate_damage(&self, seed: u64) -> u8;
}
