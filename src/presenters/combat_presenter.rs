use crate::models::ships::ship::Ship;
use crate::presenters::presenter::read_numeric_u32;
use crate::presenters::presenter::write;

pub fn display_ship_status(ship: &Ship) {
    write(Ship::aggressive_ship_scan(ship));
}

pub fn display_fleet_status(fleet: &[Ship]) {
    write(Ship::aggressive_ships_scan(fleet));
}

pub fn choose_hostile_target(hostiles: &mut [Ship]) -> &mut Ship {
    display_target_ships(hostiles);
    select_target_ship(hostiles)
}

pub fn choose_weapon_system() -> u32 {
    write("0. Fire phasers".to_string());
    write("1. Fire torpedoes".to_string());
    read_numeric_u32("Captain fire torpedoes?", 0, 1)
}

pub fn defeat_message_hostiles() {
    write("No hostile threats detected".to_string());
}

pub fn defeat_message_allies() {
    write("No help from your allies".to_string());
}

fn display_target_ships(hostiles: &mut [Ship]) {
    let mut index = 0;

    write("Captain hostile ships detected!\n".to_string());
    hostiles.iter_mut().for_each(|hostile| {
        let hostile_ship: String = format!("{}. {} Shield strength: {} Hull integrity: {}", index, hostile.name, hostile.shield_strength, hostile.hull_integrity);
        write(hostile_ship);

        index += 1;
    });
}

fn select_target_ship(hostiles: &mut [Ship]) -> &mut Ship {
    let selection = read_numeric_u32(
        "\nSir which ship should we target?",
        0,
        hostiles.len() as u32,
    );

    hostiles.get_mut(selection as usize).expect("No hostile ships")
}