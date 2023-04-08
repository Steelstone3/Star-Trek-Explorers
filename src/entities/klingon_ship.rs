use crate::{
    components::ship::{
        hull::Hull, name::faction_name::FactionName, phaser::Phaser, shield::Shield,
        torpedo::Torpedo,
    },
    systems::ship_identifier_generation::{generate_random_identifier, generate_seed},
};
use rand_derive2::RandGen;

#[derive(RandGen)]
pub struct KlingonShip {
    ship_identifier: String,
    faction: FactionName,
    shield: Shield,
    hull: Hull,
    phaser: Phaser,
    torpedo: Torpedo,
}

impl Default for KlingonShip {
    fn default() -> Self {
        Self {
            ship_identifier: generate_random_identifier(
                generate_seed(),
                FactionName::KlingonEmpire,
            ),
            faction: FactionName::KlingonEmpire,
            shield: Shield::default(),
            hull: Hull::default(),
            phaser: Phaser::default(),
            torpedo: Torpedo::default(),
        }
    }
}

#[cfg(test)]
mod klingon_ship_should {
    use crate::components::ship::{name::faction_name::FactionName, shield::Shield};

    use super::*;

    #[test]
    fn create_a_default_ship() {
        // Given
        let ship = KlingonShip::default();

        // Then
        assert_ne!(String::default(), ship.ship_identifier);
        assert_eq!(FactionName::KlingonEmpire, ship.faction);
        assert_eq!(Shield::default(), ship.shield);
        assert_eq!(Hull::default(), ship.hull);
        assert_eq!(Phaser::default(), ship.phaser);
        assert_eq!(Torpedo::default(), ship.torpedo);
    }
}
