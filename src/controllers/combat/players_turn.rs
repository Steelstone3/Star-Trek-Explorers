use crate::{
    models::ship::Ship,
    presenters::presenter::{menu_of, select_ship},
};

use super::shared::{damage_ship, PHASERS_WEAPON_SELECTION, TORPEDOS_WEAPON_SELECTION};

pub fn player_turn(player: &mut Ship, hostile_ships: &mut Vec<Ship>) {
    player.overall_capabilities();
    let mut hostile_ship_target = select_hostile_target(hostile_ships.to_vec());
    select_weapon_system(player, &mut hostile_ship_target);

    if hostile_ship_target.systems.hull_integrity == 0 {
       
        // let mut xs = vec![1, 2, 3];
        // let some_x = 2;
        hostile_ships.retain(|x| x != &hostile_ship_target);
        // println!("{:?}", hostile_ships); // prints [1, 3]
    }
}

fn select_hostile_target(hostile_ships: Vec<Ship>) -> Ship {
    select_ship("\nSelect Hostile:", hostile_ships)
}

fn select_weapon_system(player: &mut Ship, hostile_ship_target: &mut Ship) {
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
