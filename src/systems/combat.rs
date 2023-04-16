use crate::entities::{game::Game, ship::Ship};

pub fn start_combat(
    seed: u64,
    weapon_name: String,
    attacking_ship: &Ship,
    defending_ship: &mut Ship,
) {
    if !attacking_ship.is_in_play {
        return;
    }

    print_turn(attacking_ship, defending_ship);
    combat_turn(seed, weapon_name, attacking_ship, defending_ship);
    defending_ship.ship_systems.display_defensive_capabilities();
}

pub fn apply_damage_player(game: &mut Game, defending_ship: Ship) {
    game.player_ship = defending_ship;
}

pub fn apply_damage_klingon(game: &mut Game, defending_ship: Ship) {
    let ship_index = game.klingon_ships.iter().position(|&ship| {
        ship.ship_identification.serial_number == defending_ship.ship_identification.serial_number
    });
    game.klingon_ships[ship_index.unwrap()] = defending_ship;
}

fn print_turn(attacking_ship: &Ship, defending_ship: &Ship) {
    print!("Attacking Ship: ");
    attacking_ship.ship_identification.display_ship_name();
    print!("Defending Ship: ");
    defending_ship.ship_identification.display_ship_name()
}

fn combat_turn(seed: u64, weapon_name: String, attacking_ship: &Ship, defending_ship: &mut Ship) {
    let damage = attacking_ship
        .ship_systems
        .calculate_damage_from_weapon(seed, weapon_name);
    println!("| Damage: {} |", damage);
    // TODO is this updating as expected? No
    defending_ship
        .ship_systems
        .take_damage_from_hostile_ship(damage);
}

#[cfg(test)]
mod combat_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 91)]
    #[case(4545, 93)]
    fn be_able_to_let_player_damage_shields(#[case] seed: u64, #[case] remaining_shield: u8) {
        // Given
        let attacking_ship = Ship::new_federation_ship();
        let mut defending_ship = Ship::new_klingon_ship();
        let weapon_name = attacking_ship.ship_systems.phaser.to_string();

        // When
        combat_turn(seed, weapon_name, &attacking_ship, &mut defending_ship);

        // Then
        assert_eq!(remaining_shield, defending_ship.ship_systems.shield.current);
        assert_eq!(100, defending_ship.ship_systems.hull.current);
    }

    #[rstest]
    #[case(0, 20)]
    #[case(4545, 60)]
    fn be_able_to_let_player_damage_hull(#[case] seed: u64, #[case] remaining_hull: u8) {
        // Given
        let attacking_ship = Ship::new_federation_ship();
        let mut defending_ship = Ship::new_klingon_ship();

        // When
        for _ in 0..20 {
            combat_turn(
                seed,
                attacking_ship.ship_systems.phaser.to_string(),
                &attacking_ship,
                &mut defending_ship,
            );
        }

        // Then
        assert_eq!(0, defending_ship.ship_systems.shield.current);
        assert_eq!(remaining_hull, defending_ship.ship_systems.hull.current);
    }

    #[test]
    fn be_able_to_apply_damage_to_klingon_ship() {
        // Given
        let mut game = Game::default();
        let mut defending_ship = game.klingon_ships[0];
        defending_ship.ship_systems.shield.current = 50;
        defending_ship.ship_systems.hull.current = 50;

        // When
        apply_damage_klingon(&mut game, defending_ship);

        // Then
        assert_eq!(50, game.klingon_ships[0].ship_systems.shield.current);
        assert_eq!(50, game.klingon_ships[0].ship_systems.hull.current);
    }

    #[test]
    fn be_able_to_apply_damage_to_player() {
        // Given
        let mut game = Game::default();
        let mut defending_ship = game.player_ship;
        defending_ship.ship_systems.shield.current = 50;
        defending_ship.ship_systems.hull.current = 50;

        // When
        apply_damage_player(&mut game, defending_ship);

        // Then
        assert_eq!(50, game.player_ship.ship_systems.shield.current);
        assert_eq!(50, game.player_ship.ship_systems.hull.current);
    }
}
