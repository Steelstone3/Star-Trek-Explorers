use super::{
    encounters::{
        federation_encounter::generate_federation_ships, klingon_encounter::generate_klingon_ships,
    },
    random_number::get_random_number_from_range,
};
use crate::{
    models::ship::Ship,
    presenters::presenter::{menu_of, select_ship},
};

pub fn enter_combat(player: &mut Ship) {
    let _federation_ships = generate_federation_ships(get_random_number_from_range(0, 5) as u32);
    let hostile_ships = generate_klingon_ships(get_random_number_from_range(1, 10) as u32);

    //|| !klingon_ships.is_empty()
    // while player.systems.hull_integrity != 0 {
    player_turn(player, hostile_ships);
    // federation_turn();
    // klingon_turn();
    // }
}

fn player_turn(_player: &Ship, hostile_ships: Vec<Ship>) {
    let phasers = "Phasers".to_string();
    let torpedoes = "Torpedoes".to_string();

    let _hostile_ship_target = select_ship("\nSelect Hostile:", hostile_ships);
    let weapon_selection = menu_of("Select Weapon:", vec![phasers, torpedoes]);

    match weapon_selection.as_str() {
        "Phasers" => todo!(),
        "Torpedoes" => todo!(),
        _ => panic!("Invalid option"),
    }
}

// fn federation_turn() {}

// fn klingon_turn() {}
