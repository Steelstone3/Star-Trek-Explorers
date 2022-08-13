use crate::models::ships::ship::Ship;
use crate::presenters::presenter::read_numeric_u32;
use crate::presenters::presenter::write;

pub fn choose_hostile_target(hostiles: &[Ship]) -> &Ship {
    let mut index = 0;
    write("Captain hostile ships detected!".to_string());
    for hostile in hostiles {
        let hostile_list_element: String = format!("{}, {}", index, hostile.name);
        write(hostile_list_element);

        index += 1;
    }

    let selection = read_numeric_u32(
        "Captain select target hostile ship",
        0,
        hostiles.len() as u32,
    );

    hostiles.get(selection as usize).unwrap()
}
