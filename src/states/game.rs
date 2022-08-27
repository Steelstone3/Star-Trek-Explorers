use crate::models::ship::Ship;

pub struct Game <State>{
    pub state: State,
    pub player_ship: Ship,
    pub ally_ships: Vec<Ship>,
    pub neutral_ships: Vec<Ship>,
    pub hostile_ships: Vec<Ship>,
}