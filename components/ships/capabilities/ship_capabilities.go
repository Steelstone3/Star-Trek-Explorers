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

func ConstructAiShipCapabilities() ShipCapabilities {
	return ShipCapabilities{
		Shield:  ConstructShield(),
		Hull:    ConstructHull(),
		Phaser:  ConstructAiPhaser(),
		Torpedo: ConstructAiTorpedo(),
	}
}