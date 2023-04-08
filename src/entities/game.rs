use super::{
    ships::{federation_ship::FederationShip, klingon_ship::KlingonShip},
    world::World,
};

#[derive(Default)]
pub struct Game {
    pub player_ship: FederationShip,
    pub federation_ships: Vec<FederationShip>,
    pub klingon_ships: Vec<KlingonShip>,
    pub world: World,
}

#[cfg(test)]
mod game_should {
    #[test]
    fn create_a_default_game() {}
}
