use crate::{
    models::ships::ship::Ship,
    presenters::combat_presenter::{
        choose_hostile_target, choose_weapon_system, display_fleet_status, display_ship_status, defeat_message_hostiles,
    },
};

use super::combat_controller::attack_hostile_target;

pub fn run_player_turn(player: &Ship, hostiles: &mut [Ship]) {
    if !hostiles.is_empty() {
        display_ship_status(player);
        let target_ship = choose_hostile_target(hostiles);
        let weapon_selection = choose_weapon_system();
        attack_hostile_target(weapon_selection, player, target_ship);
        display_fleet_status(hostiles);
    } else {
        defeat_message_hostiles();
    }
}

#[cfg(test)]
mod player_turn_should {
    use super::*;

    #[test]
    #[ignore = "Need to implement mocking for the presenter"]
    fn not_allow_player_to_fight_if_there_are_no_hostiles() { }

    #[test]
    #[ignore = "Need to mock user inputs"]
    fn allow_player_to_damage_hostiles() {
        let player = player_ship_fixture();
        let mut hostiles = hostile_ships_fixture();

        run_player_turn(&player, &mut hostiles);

        assert_eq!('K', hostiles[0].display_symbol);
        assert_eq!("IKS Kang".to_string(), hostiles[0].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[0].faction);
        assert_eq!("Bird of Prey".to_string(), hostiles[0].class);
        assert_eq!(50, hostiles[0].shield_strength);
        assert_eq!(90, hostiles[0].hull_integrity);
    }

    fn player_ship_fixture() -> Ship {
        Ship::create_ship(
            'S',
            "USS Enterprise".to_string(),
            "Federation".to_string(),
            "Galaxy".to_string(),
        )
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
