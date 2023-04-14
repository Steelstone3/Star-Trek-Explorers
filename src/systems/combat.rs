use crate::entities::ship::Ship;

pub fn combat_turn(
    seed: u64,
    weapon_name: String,
    attacking_ship: &Ship,
    defending_ship: &mut Ship,
) {
    print_turn(attacking_ship, defending_ship);
    apply_damage(seed, weapon_name, attacking_ship, defending_ship);
    defending_ship.ship_systems.display_defensive_capabilities();
}

fn print_turn(attacking_ship: &Ship, defending_ship: &Ship) {
    print!("Attacking Ship: ");
    attacking_ship.ship_identification.display_ship_name(
        &attacking_ship.name.to_string(),
        &attacking_ship.class.to_string(),
    );
    print!("Defending Ship: ");
    defending_ship.ship_identification.display_ship_name(
        &defending_ship.name.to_string(),
        &defending_ship.class.to_string(),
    )
}

fn apply_damage(seed: u64, weapon_name: String, attacking_ship: &Ship, defending_ship: &mut Ship) {
    let damage = attacking_ship
        .ship_systems
        .calculate_damage_from_weapon(seed, weapon_name);
    println!("{}", damage);
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
    fn be_able_to_let_plater_damage_shields(#[case] seed: u64, #[case] remaining_shield: u8) {
        // Given
        let attacking_ship = Ship::new_federation_ship();
        let mut defending_ship = Ship::new_klingon_ship();
        let weapon_name = attacking_ship.ship_systems.phaser.to_string();

        // When
        apply_damage(seed, weapon_name, &attacking_ship, &mut defending_ship);

        // Then
        assert_eq!(remaining_shield, defending_ship.ship_systems.shield.current)
    }

    #[rstest]
    #[case(0, 20)]
    #[case(4545, 60)]
    fn be_able_to_let_plater_damage_hull(#[case] seed: u64, #[case] remaining_hull: u8) {
        // Given
        let attacking_ship = Ship::new_federation_ship();
        let mut defending_ship = Ship::new_klingon_ship();

        // When
        for _ in 0..20 {
            apply_damage(
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
}
