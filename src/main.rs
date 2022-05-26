use crate::factories::starship::federation_starship::constitution_class::create_federation_constitution_class;
use crate::models::character::CrewMember;
use models::ship::starship::Starship;
use crate::factories::starship::klingon_starship::bird_of_prey::create_klingon_bird_of_prey;

mod displayer;
mod models;
mod controllers;
mod factories;

fn main() {
    start_game();
}

fn start_game() {
    let player_character = create_character();
    let player_starship = create_player_starship();
    let enemy_starship = create_enemy_starship();

    displayer::write(player_character.details());
    displayer::write(player_starship.meta_data.details());

    //TODO pass the objects into the game loop and work out some game
    displayer::write("enemy encountered!".to_string());
    displayer::write(enemy_starship.meta_data.details());

    //run_game_loop();
}

fn create_enemy_starship() -> Starship {
    return create_klingon_bird_of_prey()
}

// fn run_game_loop() {
//TODO while loop with quit condition here
//
//     models::location::generate();
//end_game();
// }

// fn end_game() {}

fn create_character() -> CrewMember {
    let name = displayer::read_string("Enter crew member name:");
    let rank = displayer::read_string("Enter crew member rank:");
    let role = displayer::read_string("Enter crew member role:");
    return CrewMember::create_crew_member(name, rank, role);
}

fn create_player_starship() -> Starship {
    let name = displayer::read_string("Enter starship name USS:");
    let suffix = displayer::read_string("Enter starship suffix:");
    let serial_number = displayer::read_string("Enter starship serial number:");
    return create_federation_constitution_class(name, suffix, serial_number);
}