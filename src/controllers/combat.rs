use super::{encounters::{
    federation_encounter::generate_federation_ships, klingon_encounter::generate_klingon_ships,
}, random_number::get_random_number_from_range};
use crate::{
    models::{federation_ship::FederationShip, klingon_ship::KlingonShip}, presenters::presenter::menu_of,
};

pub fn enter_combat(player: &mut FederationShip) {
    let _federation_ships = generate_federation_ships(get_random_number_from_range(0, 5) as u32);
    let klingon_ships = generate_klingon_ships(get_random_number_from_range(1, 10) as u32);

    player_turn(player, klingon_ships);
    federation_turn();
    klingon_turn();
}

fn player_turn(_player: &FederationShip, klingon_ships: Vec<KlingonShip>) {
    
    let mut klingon_ship_names=Vec::new();

    for klingon_ship in klingon_ships {
        klingon_ship_names.push(klingon_ship.credentials())
    }

    menu_of("Select Hostile:", klingon_ship_names);
}

fn federation_turn() {}

fn klingon_turn() {}
