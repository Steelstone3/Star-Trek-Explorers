package ships

import (
	"github.com/stretchr/testify/assert"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"testing"
)

func TestConstructFederationShip(t *testing.T) {
	result := ConstructFederationShip()
	expected := Ship{
		Identification: indentifications.ShipIdentification{
			Name:         "Enterprise",
			Class:        "Galaxy",
			SerialNumber: "NCC-1701",
			Faction:      "Federation",
		},
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

	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestConstructKlingonShip(t *testing.T) {
	result := ConstructKlingonShip()
	expected := Ship{
		Identification: indentifications.ShipIdentification{
			Name:         "Pagh",
			Class:        "Sompek",
			SerialNumber: "IKS-2359",
			Faction:      "Klingon Empire",
		},
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

	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}
