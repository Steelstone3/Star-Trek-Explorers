package ships

import (
	"testing"

	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/identifications"
	"github.com/stretchr/testify/assert"
)

func TestConstructFederationShip(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := constructFederationShip()

	// When
	result := ConstructFederationShip(seed)

	// Then
	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestConstructKlingonShip(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := constructKlingonShip()

	// When
	result := ConstructKlingonShip(seed)

	// Then
	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestConstructAiKlingonShip(t *testing.T) {
	// Given
	var seed int64 = 1234
	expected := constructAiKlingonShip()

	// When
	result := ConstructAiKlingonShip(seed)

	// Then
	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestTakeDamageToShip(t *testing.T) {
	// Given
	var damage uint = 10
	var expectedShield uint = 90
	var expectedHull uint = 100
	attackingShip := constructFederationShip()
	attackingShip.Capabilities.Phaser.Damage = damage
	attackingShip.Capabilities.Torpedo.Damage = damage
	defendingShip := constructKlingonShip()

	// When
	defendingShip = attackingShip.AttackHostileShip(defendingShip)

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestTakeHullDamageToShip(t *testing.T) {
	// Given
	var damage uint = 10
	var expectedShield uint
	var expectedHull uint = 80
	attackingShip := constructFederationShip()
	attackingShip.Capabilities.Phaser.Damage = damage
	attackingShip.Capabilities.Torpedo.Damage = damage
	defendingShip := constructKlingonShip()
	defendingShip.Capabilities.Shield.CurrentShieldStrength = 0

	// When
	defendingShip = attackingShip.AttackHostileShip(defendingShip)

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCriticalDamageToShipShields(t *testing.T) {
	// Given
	var phaserDamage uint = 101
	var torpedoDamage uint = 10
	var expectedShield uint
	var expectedHull uint = 89
	attackingShip := constructFederationShip()
	attackingShip.Capabilities.Phaser.Damage = phaserDamage
	attackingShip.Capabilities.Torpedo.Damage = torpedoDamage
	defendingShip := constructKlingonShip()

	// When
	defendingShip = attackingShip.AttackHostileShip(defendingShip)

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCriticalDamageToShip(t *testing.T) {
	// Given
	var damage uint = 101
	var expectedShield uint
	var expectedHull uint
	attackingShip := constructFederationShip()
	attackingShip.Capabilities.Phaser.Damage = damage
	attackingShip.Capabilities.Torpedo.Damage = damage
	defendingShip := constructKlingonShip()

	// When
	defendingShip = attackingShip.AttackHostileShip(defendingShip)

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestTakeDamageToShipAcceptance(t *testing.T) {
	// Given
	var damage uint = 10
	var expectedShield uint
	var expectedHull uint = 20
	attackingShip := constructFederationShip()
	attackingShip.Capabilities.Phaser.Damage = damage
	attackingShip.Capabilities.Torpedo.Damage = damage
	defendingShip := constructKlingonShip()

	// When
	for i := 0; i < 14; i++ {
		defendingShip = attackingShip.AttackHostileShip(defendingShip)
	}

	// Then
	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestConstructRandomFederationShips(t *testing.T) {
	// When
	federationShips := ConstructRandomFederationShips()

	// Then
	assert.NotEmpty(t, federationShips)
}

func TestConstructRandomAiKlingonShips(t *testing.T) {
	// When
	AiKlingonShips := ConstructRandomAiKlingonShips()

	// Then
	assert.NotEmpty(t, AiKlingonShips)
}

func TestIsDestroyed(t *testing.T) {
	// Given
	ship := constructFederationShip()
	ship.Capabilities.Shield.CurrentShieldStrength = 0
	ship.Capabilities.Hull.CurrentStructuralIntegrity = 0

	// When
	isDestroyed := ship.IsDestroyed()

	// Then
	assert.True(t, isDestroyed)
}

func TestIsOperational(t *testing.T) {
	// Given
	ship := constructFederationShip()

	// When
	isDestroyed := ship.IsDestroyed()

	// Then
	assert.False(t, isDestroyed)
}

func constructFederationShip() Ship {
	return Ship{
		Identification: identifications.ShipIdentification{
			ShipName:     identifications.ShipName{Name: "Monitor"},
			ShipClass:    identifications.ShipClass{Class: "Defiant"},
			SerialNumber: identifications.SerialNumber{SerialNumber: "USS-37482"},
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

func constructKlingonShip() Ship {
	return Ship{
		Identification: identifications.ShipIdentification{
			ShipName:     identifications.ShipName{Name: "Pagh"},
			ShipClass:    identifications.ShipClass{Class: "B'rel"},
			SerialNumber: identifications.SerialNumber{SerialNumber: "IKS-37482"},
			Faction:      identifications.Faction{Name: "Klingon Empire"},
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

func constructAiKlingonShip() Ship {
	ship := constructKlingonShip()
	ship.Capabilities.Phaser.Damage = 2
	ship.Capabilities.Torpedo.Damage = 2

	return ship
}
