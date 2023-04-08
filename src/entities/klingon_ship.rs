use crate::{
    components::ship::{
        damage::DamageTaker,
        hull::Hull,
        name::{faction_name::FactionName, klingon_ship_name::KlingonShipName},
        phaser::Phaser,
        shield::Shield,
        torpedo::Torpedo,
    },
    systems::ship_identifier_generation::{generate_random_identifier, generate_seed},
};
use rand::random;
use rand_derive2::RandGen;

use super::ship::Ship;

#[derive(RandGen)]
pub struct KlingonShip {
    name: KlingonShipName,
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
            name: random(),
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

impl Ship for KlingonShip {
    fn display_ship_name(&self) {
        println!(
            "| Name: {} {} | Faction: {} |",
            self.ship_identifier, self.name, self.faction
        )
    }
}

impl DamageTaker for KlingonShip {
    fn take_damage(&mut self, damage: u8) {
        todo!()
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
