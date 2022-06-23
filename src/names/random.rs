use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn get_seeded_random_name(avaliable_names: Vec<&str>, seed: u64) -> String {
    let index = get_seeded_random_number(seed, 0, avaliable_names.len() as u64);

    return avaliable_names[index as usize].to_string();
}

pub fn get_seeded_random_number(seed: u64, lower_range: u64, upper_range: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    return rng.gen_range(lower_range..upper_range);
}

pub fn get_random_number_from_range(lower_range: u64, upper_range: u64) -> u64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(lower_range..upper_range);
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
        let avaliable_names = vec!["Challenger", "Discovery"];
        let value = get_seeded_random_name(avaliable_names, 999999);
        assert_eq!("Challenger", value);
    }
}
