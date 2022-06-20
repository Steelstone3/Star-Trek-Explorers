use crate::ship::ship_model::Ship;

pub fn create_npc() -> Ship {
    return Ship {
        display_symbol: 'K',
        name: format!("IKS {} IKS-{}", "Chang", 62711),
        faction: "Federation".to_string(),
        class: "Sovereign Class".to_string(),
    };
}

#[cfg(test)]
mod bird_of_prey_factory_should {
    use super::*;

    #[test]
    fn create_an_npc_bird_of_prey_starship() {
        let starship = create_npc();

        assert_eq!('K', starship.display_symbol);
        assert_eq!("IKS-4572 Chang", starship.name);
        assert_eq!("Klingon Empire", starship.faction);
        assert_eq!("Bird of Prey", starship.class);
    }
}
