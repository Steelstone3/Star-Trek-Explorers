package capabilities

type ShipCapabilities struct {
	Shield  Shield
	Hull    Hull
	Phaser  Phaser
	Torpedo Torpedo
}

func ConstructShipCapabilities() ShipCapabilities {
	return ShipCapabilities{
		Shield:  ConstructShield(),
		Hull:    ConstructHull(),
		Phaser:  ConstructPhaser(),
		Torpedo: ConstructTorpedo(),
	}
}
