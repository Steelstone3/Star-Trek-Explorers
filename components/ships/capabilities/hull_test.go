package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructHull(t *testing.T) {
	// Given
	expected := constructHull()

	// When
	result := ConstructHull()

	// Then
	assert.Equal(t, expected.RepairRate, result.RepairRate)
	assert.Equal(t, expected.CurrentStructuralIntegrity, result.CurrentStructuralIntegrity)
	assert.Equal(t, expected.MaximumStructuralIntegrity, result.MaximumStructuralIntegrity)
}

func TestTakeHullDamage(t *testing.T) {
	// Given
	var damage uint = 10
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 90
	hull := constructHull()

	// When
	hull = hull.TakeHullDamage(shieldStrength, damage)

	// Then
	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeCriticalHullDamage(t *testing.T) {
	// Given
	var damage uint = 101
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 0
	hull := constructHull()

	// When
	hull = hull.TakeHullDamage(shieldStrength, damage)

	// Then
	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeNoHullDamage(t *testing.T) {
	// Given
	var damage uint = 101
	var shieldStrength uint = 1
	var expectedRemainingHull uint = 100
	hull := constructHull()

	// When
	hull = hull.TakeHullDamage(shieldStrength, damage)

	// Then
	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeHullDamageAcceptance(t *testing.T) {
	// Given
	var damage uint = 10
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 50
	hull := constructHull()

	// When
	for i := 0; i < 5; i++ {		
		hull = hull.TakeHullDamage(shieldStrength, damage)
	}

	// Then
	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func constructHull() Hull {
	return Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		MaximumStructuralIntegrity: 100,
	}
}