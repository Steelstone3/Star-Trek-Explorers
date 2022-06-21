use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn get_unique_name(avaliable_names: &[&str], used_names: &[&str], seed:u64) -> String {
    let mut names = avaliable_names.to_vec();
    names.retain(|e| !used_names.contains(e));

    if names.len() > 0 {
        let index = get_seeded_random_number(seed, 0, names.len() as u64);

        return names[index as usize].to_string();
    }

    return String::from("Jerald");
}

pub fn get_random_name(avaliable_names: &[&str], seed:u64) -> String {
   let index = get_seeded_random_number(seed, 0, avaliable_names.len() as u64);

    return avaliable_names[index as usize].to_string();
}

pub fn get_seeded_random_number(seed: u64, lower_range:u64, upper_range:u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    return rng.gen_range(lower_range..upper_range);
}

#[cfg(test)]
mod federation_starship_names_should {
    use super::*;

    const AVALIABLE_NAMES: [&str; 3] = ["Enterprise", "Challenger", "Discovery"];

    #[test]
    fn get_a_random_and_unique_name() {
        let used_names = vec!["Challenger", "Discovery"];
        let starship_name = get_unique_name(&AVALIABLE_NAMES.to_vec(), &used_names, 32);

        assert_eq!("Enterprise", starship_name);
    }

    #[test]
    fn get_a_seeded_random_and_unique_name() {
        let used_names = vec![];
        let starship_name = get_unique_name(&AVALIABLE_NAMES.to_vec(), &used_names, 32);

        assert_eq!("Enterprise", starship_name);
    }

    #[test]
    fn all_unique_names_taken() {
        let used_names = vec!["Enterprise", "Challenger", "Discovery"];
        let starship_name = get_unique_name(&AVALIABLE_NAMES.to_vec(), &used_names, 32);

        assert_eq!("Jerald", starship_name);
    }

    #[test]
    fn get_a_seeded_random_number() {
        let value = get_seeded_random_number(6969, 1000, 999999);

        assert_eq!(605599, value);
    }
}
