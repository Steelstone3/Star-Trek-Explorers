pub trait Ship {
    fn display_ship_name(&self);
    fn display_ship_name_and_faction(&self);
    fn display_offensive_capabilities(&self);
    fn display_defensive_capabilities(&self);
    fn select_ship_weapon_type(&self) -> String;
    fn calculate_damage_from_weapon(&self, seed: u64, weapon_name: String) -> u8;
    fn take_damage_from_hostile_ship(&mut self, damage: u8);
}
