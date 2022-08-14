use crate::controllers::game::game_randomiser::generate_seed;
use crate::models::ships::ship::Ship;
use crate::models::ships::federation_ship_factory::create_player_federation_ship;
use crate::presenters::presenter::read_string;
use crate::presenters::presenter::write;

pub fn player_create_character() -> Ship {
    write("Welcome to the United Federation of Planets Captain. This is your first command so I expect you to take good care of your ship out there.".to_string());
    let ship_name = read_string("Name your Federation of Planets starship:");

    return create_player_federation_ship(
        ship_name.as_str(),
        generate_seed(),
    );
}

pub fn scan_ship(ship: &Ship) {
    let scanned_ship = Ship::scan_ship(ship);
    write(scanned_ship);
}