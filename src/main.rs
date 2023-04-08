use entities::{ship::Ship, federation_ship::FederationShip};

mod components;
mod entities;
mod presenters;
mod systems;

fn main() {
    let ship = FederationShip::default();

    ship.display_ship_name();
    ship.display_ship_name();
}
