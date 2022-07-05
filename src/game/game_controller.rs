use crate::ship::federation_ship_factory::create_player_federation_ship;
use crate::game::displayer::{write, read_string};
use crate::{Galaxy, Ship};
use crate::game_randomiser::random_controller::{get_random_number_from_range, RANDOM_LOWER_RANGE, RANDOM_UPPER_RANGE};

pub fn player_create_character() -> Ship {
    write("Welcome to the United Federation of Planets Captain. This is your first command so I expect you to take good care of your ship out there.".to_string());
    let ship_name = read_string("Name your Federation of Planets starship:");

    return create_player_federation_ship(
        ship_name.as_str(),
        get_random_number_from_range(RANDOM_LOWER_RANGE, RANDOM_UPPER_RANGE),
    );
}

pub fn display_galaxy(galaxy: Galaxy) {
    for star_system in galaxy.star_systems {
        write(star_system.display_symbol.to_string());
        write(star_system.name);
        write(star_system.classification);
        for planet in star_system.planets {
            write(planet.display_symbol.to_string());
            write(planet.name);
            write(planet.classification);
        }
    }
}

pub fn scan_ship(ship: Ship) {
    let scanned_ship = Ship::scan_ship(ship);
    write(scanned_ship);
}