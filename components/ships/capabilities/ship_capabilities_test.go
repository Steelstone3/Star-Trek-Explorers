package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructShipCapabilities(t *testing.T) {
	result := ConstructShipCapabilities()
	expected := ShipCapabilities{
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

	assert.Equal(t, expected.Shield, result.Shield)
	assert.Equal(t, expected.Hull, result.Hull)
	assert.Equal(t, expected.Phaser, result.Phaser)
	assert.Equal(t, expected.Torpedo, result.Torpedo)
}
