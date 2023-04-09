// use crate::models::ship::Ship;
use inquire::{Confirm, Select};

#[allow(dead_code)]
pub fn confirmation(message: &str) -> bool {
    Confirm::new(message)
        .with_default(true)
        .prompt()
        .unwrap_or_default()
}

pub fn menu_of(message: &str, menu_items: Vec<String>) -> String {
    Select::new(message, menu_items).prompt().unwrap()
}

// pub fn select_ship(message: &str, menu_items: Vec<Ship>) -> Ship {
// Select::new(message, menu_items).prompt().unwrap()
// }
