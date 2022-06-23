use crate::names::klingon_starship_names::KLINGON_STARSHIP_NAMES;
use crate::names::random::get_seeded_random_name;
use crate::names::random::get_seeded_random_number;
use crate::ship::ship_model::Ship;

const SERIAL_NUMBER_LOWER_BOUND: u64 = 100;
const SERIAL_NUMBER_UPPER_BOUND: u64 = 9999;

pub fn create_npc(seed: u64) -> Ship {
    return Ship::create(
        'K',
        format!(
            "IKS-{} {}",
            get_seeded_random_number(seed, SERIAL_NUMBER_LOWER_BOUND, SERIAL_NUMBER_UPPER_BOUND),
            get_seeded_random_name(KLINGON_STARSHIP_NAMES.to_vec(), seed)
        )
        .as_str(),
        "Klingon Empire",
        "Bird of Prey",
    );
}

#[cfg(test)]
mod bird_of_prey_factory_should {
    use super::*;

    #[test]
    fn create_an_npc_bird_of_prey_starship() {
        let starship = create_npc(45);

        assert_eq!('K', starship.display_symbol);
        assert_eq!("IKS-3159 Chang", starship.name);
        assert_eq!("Klingon Empire", starship.faction);
        assert_eq!("Bird of Prey", starship.class);
    }
}
