use super::{
    encounters::{
        federation_encounter::generate_federation_ships, klingon_encounter::generate_klingon_ships,
    },
    random_number::get_random_number_from_range,
};
use crate::{
    models::{federation_ship::FederationShip, klingon_ship::KlingonShip},
    presenters::presenter::{select_klingon_ship, menu_of},
};

pub fn enter_combat(player: &mut FederationShip) {
    let _federation_ships = generate_federation_ships(get_random_number_from_range(0, 5) as u32);
    let klingon_ships = generate_klingon_ships(get_random_number_from_range(1, 10) as u32);

    //|| !klingon_ships.is_empty()
    // while player.systems.hull_integrity != 0 {
    player_turn(player, klingon_ships);
    // federation_turn();
    // klingon_turn();
    // } while  {
    // }
}

fn player_turn(player: &FederationShip, klingon_ships: Vec<KlingonShip>) {
    let phasers = "Phasers".to_string();
    let torpedoes = "Torpedoes".to_string();
    
    let klingon_ship = select_klingon_ship("\nSelect Hostile:", klingon_ships);
    let weapon_selection = menu_of("Select Weapon:", vec![phasers, torpedoes]);

    match weapon_selection.as_str() {
        "Phasers" => fire_phasers_at_klingon_ship(player, klingon_ship),
        "Torpedoes" => fire_torpedoes_at_klingon_ship(player, klingon_ship),
        _ => panic!("Invalid option")
    }
}

// fn federation_turn() {}

// fn klingon_turn() {}

fn fire_phasers_at_klingon_ship(_player: &FederationShip, _klingon_ship: KlingonShip) {
    todo!()
}

fn fire_torpedoes_at_klingon_ship(_player: &FederationShip, _klingon_ship: KlingonShip) {
    todo!()
}