use inquire::{Select, Confirm};

use crate::models::klingon_ship::KlingonShip;

pub fn confirmation(message: &str) -> bool {
    Confirm::new(message)
    .with_default(true)
    .prompt()
    .is_ok()
}

pub fn menu_of(message: &str, menu_items: Vec<String>) -> String {
    Select::new(message, menu_items).prompt().unwrap()
}

pub fn select_klingon_ship(message: &str, menu_items: Vec<KlingonShip>) -> KlingonShip {
    Select::new(message, menu_items).prompt().unwrap()
}