use crate::{models::ships::{ship::Ship, federation_ship_factory}, controllers::game::game_randomiser::generate_seed};

pub fn generate_federation_ships(quantity: u32) -> Vec<Ship>{
    let mut federation_ships = vec![];

    for _ in 0..quantity {
        federation_ships.push(federation_ship_factory::create_federation_ship(generate_seed()));
    }

    federation_ships
}

#[cfg(test)]
mod klingon_encounter_should {
    use super::*;

    #[test]
    fn generate_some_klingon_ships() {
        let quantity = 10;
        let federation_ships = generate_federation_ships(quantity);

        assert_eq!(quantity as usize, federation_ships.len());
    }
}