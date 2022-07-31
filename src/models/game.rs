use crate::models::universe::galaxy::Galaxy;
use crate::models::ships::ship::Ship;

pub struct Game {
    pub player_ship: Ship,
    pub galaxy: Galaxy,
    pub ally_ships: Vec<Ship>,
    pub neutral_ships: Vec<Ship>,
    pub hostile_ships: Vec<Ship>,
    pub game_progress:usize
}