use crate::models::ships::ship::Ship;
use crate::presenters::presenter::read_numeric_u32;
use crate::presenters::presenter::write;

pub fn choose_hostile_target(hostiles: &mut [Ship]) -> &mut Ship {
    let mut index = 0;

    write("Captain hostile ships detected!".to_string());
    hostiles.iter_mut().for_each(|hostile| {
        let hostile_list_element: String = format!("{}, {}", index, hostile.name);
        write(hostile_list_element);

        index += 1;
    });

    let selection = read_numeric_u32("Captain select target hostile ship", 0, 0);

    hostiles.get_mut(selection as usize).unwrap()
}

pub fn choose_weapon_system() -> u32 {
    write("0. Fire phasers".to_string());
    write("1. Fire torpedoes".to_string());
    read_numeric_u32("Captain fire torpedoes?", 0, 1)
}

// fn select_target_ship(hostiles: &mut [Ship]) -> &mut Ship {
//     // let selection = read_numeric_u32(
//     //     "Captain select target hostile ship",
//     //     0,
//     //     0,
//     // );

//     write("Captain select target hostile ship".to_string());
//     let mut result = u32::MAX;

//     while result == u32::MAX || result > upper_bound || result < lower_bound {
//         let input = read_string(message);

//         result = match input.as_str().trim().parse::<u32>() {
//             Ok(result) => result,
//             Err(_e) => u32::MAX,
//         };
//     }

//         hostiles.get_mut(selection as usize).unwrap()
// }
