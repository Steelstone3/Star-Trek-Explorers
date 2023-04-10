// TODO this takes too many arguements
// It is basically two ships,
// Ship can be converged with the original design
// For this we will require ship name and class refactoring
// This will use one type and have comments to seperate the races in the enum
// We will then likely have to use arrays or something like that in the original design
// From here hopefully everything should just "work"
// pub fn combat_turn(
//     seed: u64,
//     weapon_name: String,
//     attacking_ship_name: String,
//     attacking_ship_class: String,
//     attacking_ship_identification: ShipIdentification,
//     attacking_ship_systems: ShipSystems,
//     defending_ship_name: String,
//     defending_ship_class: String,
//     defending_ship_identification: ShipIdentification,
//     defending_ship_systems: ShipSystems,
// ) -> ShipSystems {
//     print_turn(
//         attacking_ship_identification,
//         attacking_ship_name,
//         attacking_ship_class,
//         defending_ship_identification,
//         defending_ship_name,
//         defending_ship_class,
//     );
//     apply_damage(
//         seed,
//         weapon_name,
//         attacking_ship_systems,
//         defending_ship_systems,
//     )
// }

// fn print_turn(
//     attacking_ship_identification: ShipIdentification,
//     attacking_ship_name: String,
//     attacking_ship_class: String,
//     defending_ship_identification: ShipIdentification,
//     defending_ship_name: String,
//     defending_ship_class: String,
// ) {
//     print!("Attacking Ship: ");
//     attacking_ship_identification.display_ship_name(attacking_ship_name, attacking_ship_class);
//     print!("Defending Ship: ");
//     defending_ship_identification.display_ship_name(defending_ship_name, defending_ship_class)
// }

// fn apply_damage(
//     seed: u64,
//     weapon_name: String,
//     attacking_ship_systems: ShipSystems,
//     mut defending_ship_systems: ShipSystems,
// ) -> ShipSystems {
//     let damage = attacking_ship_systems.calculate_damage_from_weapon(seed, weapon_name);
//     println!("{}", damage);
//     // TODO this isn't updating as expected
//     defending_ship_systems.take_damage_from_hostile_ship(damage);
//     defending_ship_systems
// }

#[cfg(test)]
mod combat_should {
    // use std::default;

    // use super::*;
    // use crate::entities::ships::federation_ship::FederationShip;
    // use crate::entities::ships::klingon_ship::KlingonShip;
    // use rstest::rstest;

    // #[rstest]
    // #[case(0, 91)]
    // #[case(4545, 93)]
    // fn be_able_to_let_plater_damage_shields(#[case] seed: u64, #[case] remaining_shield: u8) {
    //     // Given
    //     let attacking_ship = FederationShip::default();
    //     let defending_ship = KlingonShip::default();
    //     let weapon_name = attacking_ship.ship_systems.phaser.to_string();

    //     // When
    //     let defending_ship_systems = apply_damage(
    //         seed,
    //         weapon_name,
    //         attacking_ship.ship_systems,
    //         defending_ship.ship_systems,
    //     );

    //     // Then
    //     assert_eq!(remaining_shield, defending_ship_systems.shield.current);
    //     assert_eq!(100, defending_ship_systems.hull.current);
    // }

    // #[rstest]
    // #[case(0, 20)]
    // #[case(4545, 60)]
    // fn be_able_to_let_plater_damage_hull(#[case] seed: u64, #[case] remaining_hull: u8) {
    //     // Given
    //     let attacking_ship = FederationShip {
    //         name:rand::random(),
    //         class:rand::random(),
    //         ship_identification: default(),
    //         ship_systems: ShipSystems { shield: default(), hull: default(), phaser: Phaser{max}, torpedo: () }
    //     };
    //     let defending_ship = KlingonShip::default();
    //     let weapon_name = attacking_ship.ship_systems.phaser.to_string();

    //     // When
    //     let defending_ship_systems= apply_damage(
    //         seed,
    //         weapon_name.to_string(),
    //         attacking_ship.ship_systems,
    //         defending_ship.ship_systems,
    //     );

    //     // Then
    //     assert_eq!(0, defending_ship_systems.shield.current);
    //     assert_eq!(remaining_hull, defending_ship_systems.hull.current);
    // }
}
