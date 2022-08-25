use crate::models::{federation_ship::FederationShip, klingon_ship::KlingonShip};

pub struct Game <State>{
    pub state: State,
    pub player_ship: FederationShip,
    pub federation_ships: Vec<FederationShip>,
    pub klingon_ships: Vec<KlingonShip>,
}