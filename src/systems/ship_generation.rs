use crate::{
    components::ship::name::faction_name::FactionName,
    entities::{
        game::Game,
        ships::{federation_ship::FederationShip, klingon_ship::KlingonShip},
    },
};

pub fn generate_ships(game: &mut Game, faction_name: FactionName, quantity: u8) {
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
        let quantity = u8::MAX;
        let mut game = Game::default();

        // When
        generate_ships(&mut game, FactionName::Federation, quantity);

        // Then
        assert_eq!(quantity as usize, game.federation_ships.len());
        assert_eq!(0, game.klingon_ships.len());
    }

    #[test]
    fn be_able_to_generate_klingon_ships() {
        // Given
        let quantity = u8::MAX;
        let mut game = Game::default();

        // When
        generate_ships(&mut game, FactionName::KlingonEmpire, quantity);

        // Then
        assert_eq!(quantity as usize, game.klingon_ships.len());
        assert_eq!(0, game.federation_ships.len());
    }
}
