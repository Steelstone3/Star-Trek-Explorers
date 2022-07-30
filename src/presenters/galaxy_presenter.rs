use crate::models::universe::galaxy::Galaxy;
use crate::presenters::presenter::write;

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