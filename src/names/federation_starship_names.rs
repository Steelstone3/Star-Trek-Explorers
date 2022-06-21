use rand::Rng;

pub fn get_unique_name(used_names: &[&str]) -> String {
    let mut federation_starship_names = vec!["Enterprise", "Challenger", "Discovery"];
    
    federation_starship_names.retain(|e| !used_names.contains(e));
    if federation_starship_names.len() > 0  {
        let ship_name = &federation_starship_names[rand::thread_rng().gen_range(0..federation_starship_names.len())];

        return ship_name.to_string();
    }

    return String::from("Jerald");
}

#[cfg(test)]
mod federation_starship_names_should {
    use super::*;

    #[test]
    fn get_a_random_and_unique_name() {
        let names = vec!["Challenger", "Discovery"];
        let starship_name = get_unique_name(&names);

        assert_eq!("Enterprise", starship_name);
    }

    #[test]
    fn all_unique_names_taken() {
        let names = vec!["Enterprise",  "Challenger", "Discovery"];
        let starship_name = get_unique_name(&names);

        assert_eq!("Jerald", starship_name);
    }
}