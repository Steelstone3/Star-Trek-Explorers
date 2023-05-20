package systems

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/identifications"
	"github.com/Steelstone3/Star-Trek-Explorers/entities"
	"github.com/Steelstone3/Star-Trek-Explorers/entities/ships"
	"github.com/stretchr/testify/assert"
)

func TestEndedStartCombatPlayer(t *testing.T) {
	// Given
	var criticalDamage uint
	playerShip := constructShip()
	aiShip := constructShip()
	aiShip.Capabilities.Shield.CurrentShieldStrength = criticalDamage
	aiShip.Capabilities.Hull.CurrentStructuralIntegrity = criticalDamage
	expectedGame := entities.Game{
		PlayerShip:   playerShip,
		KlingonShips: []ships.Ship{aiShip},
	}

	// When
	game, isInCombat := StartCombat(expectedGame)

	// Then
	assert.Equal(t, false, isInCombat)
	assert.Equal(t, playerShip, game.PlayerShip)
	assert.Equal(t, 0, len(game.KlingonShips))
}

func TestEndedStartCombatOpponent(t *testing.T) {
	// Given
	var criticalDamage uint
	aiShip := constructShip()
	playerShip := constructShip()
	playerShip.Capabilities.Shield.CurrentShieldStrength = criticalDamage
	playerShip.Capabilities.Hull.CurrentStructuralIntegrity = criticalDamage
	expectedGame := entities.Game{
		PlayerShip:   playerShip,
		KlingonShips: []ships.Ship{aiShip},
	}

	// When
	game, isInCombat := StartCombat(expectedGame)

	// Then
	assert.Equal(t, false, isInCombat)
	assert.Equal(t, playerShip, game.PlayerShip)
	assert.Equal(t, 1, len(game.KlingonShips))
}

func TestFilterCombatReadyShips(t *testing.T) {
	// Given
	ships := []ships.Ship{
		constructShip(),
		constructShip(),
		constructShip(),
	}

	// When
	filteredShips := filterOutDestroyedShips(ships)

	// Then
	assert.Equal(t, 3, len(filteredShips))
}

func TestFilterDestroyedShips(t *testing.T) {
	// Given
	ships := []ships.Ship{
		constructDestroyedShip(),
		constructShip(),
		constructShip(),
	}

	// When
	filteredShips := filterOutDestroyedShips(ships)

	// Then
	assert.Equal(t, 2, len(filteredShips))
}

func TestCombatTurn(t *testing.T) {
	// Given
	var expectedShield uint = 90
	var expectedHull uint = 100
	attackingShip := constructShip()
	defendingShip := constructShip()

	// When
	defendingShip = combatTurn(attackingShip, defendingShip)

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCombatTurnAcceptance(t *testing.T) {
	// Given
	var expectedShield uint
	var expectedHull uint = 20
	attackingShip := constructShip()
	defendingShip := constructShip()

	// When
	for i := 0; i < 14; i++ {
		defendingShip = combatTurn(attackingShip, defendingShip)
	}

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func constructShip() ships.Ship {
	return ships.Ship{
		Identification: identifications.ShipIdentification{
			ShipName:     identifications.ShipName{Name: "Enterprise"},
			ShipClass:    identifications.ShipClass{Class: "Defiant"},
			SerialNumber: identifications.SerialNumber{SerialNumber: "USS-1701"},
			Faction:      identifications.Faction{Name: "Federation"},
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

func constructDestroyedShip() ships.Ship {
	ship := constructShip()

	ship.Capabilities.Shield.CurrentShieldStrength = 0
	ship.Capabilities.Hull.CurrentStructuralIntegrity = 0

	return ship
}
