package indentifications

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"testing"
)

func TestConstructFederationShipIdentification(t *testing.T) {
	result := ConstructFederationShipIdentification()
	expected := ShipIdentification{
		Name:         "Enterprise",
		Class:        "Galaxy",
		SerialNumber: "NCC-1701",
		Faction:      "Federation",
	}

	asserters.AssertEqual(t, expected.Name, result.Name)
	asserters.AssertEqual(t, expected.Class, result.Class)
	asserters.AssertEqual(t, expected.Faction, result.Faction)
	asserters.AssertEqual(t, expected.SerialNumber, result.SerialNumber)
}

func TestConstructKlingonShipIdentification(t *testing.T) {
	result := ConstructKlingonShipIdentification()
	expected := ShipIdentification{
		Name:         "Pagh",
		Class:        "Sompek",
		SerialNumber: "IKS-2359",
		Faction:      "Klingon Empire",
	}

	asserters.AssertEqual(t, expected.Name, result.Name)
	asserters.AssertEqual(t, expected.Class, result.Class)
	asserters.AssertEqual(t, expected.Faction, result.Faction)
	asserters.AssertEqual(t, expected.SerialNumber, result.SerialNumber)
}
