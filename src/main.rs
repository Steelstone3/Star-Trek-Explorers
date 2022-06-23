use crate::ship::ship_model::Ship;
use crate::names::random::get_random_number_from_range;
use crate::ship::federation::sovereign_class_factory::create_player;
use crate::world::galaxy::Galaxy;

mod displayer;
mod names;
mod ship;
mod world;

fn main() {
    let galaxy = Galaxy::generate(100);

    for star_system in galaxy.star_systems {
        displayer::write(star_system.display_symbol.to_string());
        displayer::write(star_system.name);
        displayer::write(star_system.classification);
        for planet in star_system.planets {
            displayer::write(planet.display_symbol.to_string());
            displayer::write(planet.name);
            displayer::write(planet.classification);
        }
    }

    let ship_name = displayer::read_string("Name your starship:");
    let player_ship = create_player(ship_name.as_str(), get_random_number_from_range(1, 1000000));
    Ship::scan_ship(player_ship);
}
