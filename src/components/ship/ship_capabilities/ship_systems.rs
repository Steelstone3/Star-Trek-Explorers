use rand_derive2::RandGen;
use super::{shield::Shield, hull::Hull, phaser::Phaser, torpedo::Torpedo};

#[derive(PartialEq, Debug, RandGen)]
pub struct ShipSystems {
    pub shield: Shield,
    pub hull: Hull,
    pub phaser: Phaser,
    pub torpedo: Torpedo,
}