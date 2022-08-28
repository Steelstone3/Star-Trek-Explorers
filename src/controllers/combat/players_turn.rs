use crate::{
    models::ship::Ship,
    presenters::presenter::{menu_of, select_ship},
};

use super::shared::{
    damage_ship, remove_critically_damaged_ships, PHASERS_WEAPON_SELECTION,
    TORPEDOS_WEAPON_SELECTION,
};

pub fn player_turn(player: &mut Ship, hostile_ships: &mut Vec<Ship>) {
    player.overall_capabilities();
    let mut hostile_ship_target = select_hostile_target(hostile_ships.to_owned());
    select_weapon_system(player, &mut hostile_ship_target);
    remove_critically_damaged_ships(&mut hostile_ship_target);
}

fn select_hostile_target(hostile_ships: Vec<Ship>) -> Ship {
    select_ship("\nSelect Hostile:", hostile_ships)
}

fn select_weapon_system(player: &Ship, hostile_ship_target: &mut Ship) {
    damage_ship(
        menu_of(
            "Select Weapon:",
            vec![
                PHASERS_WEAPON_SELECTION.to_string(),
                TORPEDOS_WEAPON_SELECTION.to_string(),
            ],
        )
        .as_str(),
        player,
        hostile_ship_target,
    );
}
