use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct ShipStatus {
    pub shield_strength: u32,
    pub hull_integrity: u32,
    pub phaser_max_damage: u32,
    pub phaser_min_damage: u32,
    pub torpedo_max_damage: u32,
    pub torpedo_min_damage: u32,
}

#[cfg(test)]
mod ship_systems_should {
    // use super::*;

    #[test]
    #[ignore = "Not implemented yet"]
    fn do_damage_to_hostile_ship_systems() {
        
    }
}