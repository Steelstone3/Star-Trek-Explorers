use super::combat_controller::{attack_hostile_target_ai, choose_hostile_target_ai};
use crate::{models::ships::ship::Ship, presenters::combat_presenter::display_fleet_status};

pub fn run_allies_turn(seed:u64, allies: &[Ship], hostiles: &mut [Ship]) {
    display_fleet_status(&allies);
    let target_ship = choose_hostile_target_ai(seed, hostiles);
    attack_hostile_target_ai(seed, allies, target_ship);
    display_fleet_status(&hostiles);
}

#[cfg(test)]
mod allies_turn_should {
    use super::*;

    #[test]
    fn allow_allies_to_damage_hostiles() {
        let allies = allies_ships_fixture();
        let mut hostiles = hostile_ships_fixture();

        run_allies_turn(69, &allies, &mut hostiles);

        assert_eq!('K', hostiles[0].display_symbol);
        assert_eq!("IKS Kang".to_string(), hostiles[0].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[0].faction);
        assert_eq!("Bird of Prey".to_string(), hostiles[0].class);
        assert_eq!(50, hostiles[0].shield_strength);
        assert_eq!(100, hostiles[0].hull_integrity);

        assert_eq!('K', hostiles[1].display_symbol);
        assert_eq!("IKS Bat'leth".to_string(), hostiles[1].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[1].faction);
        assert_eq!("D7".to_string(), hostiles[1].class);
        assert_eq!(100, hostiles[1].shield_strength);
        assert_eq!(100, hostiles[1].hull_integrity);
    }

    fn allies_ships_fixture() -> Vec<Ship> {
        vec![
            Ship::create_ship(
                'F',
                "USS Defiant".to_string(),
                "Federation".to_string(),
                "Defiant".to_string(),
            ),
            Ship::create_ship(
                'F',
                "USS Challenger".to_string(),
                "Federation".to_string(),
                "Sovereign".to_string(),
            ),
        ]
    }

    fn hostile_ships_fixture() -> Vec<Ship> {
        vec![
            Ship::create_ship(
                'K',
                "IKS Kang".to_string(),
                "Klingon Empire".to_string(),
                "Bird of Prey".to_string(),
            ),
            Ship::create_ship(
                'K',
                "IKS Bat'leth".to_string(),
                "Klingon Empire".to_string(),
                "D7".to_string(),
            ),
        ]
    }
}
