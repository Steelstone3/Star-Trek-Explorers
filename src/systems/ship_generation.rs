use crate::{components::ship::names::faction_name::FactionName, entities::ship::Ship};

pub fn generate_ships(faction_name: FactionName) -> [Ship; 10] {
    let federation_ships: [Ship; 10] = [
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
        Ship::new_federation_ship(),
    ];

    let klingon_ships: [Ship; 10] = [
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
        Ship::new_klingon_ship(),
    ];

    match faction_name {
        FactionName::Federation => federation_ships,
        FactionName::KlingonEmpire => klingon_ships,
    }
}

#[cfg(test)]
mod ship_generation_should {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn be_able_to_generate_federation_ships() {
        // Given
        let quantity = 10;
        let stop_watch = Instant::now();

        // When
        let ships = generate_ships(FactionName::Federation);

        // Then
        assert!(Duration::from_millis(50) > stop_watch.elapsed());
        assert_eq!(quantity, ships.len());
    }

    #[test]
    fn be_able_to_generate_klingon_ships() {
        // Given
        let quantity = 10;
        let stop_watch = Instant::now();

        // When
        let ships = generate_ships(FactionName::KlingonEmpire);

        // Then
        assert!(Duration::from_millis(50) > stop_watch.elapsed());
        assert_eq!(quantity, ships.len());
    }
}
