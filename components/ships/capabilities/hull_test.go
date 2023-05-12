package capabilities

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestConstructHull(t *testing.T) {
	result := ConstructHull()
	expected := constructHull()

	assert.Equal(t, expected.RepairRate, result.RepairRate)
	assert.Equal(t, expected.CurrentStructuralIntegrity, result.CurrentStructuralIntegrity)
	assert.Equal(t, expected.MaximumStructuralIntegrity, result.MaximumStructuralIntegrity)
}

func TestTakeHullDamage(t *testing.T) {
	var damage uint = 10
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 90
	hull := constructHull()

	hull = hull.TakeHullDamage(shieldStrength, damage)

	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeCriticalHullDamage(t *testing.T) {
	var damage uint = 101
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 0
	hull := constructHull()

	hull = hull.TakeHullDamage(shieldStrength, damage)

	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeNoHullDamage(t *testing.T) {
	var damage uint = 101
	var shieldStrength uint = 1
	var expectedRemainingHull uint = 100
	hull := constructHull()

	hull = hull.TakeHullDamage(shieldStrength, damage)

	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func TestTakeHullDamageAcceptance(t *testing.T) {
	var damage uint = 10
	var shieldStrength uint = 0
	var expectedRemainingHull uint = 50
	hull := constructHull()

	hull = hull.TakeHullDamage(shieldStrength, damage)
	hull = hull.TakeHullDamage(shieldStrength, damage)
	hull = hull.TakeHullDamage(shieldStrength, damage)
	hull = hull.TakeHullDamage(shieldStrength, damage)
	hull = hull.TakeHullDamage(shieldStrength, damage)

	assert.Equal(t, expectedRemainingHull, hull.CurrentStructuralIntegrity)
}

func constructHull() Hull {
	return Hull{
		RepairRate:                 5,
		CurrentStructuralIntegrity: 100,
		MaximumStructuralIntegrity: 100,
	}
}