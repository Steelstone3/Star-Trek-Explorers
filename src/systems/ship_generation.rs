use super::random_generation::generate_random_value_from_range_u8;
use crate::{
    components::ship::name::faction_name::FactionName,
    entities::{
        game::Game,
        ships::{federation_ship::FederationShip, klingon_ship::KlingonShip},
    },
};

pub fn generate_ships(game: &mut Game, faction_name: FactionName, seed: u64) {
    let quantity = generate_random_value_from_range_u8(seed, 1, 15);
    let mut index = 0;

    match faction_name {
        FactionName::Federation => {
            let mut ships = Vec::new();

            while quantity > index {
                ships.push(FederationShip::default());
                index += 1;
            }

            game.federation_ships = ships;
        }
        FactionName::KlingonEmpire => {
            let mut ships = Vec::new();

            while quantity > index {
                ships.push(KlingonShip::default());
                index += 1;
            }

            game.klingon_ships = ships;
        }
    }
}

#[cfg(test)]
mod ship_generation_should {
    use super::*;
    use crate::entities::game::Game;

    #[test]
    fn be_able_to_generate_federation_ships() {
        // Given
        let seed = 0;
        let quantity = 12;
        let mut game = Game::default();

        // When
        generate_ships(&mut game, FactionName::Federation, seed);

        // Then
        assert_eq!(quantity, game.federation_ships.len());
        assert_eq!(0, game.klingon_ships.len());
    }

    #[test]
    fn be_able_to_generate_klingon_ships() {
        // Given
        let seed = 0;
        let quantity = 12;
        let mut game = Game::default();

        // When
        generate_ships(&mut game, FactionName::KlingonEmpire, seed);

        // Then
        assert_eq!(quantity, game.klingon_ships.len());
        assert_eq!(0, game.federation_ships.len());
    }
}
