use super::allies_turn::run_allies_turn;
use super::hostiles_turn::run_hostiles_turn;
use super::player_turn::run_player_turn;
use crate::controllers::game::game_randomiser::{generate_seed, get_seeded_random_number};
use crate::models::ships::ship::Ship;

pub fn enter_combat(player: &mut Ship, allies: &mut [Ship], hostiles: &mut [Ship]) {
    run_player_turn(player, hostiles);
    run_allies_turn(generate_seed(), allies, hostiles);
    run_hostiles_turn(generate_seed(), hostiles, allies, player);
}

pub fn choose_hostile_target_ai(seed: u64, ships: &mut [Ship]) -> &mut Ship {
    let selection = get_seeded_random_number(seed, 0, ships.len() as u64);
    ships.get_mut(selection as usize).expect("No hostile ships")
}

pub fn attack_hostile_target_ai(seed: u64, attacking_ships: &[Ship], defending_ship: &mut Ship) {
    for ship in attacking_ships {
        let random_weapon_selection = get_seeded_random_number(seed, 0, 2);
        attack_hostile_target(random_weapon_selection as u32, ship, defending_ship)
    }
}

pub fn attack_hostile_target(
    weapon_selection: u32,
    attacking_ship: &Ship,
    defending_ship: &mut Ship,
) {
    match weapon_selection {
        0 => attacking_ship.fire_phasers(defending_ship),
        1 => attacking_ship.fire_torpedoes(defending_ship),
        u32::MAX => panic!(),
        2_u32..=4294967294_u32 => panic!(),
    };
}

#[allow(dead_code)]
fn check_fight_over() -> bool {
    false
}

#[cfg(test)]
mod combat_should {
    use super::*;

    #[test]
    #[ignore]
    fn allow_combat_to_end_when_all_hostiles_are_destroyed() {
        let _player = player_ship_fixture();
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

    fn hostile_ships_fixture() -> Vec<Ship> {
        vec![Ship::create_ship(
            'K',
            "IKS Kang".to_string(),
            "Klingon Empire".to_string(),
            "Bird of Prey".to_string(),
        )]
    }
}
