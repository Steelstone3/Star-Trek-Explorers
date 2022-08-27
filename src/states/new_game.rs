use super::game::Game;
use crate::models::ship::Ship;

pub struct NewGame;

impl Game<NewGame> {
    pub fn new() -> Self {
        println!("\nNew game started...");
        println!("\nWelcome to the United Federation of Planets Captain. This is your first command so I expect you to take good care of your ship out there.\n");

        let game = Game {
            state: NewGame,
            player_ship: Ship::create_federation_ship(),
            ally_ships: vec![],
            neutral_ships: vec![],
            hostile_ships: vec![],
        };

        game.player_ship.overall_capabilities();

        game
    }
}
