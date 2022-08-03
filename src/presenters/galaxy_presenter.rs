use crate::models::universe::galaxy::Galaxy;
use crate::models::universe::star_system::StarSystem;
use crate::presenters::presenter::write;

#[allow(dead_code)]
pub fn display_galaxy(galaxy: Galaxy) {
    for star_system in galaxy.star_systems {
        write(star_system.display_symbol.to_string());
        write(star_system.name);
        write(star_system.classification);
    }
}

#[allow(dead_code)]
pub fn display_next_scanned_star_system(star_system: StarSystem) {
    write("Scanning star system...".to_string());
    
    for planet in star_system.planets {
        write(planet.display_symbol.to_string());
        write(planet.name);
        write(planet.classification);
    }
}