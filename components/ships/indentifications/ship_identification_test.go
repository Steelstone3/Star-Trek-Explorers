package indentifications

import (
	"github.com/stretchr/testify/assert"
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

	assert.Equal(t, expected.Name, result.Name)
	assert.Equal(t, expected.Class, result.Class)
	assert.Equal(t, expected.Faction, result.Faction)
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}

func TestConstructKlingonShipIdentification(t *testing.T) {
	result := ConstructKlingonShipIdentification()
	expected := ShipIdentification{
		Name:         "Pagh",
		Class:        "Sompek",
		SerialNumber: "IKS-2359",
		Faction:      "Klingon Empire",
	}

	assert.Equal(t, expected.Name, result.Name)
	assert.Equal(t, expected.Class, result.Class)
	assert.Equal(t, expected.Faction, result.Faction)
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}
