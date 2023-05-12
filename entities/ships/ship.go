package ships

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
)

type Ship struct {
	Identification indentifications.ShipIdentification
	Capabilities   capabilities.ShipCapabilities
}

func ConstructFederationShip() Ship {
	return Ship{
		Identification: indentifications.ConstructFederationShipIdentification(),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}

func ConstructKlingonShip() Ship {
	return Ship{
		Identification: indentifications.ConstructKlingonShipIdentification(),
		Capabilities:   capabilities.ConstructShipCapabilities(),
	}
}
