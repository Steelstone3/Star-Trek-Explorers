package indentifications

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstruction(t *testing.T) {
	result := ConstructFederation()
	expected := ShipIdentification{
		Name:         "Enterprise",
		Class:        "Galaxy",
		SerialNumber: "NCC-1701",
		Faction:      "Federation",
	}

	asserters.AssertEqual(expected, result)
}
