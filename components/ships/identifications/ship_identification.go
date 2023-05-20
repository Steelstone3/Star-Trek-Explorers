package identifications

type ShipIdentification struct {
	ShipName     ShipName
	ShipClass    ShipClass
	SerialNumber SerialNumber
	Faction      Faction
}

func ConstructFederationShipIdentification(seed int64) ShipIdentification {
	return ShipIdentification{
		ShipName:     constructFederationShipName(seed),
		ShipClass:    constructFederationShipClass(seed),
		SerialNumber: constructFederationSerialNumber(seed),
		Faction:      constructFederationFaction(),
	}
}

func ConstructKlingonShipIdentification(seed int64) ShipIdentification {
	return ShipIdentification{
		ShipName:     constructKlingonShipName(seed),
		ShipClass:    constructKlingonShipClass(seed),
		SerialNumber: constructKlingonSerialNumber(seed),
		Faction:      constructKlingonFaction(),
	}
}
