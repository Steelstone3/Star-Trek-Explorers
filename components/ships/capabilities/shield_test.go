package capabilities

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestConstructShield(t *testing.T) {
	result := ConstructShield()
	expected := constructShield()

	assert.Equal(t, expected.Regeneration, result.Regeneration)
	assert.Equal(t, expected.CurrentShieldStrength, result.CurrentShieldStrength)
	assert.Equal(t, expected.MaximumShieldStrength, result.MaximumShieldStrength)
}

func TestTakeShieldDamage(t *testing.T) {
	var damage uint = 10
	var expectedRemainingShield uint = 90
	shield := constructShield()

	shield = shield.TakeShieldDamage(damage)

	assert.Equal(t, expectedRemainingShield, shield.CurrentShieldStrength)
}

func TestTakeCriticalShieldDamage(t *testing.T) {
	var damage uint = 101
	var expectedRemainingShield uint = 0
	shield := constructShield()

	shield = shield.TakeShieldDamage(damage)

	assert.Equal(t, expectedRemainingShield, shield.CurrentShieldStrength)
}

func constructShield() Shield {
	return Shield{
		Regeneration:          5,
		CurrentShieldStrength: 100,
		MaximumShieldStrength: 100,
	}
}
