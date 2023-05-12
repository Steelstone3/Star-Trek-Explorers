package indentifications

type ShipIdentification struct {
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
