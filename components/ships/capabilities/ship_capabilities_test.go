package capabilities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
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

	asserters.AssertEqual(t, expected.Shield, result.Shield)
	asserters.AssertEqual(t, expected.Hull, result.Hull)
	asserters.AssertEqual(t, expected.Phaser, result.Phaser)
	asserters.AssertEqual(t, expected.Torpedo, result.Torpedo)
}
