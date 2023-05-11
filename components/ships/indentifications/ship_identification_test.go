package indentifications

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
)

func TestShipIdentificationConstruction(t *testing.T) {
	result := ConstructFederationShipIdentification()
	expected := ShipIdentification{
		Name:         "Enterprise",
		Class:        "Galaxy",
		SerialNumber: "NCC-1701",
		Faction:      "Federation",
	}

	asserters.AssertEqual(expected.Name, result.Name)
	asserters.AssertEqual(expected.Class, result.Class)
	asserters.AssertEqual(expected.Faction, result.Faction)
	asserters.AssertEqual(expected.SerialNumber, result.SerialNumber)
}
