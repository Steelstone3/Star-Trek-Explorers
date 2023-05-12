package ships

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/asserters"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
)

func TestShipConstruction(t *testing.T) {
	result := ConstructFederationShip()
	expected := Ship{
		Identification: indentifications.ShipIdentification{Name: "Enterprise", Class: "Galaxy", SerialNumber: "NCC-1701", Faction: "Federation"},
		Capabilities: capabilities.ShipCapabilities{
			Shield: capabilities.Shield{
				Regeneration:          5,
				CurrentShieldStrength: 100,
				MaximumShieldStrength: 100,
			},
			Hull: capabilities.Hull{
				RepairRate:                 5,
				CurrentStructuralIntegrity: 100,
				MaximumStructuralIntegrity: 100,
			},
			Phaser: capabilities.Phaser{
				Damage: 10,
			},
			Torpedo: capabilities.Torpedo{
				Damage: 10,
			},
		},
	}

	asserters.AssertEqual(t, expected.Identification, result.Identification)
	asserters.AssertEqual(t, expected.Capabilities, result.Capabilities)
}
