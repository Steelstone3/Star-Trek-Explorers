use crate::models::character::CrewMember;
use crate::models::starship::Starship;

mod displayer;
mod models;
mod controllers;

fn main() {
    start_game();
}

fn start_game() {
    let player_character = create_character();
    let player_starship = create_player_starship();
    let enemy_starship = create_enemy_starship();

    displayer::write(player_character.details());
    displayer::write(player_starship.details());

    //run_game_loop();
}

fn create_enemy_starship() -> Starship {
    return Starship::create("IKS".to_string(),
                            "Chang".to_string(),
                            "of house Targ".to_string(),
                            "IKS-6773".to_string(),
                            200);
}

// fn run_game_loop() {
//     models::location::generate();
//end_game();
// }

fn end_game() {}

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
    return Starship::create_default(name, suffix, serial_number);
}