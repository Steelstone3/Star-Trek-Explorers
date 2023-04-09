use super::{hull::Hull, phaser::Phaser, shield::Shield, torpedo::Torpedo};
use rand_derive2::RandGen;

#[derive(PartialEq, Debug, RandGen)]
pub struct ShipSystems {
    pub shield: Shield,
    pub hull: Hull,
    pub phaser: Phaser,
    pub torpedo: Torpedo,
}
