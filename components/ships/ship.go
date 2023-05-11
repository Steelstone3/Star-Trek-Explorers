package ships

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
)

type Ship struct {
	ship_identification indentifications.ShipIdentification
}

func ConstructFederationShip() Ship {
	return Ship{
		ship_identification: indentifications.ConstructFederationShipIdentification(),
	}
}
