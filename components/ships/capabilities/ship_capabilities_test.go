package capabilities

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructShipCapabilities(t *testing.T) {
	// Given
	expected := constructShipCapabilities()

	// When
	result := ConstructShipCapabilities()

	// Then
	assert.Equal(t, expected.Shield, result.Shield)
	assert.Equal(t, expected.Hull, result.Hull)
	assert.Equal(t, expected.Phaser, result.Phaser)
	assert.Equal(t, expected.Torpedo, result.Torpedo)
}

func TestConstructAiShipCapabilities(t *testing.T) {
	// Given
	expected := constructAiShipCapabilities()

	// When
	result := ConstructAiShipCapabilities()

	// Then
	assert.Equal(t, expected.Shield, result.Shield)
	assert.Equal(t, expected.Hull, result.Hull)
	assert.Equal(t, expected.Phaser, result.Phaser)
	assert.Equal(t, expected.Torpedo, result.Torpedo)
}

func constructShipCapabilities() ShipCapabilities {
	return ShipCapabilities{
		Shield: Shield{
			Regeneration:          5,
			CurrentShieldStrength: 100,
			MaximumShieldStrength: 100,
		},
		Hull: Hull{
			RepairRate:                 5,
			CurrentStructuralIntegrity: 100,
			MaximumStructuralIntegrity: 100,
		},
		Phaser: Phaser{
			Damage: 10,
		},
		Torpedo: Torpedo{
			Damage: 10,
		},
	}
}

func constructAiShipCapabilities() ShipCapabilities {
	shipCapabilities := constructShipCapabilities()
	shipCapabilities.Phaser.Damage = 2
	shipCapabilities.Torpedo.Damage = 2

	return shipCapabilities
}
