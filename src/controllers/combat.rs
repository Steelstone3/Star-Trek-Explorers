use crate::models::ships::ship::Ship;
use crate::presenters::combat_presenter::{choose_hostile_target, choose_weapon_system};

pub fn enter_combat(player: &Ship, allies: &[Ship], hostiles: &mut [Ship]) {
    player_turn(player, hostiles);
    allies_turn(allies, hostiles);
    hostiles_turn(hostiles, allies, player);
}

fn player_turn(player: &Ship, hostiles: &mut [Ship]) {
    let target_ship = choose_hostile_target(hostiles);
    match choose_weapon_system() {
        0 => player.fire_phasers(target_ship),
        1 => player.fire_torpedoes(target_ship),
        u32::MAX => panic!(),
        2_u32..=4294967294_u32 => panic!(),
    };
}

fn allies_turn(_allies: &[Ship], _player_damaged_hostiles: &[Ship]) {}

fn hostiles_turn(_hostiles: &[Ship], _allies: &[Ship], _player: &Ship) {}

#[allow(dead_code)]
fn check_fight_over() -> bool {
    false
}

#[cfg(test)]
mod combat_should {
    use super::*;

    #[test]
    #[ignore]
    fn allow_player_to_damage_hostiles() {
        let player = player_ship_fixture();
        let mut hostiles = hostile_ships_fixture();

        player_turn(&player, &mut hostiles);

        assert_eq!('K', hostiles[0].display_symbol);
        assert_eq!("IKS Kang".to_string(), hostiles[0].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[0].faction);
        assert_eq!("Bird of Prey".to_string(), hostiles[0].class);
        assert_eq!(50, hostiles[0].shield_strength);
        assert_eq!(90, hostiles[0].hull_integrity);
    }

    #[test]
    #[ignore]
    fn allow_hostiles_to_damage_player() {
        let player = player_ship_fixture();
        let allies = allies_ships_fixture();
        let hostiles = hostile_ships_fixture();

        hostiles_turn(&hostiles, &allies, &player);

        assert_eq!('S', player.display_symbol);
        assert_eq!("USS Enterprise".to_string(), player.name);
        assert_eq!("Federation".to_string(), player.faction);
        assert_eq!("Galaxy".to_string(), player.class);
        assert_eq!(50, player.shield_strength);
        assert_eq!(90, player.hull_integrity);
    }

    #[test]
    #[ignore]
    fn allow_allies_to_damage_hostiles() {
        let allies = allies_ships_fixture();
        let hostiles = hostile_ships_fixture();

        allies_turn(&allies, &hostiles);

        assert_eq!('K', hostiles[0].display_symbol);
        assert_eq!("IKS Kang".to_string(), hostiles[0].name);
        assert_eq!("Klingon Empire".to_string(), hostiles[0].faction);
        assert_eq!("Bird of Prey".to_string(), hostiles[0].class);
        assert_eq!(50, hostiles[0].shield_strength);
        assert_eq!(90, hostiles[0].hull_integrity);
    }

    #[test]
    #[ignore]
    fn allow_hostiles_to_damage_allies() {
        let player = player_ship_fixture();
        let allies = allies_ships_fixture();
        let hostiles = hostile_ships_fixture();

        hostiles_turn(&hostiles, &allies, &player);

        assert_eq!('F', allies[0].display_symbol);
        assert_eq!("USS Defiant".to_string(), allies[0].name);
        assert_eq!("Federation".to_string(), allies[0].faction);
        assert_eq!("Defiant".to_string(), allies[0].class);
        assert_eq!(50, allies[0].shield_strength);
        assert_eq!(90, allies[0].hull_integrity);
    }

    #[test]
    #[ignore]
    fn allow_combat_to_end_when_all_hostiles_are_destroyed() {
        let _player = player_ship_fixture();
        let _allies = allies_ships_fixture();
        let _hostiles: Vec<Ship> = vec![];
        let is_combat_concluded = check_fight_over();

        assert_eq!(true, is_combat_concluded);
    }

    #[test]
    #[ignore]
    fn allow_combat_to_end_when_player_is_destroyed() {
        let mut player = player_ship_fixture();
        player.shield_strength = 0;
        player.hull_integrity = 0;
        let _allies = allies_ships_fixture();
        let _hostiles = hostile_ships_fixture();
        let is_combat_concluded = check_fight_over();

        assert_eq!(true, is_combat_concluded);
    }

    fn player_ship_fixture() -> Ship {
        Ship::create_ship(
            'S',
            "USS Enterprise".to_string(),
            "Federation".to_string(),
            "Galaxy".to_string(),
        )
    }

    #[allow(dead_code)]
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
