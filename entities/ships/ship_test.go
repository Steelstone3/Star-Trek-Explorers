package ships

import (
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/capabilities"
	"github.com/Steelstone3/Star-Trek-Explorers/components/ships/indentifications"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructFederationShip(t *testing.T) {
	result := ConstructFederationShip()
	expected := constructFederationShip()

	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestConstructKlingonShip(t *testing.T) {
	result := ConstructKlingonShip()
	expected := constructKlingonShip()

	assert.Equal(t, expected.Identification, result.Identification)
	assert.Equal(t, expected.Capabilities, result.Capabilities)
}

func TestTakeDamageToShip(t *testing.T) {
	var damage uint = 10
	var expectedShield uint = 90
	var expectedHull uint = 100
	ship := constructKlingonShip()

	ship = ship.TakeDamageToShip(damage)

	assert.Equal(t, expectedShield, ship.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, ship.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestTakeHullDamageToShip(t *testing.T) {
	var damage uint = 10
	var expectedShield uint = 0
	var expectedHull uint = 90
	ship := constructKlingonShip()
	ship.Capabilities.Shield.CurrentShieldStrength = 0

	ship = ship.TakeDamageToShip(damage)

	assert.Equal(t, expectedShield, ship.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, ship.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCriticalDamageToShipShields(t *testing.T) {
	var damage uint = 101
	var expectedShield uint = 0
	var expectedHull uint = 99
	ship := constructKlingonShip()

	ship = ship.TakeDamageToShip(damage)

	assert.Equal(t, expectedShield, ship.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, ship.Capabilities.Hull.CurrentStructuralIntegrity)
}

func TestCriticalDamageToShip(t *testing.T) {
	var damage uint = 201
	var expectedShield uint = 0
	var expectedHull uint = 0
	ship := constructKlingonShip()

	ship = ship.TakeDamageToShip(damage)

	assert.Equal(t, expectedShield, ship.Capabilities.Shield.CurrentShieldStrength)
	assert.Equal(t, expectedHull, ship.Capabilities.Hull.CurrentStructuralIntegrity)
}

// func TestAttackHostileShip(t *testing.T) {
// 	var expectedShield uint = 90
// 	var expectedHull uint = 100
// 	attackingShip := constructFederationShip()
// 	defendingShip := constructKlingonShip()

// 	defendingShip = attackingShip.AttackHostileShip(defendingShip)

// 	assert.Equal(t, expectedShield, defendingShip.Capabilities.Shield.CurrentShieldStrength)
// 	assert.Equal(t, expectedHull, defendingShip.Capabilities.Hull.CurrentStructuralIntegrity)
// }

func constructFederationShip() Ship {
	return Ship{
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

func constructKlingonShip() Ship {
	return Ship{
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
}
