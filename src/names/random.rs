use rand::{Rng, SeedableRng, rngs::StdRng};

pub fn get_unique_name(avaliable_names: &[&str], used_names: &[&str]) -> String {
    let mut names = avaliable_names.to_vec();
    names.retain(|e| !used_names.contains(e));

    if names.len() > 0 {
        let name = &names[rand::thread_rng().gen_range(0..names.len())];
        return name.to_string();
    }

    return String::from("Jerald");
}

pub fn get_seeded_random_number(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    return rng.gen_range(1000..999999);
}

#[cfg(test)]
mod federation_starship_names_should {
    use super::*;

    const AVALIABLE_NAMES: [&str; 3] = ["Enterprise", "Challenger", "Discovery"];

    #[test]
    fn get_a_random_and_unique_name() {
        let used_names = vec!["Challenger", "Discovery"];
        let starship_name = get_unique_name(&AVALIABLE_NAMES.to_vec(), &used_names);

        assert_eq!("Enterprise", starship_name);
    }

    #[test]
    fn all_unique_names_taken() {
        let used_names = vec!["Enterprise", "Challenger", "Discovery"];
        let starship_name = get_unique_name(&AVALIABLE_NAMES.to_vec(), &used_names);

        assert_eq!("Jerald", starship_name);
    }

    #[test]
    fn get_a_seeded_random_number() {
        let value = get_seeded_random_number(6969);

        assert_eq!(605599, value);
    }
}
