use super::game::Game;
use crate::{models::federation_ship::FederationShip};

pub struct NewGame;

impl Game<NewGame> {
    pub fn new() -> Self {
        println!("\nNew game started...");
        println!("\nWelcome to the United Federation of Planets Captain. This is your first command so I expect you to take good care of your ship out there.\n");

        let game = Game {
            state: NewGame,
            player_ship: FederationShip::default(),
            federation_ships: vec![],
            klingon_ships: vec![],
        };

        game.player_ship.overall_capabilities();

        game
    }
}
