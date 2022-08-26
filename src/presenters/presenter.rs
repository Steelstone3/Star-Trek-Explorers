use inquire::Select;

#[allow(dead_code)]
pub fn menu_of(message: &str, menu_items: Vec<String>) -> String {
    Select::new(message, menu_items).prompt().unwrap()
}
