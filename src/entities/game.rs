use super::{
    world::World,
    ships::{federation_ship::FederationShip, klingon_ship::KlingonShip},
};

struct Game {
    player_ship: FederationShip,
    klingon_ships: Vec<KlingonShip>,
    world: World,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_ship: FederationShip::default(),
            klingon_ships: Default::default(),
            world: World::default(),
        }
    }
}
