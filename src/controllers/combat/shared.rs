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

    while player.systems.hull_integrity != 0  {
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

pub fn remove_critically_damaged_ships(ship: &Ship, ships: &mut Vec<Ship>) {
    if ship.systems.hull_integrity == 0 {
        ships.retain(|x| x != ship);
    }
}

// pub fn federation_turn() {}

// pub fn klingon_turn() {}

#[cfg(test)]
mod combat_should {
    use crate::{assests::{faction_names::Faction, ship_names::ShipName, ship_classification_names::ShipClassification}, models::ship_status::ShipSystems};
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
    fn remove_ship_when_it_is_critically_damaged() {
        let mut ships = generate_federation_ships(3);
        let critically_damaged_ship = Ship{
            name: ShipName::generate_random(),
            faction: Faction::FederationOfPlanets,
            class: ShipClassification::generate_random(),
            systems: ShipSystems{
                shield_strength: 0,
                hull_integrity: 0,
                phaser_max_damage: 0,
                phaser_min_damage: 0,
                torpedo_max_damage: 0,
                torpedo_min_damage: 0,
            },
        };
        
        remove_critically_damaged_ships(&critically_damaged_ship, &mut ships);

        assert_eq!(3, ships.len());
    }
}
