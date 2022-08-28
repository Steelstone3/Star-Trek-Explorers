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

    while player.systems.hull_integrity != 0 {
        player_turn(player, &mut hostile_ships);
        // federation_turn();
        // klingon_turn();
    }
}

pub fn damage_ship(weapon_selection: &str, attacking_ship: &Ship, defending_ship: &mut Ship) {
    if !attacking_ship.critically_damaged {
        match weapon_selection {
            PHASERS_WEAPON_SELECTION => attacking_ship.fire_phasers(defending_ship),
            TORPEDOS_WEAPON_SELECTION => attacking_ship.fire_torpedoes(defending_ship),
            _ => panic!("Invalid option"),
        }

        println!("{}", defending_ship.defensive_capabilities())
    } else {
        println!(
            "{} is critically damaged...",
            attacking_ship.defensive_capabilities()
        )
    }
}

pub fn remove_critically_damaged_ships(ship: &mut Ship) {
    if ship.systems.hull_integrity == 0 {
        ship.critically_damaged = true;
    }
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

    #[test]
    fn do_not_allow_attacking_ship_to_damage_defending_ship_when_critically_damaged() {
        let mut attacking_ship = Ship::create_federation_ship();
        attacking_ship.critically_damaged = true;
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

        assert_eq!(100, defending_ship.systems.shield_strength);
        assert_eq!(100, defending_ship.systems.hull_integrity);
    }

    #[test]
    fn remove_ship_when_it_is_critically_damaged() {
        let mut critically_damaged_ship = Ship::create_federation_ship();
        critically_damaged_ship.systems.shield_strength = 0;
        critically_damaged_ship.systems.hull_integrity = 0;
        critically_damaged_ship.critically_damaged = false;

        remove_critically_damaged_ships(&mut critically_damaged_ship);

        assert_eq!(true, critically_damaged_ship.critically_damaged);
    }
}
