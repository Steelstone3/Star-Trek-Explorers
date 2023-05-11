package ships

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"testing"
)

func TestConstruction(t *testing.T) {
	result := ConstructFederation()
	expected := Ship{
		ship_identification: indentifications.ShipIdentification{
			Name:         "Enterprise",
			Class:        "Galaxy",
			SerialNumber: "NCC-1709",
			Faction:      "Federation",
		},
	}

	asserters.AssertEqual(expected, result)
}
