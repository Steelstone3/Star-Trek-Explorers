use crate::{models::ships::{ship::Ship, klingon_ship_factory}, controllers::game::game_randomiser::generate_seed};

pub fn generate_klingon_ships(quantity: u32) -> Vec<Ship>{
    let mut klingon_ships = vec![];

    for _ in 0..quantity {
        klingon_ships.push(klingon_ship_factory::create_klingon_ship(generate_seed()));
    }

    klingon_ships
}

#[cfg(test)]
mod klingon_encounter_should {
    use super::*;

    #[test]
    fn generate_some_klingon_ships() {
        let quantity = 10;
        let klingon_ships = generate_klingon_ships(quantity);

        assert_eq!(quantity as usize, klingon_ships.len());
    }
}