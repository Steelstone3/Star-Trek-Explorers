pub trait DamageTaker {
    fn take_damage(&mut self);
}

pub trait DamageDealer {
    fn calculate_damage(&self) -> u8;
}
