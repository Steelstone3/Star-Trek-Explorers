// pub fn calculate_damage(seed: u64, weapon_name: String, attacking_ship: Ship) -> u8 {
//     attacking_ship.calculate_damage_from_weapon(seed, weapon_name)
// }

// pub fn apply_damage (damage: u8, defending_ship: &mut dyn Ship) { //-> dyn Ship {
//     defending_ship.take_damage_from_hostile_ship(damage);
//     // defending_ship
// }

#[cfg(test)]
mod combat_should {

    // #[rstest]
    // #[case(0, 91)]
    // #[case(4545, 93)]
    // fn be_able_to_let_plater_damage_shields(#[case] seed: u64, #[case] remaining_shield: u8) {
    //     // Given
    //     let attacking_ship = FederationShip::default();
    //     let mut defending_ship = KlingonShip::default();
    //     let weapon_name = attacking_ship.ship_systems.phaser.to_string();

    //     // When
    //     apply_damage(seed, weapon_name, &attacking_ship, &mut defending_ship);

    //     // Then
    //     assert_eq!(remaining_shield, defending_ship.ship_systems.shield.current);
    //     assert_eq!(100, defending_ship.ship_systems.hull.current);
    // }

    // #[rstest]
    // #[case(0, 20)]
    // #[case(4545, 60)]
    // fn be_able_to_let_plater_damage_hull(#[case] seed: u64, #[case] remaining_hull: u8) {
    //     // Given
    //     let attacking_ship = FederationShip::default();
    //     let mut defending_ship = KlingonShip::default();
    //     let weapon_name = attacking_ship.ship_systems.phaser.to_string();

    //     // When
    //     for _ in 0..20 {
    //         apply_damage(
    //             seed,
    //             weapon_name.to_string(),
    //             &attacking_ship,
    //             &mut defending_ship,
    //         );
    //     }

    //     // Then
    //     assert_eq!(0, defending_ship.ship_systems.shield.current);
    //     assert_eq!(remaining_hull, defending_ship.ship_systems.hull.current);
    // }
}
