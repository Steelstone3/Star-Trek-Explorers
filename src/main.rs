use crate::game::game_controller::{display_galaxy, player_create_character, scan_ship};
use crate::ship::ship_factory::Ship;
use crate::universe::galaxy::Galaxy;

mod names;
mod game_randomiser;
mod ship;
mod universe;
mod game;

fn main() {
    let galaxy = Galaxy::create_galaxy(100);
    display_galaxy(galaxy);

    let player_ship = player_create_character();
    scan_ship(player_ship);
}
