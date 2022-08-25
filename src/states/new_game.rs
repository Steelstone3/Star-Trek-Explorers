use crate::models::federation_ship::FederationShip;

use super::game::Game;

pub struct NewGame;

impl Game<NewGame> {
    pub fn new() -> Self {
        println!("New game started...\n");
        
        let game = Game {
            state: NewGame,
            player_ship: FederationShip::default(),
            federation_ships: vec![],
            klingon_ships: vec![],
        };

        game.player_ship.defensive_status();

        game
    }
}