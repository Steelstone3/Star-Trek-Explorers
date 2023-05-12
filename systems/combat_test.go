package systems

import (
	"github.com/stretchr/testify/assert"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"testing"
)

func TestCombatTurn(t *testing.T) {
	var expectedShield uint = 90
	var expectedHull uint = 100
	attackingShip := constructShip()
	defendingShip := constructShip()

	defendingShip = CombatTurn(attackingShip, defendingShip)
	
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCombatTurnAcceptance(t *testing.T) {
	var expectedShield uint = 0
	var expectedHull uint = 20
	attackingShip := constructShip()
	defendingShip := constructShip()

	for i := 0; i < 14; i++ {
		defendingShip = CombatTurn(attackingShip, defendingShip)
	}
	
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func constructShip() ships.Ship {
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