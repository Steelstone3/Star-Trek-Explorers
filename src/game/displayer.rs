use std::io;
use std::string::String;

pub fn write(message: String) {
    println!("{}", message);
}

#[allow(dead_code)]
pub fn read_numeric_i32(message: &str, lower_bound: i32, upper_bound: i32) -> i32 {
    let mut result = -1;

    while result == -1 || result > upper_bound || result < lower_bound {
        let input = read_string(message);

        result = match input.as_str().trim().parse::<i32>() {
            Ok(result) => result,
            Err(_e) => -1,
        };
    }

    result
}

#[allow(dead_code)]
pub fn read_numeric_f32(message: &str, lower_bound: f32, upper_bound: f32) -> f32 {
    let mut result = -1.0;

    while result == -1.0 || result > upper_bound || result < lower_bound {
        let input = read_string(message);

        result = match input.as_str().trim().parse::<f32>() {
            Ok(result) => result,
            Err(_e) => -1.0,
        };
    }

    result
}

pub fn read_string(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);

    match io::stdin().read_line(&mut input) {
        Ok(_) => print!(""),
        Err(_e) => println!("{}", _e),
    };

    input
}
