pub trait Ship {
    fn display_ship_name(&self);
    fn display_ship_name_and_faction(&self);
    fn display_offensive_capabilities(&self);
    fn display_defensive_capabilities(&self);
    fn take_damage_from_hostile_ship(&mut self, damage: u8);
}