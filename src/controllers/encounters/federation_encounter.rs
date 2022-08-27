use crate::models::ship::Ship;

pub fn generate_federation_ships(quantity: u32) -> Vec<Ship> {
    let mut federation_ships = vec![];

    for _ in 0..quantity {
        federation_ships.push(Ship::create_federation_ship());
    }

    federation_ships
}

#[cfg(test)]
mod federation_encounter_should {
    use super::*;

    #[test]
    fn generate_a_specific_quantity_of_federation_ships() {
        let quantity = 10;
        let federation_ships = generate_federation_ships(quantity);

        assert_eq!(quantity as usize, federation_ships.len());
    }
}
