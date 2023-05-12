package ships

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"testing"
)

func TestShipConstruction(t *testing.T) {
	result := ConstructFederationShip()
	expected := Ship{
		ship_identification: indentifications.ShipIdentification{
			Name:         "Enterprise",
			Class:        "Galaxy",
			SerialNumber: "NCC-1701",
			Faction:      "Federation",
		},
	}

	asserters.AssertEqual(t, expected.ship_identification, result.ship_identification)
}
