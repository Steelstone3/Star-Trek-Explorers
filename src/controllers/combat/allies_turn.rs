use crate::models::ships::ship::Ship;
use super::combat_controller::{attack_hostile_target_ai, choose_hostile_target_ai};

pub fn run_allies_turn(allies: &[Ship], hostiles: &mut [Ship]) {
    let target_ship = choose_hostile_target_ai(hostiles);
    attack_hostile_target_ai(allies, target_ship);
}

#[cfg(test)]
mod allies_turn_should {
    use super::*;

    #[test]
    fn allow_allies_to_damage_hostiles() {
        let allies = allies_ships_fixture();
        let mut hostiles = hostile_ships_fixture();

        run_allies_turn(&allies, &mut hostiles);

        assert_eq!('K', hostiles[0].display_symbol);
        assert_eq!("IKS Kang".to_string(), hostiles[0].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[0].faction);
        assert_eq!("Bird of Prey".to_string(), hostiles[0].class);
        assert_eq!(75, hostiles[0].shield_strength);
        assert_eq!(100, hostiles[0].hull_integrity);
    }

    fn allies_ships_fixture() -> Vec<Ship> {
        vec![Ship::create_ship(
            'F',
            "USS Defiant".to_string(),
            "Federation".to_string(),
            "Defiant".to_string(),
        )]
    }

    fn hostile_ships_fixture() -> Vec<Ship> {
        vec![Ship::create_ship(
            'K',
            "IKS Kang".to_string(),
            "Klingon Empire".to_string(),
            "Bird of Prey".to_string(),
        )]
    }
}
