package entities

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructFederationShip(t *testing.T) {
	result := ConstructGame()
	expected := constructGame()

	assert.Equal(t, expected, result)
}

func constructGame() Game {
	return Game{
		PlayerShip: constructFederationShip(),
		FederationShips: []ships.Ship{
			constructFederationShip(),
			constructFederationShip(),
			constructFederationShip(),
			constructFederationShip(),
			constructFederationShip(),
		},
		KlingonShips: []ships.Ship{
			constructAiKlingonShip(),
			constructAiKlingonShip(),
			constructAiKlingonShip(),
			constructAiKlingonShip(),
			constructAiKlingonShip(),
		},
	}
}

func constructFederationShip() ships.Ship {
	return ships.Ship{
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
}

func constructAiKlingonShip() ships.Ship {
	return ships.Ship{
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
				Damage: 2,
			},
			Torpedo: capabilities.Torpedo{
				Damage: 2,
			},
		},
	}
}
