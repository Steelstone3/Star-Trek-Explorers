use crate::names::klingon_starship_names::KLINGON_STARSHIP_NAMES;
use crate::names::random::get_seeded_random_number;
use crate::names::random::get_unique_name;
use crate::ship::ship_model::Ship;

const SERIAL_NUMBER_LOWER_BOUND: u64 = 100;
const SERIAL_NUMBER_UPPER_BOUND: u64 = 9999;

pub fn create_npc(used_names: &[&str], seed: u64) -> Ship {
    return Ship {
        display_symbol: 'K',
        name: format!(
            "IKS-{} {}",
            get_seeded_random_number(
                seed,
                SERIAL_NUMBER_LOWER_BOUND,
                SERIAL_NUMBER_UPPER_BOUND
            ),
            get_unique_name(&KLINGON_STARSHIP_NAMES, used_names, seed)
        ),
        faction: "Klingon Empire".to_string(),
        class: "Bird of Prey".to_string(),
    };
}

#[cfg(test)]
mod bird_of_prey_factory_should {
    use super::*;

    #[test]
    fn create_an_npc_bird_of_prey_starship() {
        let starship = create_npc(&["Chang"], 45);

        assert_eq!('K', starship.display_symbol);
        assert_eq!("IKS-3159 Amak", starship.name);
        assert_eq!("Klingon Empire", starship.faction);
        assert_eq!("Bird of Prey", starship.class);
    }
}
