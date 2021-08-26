mod displayer;
mod models;
mod controllers;

fn main() {
    start_game();
    run_game_loop();
    end_game();
}

fn start_game() {
    create_character();
    create_starship();
}

fn run_game_loop() {
    models::location::generate();
}

fn end_game () {

}

fn create_character() {
    let name = displayer::read_string("Enter crew member name:");
    let rank = displayer::read_string("Enter crew member rank:");
    let role = displayer::read_string("Enter crew member role:");
    let player_character = models::character::create(name, rank, role);
    displayer::write(models::character::details(player_character));
}

fn create_starship() {
    let name = displayer::read_string("Enter starship name USS:");
    let player_starship = models::starship::create(name);
    displayer::write(models::starship::details(player_starship));
}