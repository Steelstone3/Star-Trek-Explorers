use crate::models::federation_ship::FederationShip;

pub fn generate_federation_ships(quantity: u32) -> Vec<FederationShip>{
    let mut federation_ships = vec![];

    for _ in 0..quantity {
        federation_ships.push(FederationShip::default());
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