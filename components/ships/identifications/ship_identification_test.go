package identifications

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructFederationShipIdentification(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipIdentification{
		ShipName:     ShipName{"Monitor"},
		ShipClass:    ShipClass{"Defiant"},
		SerialNumber: SerialNumber{"USS-37482"},
		Faction:      Faction{Name: "Federation"},
	}

	// When
	result := ConstructFederationShipIdentification(seed)

	// Then
	assert.Equal(t, expected.ShipName, result.ShipName)
	assert.Equal(t, expected.ShipClass, result.ShipClass)
	assert.Equal(t, expected.Faction, result.Faction)
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}

func TestConstructKlingonShipIdentification(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := ShipIdentification{
		ShipName:     ShipName{"Pagh"},
		ShipClass:    ShipClass{"B'rel"},
		SerialNumber: SerialNumber{"IKS-37482"},
		Faction:      Faction{Name: "Klingon Empire"},
	}

	// When
	result := ConstructKlingonShipIdentification(seed)

	// Then
	assert.Equal(t, expected.ShipName, result.ShipName)
	assert.Equal(t, expected.ShipClass, result.ShipClass)
	assert.Equal(t, expected.Faction, result.Faction)
	assert.Equal(t, expected.SerialNumber, result.SerialNumber)
}
