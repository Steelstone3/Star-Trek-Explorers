use crate::models::ship::Ship;

pub fn generate_klingon_ships(quantity: u32) -> Vec<Ship> {
    let mut klingon_ships = vec![];

    for _ in 0..quantity {
        klingon_ships.push(Ship::create_klingon_ship());
    }

    klingon_ships
}

#[cfg(test)]
mod federation_encounter_should {
    use super::*;

    #[test]
    fn generate_a_specific_quantity_of_federation_ships() {
        let quantity = 10;
        let federation_ships = generate_klingon_ships(quantity);

        assert_eq!(quantity as usize, federation_ships.len());
    }
}
