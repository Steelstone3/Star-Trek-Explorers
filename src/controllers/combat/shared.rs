use super::players_turn::player_turn;
use crate::{
    controllers::{
        encounters::{
            federation_encounter::generate_federation_ships,
            klingon_encounter::generate_klingon_ships,
        },
        random_number::get_random_number_from_range,
    },
    models::ship::Ship,
};

pub const PHASERS_WEAPON_SELECTION: &str = "Phasers";
pub const TORPEDOS_WEAPON_SELECTION: &str = "Torpedoes";

pub fn enter_combat(player: &mut Ship) {
    let _federation_ships = generate_federation_ships(get_random_number_from_range(0, 5) as u32);
    let mut hostile_ships = generate_klingon_ships(get_random_number_from_range(1, 10) as u32);

    while player.systems.hull_integrity != 0 || !hostile_ships.is_empty() {
        player_turn(player, &mut hostile_ships);
        // federation_turn();
        // klingon_turn();
    }
}

pub fn damage_ship(weapon_selection: &str, attacking_ship: &Ship, defending_ship: &mut Ship) {
    match weapon_selection {
        PHASERS_WEAPON_SELECTION => attacking_ship.fire_phasers(defending_ship),
        TORPEDOS_WEAPON_SELECTION => attacking_ship.fire_torpedoes(defending_ship),
        _ => panic!("Invalid option"),
    }

    println!("{}", defending_ship.defensive_capabilities())
}

// pub fn federation_turn() {}

// pub fn klingon_turn() {}

#[cfg(test)]
mod combat_should {
    use super::*;

    #[test]
    fn allowing_attacking_ship_to_damage_defending_ship() {
        let attacking_ship = Ship::create_federation_ship();
        let mut defending_ship = Ship::create_klingon_ship();

        damage_ship(
            PHASERS_WEAPON_SELECTION,
            &attacking_ship,
            &mut defending_ship,
        );
        damage_ship(
            TORPEDOS_WEAPON_SELECTION,
            &attacking_ship,
            &mut defending_ship,
        );

        assert_eq!(74, defending_ship.systems.shield_strength);
        assert_eq!(100, defending_ship.systems.hull_integrity);
    }
}
