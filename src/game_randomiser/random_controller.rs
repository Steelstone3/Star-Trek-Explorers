use rand::{rngs::StdRng, Rng, SeedableRng};

pub const RANDOM_LOWER_RANGE: u64 = 1;
pub const RANDOM_UPPER_RANGE: u64 = 999999;

pub fn get_seeded_random_name(available_names: &[&str], seed: u64) -> String {
    let index = get_seeded_random_number(seed, 0, available_names.len() as u64);

    available_names[index as usize].to_string()
}

pub fn get_seeded_random_number(seed: u64, lower_range: u64, upper_range: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen_range(lower_range..upper_range)
}

pub fn get_random_number_from_range(lower_range: u64, upper_range: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_range..upper_range)
}

#[cfg(test)]
mod federation_starship_names_should {
    use super::*;

    #[test]
    fn get_a_seeded_random_number() {
        let value = get_seeded_random_number(6969, 1000, 999999);

        assert_eq!(605599, value);
    }

    #[test]
    fn get_a_seeded_random_name() {
        let available_names = ["Challenger", "Discovery"];
        
        let value = get_seeded_random_name(&available_names, 999999);

        assert_eq!("Challenger", value);
    }
}
