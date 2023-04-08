use super::{
    ships::{federation_ship::FederationShip, klingon_ship::KlingonShip},
    world::World,
};

pub struct Game {
    pub player_ship: FederationShip,
    pub federation_ships: Vec<FederationShip>,
    pub klingon_ships: Vec<KlingonShip>,
    pub world: World,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_ship: FederationShip::default(),
            federation_ships: Default::default(),
            klingon_ships: Default::default(),
            world: World::default(),
        }
    }
}

#[cfg(test)]
mod game_should {
    #[test]
    fn create_a_default_game() {}
}
