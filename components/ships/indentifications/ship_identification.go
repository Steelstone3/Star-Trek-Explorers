package indentifications

type ShipIdentification struct {
	//TODO Random Name, Class and Serial Number system for all Factions
	Name         string
	Class        string
	SerialNumber string
	Faction      string
}

func ConstructFederationShipIdentification() ShipIdentification {
	return ShipIdentification{
		Name:         "Enterprise",
		Class:        "Galaxy",
		SerialNumber: "NCC-1701",
		Faction:      "Federation",
	}
}

func ConstructKlingonShipIdentification() ShipIdentification {
	return ShipIdentification{
		Name:         "Pagh",
		Class:        "Sompek",
		SerialNumber: "IKS-2359",
		Faction:      "Klingon Empire",
	}
}
