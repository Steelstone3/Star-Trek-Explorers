use crate::models::ships::ship::Ship;

use super::combat_controller::{attack_hostile_target_ai, choose_hostile_target_ai};

pub fn run_hostiles_turn(seed:u64, hostiles: &[Ship], allies: &mut [Ship], player: &mut Ship) {
    let target_ship = choose_hostile_target_ai(seed, allies);
    attack_hostile_target_ai(seed, hostiles, target_ship);
    attack_hostile_target_ai(seed, hostiles, player);
}

#[cfg(test)]
mod hostiles_turn_should {
    use super::*;

    #[test]
    fn allow_hostiles_to_damage_player() {
        let mut player = player_ship_fixture();
        let mut allies = allies_ships_fixture();
        let hostiles = hostile_ships_fixture();

        run_hostiles_turn(69, &hostiles, &mut allies, &mut player);

        assert_eq!('S', player.display_symbol);
        assert_eq!("USS Enterprise".to_string(), player.name);
        assert_eq!("Federation".to_string(), player.faction);
        assert_eq!("Galaxy".to_string(), player.class);
        assert_eq!(50, player.shield_strength);
        assert_eq!(100, player.hull_integrity);
    }

    #[test]
    fn allow_hostiles_to_damage_allies() {
        let mut player = player_ship_fixture();
        let mut allies = allies_ships_fixture();
        let hostiles = hostile_ships_fixture();

        run_hostiles_turn(69, &hostiles, &mut allies, &mut player);

        assert_eq!('F', allies[0].display_symbol);
        assert_eq!("USS Defiant".to_string(), allies[0].name);
        assert_eq!("Federation".to_string(), allies[0].faction);
        assert_eq!("Defiant".to_string(), allies[0].class);
        assert_eq!(50, allies[0].shield_strength);
        assert_eq!(100, allies[0].hull_integrity);

        assert_eq!('F', allies[1].display_symbol);
        assert_eq!("USS Challenger".to_string(), allies[1].name);
        assert_eq!("Federation".to_string(), allies[1].faction);
        assert_eq!("Sovereign".to_string(), allies[1].class);
        assert_eq!(100, allies[1].shield_strength);
        assert_eq!(100, allies[1].hull_integrity);
    }

    fn player_ship_fixture() -> Ship {
        Ship::create_ship(
            'S',
            "USS Enterprise".to_string(),
            "Federation".to_string(),
            "Galaxy".to_string(),
        )
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
