use crate::models::universe::planet::Planet;
use crate::presenters::presenter::write;

#[allow(dead_code)]
pub fn display_scanned_planet(planet: Planet) {
    write(planet.display_symbol.to_string());
    write(planet.name);
    write(planet.classification);
}