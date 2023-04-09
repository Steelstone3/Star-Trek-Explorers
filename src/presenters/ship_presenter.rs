use super::presenter::menu_of;

pub fn select_weapon(weapon_types: Vec<String>) -> String {
    menu_of("Select Weapon:", weapon_types)
}
